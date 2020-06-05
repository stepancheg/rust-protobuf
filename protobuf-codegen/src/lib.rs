#![deny(intra_doc_link_resolution_failure)]

extern crate proc_macro;
extern crate protobuf;

use std::collections::hash_map::HashMap;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

use protobuf::descriptor::*;
use protobuf::Message;

use protobuf::prelude::*;

mod amend_io_error_util;
pub mod case_convert;
mod compiler_plugin;
mod customize;
mod enums;
mod extensions;
mod field;
mod file;
pub(crate) mod file_and_mod;
mod file_descriptor;
mod inside;
mod map;
mod message;
mod oneof;
mod protobuf_name;
mod rust_name;
mod rust_types_values;
mod serde;
mod well_known_types;

pub(crate) mod rust;
pub(crate) mod scope;
pub(crate) mod strx;
pub(crate) mod syntax;

use customize::customize_from_rustproto_for_file;
pub use customize::Customize;

pub mod code_writer;

use self::code_writer::CodeWriter;
use self::enums::*;
use self::extensions::*;
use self::message::*;
#[doc(hidden)]
pub use amend_io_error_util::amend_io_error;
use file::proto_path_to_rust_mod;
use map::map_entry;
use scope::FileScope;
use scope::RootScope;

use inside::protobuf_crate_path;
pub use protobuf_name::ProtobufAbsolutePath;
pub use protobuf_name::ProtobufIdent;
pub use protobuf_name::ProtobufRelativePath;

fn escape_byte(s: &mut String, b: u8) {
    if b == b'\n' {
        write!(s, "\\n").unwrap();
    } else if b == b'\r' {
        write!(s, "\\r").unwrap();
    } else if b == b'\t' {
        write!(s, "\\t").unwrap();
    } else if b == b'\\' || b == b'"' {
        write!(s, "\\{}", b as char).unwrap();
    } else if b == b'\0' {
        write!(s, "\\0").unwrap();
    // ASCII printable except space
    } else if b > 0x20 && b < 0x7f {
        write!(s, "{}", b as char).unwrap();
    } else {
        write!(s, "\\x{:02x}", b).unwrap();
    }
}

fn write_file_descriptor_data(
    file: &FileDescriptorProto,
    customize: &Customize,
    w: &mut CodeWriter,
) {
    let fdp_bytes = file.write_to_bytes().unwrap();
    w.write_line("static file_descriptor_proto_data: &'static [u8] = b\"\\");
    w.indented(|w| {
        const MAX_LINE_LEN: usize = 72;

        let mut s = String::new();
        for &b in &fdp_bytes {
            let prev_len = s.len();
            escape_byte(&mut s, b);
            let truncate = s.len() > MAX_LINE_LEN;
            if truncate {
                s.truncate(prev_len);
            }
            if truncate || s.len() == MAX_LINE_LEN {
                write!(s, "\\").unwrap();
                w.write_line(&s);
                s.clear();
            }
            if truncate {
                escape_byte(&mut s, b);
            }
        }
        if !s.is_empty() {
            write!(s, "\\").unwrap();
            w.write_line(&s);
            s.clear();
        }
    });
    w.write_line("\";");
    w.write_line("");
    w.lazy_static(
        "file_descriptor_proto_lazy",
        &format!(
            "{}::descriptor::FileDescriptorProto",
            protobuf_crate_path(customize)
        ),
        protobuf_crate_path(customize),
    );
    w.write_line("");
    w.def_fn(
        &format!(
            "parse_descriptor_proto() -> {}::descriptor::FileDescriptorProto",
            protobuf_crate_path(customize)
        ),
        |w| {
            w.write_line(&format!(
                "{}::parse_from_bytes(file_descriptor_proto_data).unwrap()",
                protobuf_crate_path(customize)
            ));
        },
    );
    w.write_line("");
    w.write_line("/// `FileDescriptorProto` object which was a source for this generated file");
    w.pub_fn(
        &format!(
            "file_descriptor_proto() -> &'static {}::descriptor::FileDescriptorProto",
            protobuf_crate_path(customize)
        ),
        |w| {
            w.block("file_descriptor_proto_lazy.get(|| {", "})", |w| {
                w.write_line("parse_descriptor_proto()");
            });
        },
    );
}

fn gen_file(
    file: &FileDescriptorProto,
    _files_map: &HashMap<&Path, &FileDescriptorProto>,
    root_scope: &RootScope,
    customize: &Customize,
    parser: &str,
) -> Option<compiler_plugin::GenResult> {
    // TODO: use it
    let mut customize = customize.clone();
    // options specified in invocation have precedence over options specified in file
    customize.update_with(&customize_from_rustproto_for_file(
        file.options.get_message(),
    ));

    let scope = FileScope {
        file_descriptor: file,
    }
    .to_scope();
    let lite_runtime = customize.lite_runtime.unwrap_or_else(|| {
        file.options.get_message().get_optimize_for() == file_options::OptimizeMode::LITE_RUNTIME
    });

    let mut v = Vec::new();

    {
        let mut w = CodeWriter::new(&mut v);

        w.write_generated_by("rust-protobuf", env!("CARGO_PKG_VERSION"), parser);

        w.write_line("");
        w.write_line(&format!("//! Generated file from `{}`", file.get_name()));
        if customize.inside_protobuf != Some(true) {
            w.write_line("");
            w.write_line("/// Generated files are compatible only with the same version");
            w.write_line("/// of protobuf runtime.");
            w.write_line(&format!(
                "const _PROTOBUF_VERSION_CHECK: () = {}::{};",
                protobuf_crate_path(&customize),
                protobuf::VERSION_IDENT
            ));
        }

        static NESTED_TYPE_NUMBER: protobuf::rt::Lazy<i32> = protobuf::rt::Lazy::INIT;
        let message_type_number = *NESTED_TYPE_NUMBER.get(|| {
            protobuf::reflect::MessageDescriptor::for_type::<FileDescriptorProto>()
                .get_field_by_name("message_type")
                .expect("`message_type` must exist")
                .proto()
                .get_number()
        });

        let mut path = vec![message_type_number, 0];
        for (id, message) in scope.get_messages().iter().enumerate() {
            // ignore map entries, because they are not used in map fields
            if map_entry(message).is_none() {
                path[1] = id as i32;

                w.write_line("");
                MessageGen::new(
                    message,
                    &root_scope,
                    &customize,
                    &path,
                    file.source_code_info.as_ref(),
                )
                .write(&mut w, &customize);
            }
        }

        static ENUM_TYPE_NUMBER: protobuf::rt::Lazy<i32> = protobuf::rt::Lazy::INIT;
        let enum_type_number = *ENUM_TYPE_NUMBER.get(|| {
            protobuf::reflect::MessageDescriptor::for_type::<FileDescriptorProto>()
                .get_field_by_name("enum_type")
                .expect("`enum_type` must exist")
                .proto()
                .get_number()
        });

        let mut path = vec![enum_type_number, 0];
        for (id, enum_type) in scope.get_enums().iter().enumerate() {
            path[1] = id as i32;

            w.write_line("");
            EnumGen::new(
                enum_type,
                &customize,
                root_scope,
                &path,
                file.source_code_info.as_ref(),
            )
            .write(&mut w, &customize);
        }

        write_extensions(file, &root_scope, &mut w, &customize);

        if !lite_runtime {
            w.write_line("");
            write_file_descriptor_data(file, &customize, &mut w);
        }
    }

    Some(compiler_plugin::GenResult {
        name: format!("{}.rs", proto_path_to_rust_mod(file.get_name())),
        content: v,
    })
}

// This function is also used externally by cargo plugin
// https://github.com/plietar/rust-protobuf-build
// So be careful changing its signature.
pub fn gen(
    file_descriptors: &[FileDescriptorProto],
    parser: &str,
    files_to_generate: &[PathBuf],
    customize: &Customize,
) -> Vec<compiler_plugin::GenResult> {
    let root_scope = RootScope {
        file_descriptors: file_descriptors,
    };

    let mut results: Vec<compiler_plugin::GenResult> = Vec::new();
    let files_map: HashMap<&Path, &FileDescriptorProto> = file_descriptors
        .iter()
        .map(|f| (Path::new(f.get_name()), f))
        .collect();

    for file_name in files_to_generate {
        let file = files_map.get(file_name.as_path()).expect(&format!(
            "file not found in file descriptors: {:?}, files: {:?}",
            file_name,
            files_map.keys()
        ));
        results.extend(gen_file(file, &files_map, &root_scope, customize, parser));
    }
    results
}

pub fn gen_and_write(
    file_descriptors: &[FileDescriptorProto],
    parser: &str,
    files_to_generate: &[PathBuf],
    out_dir: &Path,
    customize: &Customize,
) -> io::Result<()> {
    match out_dir.metadata() {
        Ok(m) => {
            if !m.is_dir() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("{} is not a directory", out_dir.display()),
                ));
            }
        }
        Err(e) => {
            return Err(amend_io_error(
                e,
                format!("{} does not exist or not accessible", out_dir.display()),
            ));
        }
    }

    let results = gen(file_descriptors, parser, files_to_generate, customize);

    for r in &results {
        let mut file_path = out_dir.to_owned();
        file_path.push(&r.name);
        let mut file_writer = File::create(&file_path)
            .map_err(|e| amend_io_error(e, format!("failed to create {:?}", file_path)))?;
        file_writer
            .write_all(&r.content)
            .map_err(|e| amend_io_error(e, format!("failed to write to {:?}", file_path)))?;
        file_writer
            .flush()
            .map_err(|e| amend_io_error(e, format!("failed to flush {:?}", file_path)))?;
    }

    Ok(())
}

pub fn protoc_gen_rust_main() {
    compiler_plugin::plugin_main(|r| {
        let customize = Customize::parse_from_parameter(r.parameter).expect("parse options");
        gen(
            r.file_descriptors,
            "protoc --rust-out=...",
            r.files_to_generate,
            &customize,
        )
    });
}

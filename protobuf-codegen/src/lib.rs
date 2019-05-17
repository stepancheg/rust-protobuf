extern crate protobuf;
extern crate proc_macro;

use std::collections::hash_map::HashMap;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io;
use std::io::Error;
use std::io::Write;
use std::path::{Path, PathBuf};

use protobuf::descriptor::*;
use protobuf::Message;

use protobuf::prelude::*;

mod amend_io_error_util;
mod compiler_plugin;
mod customize;
mod enums;
mod file;
mod extensions;
mod field;
mod rust_name;
mod protobuf_name;
mod map;
mod message;
mod oneof;
mod rust_types_values;
mod serde;
mod well_known_types;
pub mod case_convert;
mod file_descriptor;
pub(crate) mod file_and_mod;

pub(crate) mod scope;
pub(crate) mod syntax;
pub(crate) mod strx;
pub(crate) mod rust;

use customize::customize_from_rustproto_for_file;
pub use customize::Customize;

pub mod code_writer;

use self::code_writer::CodeWriter;
use self::enums::*;
use self::extensions::*;
use self::message::*;
#[doc(hidden)]
pub use amend_io_error_util::amend_io_error;
use map::map_entry;
use scope::RootScope;
use scope::FileScope;
use file::proto_path_to_rust_mod;

pub use protobuf_name::ProtobufIdent;
pub use protobuf_name::ProtobufAbsolutePath;
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

fn write_file_descriptor_data(file: &FileDescriptorProto, w: &mut CodeWriter) {
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
        "::protobuf::descriptor::FileDescriptorProto",
    );
    w.write_line("");
    w.def_fn(
        "parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto",
        |w| {
            w.write_line("::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()");
        },
    );
    w.write_line("");
    w.pub_fn(
        "file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto",
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
) -> Option<compiler_plugin::GenResult> {
    // TODO: use it
    let mut customize = customize.clone();
    // options specified in invocation have precedence over options specified in file
    customize.update_with(&customize_from_rustproto_for_file(
        file.options.get_message(),
    ));

    let scope = FileScope {
        file_descriptor: file,
    }.to_scope();
    let lite_runtime = customize.lite_runtime.unwrap_or_else(|| {
        file
            .options
            .get_message()
            .get_optimize_for()
            == file_options::OptimizeMode::LITE_RUNTIME
    });

    let mut v = Vec::new();

    {
        let mut w = CodeWriter::new(&mut v);

        w.write_generated_by("rust-protobuf", env!("CARGO_PKG_VERSION"));

        for message in &scope.get_messages() {
            // ignore map entries, because they are not used in map fields
            if map_entry(message).is_none() {
                w.write_line("");
                MessageGen::new(message, &root_scope, &customize).write(&mut w);
            }
        }
        for enum_type in &scope.get_enums() {
            w.write_line("");
            EnumGen::new(enum_type, &customize, root_scope).write(&mut w);
        }

        write_extensions(file, &root_scope, &mut w);

        if !lite_runtime {
            w.write_line("");
            write_file_descriptor_data(file, &mut w);
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
    files_to_generate: &[PathBuf],
    customize: &Customize,
) -> Vec<compiler_plugin::GenResult> {
    let root_scope = RootScope {
        file_descriptors: file_descriptors,
    };

    let mut results: Vec<compiler_plugin::GenResult> = Vec::new();
    let files_map: HashMap<&Path, &FileDescriptorProto> =
        file_descriptors.iter().map(|f| (Path::new(f.get_name()), f)).collect();

    for file_name in files_to_generate {
        let file = files_map.get(file_name.as_path()).expect(&format!(
            "file not found in file descriptors: {:?}, files: {:?}",
            file_name, files_map.keys()
        ));
        results.extend(gen_file(file, &files_map, &root_scope, customize));
    }
    results
}

pub fn gen_and_write(
    file_descriptors: &[FileDescriptorProto],
    files_to_generate: &[PathBuf],
    out_dir: &Path,
    customize: &Customize,
) -> io::Result<()> {
    let results = gen(file_descriptors, files_to_generate, customize);

    let out_dir_meta = std::fs::metadata(out_dir);
    if out_dir_meta.is_ok() {
        if out_dir_meta.unwrap().is_file() {
            return Err(Error::new(std::io::ErrorKind::AlreadyExists, "out_dir is a file"));
        }
    } else {
        std::fs::create_dir_all(out_dir);
    }

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
        gen(r.file_descriptors, r.files_to_generate, &customize)
    });
}

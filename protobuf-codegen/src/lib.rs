#![deny(rustdoc::broken_intra_doc_links)]

pub mod case_convert;
mod compiler_plugin;
mod customize;
mod enums;
mod extensions;
mod field;
pub(crate) mod file_and_mod;
mod inside;
mod map;
mod message;
mod oneof;
mod paths;
mod proto_path;
mod protobuf_abs_path;
mod protobuf_ident;
mod protobuf_path;
mod protobuf_rel_path;
pub(crate) mod rust;
mod rust_name;
mod rust_types_values;
pub(crate) mod scope;
mod serde;
pub(crate) mod strx;
pub(crate) mod syntax;
mod well_known_types;

pub mod code_writer;

use std::collections::hash_map::HashMap;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io;
use std::path::Path;

use customize::customize_from_rustproto_for_file;
pub use customize::Customize;
use inside::protobuf_crate_path;
#[doc(hidden)]
pub use paths::proto_name_to_rs;
pub use proto_path::ProtoPath;
pub use proto_path::ProtoPathBuf;
use protobuf::descriptor::*;
use protobuf::reflect::FileDescriptor;
use protobuf::Message;
pub use protobuf_abs_path::ProtobufAbsolutePath;
pub use protobuf_ident::ProtobufIdent;
pub use protobuf_path::ProtobufPath;
pub use protobuf_rel_path::ProtobufRelativePath;
use scope::FileScope;
use scope::RootScope;

use self::code_writer::CodeWriter;
use self::enums::*;
use self::extensions::*;
use self::message::*;
use crate::paths::proto_path_to_fn_file_descriptor;
use crate::paths::proto_path_to_rust_mod;
use crate::rust::EXPR_VEC_NEW;
use crate::scope::WithScope;
use crate::well_known_types::gen_well_known_types_mod;

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

fn write_file_descriptor(
    file_descriptor: &FileDescriptor,
    customize: &Customize,
    w: &mut CodeWriter,
) {
    w.write_line("/// `FileDescriptor` object which allows dynamic access to files");
    w.pub_fn(
        &format!(
            "file_descriptor() -> {}::reflect::FileDescriptor",
            protobuf_crate_path(customize)
        ),
        |w| {
            w.lazy_static(
                "file_descriptor_lazy",
                &format!(
                    "{}::reflect::GeneratedFileDescriptor",
                    protobuf_crate_path(customize)
                ),
                &format!("{}", protobuf_crate_path(customize)),
            );
            w.block(
                "let file_descriptor = file_descriptor_lazy.get(|| {",
                "});",
                |w| {
                    w.write_line(&format!("let mut deps = {};", EXPR_VEC_NEW));
                    for f in &file_descriptor.proto().dependency {
                        w.write_line(&format!(
                            "deps.push({}());",
                            proto_path_to_fn_file_descriptor(f, customize)
                        ));
                    }

                    let scope = FileScope { file_descriptor };

                    w.write_line(&format!("let mut messages = {};", EXPR_VEC_NEW));
                    for m in scope.find_messages_except_map() {
                        if m.is_map() {
                            continue;
                        }
                        w.write_line(&format!(
                            "messages.push({}::generated_message_descriptor_data());",
                            m.rust_name_to_file(),
                        ));
                    }

                    w.write_line(&format!("let mut enums = {};", EXPR_VEC_NEW));
                    for e in scope.find_enums() {
                        w.write_line(&format!(
                            "enums.push({}::generated_enum_descriptor_data());",
                            e.rust_name_to_file(),
                        ));
                    }

                    w.write_line(&format!(
                        "{}::reflect::GeneratedFileDescriptor::new_generated(",
                        protobuf_crate_path(&customize),
                    ));
                    w.indented(|w| {
                        w.write_line(&format!("file_descriptor_proto(),"));
                        w.write_line(&format!("deps,"));
                        w.write_line(&format!("messages,"));
                        w.write_line(&format!("enums,"));
                    });
                    w.write_line(")");
                },
            );
            w.write_line(&format!(
                "{}::reflect::FileDescriptor::new_generated_2(file_descriptor)",
                protobuf_crate_path(&customize),
            ));
        },
    );
}

fn write_file_descriptor_data(file: &FileDescriptor, customize: &Customize, w: &mut CodeWriter) {
    let fdp_bytes = file.proto().write_to_bytes().unwrap();
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
    w.write_line("/// `FileDescriptorProto` object which was a source for this generated file");
    w.pub_fn(
        &format!(
            "file_descriptor_proto() -> &'static {}::descriptor::FileDescriptorProto",
            protobuf_crate_path(customize)
        ),
        |w| {
            w.lazy_static(
                "file_descriptor_proto_lazy",
                &format!(
                    "{}::descriptor::FileDescriptorProto",
                    protobuf_crate_path(customize)
                ),
                &format!("{}", protobuf_crate_path(customize)),
            );
            w.block("file_descriptor_proto_lazy.get(|| {", "})", |w| {
                w.write_line(&format!(
                    "{}::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()",
                    protobuf_crate_path(customize)
                ));
            });
        },
    );
    w.write_line("");
    write_file_descriptor(file, &customize, w);
}

pub(crate) struct FileIndex {
    messsage_to_index: HashMap<ProtobufRelativePath, u32>,
    enum_to_index: HashMap<ProtobufRelativePath, u32>,
}

impl FileIndex {
    fn index(file_scope: &FileScope) -> FileIndex {
        FileIndex {
            messsage_to_index: file_scope
                .find_messages()
                .into_iter()
                .map(|m| m.protobuf_name_to_package())
                .enumerate()
                .map(|(i, n)| (n, i as u32))
                .collect(),
            enum_to_index: file_scope
                .find_enums()
                .into_iter()
                .map(|m| m.protobuf_name_to_package())
                .enumerate()
                .map(|(i, n)| (n, i as u32))
                .collect(),
        }
    }
}

struct GenFileResult {
    compiler_plugin_result: compiler_plugin::GenResult,
    mod_name: String,
}

fn gen_file(
    file_descriptor: &FileDescriptor,
    _files_map: &HashMap<&ProtoPath, &FileDescriptor>,
    root_scope: &RootScope,
    customize: &Customize,
    parser: &str,
) -> GenFileResult {
    // TODO: use it
    let mut customize = customize.clone();
    // options specified in invocation have precedence over options specified in file
    customize.update_with(&customize_from_rustproto_for_file(
        file_descriptor.proto().options.get_or_default(),
    ));

    let file_scope = FileScope { file_descriptor };
    let scope = file_scope.to_scope();
    let lite_runtime = customize.lite_runtime.unwrap_or_else(|| {
        file_descriptor
            .proto()
            .options
            .get_or_default()
            .get_optimize_for()
            == file_options::OptimizeMode::LITE_RUNTIME
    });

    let file_index = FileIndex::index(&file_scope);

    let mut v = Vec::new();

    {
        let mut w = CodeWriter::new(&mut v);

        w.write_generated_by("rust-protobuf", env!("CARGO_PKG_VERSION"), parser);

        w.write_line("");
        w.write_line(&format!(
            "//! Generated file from `{}`",
            file_descriptor.proto().get_name()
        ));
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

        static NESTED_TYPE_NUMBER: protobuf::rt::LazyV2<i32> = protobuf::rt::LazyV2::INIT;
        let message_type_number = *NESTED_TYPE_NUMBER.get(|| {
            protobuf::reflect::MessageDescriptor::for_type::<FileDescriptorProto>()
                .get_field_by_name("message_type")
                .expect("`message_type` must exist")
                .get_proto()
                .get_number()
        });

        let mut path = vec![message_type_number, 0];
        for (id, message) in scope.get_messages().iter().enumerate() {
            // ignore map entries, because they are not used in map fields
            if !message.is_map() {
                path[1] = id as i32;

                w.write_line("");
                MessageGen::new(
                    file_descriptor,
                    message,
                    &file_index,
                    &root_scope,
                    &customize,
                    &path,
                    file_descriptor.proto().source_code_info.as_ref(),
                )
                .write(&mut w);
            }
        }

        static ENUM_TYPE_NUMBER: protobuf::rt::LazyV2<i32> = protobuf::rt::LazyV2::INIT;
        let enum_type_number = *ENUM_TYPE_NUMBER.get(|| {
            protobuf::reflect::MessageDescriptor::for_type::<FileDescriptorProto>()
                .get_field_by_name("enum_type")
                .expect("`enum_type` must exist")
                .get_proto()
                .get_number()
        });

        let mut path = vec![enum_type_number, 0];
        for (id, enum_type) in scope.get_enums().iter().enumerate() {
            path[1] = id as i32;

            w.write_line("");
            EnumGen::new(
                enum_type,
                &file_index,
                &customize,
                root_scope,
                &path,
                file_descriptor.proto().source_code_info.as_ref(),
            )
            .write(&mut w);
        }

        write_extensions(file_descriptor, &root_scope, &mut w, &customize);

        if !lite_runtime {
            w.write_line("");
            write_file_descriptor_data(file_descriptor, &customize, &mut w);
        }
    }

    GenFileResult {
        compiler_plugin_result: compiler_plugin::GenResult {
            name: proto_name_to_rs(file_descriptor.proto().get_name()),
            content: v,
        },
        mod_name: proto_path_to_rust_mod(file_descriptor.proto().get_name()).into_string(),
    }
}

fn gen_mod_rs(mods: &[String]) -> compiler_plugin::GenResult {
    let mut v = Vec::new();
    let mut w = CodeWriter::new(&mut v);
    w.comment(&format!("{}generated", "@"));
    w.write_line("");
    for m in mods {
        w.write_line(&format!("pub mod {};", m));
    }
    drop(w);
    compiler_plugin::GenResult {
        name: "mod.rs".to_owned(),
        content: v,
    }
}

pub fn gen(
    file_descriptors: &[FileDescriptorProto],
    parser: &str,
    files_to_generate: &[ProtoPathBuf],
    customize: &Customize,
) -> anyhow::Result<Vec<compiler_plugin::GenResult>> {
    let file_descriptors = FileDescriptor::new_dynamic_fds(file_descriptors.to_vec());

    let root_scope = RootScope {
        file_descriptors: &file_descriptors,
    };

    let mut results: Vec<compiler_plugin::GenResult> = Vec::new();
    let files_map: HashMap<&ProtoPath, &FileDescriptor> = file_descriptors
        .iter()
        .map(|f| Ok((ProtoPath::new(f.proto().get_name())?, f)))
        .collect::<Result<_, anyhow::Error>>()?;

    let mut mods = Vec::new();

    for file_name in files_to_generate {
        let file = files_map.get(file_name.as_path()).expect(&format!(
            "file not found in file descriptors: {:?}, files: {:?}",
            file_name,
            files_map.keys()
        ));
        let gen_file_result = gen_file(file, &files_map, &root_scope, customize, parser);
        results.push(gen_file_result.compiler_plugin_result);
        mods.push(gen_file_result.mod_name);
    }

    if customize.inside_protobuf.unwrap_or(false) {
        results.push(gen_well_known_types_mod(&file_descriptors));
    }

    if customize.gen_mod_rs.unwrap_or(false) {
        results.push(gen_mod_rs(&mods));
    }

    Ok(results)
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("output path `{0}` is not a directory")]
    OutputIsNotDirectory(String),
    #[error("output path `{0}` does not exist or not accessible")]
    OutputDoesNotExistOrNotAccssible(String, #[source] io::Error),
    #[error("failed to create file `{0}`: {1}")]
    FailedToWriteFile(String, #[source] io::Error),
}

pub fn gen_and_write(
    file_descriptors: &[FileDescriptorProto],
    parser: &str,
    files_to_generate: &[ProtoPathBuf],
    out_dir: &Path,
    customize: &Customize,
) -> anyhow::Result<()> {
    match out_dir.metadata() {
        Ok(m) => {
            if !m.is_dir() {
                return Err(Error::OutputIsNotDirectory(out_dir.display().to_string()).into());
            }
        }
        Err(e) => {
            return Err(
                Error::OutputDoesNotExistOrNotAccssible(out_dir.display().to_string(), e).into(),
            );
        }
    }

    let results = gen(file_descriptors, parser, files_to_generate, customize)?;

    for r in &results {
        let mut file_path = out_dir.to_owned();
        file_path.push(&r.name);
        fs::write(&file_path, r.content.as_slice())
            .map_err(|e| Error::FailedToWriteFile(file_path.display().to_string(), e))?;
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
    })
    .expect("plugin failed");
}

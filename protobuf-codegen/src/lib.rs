#![deny(rustdoc::broken_intra_doc_links)]

mod compiler_plugin;
mod customize;
mod gen;
mod map;
pub mod protoc_gen_rust;

use std::collections::hash_map::HashMap;
use std::fs;
use std::io;
use std::path::Path;

use customize::customize_from_rustproto_for_file;
pub use customize::Customize;
use gen::code_writer::CodeWriter;
use gen::enums::*;
use gen::extensions::*;
use gen::inside::protobuf_crate_path;
use gen::message::*;
#[doc(hidden)]
pub use gen::paths::proto_name_to_rs;
use gen::paths::proto_path_to_rust_mod;
use gen::scope::FileScope;
use gen::scope::RootScope;
use gen::scope::WithScope;
use gen::well_known_types::gen_well_known_types_mod;
use protobuf::descriptor::*;
use protobuf::reflect::FileDescriptor;
use protobuf_parse::ProtoPath;
use protobuf_parse::ProtoPathBuf;
use protobuf_parse::ProtobufRelativePath;

use crate::gen::file_descriptor::write_file_descriptor_data;

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

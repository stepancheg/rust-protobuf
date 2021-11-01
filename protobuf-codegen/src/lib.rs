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

pub use customize::Customize;
#[doc(hidden)]
pub use gen::paths::proto_name_to_rs;
use gen::scope::FileScope;
use gen::scope::RootScope;
use gen::scope::WithScope;
use gen::well_known_types::gen_well_known_types_mod;
use protobuf::descriptor::*;
use protobuf::reflect::FileDescriptor;
use protobuf_parse::ProtoPath;
use protobuf_parse::ProtoPathBuf;
use protobuf_parse::ProtobufRelativePath;

use crate::gen::file::gen_file;
use crate::gen::mod_rs::gen_mod_rs;

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

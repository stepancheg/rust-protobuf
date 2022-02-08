use std::fs;
use std::io;
use std::path::PathBuf;

use protobuf::descriptor::FileDescriptorSet;
use protobuf::Message;
use protobuf_parse::ParsedAndTypechecked;
use protobuf_parse::ProtoPathBuf;
use protoc::Protoc;

use crate::codegen::remove_path_prefix;
use crate::codegen::Codegen;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("file `{0}` is not found in includes {}")]
    NotFound(String, String),
}

pub(crate) fn parse_and_typecheck(codegen: &Codegen) -> anyhow::Result<ParsedAndTypechecked> {
    let protoc = match codegen.protoc.clone() {
        Some(protoc) => protoc,
        None => Protoc::from_env_path(),
    };
    protoc.check()?;

    let temp_dir = tempfile::Builder::new().prefix("protoc-rust").tempdir()?;
    let temp_file = temp_dir.path().join("descriptor.pbbin");

    protoc
        .descriptor_set_out_args()
        .out(&temp_file)
        .includes(&codegen.includes)
        .inputs(&codegen.inputs)
        .include_imports(true)
        .extra_args(codegen.extra_args.iter())
        .write_descriptor_set()?;

    let fds = fs::read(temp_file)?;
    drop(temp_dir);

    let fds: protobuf::descriptor::FileDescriptorSet = FileDescriptorSet::parse_from_bytes(&fds)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let default_includes = vec![PathBuf::from(".")];
    let includes = if codegen.includes.is_empty() {
        &default_includes
    } else {
        &codegen.includes
    };

    let mut files_to_generate = Vec::new();
    'outer: for file in &codegen.inputs {
        for include in includes {
            if let Some(truncated) = remove_path_prefix(file, include) {
                files_to_generate.push(ProtoPathBuf::from_path(&truncated)?);
                continue 'outer;
            }
        }

        return Err(Error::NotFound(file.display().to_string(), format!("{:?}", includes)).into());
    }

    Ok(ParsedAndTypechecked {
        relative_paths: files_to_generate,
        file_descriptors: fds.file,
        parser: format!("protoc {}", protoc.version()?),
    })
}

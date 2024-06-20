#![doc(hidden)]

use std::fs;
use std::fs::create_dir_all;
use std::io;
use std::path::Path;

use protobuf::descriptor::FileDescriptorProto;
use protobuf_parse::ProtoPathBuf;

use crate::customize::CustomizeCallback;
use crate::gen::all::gen_all;
use crate::logging::init_logging;
use crate::Customize;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("output path `{0}` is not a directory")]
    OutputIsNotDirectory(String),
    #[error("output path `{0}` does not exist or not accessible")]
    OutputDoesNotExistOrNotAccssible(String, #[source] io::Error),
    #[error("failed to create file `{0}`: {1}")]
    FailedToWriteFile(String, #[source] io::Error),
    #[error("failed to create directory hierarchy for path `{0}`")]
    FailedToCreateDirectoryHierarchy(String, #[source] io::Error),
}

#[doc(hidden)]
pub fn gen_and_write(
    file_descriptors: &[FileDescriptorProto],
    parser: &str,
    files_to_generate: &[ProtoPathBuf],
    out_dir: &Path,
    customize: &Customize,
    customize_callback: &dyn CustomizeCallback,
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

    init_logging(&customize)?;

    let results = gen_all(
        file_descriptors,
        parser,
        files_to_generate,
        customize,
        customize_callback,
    )?;

    for r in &results {
        let mut file_path = out_dir.to_owned();
        file_path.push(&r.name);
        if customize.gen_mod_rs_hierarchy_out_dir_mod_name.is_some() {
            if let Some(parent) = file_path.parent() {
                create_dir_all(parent).map_err(|error| {
                    Error::FailedToCreateDirectoryHierarchy(parent.display().to_string(), error)
                })?;
            }
        }
        fs::write(&file_path, r.content.as_slice())
            .map_err(|e| Error::FailedToWriteFile(file_path.display().to_string(), e))?;
    }

    Ok(())
}

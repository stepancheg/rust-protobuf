use std::fs;
use std::path::PathBuf;

use protobuf::descriptor::FileDescriptorSet;
use protobuf::Message;
use protoc::Protoc;

use crate::pure::parse_and_typecheck::path_to_proto_path;
use crate::ParsedAndTypechecked;
use crate::ProtoPathBuf;

/// Parse `.proto` files using `protoc` command.
pub(crate) fn parse_and_typecheck(
    includes: &[PathBuf],
    input: &[PathBuf],
) -> anyhow::Result<ParsedAndTypechecked> {
    let temp_dir = tempfile::Builder::new()
        .prefix("protobuf-parse")
        .tempdir()?;
    let temp_file = temp_dir.path().join("descriptor.pbbin");

    let relative_paths: Vec<ProtoPathBuf> = input
        .iter()
        .map(|p| path_to_proto_path(p, includes))
        .collect::<anyhow::Result<_>>()?;

    Protoc::from_env_path()
        .descriptor_set_out_args()
        .inputs(input)
        .includes(includes)
        .out(&temp_file)
        .include_imports(true)
        .write_descriptor_set()?;

    let fds = fs::read(temp_file)?;
    drop(temp_dir);

    let fds: protobuf::descriptor::FileDescriptorSet = FileDescriptorSet::parse_from_bytes(&fds)?;

    Ok(ParsedAndTypechecked {
        relative_paths,
        file_descriptors: fds.file,
    })
}

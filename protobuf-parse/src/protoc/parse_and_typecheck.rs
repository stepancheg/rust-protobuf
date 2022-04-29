use std::fs;

use protobuf::descriptor::FileDescriptorSet;
use protobuf::Message;

use crate::protoc::command::Protoc;
use crate::pure::parse_and_typecheck::path_to_proto_path;
use crate::ParsedAndTypechecked;
use crate::Parser;
use crate::ProtoPathBuf;

/// Parse `.proto` files using `protoc` command.
pub(crate) fn parse_and_typecheck(parser: &Parser) -> anyhow::Result<ParsedAndTypechecked> {
    let temp_dir = tempfile::Builder::new()
        .prefix("protobuf-parse")
        .tempdir()?;
    let temp_file = temp_dir.path().join("descriptor.pbbin");

    let relative_paths: Vec<ProtoPathBuf> = parser
        .inputs
        .iter()
        .map(|p| path_to_proto_path(p, &parser.includes))
        .collect::<anyhow::Result<_>>()?;

    let protoc = match &parser.protoc {
        Some(protoc) => Protoc::from_path(protoc),
        None => Protoc::from_env_path(),
    };

    protoc
        .descriptor_set_out_args()
        .inputs(&parser.inputs)
        .includes(&parser.includes)
        .out(&temp_file)
        .include_imports(true)
        .extra_args(&parser.protoc_extra_args)
        .capture_stderr(parser.capture_stderr)
        .write_descriptor_set()?;

    let version = protoc.version()?;

    let fds = fs::read(temp_file)?;
    drop(temp_dir);

    let fds: protobuf::descriptor::FileDescriptorSet = FileDescriptorSet::parse_from_bytes(&fds)?;

    Ok(ParsedAndTypechecked {
        relative_paths,
        file_descriptors: fds.file,
        parser: format!("protoc {}", version),
    })
}

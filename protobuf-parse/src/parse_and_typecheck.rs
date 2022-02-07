use std::path::PathBuf;

use crate::protoc;
use crate::pure;
use crate::which_parser::WhichParser;
use crate::ProtoPathBuf;

/// Result of parsing `.proto` files.
#[doc(hidden)]
pub struct ParsedAndTypechecked {
    /// One entry for each input `.proto` file.
    pub relative_paths: Vec<ProtoPathBuf>,
    /// All parsed `.proto` files including dependencies of input files.
    pub file_descriptors: Vec<protobuf::descriptor::FileDescriptorProto>,
}

/// Parse `.proto` files and typecheck them using pure Rust parser of `protoc` command.
pub(crate) fn parse_and_typecheck(
    which_parser: WhichParser,
    includes: &[PathBuf],
    input: &[PathBuf],
) -> anyhow::Result<ParsedAndTypechecked> {
    match which_parser {
        WhichParser::Pure => pure::parse_and_typecheck(includes, input),
        WhichParser::Protoc => protoc::parse_and_typecheck(includes, input),
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::fs;

    use crate::which_parser::WhichParser;

    #[test]
    fn parse_and_typecheck() {
        let dir = tempfile::tempdir().unwrap();
        let a_proto = dir.path().join("a.proto");
        let b_proto = dir.path().join("b.proto");
        fs::write(&a_proto, "syntax = 'proto3'; message Apple {}").unwrap();
        fs::write(
            &b_proto,
            "syntax = 'proto3'; import 'a.proto'; message Banana { Apple a = 1; }",
        )
        .unwrap();

        let pure = super::parse_and_typecheck(
            WhichParser::Pure,
            &[dir.path().to_path_buf()],
            &[b_proto.clone()],
        )
        .unwrap();
        let protoc = super::parse_and_typecheck(
            WhichParser::Protoc,
            &[dir.path().to_path_buf()],
            &[b_proto.clone()],
        )
        .unwrap();

        assert_eq!(pure.relative_paths, protoc.relative_paths);
        assert_eq!(2, pure.file_descriptors.len());
        assert_eq!(2, protoc.file_descriptors.len());
        // TODO: make result more deterministic
        assert_eq!(
            HashSet::from(["a.proto", "b.proto"]),
            pure.file_descriptors.iter().map(|d| d.get_name()).collect()
        );
        assert_eq!(
            HashSet::from(["a.proto", "b.proto"]),
            protoc
                .file_descriptors
                .iter()
                .map(|d| d.get_name())
                .collect()
        );
        assert_eq!(1, protoc.file_descriptors[0].message_type.len());
        assert_eq!(1, pure.file_descriptors[0].message_type.len());
        assert_eq!(
            "Banana",
            pure.file_descriptors
                .iter()
                .find(|d| d.get_name() == "b.proto")
                .unwrap()
                .message_type[0]
                .get_name()
        );
        assert_eq!(
            "Banana",
            protoc
                .file_descriptors
                .iter()
                .find(|d| d.get_name() == "b.proto")
                .unwrap()
                .message_type[0]
                .get_name()
        );
    }
}

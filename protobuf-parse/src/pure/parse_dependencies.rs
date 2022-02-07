use protobuf::descriptor::FileDescriptorProto;

use crate::pure::convert::populate_dependencies;
use crate::pure::model;
use crate::pure::parser::ParserErrorWithLocation;

/// Parse imports from a `.proto` file.
///
/// The result is [`FileDescriptorProto`] object with only `*dependency` fields filled.
pub fn parse_dependencies(content: &str) -> Result<FileDescriptorProto, ParserErrorWithLocation> {
    let input = model::FileDescriptor::parse(content)?;
    let mut output = FileDescriptorProto::new();
    populate_dependencies(&input, &mut output);
    Ok(output)
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_dependencies() {
        let deps = crate::pure::parse_dependencies::parse_dependencies(
            r"
syntax = 'proto3';

import 'google/protobuf/field_mask.proto';
import public 'google/protobuf/struct.proto';

message IgnoreMe {}
",
        )
        .unwrap();
        assert_eq!(
            &[
                "google/protobuf/field_mask.proto",
                "google/protobuf/struct.proto",
            ],
            &deps.dependency[..]
        );
        assert_eq!(&[1], &deps.public_dependency[..]);
    }
}

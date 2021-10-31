use crate::convert::populate_dependencies;
use crate::model;
use crate::parser::ParserErrorWithLocation;
use protobuf::descriptor::FileDescriptorProto;

pub(crate) fn parse_dependencies(
    content: &str,
) -> Result<FileDescriptorProto, ParserErrorWithLocation> {
    let input = model::FileDescriptor::parse(content)?;
    let mut output = FileDescriptorProto::new();
    populate_dependencies(&input, &mut output);
    Ok(output)
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_dependencies() {
        let deps = crate::parse_dependencies::parse_dependencies(
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

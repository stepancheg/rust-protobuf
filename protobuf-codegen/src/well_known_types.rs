use crate::protobuf_name::ProtobufAbsolutePath;
use crate::protobuf_name::ProtobufRelativePath;

static NAMES: &'static [&'static str] = &[
    "Any",
    "Api",
    "BoolValue",
    "BytesValue",
    "DoubleValue",
    "Duration",
    "Empty",
    "Enum",
    "EnumValue",
    "Field",
    // TODO: dotted names
    "Field.Cardinality",
    "Field.Kind",
    "FieldMask",
    "FloatValue",
    "Int32Value",
    "Int64Value",
    "ListValue",
    "Method",
    "Mixin",
    "NullValue",
    "Option",
    "SourceContext",
    "StringValue",
    "Struct",
    "Syntax",
    "Timestamp",
    "Type",
    "UInt32Value",
    "UInt64Value",
    "Value",
];

fn is_well_known_type(name: &ProtobufRelativePath) -> bool {
    NAMES.iter().any(|&n| n == name.path)
}

pub fn is_well_known_type_full(name: &ProtobufAbsolutePath) -> Option<ProtobufRelativePath> {
    if let Some(ref rem) = name.remove_prefix(&ProtobufAbsolutePath::from(".google.protobuf")) {
        if is_well_known_type(rem) {
            Some(rem.clone())
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_well_known_type_full() {
        assert_eq!(
            Some(ProtobufRelativePath::from("BoolValue")),
            is_well_known_type_full(&ProtobufAbsolutePath::from(".google.protobuf.BoolValue"))
        );
        assert_eq!(
            None,
            is_well_known_type_full(&ProtobufAbsolutePath::from(".google.protobuf.Fgfg"))
        );
    }
}

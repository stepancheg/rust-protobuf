static NAMES: &'static [&'static str] = &[
    "Any",
    "Api",
    "Duration",
    "Empty",
    "FieldMask",
    "SourceContext",
    "Struct",
    "Timestamp",
    "Type",
    "DoubleValue",
    "FloatValue",
    "Int64Value",
    "UInt64Value",
    "Int32Value",
    "UInt32Value",
    "BoolValue",
    "StringValue",
    "BytesValue",
    "Value",
];

pub fn is_well_known_type(name: &str) -> bool {
    NAMES.iter().any(|&n| n == name)
}

pub fn is_well_known_type_full(name: &str) -> Option<&str> {
    if let Some(dot) = name.rfind('.') {
        if &name[.. dot] == ".google.protobuf" && is_well_known_type(&name[dot + 1 ..]) {
            Some(&name[dot + 1 ..])
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
        assert_eq!(Some("BoolValue"), is_well_known_type_full(".google.protobuf.BoolValue"));
        assert_eq!(None, is_well_known_type_full(".google.protobuf.Fgfg"));
    }
}

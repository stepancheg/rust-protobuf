use protobuf::Message;

use protobuf_test_common::*;

use super::test_fmt_text_format_pb::*;

#[test]
fn test_show() {
    let mut m = TestTypes::new();
    m.set_bool_singular(true);
    assert_eq!("bool_singular: true", &*format!("{:?}", m));
}

#[test]
fn test_pretty() {
    let mut tm = TestMessage::new();
    tm.set_value(23);
    let mut m = TestTypes::new();
    m.set_test_message_singular(tm);
    m.set_string_singular("abc".to_string());
    m.mut_string_repeated().push("def".to_string());
    m.mut_string_repeated().push("ghi".to_string());
    assert_eq!(
        "string_singular: \"abc\"\ntest_message_singular {\n  value: 23\n}\nstring_repeated: \"def\"\nstring_repeated: \"ghi\"\n",
        &*format!("{:#?}", m)
    );
}

#[test]
fn test_rust_identifier() {
    let mut m = TestTextFormatRustIdentifier::new();
    m.set_field_const(true);
    assert_eq!("const: true", &*format!("{:?}", m));
}

#[test]
fn test_empty() {
    test_text_format_str_descriptor("", TestTypes::descriptor_static());
}

#[test]
fn test_enum() {
    test_text_format_str_descriptor("test_enum_singular: DARK", TestTypes::descriptor_static());

    test_text_format_str_descriptor(
        "test_enum_repeated: DARK test_enum_repeated: LIGHT test_enum_repeated: LIGHT",
        TestTypes::descriptor_static(),
    );
}

#[test]
fn test_int() {
    test_text_format_str_descriptor("uint32_singular: 98", TestTypes::descriptor_static());
    test_text_format_str_descriptor("uint64_singular: 100", TestTypes::descriptor_static());

    test_text_format_str_descriptor("int32_singular: 98", TestTypes::descriptor_static());
    test_text_format_str_descriptor("int64_singular: 100", TestTypes::descriptor_static());
    test_text_format_str_descriptor("int32_singular: -98", TestTypes::descriptor_static());
    test_text_format_str_descriptor("int64_singular: -100", TestTypes::descriptor_static());

    test_text_format_str_descriptor("fixed32_singular: 98", TestTypes::descriptor_static());
    test_text_format_str_descriptor("fixed64_singular: 100", TestTypes::descriptor_static());
    test_text_format_str_descriptor("sfixed32_singular: 98", TestTypes::descriptor_static());
    test_text_format_str_descriptor("sfixed64_singular: 100", TestTypes::descriptor_static());
    test_text_format_str_descriptor("sfixed32_singular: -98", TestTypes::descriptor_static());
    test_text_format_str_descriptor("sfixed64_singular: -100", TestTypes::descriptor_static());

    test_text_format_str_descriptor(
        "int32_repeated: 98 int32_repeated: -99",
        TestTypes::descriptor_static(),
    );
}

#[test]
fn test_parse_float() {
    test_text_format_str_descriptor("float_singular: 98.5", TestTypes::descriptor_static());
    test_text_format_str_descriptor("float_singular: -99.5", TestTypes::descriptor_static());
    test_text_format_str_descriptor("float_singular: -99", TestTypes::descriptor_static());
    test_text_format_str_descriptor("double_singular: 98.5", TestTypes::descriptor_static());
    test_text_format_str_descriptor("double_singular: -99.5", TestTypes::descriptor_static());
    test_text_format_str_descriptor("double_singular: 99", TestTypes::descriptor_static());
}

#[test]
fn test_bool() {
    test_text_format_str_descriptor("bool_singular: true", TestTypes::descriptor_static());
    test_text_format_str_descriptor("bool_singular: false", TestTypes::descriptor_static());
    test_text_format_str_descriptor(
        "bool_repeated: false bool_repeated: false bool_repeated: true",
        TestTypes::descriptor_static(),
    );
}

#[test]
fn test_string_bytes() {
    test_text_format_str_descriptor("string_singular: \"\"", TestTypes::descriptor_static());
    test_text_format_str_descriptor("string_singular: \"a b\"", TestTypes::descriptor_static());
    test_text_format_str_descriptor("string_singular: \"a\\nb\"", TestTypes::descriptor_static());

    test_text_format_str_descriptor("bytes_singular: \"\"", TestTypes::descriptor_static());
    test_text_format_str_descriptor("bytes_singular: \"a b\"", TestTypes::descriptor_static());
    test_text_format_str_descriptor("bytes_singular: \"a\\nb\"", TestTypes::descriptor_static());
    test_text_format_str_descriptor(
        "bytes_singular: \"a\\xfeb\"",
        TestTypes::descriptor_static(),
    );

    test_text_format_str_descriptor(
        "string_repeated: \"ab\" bytes_repeated: \"cd\" string_repeated: \"ef\"",
        TestTypes::descriptor_static(),
    );

    test_text_format_str_descriptor(
        "string_singular: \"quote\\\"newline\\nbackslash\\\\del\\177\"",
        TestTypes::descriptor_static(),
    );
}

#[test]
fn test_message() {
    test_text_format_str_descriptor("test_message_singular {}", TestTypes::descriptor_static());

    test_text_format_str_descriptor(
        "test_message_singular { value: 10 }",
        TestTypes::descriptor_static(),
    );

    test_text_format_str_descriptor(
        "test_message_repeated { value: 10 } test_message_repeated { value: 20 }",
        TestTypes::descriptor_static(),
    );

    test_text_format_str_descriptor("test_message_singular <>", TestTypes::descriptor_static());

    test_text_format_str_descriptor(
        "test_message_singular < value: 10 >",
        TestTypes::descriptor_static(),
    );

    assert!(
        parse_using_rust_protobuf(
            "test_message_singular < value: 10 }",
            TestTypes::descriptor_static()
        )
        .is_err(),
        "Parsing a message with mismatched message start and terminator symbols should fail."
    );

    assert!(
        parse_using_rust_protobuf(
            "test_message_singular { value: 10 >",
            TestTypes::descriptor_static()
        )
        .is_err(),
        "Parsing a message with mismatched message start and terminator symbols should fail."
    );
}

#[test]
fn test_map_keys_sorted() {
    // TODO
    // When generating text format for a .proto, maps are sorted by key.
    // Numeric keys are sorted numerically.
}

#[test]
fn test_reflect() {
    for m in special_messages_typed::<TestTypes>() {
        test_text_format_message(&m);
        // This slow test is equivalent to the fast test below.
        // Do just one iteration as smoke test.
        // `break` statement can be commented out for easier debugging.
        break;
    }

    let mut l = TestTypesList::new();
    // TODO: make `ts` field public in codegen
    l.set_ts(special_messages_typed().into());
    test_text_format_message(&l);
}

#[test]
fn test_parse_error() {
    let e = protobuf::text_format::parse_from_str::<TestTypes>("nonexistent: 42").unwrap_err();
    let _error: &dyn std::error::Error = &e;
    assert_eq!(e.to_string(), "1:1: UnknownField(\"nonexistent\")");
}

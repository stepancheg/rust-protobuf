use std::default::Default;

use super::test_text_format_pb::*;

use protobuf::text_format::print_to_string;

fn t<F : FnMut(&mut TestTypes)>(expected: &str, mut setter: F) {
    let mut m = TestTypes::new();
    setter(&mut m);
    assert_eq!(&*print_to_string(&m), expected);
}

#[test]
fn test_singular() {
    t("int32_singular: 99", |m| m.set_int32_singular(99));
    t("double_singular: 99", |m| m.set_double_singular(99.0));
    t("float_singular: 99", |m| m.set_float_singular(99.0));
    t("int32_singular: 99", |m| m.set_int32_singular(99));
    t("int64_singular: 99", |m| m.set_int64_singular(99));
    t("uint32_singular: 99", |m| m.set_uint32_singular(99));
    t("uint64_singular: 99", |m| m.set_uint64_singular(99));
    t("sint32_singular: 99", |m| m.set_sint32_singular(99));
    t("sint64_singular: 99", |m| m.set_sint64_singular(99));
    t("fixed32_singular: 99", |m| m.set_fixed32_singular(99));
    t("fixed64_singular: 99", |m| m.set_fixed64_singular(99));
    t("sfixed32_singular: 99", |m| m.set_sfixed32_singular(99));
    t("sfixed64_singular: 99", |m| m.set_sfixed64_singular(99));
    t("bool_singular: true", |m| m.set_bool_singular(true));
    t("string_singular: \"abc\"", |m| {
        m.set_string_singular("abc".to_string())
    });
    t("bytes_singular: \"def\"", |m| {
        m.set_bytes_singular(b"def".to_vec())
    });
    t("test_enum_singular: DARK", |m| {
        m.set_test_enum_singular(TestEnum::DARK)
    });
    t("test_message_singular {}", |m| {
        m.mut_test_message_singular();
    });
}

#[test]
fn test_repeated_one() {
    t("int32_repeated: 99", |m| m.mut_int32_repeated().push(99));
    t(
        "double_repeated: 99",
        |m| m.mut_double_repeated().push(99.0),
    );
    t("float_repeated: 99", |m| m.mut_float_repeated().push(99.0));
    t("int32_repeated: 99", |m| m.mut_int32_repeated().push(99));
    t("int64_repeated: 99", |m| m.mut_int64_repeated().push(99));
    t("uint32_repeated: 99", |m| m.mut_uint32_repeated().push(99));
    t("uint64_repeated: 99", |m| m.mut_uint64_repeated().push(99));
    t("sint32_repeated: 99", |m| m.mut_sint32_repeated().push(99));
    t("sint64_repeated: 99", |m| m.mut_sint64_repeated().push(99));
    t(
        "fixed32_repeated: 99",
        |m| m.mut_fixed32_repeated().push(99),
    );
    t(
        "fixed64_repeated: 99",
        |m| m.mut_fixed64_repeated().push(99),
    );
    t("sfixed32_repeated: 99", |m| {
        m.mut_sfixed32_repeated().push(99)
    });
    t("sfixed64_repeated: 99", |m| {
        m.mut_sfixed64_repeated().push(99)
    });
    t(
        "bool_repeated: false",
        |m| m.mut_bool_repeated().push(false),
    );
    t("string_repeated: \"abc\"", |m| {
        m.mut_string_repeated().push("abc".to_string())
    });
    t("bytes_repeated: \"def\"", |m| {
        m.mut_bytes_repeated().push(b"def".to_vec())
    });
    t("test_enum_repeated: DARK", |m| {
        m.mut_test_enum_repeated().push(TestEnum::DARK)
    });
    t("test_message_repeated {}", |m| {
        m.mut_test_message_repeated().push(Default::default());
    });
}

#[test]
fn test_repeated_multiple() {
    t(
        "uint32_singular: 30 int32_repeated: 10 int32_repeated: -20",
        |m| {
            m.set_uint32_singular(30);
            m.mut_int32_repeated().push(10);
            m.mut_int32_repeated().push(-20);
        },
    );
}

#[test]
fn test_complex_message() {
    t("test_message_singular {value: 30}", |m| {
        m.mut_test_message_singular().set_value(30)
    });
}

#[test]
fn test_show() {
    let mut m = TestTypes::new();
    m.set_bool_singular(true);
    assert_eq!("bool_singular: true", &*format!("{:?}", m));
}

#[test]
fn test_string_escaped() {
    let mut m = TestTypes::new();
    m.set_string_singular("quote\"newline\nbackslash\\del\x7f".to_string());
    assert_eq!(
        "string_singular: \"quote\\\"newline\\nbackslash\\\\del\\177\"",
        &*format!("{:?}", m)
    );
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

use std::f32;
use std::f64;

use protobuf::Message;

use protobuf_test_common::*;

use super::test_fmt_json_pb::*;

#[test]
fn empty() {
    test_json_print_parse_message("{}", &TestTypes::new());
}

#[test]
fn test_bool() {
    let mut m = TestTypes::new();
    m.set_bool_singular(true);
    test_json_print_parse_message("{bool_singular: true}", &m);

    let mut m = TestTypes::new();
    m.set_bool_repeated(vec![true, false, false]);
    test_json_print_parse_message("{bool_repeated: [true, false, false]}", &m);
}

#[test]
fn test_float() {
    let mut m = TestTypes::new();
    m.set_float_singular(10.0);
    test_json_print_parse_message("{float_singular: 10.0}", &m);

    let mut m = TestTypes::new();
    m.float_repeated.push(11.0);
    m.float_repeated.push(-12.5);
    m.float_repeated.push(f32::NAN);
    m.float_repeated.push(f32::NEG_INFINITY);
    test_json_print_parse_message("{float_repeated: [11.0, -12.5, \"NaN\", \"-Infinity\"]}", &m);
}

#[test]
fn test_double() {
    let mut m = TestTypes::new();
    m.set_double_singular(12.0);
    test_json_print_parse_message("{double_singular: 12.0}", &m);

    let mut m = TestTypes::new();
    m.double_repeated.push(13.0);
    m.double_repeated.push(f64::INFINITY);
    test_json_print_parse_message("{double_repeated: [13.0, \"Infinity\"]}", &m);
}

#[test]
fn test_int32() {
    let mut m = TestTypes::new();
    m.set_int32_singular(1234);
    test_json_print_parse_message("{int32_singular: 1234}", &m);

    let mut m = TestTypes::new();
    m.int32_repeated.push(10);
    m.int32_repeated.push(-20);
    test_json_print_parse_message("{int32_repeated: [10, -20]}", &m);
}

#[test]
fn test_int64() {
    let mut m = TestTypes::new();
    m.set_int64_singular(1234567890123456789);
    test_json_print_parse_message("{int64_singular: \"1234567890123456789\"}", &m);

    let mut m = TestTypes::new();
    m.int64_repeated.push(2345678901234567890);
    m.int64_repeated.push(-2345678901234567890);
    test_json_print_parse_message(
        "{int64_repeated: [\"2345678901234567890\", \"-2345678901234567890\"]}", &m);
}

#[test]
fn test_sint32() {
    let mut m = TestTypes::new();
    m.set_sint32_singular(1234);
    test_json_print_parse_message("{sint32_singular: 1234}", &m);

    let mut m = TestTypes::new();
    m.sint32_repeated.push(10);
    m.sint32_repeated.push(-20);
    test_json_print_parse_message("{sint32_repeated: [10, -20]}", &m);
}

#[test]
fn test_sint64() {
    let mut m = TestTypes::new();
    m.set_sint64_singular(1234567890123456789);
    test_json_print_parse_message("{sint64_singular: \"1234567890123456789\"}", &m);

    let mut m = TestTypes::new();
    m.sint64_repeated.push(2345678901234567890);
    m.sint64_repeated.push(-2345678901234567890);
    test_json_print_parse_message(
        "{sint64_repeated: [\"2345678901234567890\", \"-2345678901234567890\"]}", &m);
}

#[test]
fn test_sfixed32() {
    let mut m = TestTypes::new();
    m.set_sfixed32_singular(1234);
    test_json_print_parse_message("{sfixed32_singular: 1234}", &m);

    let mut m = TestTypes::new();
    m.sfixed32_repeated.push(10);
    m.sfixed32_repeated.push(-20);
    test_json_print_parse_message("{sfixed32_repeated: [10, -20]}", &m);
}

#[test]
fn test_sfixed64() {
    let mut m = TestTypes::new();
    m.set_sfixed64_singular(1234567890123456789);
    test_json_print_parse_message("{sfixed64_singular: \"1234567890123456789\"}", &m);

    let mut m = TestTypes::new();
    m.sfixed64_repeated.push(2345678901234567890);
    m.sfixed64_repeated.push(-2345678901234567890);
    test_json_print_parse_message(
        "{sfixed64_repeated: [\"2345678901234567890\", \"-2345678901234567890\"]}", &m);
}

#[test]
fn test_uint32() {
    let mut m = TestTypes::new();
    m.set_uint32_singular(1234);
    test_json_print_parse_message("{uint32_singular: 1234}", &m);

    let mut m = TestTypes::new();
    m.uint32_repeated.push(10);
    m.uint32_repeated.push(20300);
    test_json_print_parse_message("{uint32_repeated: [10, 20300]}", &m);
}

#[test]
fn test_uint64() {
    let mut m = TestTypes::new();
    m.set_uint64_singular(1234567890123456789);
    test_json_print_parse_message(
        "{uint64_singular: \"1234567890123456789\"}", &m);

    let mut m = TestTypes::new();
    m.uint64_repeated.push(2345678901234567890);
    m.uint64_repeated.push(1345678901234567890);
    test_json_print_parse_message(
        "{uint64_repeated: [\"2345678901234567890\", \"1345678901234567890\"]}", &m);
}

#[test]
fn test_fixed32() {
    let mut m = TestTypes::new();
    m.set_fixed32_singular(1234);
    test_json_print_parse_message("{fixed32_singular: 1234}", &m);

    let mut m = TestTypes::new();
    m.fixed32_repeated.push(10);
    m.fixed32_repeated.push(20300);
    test_json_print_parse_message("{fixed32_repeated: [10, 20300]}", &m);
}

#[test]
fn test_fixed64() {
    let mut m = TestTypes::new();
    m.set_fixed64_singular(1234567890123456789);
    test_json_print_parse_message(
        "{fixed64_singular: \"1234567890123456789\"}", &m);

    let mut m = TestTypes::new();
    m.fixed64_repeated.push(2345678901234567890);
    m.fixed64_repeated.push(1345678901234567890);
    test_json_print_parse_message(
        "{fixed64_repeated: [\"2345678901234567890\", \"1345678901234567890\"]}", &m);
}

#[test]
fn test_string() {
    let mut m = TestTypes::new();
    m.set_string_singular("ab".to_owned());
    test_json_print_parse_message("{string_singular: \"ab\"}", &m);

    let mut m = TestTypes::new();
    m.set_string_repeated(vec!["".to_owned(), "\0".to_owned(), "A\nB".to_owned()].into());
    test_json_print_parse_message("{string_repeated: [\"\", \"\\u0000\", \"A\\nB\"]}", &m);
}

#[test]
fn test_bytes() {
    let mut m = TestTypes::new();
    m.set_bytes_singular(b"ab".to_vec());
    test_json_print_parse_message("{bytes_singular: \"YWI=\"}", &m);

    let mut m = TestTypes::new();
    m.set_bytes_repeated(vec![b"".to_vec(), b"\0".to_vec(), b"A\nB".to_vec()].into());
    test_json_print_parse_message("{bytes_repeated: [\"\", \"AA==\", \"QQpC\"]}", &m);
}

#[test]
fn test_enum() {
    let mut m = TestTypes::new();
    m.set_test_enum_singular(TestEnum::DARK);
    test_json_print_parse_message("{test_enum_singular: \"DARK\"}", &m);
    test_json_parse_message("{test_enum_singular: 10}", &m);

    let mut m = TestTypes::new();
    m.set_test_enum_repeated(vec![TestEnum::DARK, TestEnum::LIGHT]);
    test_json_print_parse_message("{test_enum_repeated: [\"DARK\", \"LIGHT\"]}", &m);
    test_json_parse_message("{test_enum_repeated: [\"DARK\", 20]}", &m);
}

#[test]
fn test_reflect() {
    for m in special_messages(TestTypes::descriptor_static()) {
        test_json_message(&*m);
    }
}

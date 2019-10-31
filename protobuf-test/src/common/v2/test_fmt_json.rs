use std::f32;
use std::f64;

use protobuf::json;
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
    test_json_print_parse_message("{\"boolSingular\": true}", &m);

    let mut m = TestTypes::new();
    m.set_bool_repeated(vec![true, false, false]);
    test_json_print_parse_message("{\"boolRepeated\": [true, false, false]}", &m);
}

#[test]
fn test_float() {
    let mut m = TestTypes::new();
    m.set_float_singular(10.0);
    test_json_print_parse_message("{\"floatSingular\": 10.0}", &m);

    let mut m = TestTypes::new();
    m.float_repeated.push(11.0);
    m.float_repeated.push(-12.5);
    m.float_repeated.push(f32::NAN);
    m.float_repeated.push(f32::NEG_INFINITY);
    test_json_print_parse_message(
        "{\"floatRepeated\": [11.0, -12.5, \"NaN\", \"-Infinity\"]}",
        &m,
    );
}

#[test]
fn test_double() {
    let mut m = TestTypes::new();
    m.set_double_singular(12.0);
    test_json_print_parse_message("{\"doubleSingular\": 12.0}", &m);
    test_json_parse_message("{\"doubleSingular\": \"12.0\"}", &m);

    let mut m = TestTypes::new();
    m.double_repeated.push(13.0);
    m.double_repeated.push(f64::INFINITY);
    test_json_print_parse_message("{\"doubleRepeated\": [13.0, \"Infinity\"]}", &m);
}

#[test]
fn test_int32() {
    let mut m = TestTypes::new();
    m.set_int32_singular(1234);
    test_json_print_parse_message("{\"int32Singular\": 1234}", &m);

    let mut m = TestTypes::new();
    m.int32_repeated.push(10);
    m.int32_repeated.push(-20);
    test_json_print_parse_message("{\"int32Repeated\": [10, -20]}", &m);
}

#[test]
fn test_int64() {
    let mut m = TestTypes::new();
    m.set_int64_singular(1234567890123456789);
    test_json_print_parse_message("{\"int64Singular\": \"1234567890123456789\"}", &m);

    let mut m = TestTypes::new();
    m.int64_repeated.push(2345678901234567890);
    m.int64_repeated.push(-2345678901234567890);
    test_json_print_parse_message(
        "{\"int64Repeated\": [\"2345678901234567890\", \"-2345678901234567890\"]}",
        &m,
    );
}

#[test]
fn test_sint32() {
    let mut m = TestTypes::new();
    m.set_sint32_singular(1234);
    test_json_print_parse_message("{\"sint32Singular\": 1234}", &m);

    let mut m = TestTypes::new();
    m.sint32_repeated.push(10);
    m.sint32_repeated.push(-20);
    test_json_print_parse_message("{\"sint32Repeated\": [10, -20]}", &m);
}

#[test]
fn test_sint64() {
    let mut m = TestTypes::new();
    m.set_sint64_singular(1234567890123456789);
    test_json_print_parse_message("{\"sint64Singular\": \"1234567890123456789\"}", &m);

    let mut m = TestTypes::new();
    m.sint64_repeated.push(2345678901234567890);
    m.sint64_repeated.push(-2345678901234567890);
    test_json_print_parse_message(
        "{\"sint64Repeated\": [\"2345678901234567890\", \"-2345678901234567890\"]}",
        &m,
    );
}

#[test]
fn test_sfixed32() {
    let mut m = TestTypes::new();
    m.set_sfixed32_singular(1234);
    test_json_print_parse_message("{\"sfixed32Singular\": 1234}", &m);

    let mut m = TestTypes::new();
    m.sfixed32_repeated.push(10);
    m.sfixed32_repeated.push(-20);
    test_json_print_parse_message("{\"sfixed32Repeated\": [10, -20]}", &m);
}

#[test]
fn test_sfixed64() {
    let mut m = TestTypes::new();
    m.set_sfixed64_singular(1234567890123456789);
    test_json_print_parse_message("{\"sfixed64Singular\": \"1234567890123456789\"}", &m);

    let mut m = TestTypes::new();
    m.sfixed64_repeated.push(2345678901234567890);
    m.sfixed64_repeated.push(-2345678901234567890);
    test_json_print_parse_message(
        "{\"sfixed64Repeated\": [\"2345678901234567890\", \"-2345678901234567890\"]}",
        &m,
    );
}

#[test]
fn test_uint32() {
    let mut m = TestTypes::new();
    m.set_uint32_singular(1234);
    test_json_print_parse_message("{\"uint32Singular\": 1234}", &m);

    let mut m = TestTypes::new();
    m.uint32_repeated.push(10);
    m.uint32_repeated.push(20300);
    test_json_print_parse_message("{\"uint32Repeated\": [10, 20300]}", &m);
}

#[test]
fn test_uint64() {
    let mut m = TestTypes::new();
    m.set_uint64_singular(1234567890123456789);
    test_json_print_parse_message("{\"uint64Singular\": \"1234567890123456789\"}", &m);

    let mut m = TestTypes::new();
    m.uint64_repeated.push(2345678901234567890);
    m.uint64_repeated.push(1345678901234567890);
    test_json_print_parse_message(
        "{\"uint64Repeated\": [\"2345678901234567890\", \"1345678901234567890\"]}",
        &m,
    );
}

#[test]
fn test_fixed32() {
    let mut m = TestTypes::new();
    m.set_fixed32_singular(1234);
    test_json_print_parse_message("{\"fixed32Singular\": 1234}", &m);

    let mut m = TestTypes::new();
    m.fixed32_repeated.push(10);
    m.fixed32_repeated.push(20300);
    test_json_print_parse_message("{\"fixed32Repeated\": [10, 20300]}", &m);
}

#[test]
fn test_fixed64() {
    let mut m = TestTypes::new();
    m.set_fixed64_singular(1234567890123456789);
    test_json_print_parse_message("{\"fixed64Singular\": \"1234567890123456789\"}", &m);
    test_json_parse_message("{\"fixed64Singular\": 1234567890123456789}", &m);

    let mut m = TestTypes::new();
    m.fixed64_repeated.push(2345678901234567890);
    m.fixed64_repeated.push(1345678901234567890);
    test_json_print_parse_message(
        "{\"fixed64Repeated\": [\"2345678901234567890\", \"1345678901234567890\"]}",
        &m,
    );
    test_json_parse_message(
        "{\"fixed64Repeated\": [\"2345678901234567890\", 1345678901234567890]}",
        &m,
    );
}

#[test]
fn test_string() {
    let mut m = TestTypes::new();
    m.set_string_singular("ab".to_owned());
    test_json_print_parse_message("{\"stringSingular\": \"ab\"}", &m);

    let mut m = TestTypes::new();
    m.set_string_repeated(vec!["".to_owned(), "\0".to_owned(), "A\nB".to_owned()].into());
    test_json_print_parse_message("{\"stringRepeated\": [\"\", \"\\u0000\", \"A\\nB\"]}", &m);
}

#[test]
fn test_bytes() {
    let mut m = TestTypes::new();
    m.set_bytes_singular(b"ab".to_vec());
    test_json_print_parse_message("{\"bytesSingular\": \"YWI=\"}", &m);

    let mut m = TestTypes::new();
    m.set_bytes_repeated(vec![b"".to_vec(), b"\0".to_vec(), b"A\nB".to_vec()].into());
    test_json_print_parse_message("{\"bytesRepeated\": [\"\", \"AA==\", \"QQpC\"]}", &m);
}

#[test]
fn test_enum() {
    let mut m = TestTypes::new();
    m.set_test_enum_singular(TestEnum::DARK);
    test_json_print_parse_message("{\"testEnumSingular\": \"DARK\"}", &m);
    test_json_parse_message("{\"testEnumSingular\": 10}", &m);

    let mut m = TestTypes::new();
    m.set_test_enum_repeated(vec![TestEnum::DARK.into(), TestEnum::LIGHT.into()]);
    test_json_print_parse_message("{\"testEnumRepeated\": [\"DARK\", \"LIGHT\"]}", &m);
    test_json_parse_message("{\"testEnumRepeated\": [\"DARK\", 20]}", &m);
}

#[test]
fn test_enum_int() {
    let mut m = TestTypes::new();
    m.set_test_enum_singular(TestEnum::DARK);
    let print_options = json::PrintOptions {
        enum_values_int: true,
        ..Default::default()
    };
    let json = json::print_to_string_with_options(&m, &print_options).unwrap();
    assert_eq!("{\"testEnumSingular\": 10}", json);
}

#[test]
fn test_map_field_int_key() {
    let mut m = TestTypes::new();
    m.fixed64_map_field.insert(10, 20);
    test_json_print_parse_message("{\"fixed64MapField\": {\"10\": \"20\"}}", &m);

    m.fixed64_map_field.insert(30, 40);
    let json = json::print_to_string(&m).unwrap();
    assert!(
        json == "{\"fixed64MapField\": {\"10\": \"20\", \"30\": \"40\"}}"
            || json == "{\"fixed64MapField\": {\"30\": \"40\", \"10\": \"20\"}}"
    );
}

#[test]
fn test_map_field_string_key() {
    let mut m = TestTypes::new();
    m.uint64_map_field.insert("foo".to_owned(), 20);
    test_json_print_parse_message("{\"uint64MapField\": {\"foo\": \"20\"}}", &m);
}

/// Proto3 JSON parsers are required to accept both the converted `lowerCamelCase` name
/// and the proto field name.
#[test]
fn test_accepts_both_json_and_original() {
    let mut m = TestTypes::new();
    m.set_bool_singular(true);
    test_json_parse_message("{\"boolSingular\": true}", &m);
    test_json_parse_message("{\"bool_singular\": true}", &m);

    let mut m = TestTypes::new();
    m.set_bool_repeated(vec![true, false, false]);
    test_json_parse_message("{\"boolRepeated\": [true, false, false]}", &m);
    test_json_parse_message("{\"bool_repeated\": [true, false, false]}", &m);
}

#[test]
fn test_use_original_field_names() {
    let mut m = TestTypes::new();
    m.set_bool_singular(true);
    let print_options = json::PrintOptions {
        proto_field_name: true,
        ..Default::default()
    };
    let json = json::print_to_string_with_options(&m, &print_options).unwrap();
    assert_eq!("{\"bool_singular\": true}", json);
}

#[test]
fn test_always_output_default_values() {
    let mut m = TestIncludeDefaultValues::new();
    m.set_sss("asd".to_owned());
    let print_options = json::PrintOptions {
        always_output_default_values: true,
        ..Default::default()
    };
    let json = json::print_to_string_with_options(&m, &print_options).unwrap();
    assert_eq!("{\"iii\": 0, \"sss\": \"asd\"}", json);
}

#[test]
fn test_ignore_unknown_fields() {
    let mut expected = TestTypes::new();
    expected.set_bool_singular(true);

    let parse_options = json::ParseOptions {
        ignore_unknown_fields: true,
        ..Default::default()
    };
    let mut m = TestTypes::new();
    let json = "{\"fgfgfg\": 12, \"bool_singular\": true, \"x\": [{\"a\": 12.4}]}";
    json::merge_from_str_with_options(&mut m, json, &parse_options).unwrap();

    assert_eq!(expected, m);
}

#[test]
fn test_reflect() {
    for m in special_messages(TestTypes::descriptor_static()) {
        test_json_message(&*m);
    }
}

#[test]
fn test_use_json_name() {
    let mut m = TestJsonName::new();
    m.set_field_with_json_name(true);
    let json = json::print_to_string(&m).unwrap();
    assert_eq!("{\"Field With json_name\": true}", json);
}

#[test]
fn test_more_than_one() {
    let mut m = TestTypes::new();
    m.set_bool_singular(true);
    m.set_bool_repeated(vec![true, false, false]);
    m.uint64_map_field.insert("foo".to_owned(), 20);

    test_json_print_parse_message(
        "{\"boolSingular\": true, \"boolRepeated\": [true, false, false], \"uint64MapField\": {\"foo\": \"20\"}}",
        &m,
    );
}

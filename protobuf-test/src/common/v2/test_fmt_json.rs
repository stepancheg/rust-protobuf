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

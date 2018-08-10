use protobuf::well_known_types::*;

use protobuf_test_common::*;

use super::test_fmt_json_well_known_pb::*;

#[test]
fn test_duration() {
    let mut m = TestFmtJsonWellKnownTypes::new();
    let mut d = Duration::new();
    d.set_seconds(1);
    d.set_nanos(340012);
    m.set_duration(d);
    test_json_print_parse_message("{duration: \"1.000340012s\"}", &m);

    let mut m = TestFmtJsonWellKnownTypes::new();
    let mut d = Duration::new();
    d.set_seconds(1);
    m.set_duration(d);
    test_json_parse_message("{duration: \"1s\"}", &m);
}

#[test]
fn test_null_value() {
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.set_null_values(vec![NullValue::NULL_VALUE]);
    test_json_print_parse_message("{nullValues: [null]}", &m);
}

#[test]
fn test_value() {
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_value().set_bool_value(true);
    test_json_print_parse_message("{value: true}", &m);
    m.mut_value().set_number_value(12.0);
    test_json_print_parse_message("{value: 12.0}", &m);
    m.mut_value().set_string_value("ab".to_owned());
    test_json_print_parse_message("{value: \"ab\"}", &m);
    m.mut_value().set_null_value(NullValue::NULL_VALUE);
    test_json_print_parse_message("{value: null}", &m);
    // TODO: list
    // TODO: struct
}

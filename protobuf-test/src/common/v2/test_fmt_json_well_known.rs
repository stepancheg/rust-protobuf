use protobuf::well_known_types::*;

use protobuf_test_common::*;

use protobuf::prelude::*;

use super::test_fmt_json_well_known_pb::*;

#[test]
fn test_duration() {
    let mut m = TestFmtJsonWellKnownTypes::new();
    let mut d = Duration::new();
    d.seconds = 1;
    d.nanos = 340012;
    m.set_duration(d);
    test_json_print_parse_message("{\"duration\": \"1.000340012s\"}", &m);

    let mut m = TestFmtJsonWellKnownTypes::new();
    let mut d = Duration::new();
    d.seconds = 1;
    m.set_duration(d);
    test_json_parse_message("{\"duration\": \"1s\"}", &m);
}

#[test]
fn test_timestamp() {
    let mut m = TestFmtJsonWellKnownTypes::new();
    let mut t = Timestamp::new();
    t.seconds = 1;
    t.nanos = 1;
    m.set_timestamp(t);
    test_json_print_parse_message("{\"timestamp\": \"1970-01-01T00:00:01.000000001Z\"}", &m);
}

#[test]
fn test_null_value() {
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.set_null_values(vec![NullValue::NULL_VALUE.into()]);
    test_json_print_parse_message("{\"nullValues\": [null]}", &m);
}

#[test]
fn test_value() {
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_value().set_bool_value(true);
    test_json_print_parse_message("{\"value\": true}", &m);
    m.mut_value().set_number_value(12.0);
    test_json_print_parse_message("{\"value\": 12.0}", &m);
    m.mut_value().set_string_value("ab".to_owned());
    test_json_print_parse_message("{\"value\": \"ab\"}", &m);
    m.mut_value().set_null_value(NullValue::NULL_VALUE);
    test_json_print_parse_message("{\"value\": null}", &m);

    m.mut_value().set_list_value({
        let mut l = ListValue::new();
        l.values.push({
            let mut v = Value::new();
            v.set_bool_value(true);
            v
        });
        l.values.push({
            let mut v = Value::new();
            v.set_number_value(12.0);
            v
        });
        l
    });
    test_json_print_parse_message("{\"value\": [true, 12.0]}", &m);

    m.mut_value().set_struct_value(Struct::new());
    test_json_print_parse_message("{\"value\": {}}", &m);
}

#[test]
fn test_list_value() {
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_list_value().values.push({
        let mut v = Value::new();
        v.set_number_value(2.0);
        v
    });
    m.mut_list_value().values.push({
        let mut v = Value::new();
        v.set_bool_value(false);
        v
    });
    test_json_print_parse_message("{\"listValue\": [2.0, false]}", &m);
}

#[test]
fn test_struct() {
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_struct_value().fields.insert("ab".to_owned(), {
        let mut v = Value::new();
        v.set_number_value(3.0);
        v
    });
    test_json_print_parse_message("{\"structValue\": {\"ab\": 3.0}}", &m);
}

#[test]
fn test_wrappers() {
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_double_value().value = 10.0;
    test_json_print_parse_message("{\"doubleValue\": 10.0}", &m);
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_float_value().value = 12.0;
    test_json_print_parse_message("{\"floatValue\": 12.0}", &m);
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_int64_value().value = -13;
    test_json_print_parse_message("{\"int64Value\": \"-13\"}", &m);
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_uint64_value().value = 13;
    test_json_print_parse_message("{\"uint64Value\": \"13\"}", &m);
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_int32_value().value = -13;
    test_json_print_parse_message("{\"int32Value\": -13}", &m);
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_uint32_value().value = 13;
    test_json_print_parse_message("{\"uint32Value\": 13}", &m);
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_bool_value().value = true;
    test_json_print_parse_message("{\"boolValue\": true}", &m);
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_string_value().value = "ab".to_owned();
    test_json_print_parse_message("{\"stringValue\": \"ab\"}", &m);
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.mut_bytes_value().value = b"ab".to_vec();
    test_json_print_parse_message("{\"bytesValue\": \"YWI=\"}", &m);
}

#[test]
fn test_any() {
    let mut m = TestFmtJsonWellKnownTypes::new();
    m.any_value.mut_message();
    // TODO
}

#[test]
fn test_field_mask() {
    let mut m = TestFmtJsonWellKnownTypes::new();

    m.set_field_mask(FieldMask::new());
    test_json_print_parse_message("{\"fieldMask\": \"\"}", &m);

    m.set_field_mask({
        let mut v = FieldMask::new();
        v.paths = vec!["a.b".to_owned()].into();
        v
    });
    test_json_print_parse_message("{\"fieldMask\": \"a.b\"}", &m);

    m.set_field_mask({
        let mut v = FieldMask::new();
        v.paths = vec!["ab".to_owned(), "c.d.e".to_owned()].into();
        v
    });
    test_json_print_parse_message("{\"fieldMask\": \"ab,c.d.e\"}", &m);
}

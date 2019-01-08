use protobuf::text_format::print_to_string;

use super::test_map_simple_pb::*;

use protobuf_test_common::*;

#[test]
fn test_map() {
    let mut map = TestMap::new();
    let mut entry = TestMapEntry::new();
    entry.set_v(10);

    test_serialize_deserialize("", &map);

    map.mut_m().insert("two".to_owned(), 2);
    test_serialize_deserialize("0a 07 0a 03 74 77 6f 10 02", &map);

    map.mut_m().insert("sixty six".to_owned(), 66);
    // Insert map entry sub message
    map.mut_mm().insert("map".to_owned(), entry);
    // cannot (easily) test hex, because order is not specified
    test_serialize_deserialize_no_hex(&map);
}

#[test]
fn test_map_negative_i32_value() {
    let mut map = TestMap::new();
    map.mut_m().insert("two".to_owned(), -2);
    test_serialize_deserialize("0a 10 0a 03 74 77 6f 10 fe ff ff ff ff ff ff ff ff 01", &map);
}

#[test]
fn test_map_with_object() {
    let mut map = TestMap::new();

    let mut entry = TestMapEntry::new();
    entry.set_v(10);

    test_serialize_deserialize("", &map);

    map.mut_mm().insert("map".to_owned(), entry);
    // cannot (easily) test hex, because order is not specified
    test_serialize_deserialize_no_hex(&map);
}

#[test]
fn test_map_unset_default_fields() {
    // unset key and value
    let mut m = TestMap::new();
    m.m.insert("".to_owned(), 0);
    test_deserialize("0a 00", &m);

    // unset value
    let mut m = TestMap::new();
    m.m.insert("ab".to_owned(), 0);
    test_deserialize("0a 04 0a 02 61 62", &m);

    // unset key
    let mut m = TestMap::new();
    m.m.insert("".to_owned(), 17);
    test_deserialize("0a 02 10 11", &m);
}

#[test]
fn text_format() {
    let mut map = TestMap::new();

    assert_eq!(&*print_to_string(&map), "");

    map.mut_m().insert("two".to_owned(), 2);

    assert_eq!(&*print_to_string(&map), "m {key: \"two\" value: 2}")
}

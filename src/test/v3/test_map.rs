use protobuf::text_format::print_to_string;

use super::test_map_pb::*;

use test::*;

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
fn text_format() {
    let mut map = TestMap::new();

    assert_eq!(&*print_to_string(&map), "");

    map.mut_m().insert("two".to_owned(), 2);

    assert_eq!(&*print_to_string(&map), "m {key: \"two\" value: 2}")
}

use bytes::Bytes;

use protobuf::*;

use protobuf_test_common::*;

use super::test_map_carllerche_pb::*;

#[test]
fn test_string_to_int32() {
    let mut map = TestMapCarllerche::new();
    map.string_to_int32.insert(Chars::from("abc"), 17);
    test_serialize_deserialize("0a 07 0a 03 61 62 63 10 11", &map);
    //                          field 1, length-delimited
    //                             length
    //                                field 1, wire type 2
    //                                   length
    //                                      a  b  c
    //                                               field 2, varint
    //                                                  17
}

#[test]
fn test_int32_to_bytes() {
    let mut map = TestMapCarllerche::new();
    map.int32_to_string.insert(17, Chars::from("abc"));
    test_serialize_deserialize("12 07 08 11 12 03 61 62 63", &map);
    let mut map = TestMapCarllerche::new();
    map.int32_to_bytes.insert(17, Bytes::from("abc"));
    test_serialize_deserialize("1a 07 08 11 12 03 61 62 63", &map);
}

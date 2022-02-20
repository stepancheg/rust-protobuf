use protobuf::Message;

use crate::serialize_then_parse_as_dynamic_and_serialize_and_parse;
use crate::serialize_then_parse_as_dynamic_then_serialize;
use crate::test_serialize_deserialize;
use crate::test_serialize_deserialize_no_hex;

pub fn test_serialize_deserialize_no_hex_with_dynamic<M: Message + PartialEq>(m: &M) {
    test_serialize_deserialize_no_hex(m);
    serialize_then_parse_as_dynamic_and_serialize_and_parse(m);
}

pub fn test_serialize_deserialize_with_dynamic<M: Message + PartialEq>(hex: &str, m: &M) {
    test_serialize_deserialize(hex, m);
    serialize_then_parse_as_dynamic_then_serialize(m);
}

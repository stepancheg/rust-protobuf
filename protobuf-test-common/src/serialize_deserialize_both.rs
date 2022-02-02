use protobuf::Message;

use crate::serialize_and_parse_as_dynamic_and_serialize;
use crate::test_serialize_deserialize;
use crate::test_serialize_deserialize_no_hex;

pub fn test_serialize_deserialize_no_hex_with_dynamic<M: Message + PartialEq>(m: &M) {
    test_serialize_deserialize_no_hex(m);
    serialize_and_parse_as_dynamic_and_serialize(m);
}

pub fn test_serialize_deserialize_with_dynamic<M: Message + PartialEq>(hex: &str, m: &M) {
    test_serialize_deserialize(hex, m);
    serialize_and_parse_as_dynamic_and_serialize(m);
}

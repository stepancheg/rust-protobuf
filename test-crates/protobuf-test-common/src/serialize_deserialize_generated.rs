use std::fmt;

use protobuf::*;

use crate::hex::decode_hex;
use crate::hex::encode_hex;

pub fn test_serialize_deserialize_length_delimited<M: Message + PartialEq + fmt::Debug>(msg: &M) {
    let serialized_bytes = msg.write_length_delimited_to_bytes().unwrap();
    let mut is = CodedInputStream::from_bytes(&serialized_bytes);
    let parsed = is.read_message().unwrap();
    is.check_eof().unwrap();
    assert_eq!(*msg, parsed);
}

pub fn test_serialize_deserialize_no_hex<M: MessageFull + PartialEq>(msg: &M) {
    let serialized_bytes = msg.write_to_bytes().unwrap();
    let parsed = M::parse_from_bytes(&serialized_bytes).unwrap();
    assert_eq!(*msg, parsed);
}

pub fn test_serialize_deserialize<M: Message + PartialEq + fmt::Debug>(hex: &str, msg: &M) {
    let expected_bytes = decode_hex(hex);
    let expected_hex = encode_hex(&expected_bytes);
    let serialized = msg.write_to_bytes().unwrap();
    let serialized_hex = encode_hex(&serialized);
    assert_eq!(expected_hex, serialized_hex, "message {}", M::NAME);
    let parsed = M::parse_from_bytes(&expected_bytes).unwrap();
    assert_eq!(*msg, parsed);

    assert_eq!(expected_bytes.len(), msg.compute_size() as usize);

    test_serialize_deserialize_length_delimited(msg);
}

pub fn test_deserialize<M: MessageFull + PartialEq>(hex: &str, msg: &M) {
    let bytes = decode_hex(hex);
    let parsed = M::parse_from_bytes(&bytes).unwrap();
    assert_eq!(*msg, parsed);
}

pub fn test_serialize<M: MessageFull>(hex: &str, msg: &M) {
    let hex = encode_hex(&decode_hex(hex));

    let serialized = msg.write_to_bytes().unwrap();
    let serialized_hex = encode_hex(&serialized);

    assert_eq!(serialized_hex, hex);
}

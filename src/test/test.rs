use protobuf::hex::encode_hex;
use protobuf::hex::decode_hex;

use protobuf::*;

pub fn test_serialize_deserialize_length_delimited<M : Message + MessageStatic>(msg: &M) {
    let serialized_bytes = msg.write_length_delimited_to_bytes().unwrap();
    let parsed = parse_length_delimited_from_bytes::<M>(&serialized_bytes).unwrap();
    assert!(*msg == parsed);
}

pub fn test_serialize_deserialize_no_hex<M : Message + MessageStatic>(msg: &M) {
    let serialized_bytes = msg.write_to_bytes().unwrap();
    let parsed = parse_from_bytes::<M>(&serialized_bytes).unwrap();
    assert!(*msg == parsed);
}

pub fn test_serialize_deserialize<M : Message + MessageStatic>(hex: &str, msg: &M) {
    let expected_bytes = decode_hex(hex);
    let expected_hex = encode_hex(&expected_bytes);
    let serialized = msg.write_to_bytes().unwrap();
    let serialized_hex = encode_hex(&serialized);
    assert_eq!(expected_hex, serialized_hex);
    let parsed = parse_from_bytes::<M>(&expected_bytes).unwrap();
    assert!(*msg == parsed);

    assert_eq!(expected_bytes.len(), msg.compute_size() as usize);

    test_serialize_deserialize_length_delimited(msg);
}

pub fn test_deserialize<M : Message + MessageStatic>(hex: &str, msg: &M) {
    let bytes = decode_hex(hex);
    let parsed = parse_from_bytes::<M>(&bytes).unwrap();
    assert!(*msg == parsed);
}

pub fn test_serialize<M : Message + MessageStatic>(hex: &str, msg: &M) {
    let hex = encode_hex(&decode_hex(hex));

    let serialized = msg.write_to_bytes().unwrap();
    let serialized_hex = encode_hex(&serialized);

    assert_eq!(serialized_hex, hex);
}

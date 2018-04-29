use hex::encode_hex;
use hex::decode_hex;

use protobuf::*;

pub fn test_serialize_deserialize_length_delimited<M : Message + PartialEq>(msg: &M) {
    let serialized_bytes = msg.write_length_delimited_to_bytes().unwrap();
    let parsed = parse_length_delimited_from_bytes::<M>(&serialized_bytes).unwrap();
    assert_eq!(*msg, parsed);
}

pub fn test_serialize_deserialize_no_hex<M : Message + PartialEq>(msg: &M) {
    let serialized_bytes = msg.write_to_bytes().unwrap();
    let parsed = parse_from_bytes::<M>(&serialized_bytes).unwrap();
    assert_eq!(*msg, parsed);
}

pub fn test_serialize_deserialize<M : Message + PartialEq>(hex: &str, msg: &M) {
    let expected_bytes = decode_hex(hex);
    let expected_hex = encode_hex(&expected_bytes);
    let serialized = msg.write_to_bytes().unwrap();
    let serialized_hex = encode_hex(&serialized);
    assert_eq!(
        expected_hex,
        serialized_hex,
        "message {}",
        M::descriptor_static().name()
    );
    let parsed = parse_from_bytes::<M>(&expected_bytes).unwrap();
    assert_eq!(*msg, parsed);

    assert_eq!(expected_bytes.len(), msg.compute_size() as usize);

    test_serialize_deserialize_length_delimited(msg);
}

pub fn test_deserialize<M : Message + PartialEq>(hex: &str, msg: &M) {
    let bytes = decode_hex(hex);
    let parsed = parse_from_bytes::<M>(&bytes).unwrap();
    assert_eq!(*msg, parsed);
}

pub fn test_serialize<M : Message>(hex: &str, msg: &M) {
    let hex = encode_hex(&decode_hex(hex));

    let serialized = msg.write_to_bytes().unwrap();
    let serialized_hex = encode_hex(&serialized);

    assert_eq!(serialized_hex, hex);
}

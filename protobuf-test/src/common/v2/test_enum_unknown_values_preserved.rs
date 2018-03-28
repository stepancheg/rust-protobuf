use super::test_enum_unknown_values_preserved_pb::*;

use protobuf::*;
use protobuf_test_common::*;

#[test]
fn unknown_values_preserved() {
    let mut new = NewMessage::new();
    new.set_eee(NewEnum::C);

    test_serialize_deserialize("08 1e", &new);

    // `OldEnum` doesn't have variant `C = 30`,
    // but message still properly serialized and deserialized.

    let old: OldMessage = parse_from_bytes(&hex::decode_hex("08 1e")).expect("parse");

    test_serialize_deserialize("08 1e", &old);
}

use core::*;
use hex::*;
use descriptor;

use shrug::*;

fn test_serialize_deserialize_length_delimited<M : Message>(msg: &M) {
    let serialized_bytes = msg.write_length_delimited_to_bytes();
    let parsed = parse_length_delimited_from_bytes::<M>(serialized_bytes);
    assert_eq!(msg, &parsed);
}

fn test_serialize_deserialize<M : Message>(hex: &str, msg: &M) {
    let expected_bytes = decode_hex(hex);
    let expected_hex = encode_hex(expected_bytes);
    let serialized = msg.write_to_bytes();
    let serialized_hex = encode_hex(serialized);
    assert_eq!(expected_hex, serialized_hex);
    let parsed = parse_from_bytes::<M>(expected_bytes);
    assert_eq!(msg, &parsed);

    assert_eq!(expected_bytes.len(), msg.serialized_size() as uint);

    test_serialize_deserialize_length_delimited(msg);
}

fn test_deserialize<M : Message>(hex: &str, msg: &M) {
    let bytes = decode_hex(hex);
    let parsed = parse_from_bytes::<M>(bytes);
    assert_eq!(msg, &parsed);
}

#[test]
fn test1() {
    test_serialize_deserialize("08 96 01", &Test1 { a: Some(150) });
}

#[test]
fn test2() {
    test_serialize_deserialize("12 07 74 65 73 74 69 6e 67", &Test2 { b: Some(~"testing") });
}

#[test]
fn test3() {
    test_serialize_deserialize("1a 03 08 96 01", &Test3 { c: Some(Test1 { a: Some(150) }) });
}

#[test]
fn test4() {
    test_serialize_deserialize("22 06 03 8E 02 9E A7 05", &Test4 { d: ~[3, 270, 86942] });
}

#[test]
fn test_read_unpacked_expect_packed() {
    test_deserialize("20 11 20 e8 07", &TestPackedUnpacked { packed: ~[], unpacked: ~[17, 1000] });
}

#[test]
fn test_read_packed_expect_unpacked() {
    test_deserialize("2a 03 11 e8 07", &TestPackedUnpacked { packed: ~[17, 1000], unpacked: ~[] });
}

#[test]
fn test_empty() {
    test_serialize_deserialize("", &TestEmpty::new());
}

#[test]
#[should_fail]
fn test_write_missing_required() {
    (TestRequired { b: None }).write_to_bytes();
}

#[test]
#[should_fail]
fn test_read_missing_required() {
    parse_from_bytes::<TestRequired>([]);
}

#[test]
#[should_fail]
fn test_read_junk() {
    parse_from_bytes::<Test1>(decode_hex("00"));
}

#[test]
fn test_file_descriptor_proto() {
    let p: descriptor::FileDescriptorProto = file_descriptor_proto();
    assert!(p.has_name());
    assert_eq!(Some(~"proto/shrug.proto"), p.name);
}

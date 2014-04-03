use core::*;
use hex::*;
use descriptor;

use shrug::*;

fn test_serialize_deserialize_length_delimited<M : Message>(msg: &M) {
    let serialized_bytes = msg.write_length_delimited_to_bytes();
    let parsed = parse_length_delimited_from_bytes::<M>(serialized_bytes);
    assert!(*msg == parsed);
}

fn test_serialize_deserialize<M : Message>(hex: &str, msg: &M) {
    let expected_bytes = decode_hex(hex);
    let expected_hex = encode_hex(expected_bytes);
    let serialized = msg.write_to_bytes();
    let serialized_hex = encode_hex(serialized);
    assert_eq!(expected_hex, serialized_hex);
    let parsed = parse_from_bytes::<M>(expected_bytes);
    assert!(*msg == parsed);

    assert_eq!(expected_bytes.len(), msg.serialized_size() as uint);

    test_serialize_deserialize_length_delimited(msg);
}

fn test_deserialize<M : Message>(hex: &str, msg: &M) {
    let bytes = decode_hex(hex);
    let parsed = parse_from_bytes::<M>(bytes);
    assert!(*msg == parsed);
}

#[test]
fn test1() {
    let mut test1 = Test1::new();
    test1.set_a(150);
    test_serialize_deserialize("08 96 01", &test1);
}

#[test]
fn test2() {
    let mut test2 = Test2::new();
    test2.set_b(~"testing");
    test_serialize_deserialize("12 07 74 65 73 74 69 6e 67", &test2);
}

#[test]
fn test3() {
    let mut test1 = Test1::new();
    test1.set_a(150);
    let mut test3 = Test3::new();
    test3.set_c(test1);
    test_serialize_deserialize("1a 03 08 96 01", &test3);
}

#[test]
fn test4() {
    let mut test4 = Test4::new();
    test4.set_d(~[3, 270, 86942]);
    test_serialize_deserialize("22 06 03 8E 02 9E A7 05", &test4);
}

#[test]
fn test_read_unpacked_expect_packed() {
    let mut test_packed_unpacked = TestPackedUnpacked::new();
    test_packed_unpacked.set_packed(~[]);
    test_packed_unpacked.set_unpacked(~[17, 1000]);
    test_deserialize("20 11 20 e8 07", &test_packed_unpacked);
}

#[test]
fn test_read_packed_expect_unpacked() {
    let mut test_packed_unpacked = TestPackedUnpacked::new();
    test_packed_unpacked.set_packed(~[17, 1000]);
    test_packed_unpacked.set_unpacked(~[]);
    test_deserialize("2a 03 11 e8 07", &test_packed_unpacked);
}

#[test]
fn test_empty() {
    test_serialize_deserialize("", &TestEmpty::new());
}

#[test]
#[should_fail]
fn test_write_missing_required() {
    TestRequired::new().write_to_bytes();
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
    assert_eq!("proto/shrug.proto", p.get_name());
}

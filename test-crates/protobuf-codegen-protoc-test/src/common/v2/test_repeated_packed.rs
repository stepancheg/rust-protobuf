use protobuf::reflect::Syntax;
use protobuf::MessageFull;
use protobuf_test_common::*;

use super::test_repeated_packed_pb::*;

// varint field number = 4
// unpacked tag = 20
// packed tag   = 22
// sfixed32 field number = 5
// unpacked tag = 2d
// packed tag   = 2a

#[test]
fn test_write_unpacked() {
    let mut test = TestUnpacked::new();
    test.varints = vec![17i32, 1000];
    test_serialize_deserialize("20 11 20 e8 07", &test);
    let mut test = TestUnpacked::new();
    test.sfixed32s = vec![17i32, 1000];
    test_serialize_deserialize("2d 11 00 00 00 2d e8 03 00 00", &test);
}

#[test]
fn test_read_unpacked_to_unpacked() {
    let mut test = TestUnpacked::new();
    test.varints = vec![17i32, 1000];
    test_deserialize("20 11 20 e8 07", &test);
    let mut test = TestUnpacked::new();
    test.sfixed32s = vec![17i32, 1000];
    test_deserialize("2d 11 00 00 00 2d e8 03 00 00", &test);
}

#[test]
fn test_read_packed_to_unpacked() {
    let mut test = TestUnpacked::new();
    test.varints = vec![17i32, 1000];
    test_deserialize("22 03 11 e8 07", &test);
    let mut test = TestUnpacked::new();
    test.sfixed32s = vec![17i32, 1000];
    test_deserialize("2a 08 11 00 00 00 e8 03 00 00", &test);
}

#[test]
fn test_write_packed_varint() {
    let mut test = TestPacked::new();
    test.varints = vec![17i32, 1000];
    test_serialize_deserialize("22 03 11 e8 07", &test);
}

#[test]
fn test_write_packed_fixed() {
    let mut test = TestPacked::new();
    test.sfixed32s = vec![17i32, 1000];
    test_serialize_deserialize("2a 08 11 00 00 00 e8 03 00 00", &test);
}

#[test]
fn test_read_unpacked_to_packed() {
    let mut test = TestPacked::new();
    test.varints = vec![17i32, 1000];
    test_deserialize("20 11 20 e8 07", &test);
    let mut test = TestPacked::new();
    test.sfixed32s = vec![17i32, 1000];
    test_deserialize("2d 11 00 00 00 2d e8 03 00 00", &test);
}

#[test]
fn test_read_packed_to_packed() {
    let mut test = TestPacked::new();
    test.varints = vec![17i32, 1000];
    test_deserialize("22 03 11 e8 07", &test);
    let mut test = TestPacked::new();
    test.sfixed32s = vec![17i32, 1000];
    test_deserialize("2a 08 11 00 00 00 e8 03 00 00", &test);
}

#[test]
fn test_issue_281() {
    // Data len len was incorrectly computed.
    // For 100 elements, bytes len is 400
    // and varint len of 400 is 2,
    // while varint len of 100 is 1.
    let mut test = TestIssue281::new();
    test.values = (0..100).collect();
    test_serialize_deserialize_no_hex(&test);
}

#[test]
fn test_write_packed_default() {
    let mut test = TestPackedDefault::new();
    test.varints = vec![0, 1, 2, 3, 4, 5];

    // Proto3 packs primitives by default, proto2 does not.
    let expected_hex = match TestPackedDefault::descriptor().file_descriptor().syntax() {
        Syntax::Proto2 => "08 00 08 01 08 02 08 03 08 04 08 05",
        Syntax::Proto3 => "0a 06 00 01 02 03 04 05",
    };
    test_serialize_deserialize(expected_hex, &test);
}

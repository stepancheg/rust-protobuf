use super::test_repeated_packed_pb::*;

use protobuf_test_common::*;


// varint field number = 4
// unpacked tag = 20
// packed tag   = 22
// sfixed32 field number = 5
// unpacked tag = 2d
// packed tag   = 2a

#[test]
fn test_write_unpacked() {
    let mut test = TestUnpacked::new();
    test.set_varints([17i32, 1000].to_vec());
    test_serialize_deserialize("20 11 20 e8 07", &test);
    let mut test = TestUnpacked::new();
    test.set_sfixed32s([17i32, 1000].to_vec());
    test_serialize_deserialize("2d 11 00 00 00 2d e8 03 00 00", &test);
}

#[test]
fn test_read_unpacked_to_unpacked() {
    let mut test = TestUnpacked::new();
    test.set_varints([17i32, 1000].to_vec());
    test_deserialize("20 11 20 e8 07", &test);
    let mut test = TestUnpacked::new();
    test.set_sfixed32s([17i32, 1000].to_vec());
    test_deserialize("2d 11 00 00 00 2d e8 03 00 00", &test);
}

#[test]
fn test_read_packed_to_unpacked() {
    let mut test = TestUnpacked::new();
    test.set_varints([17i32, 1000].to_vec());
    test_deserialize("22 03 11 e8 07", &test);
    let mut test = TestUnpacked::new();
    test.set_sfixed32s([17i32, 1000].to_vec());
    test_deserialize("2a 08 11 00 00 00 e8 03 00 00", &test);
}


#[test]
fn test_write_packed() {
    let mut test = TestPacked::new();
    test.set_varints([17i32, 1000].to_vec());
    test_serialize_deserialize("22 03 11 e8 07", &test);
    let mut test = TestPacked::new();
    test.set_sfixed32s([17i32, 1000].to_vec());
    test_serialize_deserialize("2a 08 11 00 00 00 e8 03 00 00", &test);
}

#[test]
fn test_read_unpacked_to_packed() {
    let mut test = TestPacked::new();
    test.set_varints([17i32, 1000].to_vec());
    test_deserialize("20 11 20 e8 07", &test);
    let mut test = TestPacked::new();
    test.set_sfixed32s([17i32, 1000].to_vec());
    test_deserialize("2d 11 00 00 00 2d e8 03 00 00", &test);
}

#[test]
fn test_read_packed_to_packed() {
    let mut test = TestPacked::new();
    test.set_varints([17i32, 1000].to_vec());
    test_deserialize("22 03 11 e8 07", &test);
    let mut test = TestPacked::new();
    test.set_sfixed32s([17i32, 1000].to_vec());
    test_deserialize("2a 08 11 00 00 00 e8 03 00 00", &test);
}

#[test]
fn test_issue_281() {
    // Data len len was incorrectly computed.
    // For 100 elements, bytes len is 400
    // and varint len of 400 is 2,
    // while varint len of 100 is 1.
    let mut test = TestIssue281::new();
    test.set_values((0..100).collect());
    test_serialize_deserialize_no_hex(&test);
}

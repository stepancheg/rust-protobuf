use super::test_repeated_packed_pb::*;

use protobuf_test_common::*;


// varint field number = 4
// unpacked tag = 20
// packed tag   = 22

#[test]
fn test_write_unpacked() {
    let mut test = TestUnpacked::new();
    test.set_varints([17i32, 1000].to_vec());
    test_serialize_deserialize("20 11 20 e8 07", &test);
}

#[test]
fn test_read_unpacked_to_unpacked() {
    let mut test = TestUnpacked::new();
    test.set_varints([17i32, 1000].to_vec());
    test_deserialize("20 11 20 e8 07", &test);
}

#[test]
fn test_read_packed_to_unpacked() {
    let mut test = TestUnpacked::new();
    test.set_varints([17i32, 1000].to_vec());
    test_deserialize("22 03 11 e8 07", &test);
}


#[test]
fn test_write_packed() {
    let mut test = TestPacked::new();
    test.set_varints([17i32, 1000].to_vec());
    test_serialize_deserialize("22 03 11 e8 07", &test);
}

#[test]
fn test_read_unpacked_to_packed() {
    let mut test = TestPacked::new();
    test.set_varints([17i32, 1000].to_vec());
    test_deserialize("20 11 20 e8 07", &test);
}

#[test]
fn test_read_packed_to_packed() {
    let mut test = TestPacked::new();
    test.set_varints([17i32, 1000].to_vec());
    test_deserialize("22 03 11 e8 07", &test);
}

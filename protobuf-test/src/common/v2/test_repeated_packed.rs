use super::test_repeated_packed_pb::*;

use protobuf_test_common::*;



#[test]
fn test_read_unpacked_expect_packed() {
    let mut test_packed_unpacked = TestPackedUnpacked::new();
    test_packed_unpacked.set_packed(Vec::new());
    test_packed_unpacked.set_unpacked([17i32, 1000].to_vec());
    test_deserialize("20 11 20 e8 07", &test_packed_unpacked);
}

#[test]
fn test_read_packed_expect_unpacked() {
    let mut test_packed_unpacked = TestPackedUnpacked::new();
    test_packed_unpacked.set_packed([17i32, 1000].to_vec());
    test_packed_unpacked.set_unpacked(Vec::new());
    test_deserialize("2a 03 11 e8 07", &test_packed_unpacked);
}

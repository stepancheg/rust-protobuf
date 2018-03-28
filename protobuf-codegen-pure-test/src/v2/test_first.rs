use protobuf_test_common::*;
use super::test_first_pb::*;

#[test]
fn test1() {
    let mut test1 = Test1::new();
    test1.set_a(150);
    test_serialize_deserialize("08 96 01", &test1);
}

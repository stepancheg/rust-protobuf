use protobuf::*;

use test::*;

use test_oneof_proto::*;

#[test]
fn test1() {
    let mut test1 = TestOneof::new();
    test1.set_uint32_field(150);
    test_serialize_deserialize("28 96 01", &test1);
}

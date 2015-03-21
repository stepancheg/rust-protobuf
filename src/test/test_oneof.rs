use protobuf::*;

use test::*;

use test_oneof_proto::*;

#[test]
fn test_simple() {
    let mut test_message = TestOneof::new();
    test_message.set_uint32_field(150);
    test_serialize_deserialize("28 96 01", &test_message);
}

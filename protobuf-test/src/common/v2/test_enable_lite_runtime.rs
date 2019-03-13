use protobuf_test_common::*;

use super::test_enable_lite_runtime_pb::*;

#[test]
fn test_lite_runtime() {
    let mut m = TestLiteRuntime::new();
    m.set_v(10);
    test_serialize_deserialize("08 0a", &m);

    // test it doesn't crash
    format!("{:?}", m);
}

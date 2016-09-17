use test::*;

use super::pb_test_lite_runtime;

#[test]
fn test_lite_runtime() {
    let mut m = pb_test_lite_runtime::TestLiteRuntime::new();
    m.set_v(10);
    test_serialize_deserialize("08 0a", &m);

    // test it doesn't crash
    format!("{:?}", m);
}

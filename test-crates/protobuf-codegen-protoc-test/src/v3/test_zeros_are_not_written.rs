use protobuf_test_common::*;

use super::test_zeros_are_not_written_pb::*;

#[test]
fn test_zeros_are_not_written() {
    let mut m = TestZerosAreNotWritten::new();
    m.bool_field = false;
    m.enum_field = TestEnumDescriptor::UNDEFINED.into();
    m.fixed32_field = 0;
    test_serialize("", &m);
}

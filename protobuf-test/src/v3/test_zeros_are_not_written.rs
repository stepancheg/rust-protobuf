use protobuf_test_common::*;

use super::test_zeros_are_not_written_pb::*;

#[test]
fn test_zeros_are_not_written() {
    let mut m = TestZerosAreNotWritten::new();
    m.set_bool_field(false);
    m.set_enum_field(TestEnumDescriptor::UNDEFINED);
    m.set_fixed32_field(0);
    test_serialize("", &m);
}

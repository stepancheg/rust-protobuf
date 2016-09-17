use test::*;

use super::test_basic_pb::*;

#[test]
fn test_zeros_are_not_written() {
    let mut m = TestTypesSingular::new();
    m.set_bool_field(false);
    m.set_enum_field(TestEnumDescriptor::UNKNOWN);
    m.set_fixed32_field(0);
    test_serialize("", &m);
}

use super::test_oneof_default_value_pb::*;

#[test]
fn test() {
    let m = TestOneofDefaultValue::new();
    assert_eq!(9.0, m.double_field());
    assert_eq!("ss", m.string_field());
    assert_eq!(b"bb", m.bytes_field());
}

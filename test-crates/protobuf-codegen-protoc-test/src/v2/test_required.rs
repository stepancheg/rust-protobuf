use protobuf::Message;

use super::test_required_pb::*;

#[test]
fn test_write_missing_required() {
    assert!(TestRequired::new().write_to_bytes().is_err());
}

#[test]
fn test_read_missing_required() {
    assert!(TestRequired::parse_from_bytes(&[]).is_err());
}

#[test]
fn test_is_initialized_is_recursive() {
    let mut m = TestRequiredOuter::new();
    assert!(!m.is_initialized());
    m.inner = Some(Default::default()).into();
    assert!(!m.is_initialized());
    m.inner.as_mut().unwrap().set_b(false);
    assert!(m.is_initialized());
}

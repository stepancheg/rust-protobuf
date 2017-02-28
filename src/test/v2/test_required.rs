use protobuf::*;

use super::test_required_pb::*;


#[test]
#[should_panic]
fn test_write_missing_required() {
    TestRequired::new().write_to_bytes().unwrap();
}

#[test]
#[should_panic]
fn test_read_missing_required() {
    parse_from_bytes::<TestRequired>(&[]).unwrap();
}

#[test]
fn test_is_initialized_is_recursive() {
    let mut m = TestRequiredOuter::new();
    assert!(!m.is_initialized());
    m.mut_inner();
    assert!(!m.is_initialized());
    m.mut_inner().set_b(false);
    assert!(m.is_initialized());
}

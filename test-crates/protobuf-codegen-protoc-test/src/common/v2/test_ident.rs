use protobuf::MessageFull;

use super::test_ident_pb::*;

#[test]
fn test() {
    let _ = TestType::new();
}

#[test]
fn test_reflect() {
    Self_::new();
    // instantiate reflection
    assert_eq!("Self", Self_::descriptor().name());
}

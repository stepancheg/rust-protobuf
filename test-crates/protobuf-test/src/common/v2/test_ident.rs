use protobuf::MessageFull;

use super::test_ident_pb::*;

#[test]
fn test() {
    let _ = TestType::new();
}

#[test]
fn test_reflect() {
    message_Self::new();
    // instantiate reflection
    assert_eq!("Self", message_Self::descriptor().name());
}

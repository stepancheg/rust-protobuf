use protobuf::MessageFull;

use super::test_optional_pb::*;

#[test]
fn test() {
    let _message = TestOptionalProto3::new();
}

#[test]
fn reflect_all_oneofs() {
    let descriptor = TestOptionalProto3::descriptor_static();
    let oneofs = descriptor.all_oneofs().collect::<Vec<_>>();
    assert!(oneofs.len() > 1);
    assert!(!oneofs[0].is_synthetic());
    for oneof in &oneofs[1..] {
        assert!(oneof.is_synthetic());
        let mut fields = oneof.fields().collect::<Vec<_>>();
        assert_eq!(1, fields.len());
        let field = fields.swap_remove(0);
        assert_eq!(None, field.containing_oneof());
        assert_eq!(
            Some(oneof),
            field.containing_oneof_including_synthetic().as_ref()
        );
    }
}

#[test]
fn reflect_oneofs() {
    let descriptor = TestOptionalProto3::descriptor_static();
    let oneofs = descriptor.oneofs().collect::<Vec<_>>();
    assert_eq!(1, oneofs.len());
    assert!(!oneofs[0].is_synthetic());
}

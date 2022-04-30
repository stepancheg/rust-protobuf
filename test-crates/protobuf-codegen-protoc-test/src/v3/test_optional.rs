use protobuf::reflect::ReflectValueRef;
use protobuf::MessageFull;
use protobuf_test_common::test_serialize_deserialize_with_dynamic;

use super::test_optional_pb::*;

#[test]
fn serialize_deserialize() {
    let mut message = TestOptionalProto3::new();
    message.iii = Some(0x1a);
    test_serialize_deserialize_with_dynamic("f8 01 1a", &message);
}

#[test]
fn field_types() {
    let message = TestOptionalProto3::new();
    let _iii: &Option<i32> = &message.iii;
    let _sss: &Option<String> = &message.sss;
}

#[test]
fn reflect_all_oneofs() {
    let descriptor = TestOptionalProto3::descriptor();
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
    let descriptor = TestOptionalProto3::descriptor();
    let oneofs = descriptor.oneofs().collect::<Vec<_>>();
    assert_eq!(1, oneofs.len());
    assert!(!oneofs[0].is_synthetic());

    let mut message = TestOptionalProto3::new();

    let iii = descriptor.field_by_name("iii").unwrap();

    assert_eq!(None, iii.get_singular(&mut message));

    iii.set_singular_field(&mut message, 17.into());
    assert_eq!(Some(17), message.iii);
    assert_eq!(
        Some(ReflectValueRef::I32(17)),
        iii.get_singular(&mut message)
    );
}

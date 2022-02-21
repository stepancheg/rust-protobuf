use protobuf::EnumOrUnknown;
use protobuf::MessageFull;

use super::test_reflect_default_pb::*;

#[test]
fn test_regular() {
    let m = TestReflectDefault::new();

    let descriptor = TestReflectDefault::descriptor_static();

    let i = descriptor.field_by_name("i").unwrap();
    assert_eq!(10, i.get_singular_field_or_default(&m).to_i32().unwrap());

    let s = descriptor.field_by_name("s").unwrap();
    assert_eq!("sss", s.get_singular_field_or_default(&m).to_str().unwrap());

    let e = descriptor.field_by_name("e").unwrap();
    assert_eq!(
        EnumOrUnknown::new(Fruit::BANANA),
        e.get_singular_field_or_default(&m)
            .downcast_clone()
            .unwrap()
    );
}

#[test]
fn test_oneof() {
    let m = TestReflectDefault::new();

    let descriptor = TestReflectDefault::descriptor_static();

    let i = descriptor.field_by_name("oi").unwrap();
    assert_eq!(10, i.get_singular_field_or_default(&m).to_i32().unwrap());

    let s = descriptor.field_by_name("os").unwrap();
    assert_eq!("sss", s.get_singular_field_or_default(&m).to_str().unwrap());

    let e = descriptor.field_by_name("oe").unwrap();
    assert_eq!(
        EnumOrUnknown::new(Fruit::BANANA),
        e.get_singular_field_or_default(&m)
            .downcast_clone()
            .unwrap()
    );
}

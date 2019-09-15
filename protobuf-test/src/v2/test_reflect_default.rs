use super::test_reflect_default_pb::*;
use protobuf::Message;

#[test]
fn test_regular() {
    let m = TestReflectDefault::new();

    let i = m.descriptor().field_by_name("i");
    assert_eq!(10, i.get_singular_field_or_default(&m).to_i32().unwrap());

    let s = m.descriptor().field_by_name("s");
    assert_eq!("sss", s.get_singular_field_or_default(&m).to_str().unwrap());

    let e = m.descriptor().field_by_name("e");
    assert_eq!(Fruit::BANANA, e.get_singular_field_or_default(&m).downcast_clone().unwrap());
}

#[test]
fn test_oneof() {
    let m = TestReflectDefault::new();

    let i = m.descriptor().field_by_name("oi");
    assert_eq!(10, i.get_singular_field_or_default(&m).to_i32().unwrap());

    let s = m.descriptor().field_by_name("os");
    assert_eq!("sss", s.get_singular_field_or_default(&m).to_str().unwrap());

    let e = m.descriptor().field_by_name("oe");
    assert_eq!(Fruit::BANANA, e.get_singular_field_or_default(&m).downcast_clone().unwrap());
}

use super::test_reflect_default_pb::*;
use protobuf::Message;
use protobuf::ProtobufEnum;

#[test]
fn test_regular() {
    let m = TestReflectDefault::new();

    let i = m.descriptor().field_by_name("i").unwrap();
    assert_eq!(10, i.get_i32(&m));

    let s = m.descriptor().field_by_name("s").unwrap();
    assert_eq!("sss", s.get_str(&m));

    let e = m.descriptor().field_by_name("e").unwrap();
    assert_eq!(Fruit::BANANA.descriptor(), e.get_enum(&m));
}

#[test]
fn test_oneof() {
    let m = TestReflectDefault::new();

    let i = m.descriptor().field_by_name("oi").unwrap();
    assert_eq!(10, i.get_i32(&m));

    let s = m.descriptor().field_by_name("os").unwrap();
    assert_eq!("sss", s.get_str(&m));

    let e = m.descriptor().field_by_name("oe").unwrap();
    assert_eq!(Fruit::BANANA.descriptor(), e.get_enum(&m));
}

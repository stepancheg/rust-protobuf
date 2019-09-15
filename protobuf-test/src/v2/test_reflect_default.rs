use super::test_reflect_default_pb::*;
use protobuf::Message;
use protobuf::ProtobufEnum;

#[test]
fn test_defaults() {
    let m = TestReflectDefault::new();

    let i = m.descriptor().field_by_name("i");
    assert_eq!(10, i.get_i32(&m));

    let s = m.descriptor().field_by_name("s");
    assert_eq!("sss", s.get_str(&m));

    let e = m.descriptor().field_by_name("e");
    assert_eq!(Fruit::BANANA.descriptor(), e.get_enum(&m));
}

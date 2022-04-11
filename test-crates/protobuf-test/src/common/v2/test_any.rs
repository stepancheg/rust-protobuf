use protobuf::well_known_types::Any;
use protobuf::MessageFull;

use super::test_any_pb::MessageOne;
use super::test_any_pb::MessageTwo;

#[test]
fn test_static() {
    let mut m1 = MessageOne::new();
    m1.set_i(10);
    let any = Any::pack(&m1).unwrap();
    assert_eq!("type.googleapis.com/test_any.MessageOne", any.type_url);
    assert!(any.is::<MessageOne>());
    assert!(!any.is::<MessageTwo>());
    assert_eq!(Some(m1), any.unpack::<MessageOne>().unwrap());
    assert_eq!(None, any.unpack::<MessageTwo>().unwrap());
}

#[test]
fn test_dynamic() {
    let mut m1 = MessageOne::new();
    m1.set_i(10);
    let any = Any::pack_dyn(&m1).unwrap();
    assert_eq!("type.googleapis.com/test_any.MessageOne", any.type_url);
    assert!(any.is_dyn(&MessageOne::descriptor()));
    assert!(!any.is_dyn(&MessageTwo::descriptor()));
    assert_eq!(
        m1,
        *any.unpack_dyn(&MessageOne::descriptor())
            .unwrap()
            .unwrap()
            .downcast_box::<MessageOne>()
            .unwrap()
    );
    assert!(any.unpack_dyn(&MessageTwo::descriptor()).unwrap().is_none());
}

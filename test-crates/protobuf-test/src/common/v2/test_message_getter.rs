use super::test_message_getter_pb::MessageForTestGetter;

#[test]
fn get_returns_default_value() {
    let m = MessageForTestGetter::new();
    assert_eq!(0, m.i());
    assert_eq!(false, m.b());
}

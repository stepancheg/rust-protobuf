use protobuf::reflect::ReflectValueBox;
use protobuf::MessageFull;

use super::test_reflect_clear_pb::*;

#[test]
fn test_generated() {
    let mut map = TestMessage::default().e().clone();
    map.insert("key".to_string(), "value".to_string());

    let mut msg = TestMessage::default();
    msg.set_a(1);
    msg.set_b("b".to_string());
    msg.set_c(test_message::Nested::default());
    msg.set_d(vec![1, 2, 3]);
    msg.set_e(map);

    let msg_desc = TestMessage::descriptor();

    let a_field = msg_desc.field_by_name("a").unwrap();
    let b_field = msg_desc.field_by_name("b").unwrap();
    let c_field = msg_desc.field_by_name("c").unwrap();
    let d_field = msg_desc.field_by_name("d").unwrap();
    let e_field = msg_desc.field_by_name("e").unwrap();

    a_field.clear_field(&mut msg);
    b_field.clear_field(&mut msg);
    c_field.clear_field(&mut msg);
    d_field.clear_field(&mut msg);
    e_field.clear_field(&mut msg);

    assert_eq!(TestMessage::default(), msg);
}

#[test]
fn test_dynamic() {
    let msg_desc = TestMessage::descriptor();
    let a_field = msg_desc.field_by_name("a").unwrap();
    let b_field = msg_desc.field_by_name("b").unwrap();
    let c_field = msg_desc.field_by_name("c").unwrap();
    let d_field = msg_desc.field_by_name("d").unwrap();
    let e_field = msg_desc.field_by_name("e").unwrap();

    let mut msg = msg_desc.new_instance();

    let mut map = TestMessage::default().e().clone();
    map.insert("key".to_string(), "value".to_string());

    a_field.set_singular_field(msg.as_mut(), 1.into());
    b_field.set_singular_field(msg.as_mut(), "b".to_string().into());
    c_field.set_singular_field(
        msg.as_mut(),
        ReflectValueBox::Message(Box::new(test_message::Nested::default())),
    );
    let mut d_repeated = d_field.mut_repeated(msg.as_mut());
    d_repeated.push(1.into());

    let mut e_map = e_field.mut_map(msg.as_mut());
    e_map.insert("key".to_string().into(), "value".to_string().into());

    a_field.clear_field(msg.as_mut());
    b_field.clear_field(msg.as_mut());
    c_field.clear_field(msg.as_mut());
    d_field.clear_field(msg.as_mut());
    e_field.clear_field(msg.as_mut());

    let msg_empty = msg_desc.new_instance();
    assert_eq!(msg.to_string(), msg_empty.to_string());
}

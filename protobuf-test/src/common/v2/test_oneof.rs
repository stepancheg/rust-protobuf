use protobuf_test_common::*;

use super::test_oneof_pb::*;

#[test]
fn test_simple() {
    let mut test_message = TestOneof::new();
    test_message.set_uint32_field(150);
    test_serialize_deserialize("28 96 01", &test_message);
}

#[test]
fn test_set_clear_field() {
    let mut test_message = TestOneof::new();

    test_message.set_int32_field(10);
    assert!(test_message.has_int32_field());
    assert_eq!(10, test_message.get_int32_field());
    assert!(!test_message.has_bool_field());
    assert_eq!(false, test_message.get_bool_field());

    test_message.set_bool_field(true);
    assert!(test_message.has_bool_field());
    assert_eq!(true, test_message.get_bool_field());
    assert!(!test_message.has_int32_field());
    assert_eq!(0, test_message.get_int32_field());

    test_message.clear_int32_field();
    assert!(!test_message.has_int32_field());
    assert!(!test_message.has_bool_field());
    assert_eq!(false, test_message.get_bool_field());
    assert_eq!(0, test_message.get_int32_field());
}

#[test]
fn test_types() {
    fn t<F>(f: F)
    where
        F : Fn(&mut TestOneof),
    {
        let mut o = TestOneof::new();
        f(&mut o);
        test_serialize_deserialize_no_hex(&o);
    }

    t(|o| o.set_double_field(10.0));
    t(|o| o.set_float_field(11.0));
    t(|o| o.set_int32_field(12));
    t(|o| o.set_int64_field(13));
    t(|o| o.set_uint32_field(14));
    t(|o| o.set_uint64_field(15));
    t(|o| o.set_sint32_field(16));
    t(|o| o.set_sint64_field(17));
    t(|o| o.set_fixed32_field(18));
    t(|o| o.set_fixed64_field(19));
    t(|o| o.set_sfixed32_field(20));
    t(|o| o.set_sfixed64_field(21));
    t(|o| o.set_bool_field(true));
    t(|o| o.set_string_field("asas".to_string()));
    t(|o| o.set_bytes_field(vec![99, 100]));
    t(|o| o.set_enum_field(EnumForOneof::A));
    t(|o| o.mut_message_field().set_f(22));
}

use protobuf::reflect::ReflectValueBox;
use protobuf::reflect::{FieldDescriptor, ReflectFieldRef, ReflectValueRef};
use protobuf::Message;
use protobuf::ProtobufEnum;

use super::test_reflect_pb::*;
use protobuf::reflect::RuntimeFieldType;

use protobuf_test_common::value_for_runtime_type;

#[test]
fn test_get_sub_message_via_reflection() {
    let mut m = M::new();
    m.mut_sub_m().set_n(42);
    assert!(m.has_sub_m());

    let descriptor = m.descriptor().get_field_by_name("sub_m").unwrap();
    assert_eq!("sub_m", descriptor.name());

    let sub_m = descriptor.get_message(&m);
    assert_eq!("test_reflect.SubM", sub_m.descriptor().full_name());
    assert_eq!(
        42,
        sub_m
            .descriptor()
            .get_field_by_name("n")
            .unwrap()
            .get_singular_field_or_default(sub_m)
            .to_i32()
            .unwrap()
    );
}

#[test]
fn test_singular_basic() {
    let mut message = TestTypesSingular::new();
    let descriptor = message.descriptor();

    let bool_field = descriptor.get_field_by_name("bool_field").unwrap();
    assert!(!bool_field.has_field(&message));

    bool_field.set_singular_field(&mut message, ReflectValueBox::Bool(true));
    assert!(bool_field.has_field(&message));
    assert_eq!(
        true,
        bool_field
            .get_singular_field_or_default(&message)
            .to_bool()
            .unwrap()
    );
}

fn test_singular_field(message: &mut dyn Message, field: &FieldDescriptor) {
    assert!(!field.has_field(message));

    // should not crash
    field.get_singular_field_or_default(message);

    let value = value_for_runtime_type(field.singular_runtime_type());
    field.set_singular_field(message, value);
}

#[test]
fn test_singular() {
    let mut message = TestTypesSingular::new();
    let descriptor = message.descriptor();

    for field in descriptor.fields() {
        test_singular_field(&mut message, field);
    }
}

#[test]
fn test_repeated_debug() {
    let mut message = TestTypesRepeated::new();
    message.set_int32_field(vec![10, 20, 30]);
    let field = message
        .descriptor()
        .get_field_by_name("int32_field")
        .unwrap()
        .get_repeated(&message);
    assert_eq!("[10, 20, 30]", format!("{:?}", field));
}

fn test_repeated_field(message: &mut dyn Message, field: &FieldDescriptor) {
    assert!(!field.has_field(message));

    let mut expected = Vec::new();

    // test mut interface
    {
        let mut repeated = field.mut_repeated(message);

        for i in 0..3 {
            let value = value_for_runtime_type(repeated.element_type());
            expected.push(value.clone());
            repeated.push(value.clone());
            let fetched = repeated.get(i);
            assert_eq!(value, fetched);
        }

        assert_eq!(expected, repeated);
        assert_eq!(repeated, expected);
    }

    // test read interface
    {
        let repeated = field.get_repeated(message);
        assert_eq!(3, repeated.len());

        assert_eq!(expected, repeated);
        assert_eq!(repeated, expected);
    }
}

#[test]
fn test_repeated() {
    let mut message = TestTypesRepeated::new();
    let descriptor = message.descriptor();

    for field in descriptor.fields() {
        test_repeated_field(&mut message, field);
    }
}

fn test_map_field(message: &mut dyn Message, field: &FieldDescriptor) {
    assert!(field.get_map(message).is_empty());
    assert_eq!(0, field.get_map(message).len());
    assert!(field.mut_map(message).is_empty());
    assert_eq!(0, field.mut_map(message).len());

    let (k, v) = match field.runtime_field_type() {
        RuntimeFieldType::Map(k, v) => (k, v),
        _ => panic!("not a map"),
    };

    {
        let map = field.get_map(message);
        assert!(map.is_empty());
        assert_eq!(0, map.len());

        assert_eq!(None, map.get(value_for_runtime_type(k).as_value_ref()));
    }

    {
        let mut map = field.mut_map(message);
        assert!(map.is_empty());
        assert_eq!(0, map.len());

        assert_eq!(None, map.get(value_for_runtime_type(k).as_value_ref()));

        let key = value_for_runtime_type(k);
        let value = value_for_runtime_type(v);

        map.insert(key.clone(), value.clone());

        assert_eq!(Some(value.as_value_ref()), map.get(key.as_value_ref()));

        assert_eq!(1, map.len());
    }
}

#[test]
fn test_map() {
    let mut message = TestTypesMap::new();
    let descriptor = message.descriptor();

    for field in descriptor.fields() {
        test_map_field(&mut message, field);
    }
}

#[test]
fn test_nested_message() {
    assert_eq!(
        "test_reflect.WithNestedMessage.NestedMessage",
        with_nested_message::NestedMessage::descriptor_static().full_name()
    );
}

#[test]
fn test_nested_enum() {
    assert_eq!(
        "test_reflect.WithNestedMessage.NestedEnum",
        with_nested_message::NestedEnum::enum_descriptor_static().full_name()
    );
}

#[test]
fn test_mut_message() {
    let mut m = TestTypesSingular::new();
    {
        let message_field_field = m.descriptor().get_field_by_name("message_field").unwrap();
        let sub_m = message_field_field.mut_message(&mut m);
        let n_field = sub_m.descriptor().get_field_by_name("n").unwrap();
        n_field.set_singular_field(sub_m, ReflectValueBox::I32(10));
        // TODO: test `mut_message` works for oneof fields
    }
    assert_eq!(10, m.get_message_field().get_n());
}

#[test]
fn test_get_reflect_singular() {
    let mut m = TestTypesSingular::new();
    m.set_int64_field(10);
    let f = m.descriptor().get_field_by_name("int64_field").unwrap();
    match f.get_reflect(&m) {
        ReflectFieldRef::Optional(Some(ReflectValueRef::I64(10))) => {}
        _ => panic!(),
    }
}

#[test]
fn test_get_reflect_repeated() {
    let mut m = TestTypesRepeated::new();
    m.set_int64_field(vec![10, 20]);
    let f = m.descriptor().get_field_by_name("int64_field").unwrap();
    match f.get_reflect(&m) {
        ReflectFieldRef::Repeated(repeated) => {
            assert_eq!(2, repeated.len());
            assert_eq!(ReflectValueRef::I64(10), repeated.get(0));
            assert_eq!(ReflectValueRef::I64(20), repeated.get(1));
        }
        _ => panic!(),
    }
}

#[test]
fn test_get_reflect_map() {
    let mut m = TestTypesMap::new();
    m.set_int64_field(vec![(10, 33), (20, 44)].into_iter().collect());
    let f = m.descriptor().get_field_by_name("int64_field").unwrap();
    match f.get_reflect(&m) {
        ReflectFieldRef::Map(map) => {
            assert_eq!(2, map.len());
            assert_eq!(
                Some(ReflectValueRef::I64(33)),
                map.get(ReflectValueRef::I64(10))
            );
            assert_eq!(
                Some(ReflectValueRef::I64(44)),
                map.get(ReflectValueRef::I64(20))
            );
        }
        _ => panic!(),
    }
}

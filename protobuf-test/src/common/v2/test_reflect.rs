use protobuf::Message;
use protobuf::reflect::ReflectValueBox;
use protobuf::reflect::FieldDescriptor;
use protobuf::reflect::RuntimeTypeDynamic;
use protobuf::reflect::RuntimeTypeBox;

use super::test_reflect_pb::*;


#[test]
fn test_get_sub_message_via_reflection() {
    let mut m = M::new();
    m.mut_sub_m().set_n(42);
    assert!(m.has_sub_m());

    let descriptor = m.descriptor().field_by_name("sub_m");
    assert_eq!("sub_m", descriptor.name());

    let sub_m = descriptor.get_message(&m);
    assert_eq!("test_reflect.SubM", sub_m.descriptor().full_name());
    assert_eq!(42, sub_m.descriptor().field_by_name("n").get_i32(sub_m));
}

#[test]
fn test_singular_basic() {
    let mut message = TestTypesSingular::new();
    let descriptor = message.descriptor();

    let bool_field = descriptor.field_by_name("bool_field");
    assert!(!bool_field.has_field(&message));

    bool_field.set_singular_field(&mut message, ReflectValueBox::Bool(true));
    assert!(bool_field.has_field(&message));
    assert_eq!(true, bool_field.get_bool(&message));
}

fn value_for_runtime_type(field_type: &RuntimeTypeDynamic) -> ReflectValueBox {
    match field_type.runtime_type_box() {
        RuntimeTypeBox::U32 => ReflectValueBox::U32(11),
        RuntimeTypeBox::U64 => ReflectValueBox::U64(12),
        RuntimeTypeBox::I32 => ReflectValueBox::I32(13),
        RuntimeTypeBox::I64 => ReflectValueBox::I64(14),
        RuntimeTypeBox::F32 => ReflectValueBox::F32(15.5),
        RuntimeTypeBox::F64 => ReflectValueBox::F64(16.5),
        RuntimeTypeBox::Bool => ReflectValueBox::Bool(true),
        RuntimeTypeBox::String |
        RuntimeTypeBox::Chars => ReflectValueBox::String("here".to_owned()),
        RuntimeTypeBox::VecU8 |
        RuntimeTypeBox::CarllercheBytes => ReflectValueBox::Bytes(b"there".as_ref().to_owned()),
        RuntimeTypeBox::Enum(e) => ReflectValueBox::Enum(&e.values()[0]),
        RuntimeTypeBox::Message(m) => ReflectValueBox::Message(m.new_instance()),
    }
}

fn test_singular_field(message: &mut Message, field: &FieldDescriptor) {
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
    let field = message.descriptor().field_by_name("int32_field").get_repeated(&message);
    assert_eq!("[10, 20, 30]", format!("{:?}", field));
}

fn test_repeated_field(message: &mut Message, field: &FieldDescriptor) {
    assert_eq!(0, field.len_field(message));
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


fn test_map_field(message: &mut Message, field: &FieldDescriptor) {
    assert!(field.get_map(message).is_empty());
    assert_eq!(0, field.get_map(message).len());
    assert!(field.mut_map(message).is_empty());
    assert_eq!(0, field.mut_map(message).len());

    // TODO: insert/query
}

#[test]
fn test_map() {
    let mut message = TestTypesMap::new();
    let descriptor = message.descriptor();

    for field in descriptor.fields() {
        test_map_field(&mut message, field);
    }
}

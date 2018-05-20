use protobuf::Message;
use protobuf::reflect::ReflectValueBox;
use protobuf::reflect::FieldDescriptor;
use protobuf::descriptor::FieldDescriptorProto_Type;

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

fn value_for_type(t: FieldDescriptorProto_Type) -> ReflectValueBox {
    match t {
        FieldDescriptorProto_Type::TYPE_DOUBLE => ReflectValueBox::F64(11.0),
        FieldDescriptorProto_Type::TYPE_FLOAT => ReflectValueBox::F32(12.0),
        FieldDescriptorProto_Type::TYPE_INT32    |
        FieldDescriptorProto_Type::TYPE_SFIXED32 |
        FieldDescriptorProto_Type::TYPE_SINT32  => ReflectValueBox::I32(13),
        FieldDescriptorProto_Type::TYPE_UINT32   |
        FieldDescriptorProto_Type::TYPE_FIXED32 => ReflectValueBox::U32(14),
        FieldDescriptorProto_Type::TYPE_INT64    |
        FieldDescriptorProto_Type::TYPE_SFIXED64 |
        FieldDescriptorProto_Type::TYPE_SINT64  => ReflectValueBox::I64(13),
        FieldDescriptorProto_Type::TYPE_UINT64   |
        FieldDescriptorProto_Type::TYPE_FIXED64 => ReflectValueBox::U64(14),
        FieldDescriptorProto_Type::TYPE_BOOL => ReflectValueBox::Bool(true),
        FieldDescriptorProto_Type::TYPE_STRING => ReflectValueBox::String("aa".to_owned()),
        FieldDescriptorProto_Type::TYPE_BYTES => ReflectValueBox::Bytes(b"bb".as_ref().to_owned()),
        t => panic!("cannot generated value for type: {:?}", t),
    }
}

fn value_for_field(field: &FieldDescriptor) -> ReflectValueBox {
    match field.proto().get_field_type() {
        FieldDescriptorProto_Type::TYPE_ENUM => {
            ReflectValueBox::Enum(&field.enum_descriptor().values()[0])
        }
        FieldDescriptorProto_Type::TYPE_MESSAGE => {
            ReflectValueBox::Message(field.message_descriptor().new_instance())
        }
        t => value_for_type(t),
    }
}

fn test_field(message: &mut Message, field: &FieldDescriptor) {
    assert!(!field.has_field(message));

    // should not crash
    field.get_singular_field_or_default(message);

    let value = value_for_field(field);
    field.set_singular_field(message, value);
}

#[test]
fn test_singular() {
    let mut message = TestTypesSingular::new();
    let descriptor = message.descriptor();

    for field in descriptor.fields() {
        test_field(&mut message, field);
    }
}

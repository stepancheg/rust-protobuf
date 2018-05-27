use protobuf::reflect::RuntimeTypeDynamic;
use protobuf::reflect::ReflectValueBox;
use protobuf::reflect::RuntimeTypeBox;


pub fn value_for_runtime_type(field_type: &RuntimeTypeDynamic) -> ReflectValueBox {
    match field_type.to_box() {
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


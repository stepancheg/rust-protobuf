use protobuf::reflect::FieldDescriptor;
use protobuf::reflect::MessageDescriptor;
use protobuf::reflect::ReflectValueBox;
use protobuf::reflect::RuntimeFieldType;
use protobuf::reflect::RuntimeType;
use protobuf::well_known_types::struct_::value;
use protobuf::well_known_types::struct_::Value;
use protobuf::MessageDyn;
use protobuf::MessageFull;

pub fn value_for_runtime_type(field_type: &RuntimeType) -> ReflectValueBox {
    match field_type {
        RuntimeType::U32 => ReflectValueBox::U32(11),
        RuntimeType::U64 => ReflectValueBox::U64(12),
        RuntimeType::I32 => ReflectValueBox::I32(13),
        RuntimeType::I64 => ReflectValueBox::I64(14),
        RuntimeType::F32 => ReflectValueBox::F32(15.5),
        RuntimeType::F64 => ReflectValueBox::F64(16.5),
        RuntimeType::Bool => ReflectValueBox::Bool(true),
        RuntimeType::String => ReflectValueBox::String("here".to_owned()),
        RuntimeType::VecU8 => ReflectValueBox::Bytes(b"there".as_ref().to_owned()),
        RuntimeType::Enum(e) => ReflectValueBox::from(e.default_value()),
        RuntimeType::Message(m) => ReflectValueBox::Message(m.new_instance()),
    }
}

fn values_for_message_type(descriptor: &MessageDescriptor) -> Vec<Box<dyn MessageDyn>> {
    if descriptor == &Value::descriptor() {
        // special handling because empty `Value` is not valid
        let mut value = Value::new();
        value.kind = Some(value::Kind::NumberValue(23.0));
        vec![Box::new(value)]
    } else {
        vec![
            // TODO: populated messages
            descriptor.new_instance(),
        ]
    }
}

pub fn values_for_runtime_type(field_type: &RuntimeType) -> Vec<ReflectValueBox> {
    match field_type {
        RuntimeType::U32 => vec![
            ReflectValueBox::U32(11),
            ReflectValueBox::U32(0),
            ReflectValueBox::U32(0x7fff_ffff),
            ReflectValueBox::U32(0x8000_0000),
            ReflectValueBox::U32(0xffff_ffff),
        ],
        RuntimeType::U64 => vec![
            ReflectValueBox::U64(12),
            ReflectValueBox::U64(0),
            ReflectValueBox::U64(0xffff_ffff),
            ReflectValueBox::U64(0xffff_ffff_ffff_ffff),
        ],
        RuntimeType::I32 => vec![
            ReflectValueBox::I32(13),
            ReflectValueBox::I32(0),
            ReflectValueBox::I32(-1),
            ReflectValueBox::I32(0x7fff_ffff),
            ReflectValueBox::I32(0x1000_0000),
        ],
        RuntimeType::I64 => vec![
            ReflectValueBox::I64(14),
            ReflectValueBox::I64(0),
            ReflectValueBox::I64(-1),
            ReflectValueBox::I64(0x7fff_ffff_ffff_ffff),
            ReflectValueBox::I64(0x1000_0000_0000_0000),
        ],
        RuntimeType::F32 => vec![
            ReflectValueBox::F32(15.5),
            ReflectValueBox::F32(0.0),
            ReflectValueBox::F32(-0.0),
            ReflectValueBox::F32(1. / 3.0),
            ReflectValueBox::F32(-1. / 3.0),
            ReflectValueBox::F32(f32::NAN),
            ReflectValueBox::F32(f32::INFINITY),
            ReflectValueBox::F32(-f32::INFINITY),
        ],
        RuntimeType::F64 => vec![
            ReflectValueBox::F64(16.5),
            ReflectValueBox::F64(0.0),
            ReflectValueBox::F64(-0.0),
            ReflectValueBox::F64(1.0 / 3.0),
            ReflectValueBox::F64(-1.0 / 3.0),
            ReflectValueBox::F64(f64::NAN),
            ReflectValueBox::F64(f64::INFINITY),
            ReflectValueBox::F64(-f64::INFINITY),
        ],
        RuntimeType::Bool => vec![ReflectValueBox::Bool(true), ReflectValueBox::Bool(false)],
        RuntimeType::String => vec![
            ReflectValueBox::String("here".to_owned()),
            ReflectValueBox::String("".to_owned()),
            ReflectValueBox::String(" \t\n".to_owned()),
            ReflectValueBox::String("\0".to_owned()),
        ],
        RuntimeType::VecU8 => vec![
            ReflectValueBox::Bytes(b"there".as_ref().to_owned()),
            ReflectValueBox::Bytes(b"".as_ref().to_owned()),
        ],
        RuntimeType::Enum(e) => vec![
            ReflectValueBox::from(e.values().next().unwrap()),
            ReflectValueBox::from(e.values().last().unwrap()),
        ],
        RuntimeType::Message(m) => values_for_message_type(&m)
            .into_iter()
            .map(ReflectValueBox::from)
            .collect(),
    }
}

pub fn special_values_for_field(
    f: &FieldDescriptor,
    d: &MessageDescriptor,
) -> Vec<Box<dyn MessageDyn>> {
    let mut r = Vec::new();
    match f.runtime_field_type() {
        RuntimeFieldType::Singular(t) => {
            for v in values_for_runtime_type(&t) {
                let mut m = d.new_instance();
                f.set_singular_field(&mut *m, v);
                r.push(m);
            }
        }
        RuntimeFieldType::Repeated(t) => {
            // TODO: empty repeated
            // TODO: repeated of more than one element
            let mut m = d.new_instance();
            f.mut_repeated(&mut *m).push(value_for_runtime_type(&t));
            r.push(m);
        }
        RuntimeFieldType::Map(k, v) => {
            // TODO: empty map
            // TODO: map of more than one element
            let mut m = d.new_instance();
            let k = value_for_runtime_type(&k);
            let v = value_for_runtime_type(&v);
            f.mut_map(&mut *m).insert(k, v);
            r.push(m);
        }
    }
    r
}

pub fn special_messages(d: &MessageDescriptor) -> Vec<Box<dyn MessageDyn>> {
    let mut r = Vec::new();
    for f in d.fields() {
        r.extend(special_values_for_field(&f, d));
    }
    r
}

pub fn special_messages_typed<M: MessageFull>() -> Vec<M> {
    let mut r = Vec::new();
    for m in special_messages(&M::descriptor()) {
        r.push(*m.downcast_box().unwrap());
    }
    r
}

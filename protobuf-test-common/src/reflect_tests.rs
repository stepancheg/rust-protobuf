use std::f32;
use std::f64;

use protobuf::reflect::FieldDescriptor;
use protobuf::reflect::MessageDescriptor;
use protobuf::reflect::ReflectValueBox;
use protobuf::reflect::RuntimeFieldType;
use protobuf::reflect::RuntimeTypeBox;
use protobuf::reflect::RuntimeTypeDynamic;
use protobuf::well_known_types::value;
use protobuf::well_known_types::Value;
use protobuf::Message;

pub fn value_for_runtime_type(field_type: &dyn RuntimeTypeDynamic) -> ReflectValueBox {
    match field_type.to_box() {
        RuntimeTypeBox::U32 => ReflectValueBox::U32(11),
        RuntimeTypeBox::U64 => ReflectValueBox::U64(12),
        RuntimeTypeBox::I32 => ReflectValueBox::I32(13),
        RuntimeTypeBox::I64 => ReflectValueBox::I64(14),
        RuntimeTypeBox::F32 => ReflectValueBox::F32(15.5),
        RuntimeTypeBox::F64 => ReflectValueBox::F64(16.5),
        RuntimeTypeBox::Bool => ReflectValueBox::Bool(true),
        RuntimeTypeBox::String => ReflectValueBox::String("here".to_owned()),
        RuntimeTypeBox::VecU8 => ReflectValueBox::Bytes(b"there".as_ref().to_owned()),
        RuntimeTypeBox::Enum(e) => ReflectValueBox::from(&e.values()[0]),
        RuntimeTypeBox::Message(m) => ReflectValueBox::Message(m.new_instance()),
    }
}

fn values_for_message_type(descriptor: &MessageDescriptor) -> Vec<Box<dyn Message>> {
    if descriptor == Value::descriptor_static() {
        // special handling because empty `Value` is not valid
        let mut value = Value::new();
        value.kind = Some(value::Kind::number_value(23.0));
        vec![Box::new(value)]
    } else {
        vec![
            // TODO: populated messages
            descriptor.new_instance(),
        ]
    }
}

pub fn values_for_runtime_type(field_type: &dyn RuntimeTypeDynamic) -> Vec<ReflectValueBox> {
    match field_type.to_box() {
        RuntimeTypeBox::U32 => vec![
            ReflectValueBox::U32(11),
            ReflectValueBox::U32(0),
            ReflectValueBox::U32(0x7fff_ffff),
            ReflectValueBox::U32(0x8000_0000),
            ReflectValueBox::U32(0xffff_ffff),
        ],
        RuntimeTypeBox::U64 => vec![
            ReflectValueBox::U64(12),
            ReflectValueBox::U64(0),
            ReflectValueBox::U64(0xffff_ffff),
            ReflectValueBox::U64(0xffff_ffff_ffff_ffff),
        ],
        RuntimeTypeBox::I32 => vec![
            ReflectValueBox::I32(13),
            ReflectValueBox::I32(0),
            ReflectValueBox::I32(-1),
            ReflectValueBox::I32(0x7fff_ffff),
            ReflectValueBox::I32(0x1000_0000),
        ],
        RuntimeTypeBox::I64 => vec![
            ReflectValueBox::I64(14),
            ReflectValueBox::I64(0),
            ReflectValueBox::I64(-1),
            ReflectValueBox::I64(0x7fff_ffff_ffff_ffff),
            ReflectValueBox::I64(0x1000_0000_0000_0000),
        ],
        RuntimeTypeBox::F32 => vec![
            ReflectValueBox::F32(15.5),
            ReflectValueBox::F32(0.0),
            ReflectValueBox::F32(-0.0),
            ReflectValueBox::F32(1. / 3.0),
            ReflectValueBox::F32(-1. / 3.0),
            ReflectValueBox::F32(f32::NAN),
            ReflectValueBox::F32(f32::INFINITY),
            ReflectValueBox::F32(-f32::INFINITY),
        ],
        RuntimeTypeBox::F64 => vec![
            ReflectValueBox::F64(16.5),
            ReflectValueBox::F64(0.0),
            ReflectValueBox::F64(-0.0),
            ReflectValueBox::F64(1.0 / 3.0),
            ReflectValueBox::F64(-1.0 / 3.0),
            ReflectValueBox::F64(f64::NAN),
            ReflectValueBox::F64(f64::INFINITY),
            ReflectValueBox::F64(-f64::INFINITY),
        ],
        RuntimeTypeBox::Bool => vec![ReflectValueBox::Bool(true), ReflectValueBox::Bool(false)],
        RuntimeTypeBox::String => vec![
            ReflectValueBox::String("here".to_owned()),
            ReflectValueBox::String("".to_owned()),
            ReflectValueBox::String(" \t\n".to_owned()),
            ReflectValueBox::String("\0".to_owned()),
        ],
        RuntimeTypeBox::VecU8 => vec![
            ReflectValueBox::Bytes(b"there".as_ref().to_owned()),
            ReflectValueBox::Bytes(b"".as_ref().to_owned()),
        ],
        RuntimeTypeBox::Enum(e) => vec![
            ReflectValueBox::from(&e.values()[0]),
            ReflectValueBox::from(&e.values()[e.values().len() - 1]),
        ],
        RuntimeTypeBox::Message(m) => values_for_message_type(m)
            .into_iter()
            .map(ReflectValueBox::from)
            .collect(),
    }
}

pub fn special_values_for_field(
    f: &FieldDescriptor,
    d: &MessageDescriptor,
) -> Vec<Box<dyn Message>> {
    let mut r = Vec::new();
    match f.runtime_field_type() {
        RuntimeFieldType::Singular(t) => {
            for v in values_for_runtime_type(t) {
                let mut m = d.new_instance();
                f.set_singular_field(&mut *m, v);
                r.push(m);
            }
        }
        RuntimeFieldType::Repeated(t) => {
            // TODO: empty repeated
            // TODO: repeated of more than one element
            let mut m = d.new_instance();
            f.mut_repeated(&mut *m).push(value_for_runtime_type(t));
            r.push(m);
        }
        RuntimeFieldType::Map(k, v) => {
            // TODO: empty map
            // TODO: map of more than one element
            let mut m = d.new_instance();
            let k = value_for_runtime_type(k);
            let v = value_for_runtime_type(v);
            f.mut_map(&mut *m).insert(k, v);
            r.push(m);
        }
    }
    r
}

pub fn special_messages(d: &MessageDescriptor) -> Vec<Box<dyn Message>> {
    let mut r = Vec::new();
    for f in d.fields() {
        r.extend(special_values_for_field(f, d));
    }
    r
}

pub fn special_messages_typed<M: Message>() -> Vec<M> {
    let mut r = Vec::new();
    for m in special_messages(M::descriptor_static()) {
        r.push(*m.downcast_box().unwrap());
    }
    r
}

use crate::reflect::repeated::iter::ReflectRepeatedIter;
use crate::reflect::repeated::ReflectRepeated;
use crate::reflect::EnumDescriptor;
use crate::reflect::MessageDescriptor;
use crate::reflect::MessageRef;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeTypeBox;
use crate::MessageDyn;

/// Container of repeated values for dynamic messages.
///
/// It is logically similar to `Vec<ReflectValueBox>`, but:
/// * more efficient
/// * asserts all the elements are of the same type, the type which is specified at construction
#[derive(Debug, Clone)]
pub(crate) enum DynamicRepeated {
    U32(Vec<u32>),
    U64(Vec<u64>),
    I32(Vec<i32>),
    I64(Vec<i64>),
    F32(Vec<f32>),
    F64(Vec<f64>),
    Bool(Vec<bool>),
    String(Vec<String>),
    Bytes(Vec<Vec<u8>>),
    Enum(EnumDescriptor, Vec<i32>),
    Message(MessageDescriptor, Vec<Box<dyn MessageDyn>>),
}

impl ReflectRepeated for DynamicRepeated {
    fn reflect_iter(&self) -> ReflectRepeatedIter {
        match self {
            DynamicRepeated::U32(v) => ReflectRepeatedIter::new_slice(&v),
            DynamicRepeated::U64(v) => ReflectRepeatedIter::new_slice(&v),
            DynamicRepeated::I32(v) => ReflectRepeatedIter::new_slice(&v),
            DynamicRepeated::I64(v) => ReflectRepeatedIter::new_slice(&v),
            DynamicRepeated::F32(v) => ReflectRepeatedIter::new_slice(&v),
            DynamicRepeated::F64(v) => ReflectRepeatedIter::new_slice(&v),
            DynamicRepeated::Bool(v) => ReflectRepeatedIter::new_slice(&v),
            DynamicRepeated::String(v) => ReflectRepeatedIter::new_slice(&v),
            DynamicRepeated::Bytes(v) => ReflectRepeatedIter::new_slice(&v),
            _ => unimplemented!(),
        }
    }

    fn len(&self) -> usize {
        match self {
            DynamicRepeated::U32(v) => v.len(),
            DynamicRepeated::U64(v) => v.len(),
            DynamicRepeated::I32(v) => v.len(),
            DynamicRepeated::I64(v) => v.len(),
            DynamicRepeated::F32(v) => v.len(),
            DynamicRepeated::F64(v) => v.len(),
            DynamicRepeated::Bool(v) => v.len(),
            DynamicRepeated::String(v) => v.len(),
            DynamicRepeated::Bytes(v) => v.len(),
            DynamicRepeated::Enum(.., v) => v.len(),
            DynamicRepeated::Message(.., v) => v.len(),
        }
    }

    fn get(&self, index: usize) -> ReflectValueRef {
        match self {
            DynamicRepeated::U32(v) => ReflectValueRef::U32(v[index]),
            DynamicRepeated::U64(v) => ReflectValueRef::U64(v[index]),
            DynamicRepeated::I32(v) => ReflectValueRef::I32(v[index]),
            DynamicRepeated::I64(v) => ReflectValueRef::I64(v[index]),
            DynamicRepeated::F32(v) => ReflectValueRef::F32(v[index]),
            DynamicRepeated::F64(v) => ReflectValueRef::F64(v[index]),
            DynamicRepeated::Bool(v) => ReflectValueRef::Bool(v[index]),
            DynamicRepeated::String(v) => ReflectValueRef::String(&v[index]),
            DynamicRepeated::Bytes(v) => ReflectValueRef::Bytes(&v[index]),
            DynamicRepeated::Enum(descriptor, v) => {
                ReflectValueRef::Enum(descriptor.clone(), v[index])
            }
            DynamicRepeated::Message(_descriptor, v) => {
                ReflectValueRef::Message(MessageRef::from(&*v[index]))
            }
        }
    }

    fn set(&mut self, index: usize, value: ReflectValueBox) {
        match self {
            DynamicRepeated::U32(v) => v.set(index, value),
            DynamicRepeated::U64(v) => v.set(index, value),
            DynamicRepeated::I32(v) => v.set(index, value),
            DynamicRepeated::I64(v) => v.set(index, value),
            DynamicRepeated::F32(v) => v.set(index, value),
            DynamicRepeated::F64(v) => v.set(index, value),
            DynamicRepeated::Bool(v) => v.set(index, value),
            DynamicRepeated::String(v) => v.set(index, value),
            DynamicRepeated::Bytes(v) => v.set(index, value),
            DynamicRepeated::Enum(descriptor, vs) => match value {
                ReflectValueBox::Enum(value_description, v) => {
                    assert_eq!(*descriptor, value_description);
                    vs[index] = v;
                }
                _ => panic!("Expected enum value"),
            },
            DynamicRepeated::Message(descriptor, vs) => match value {
                ReflectValueBox::Message(message) => {
                    assert_eq!(*descriptor, message.descriptor_dyn());
                    vs[index] = message;
                }
                _ => panic!("Expected message value"),
            },
        }
    }

    fn push(&mut self, value: ReflectValueBox) {
        match self {
            DynamicRepeated::U32(vs) => ReflectRepeated::push(vs, value),
            DynamicRepeated::U64(vs) => ReflectRepeated::push(vs, value),
            DynamicRepeated::I32(vs) => ReflectRepeated::push(vs, value),
            DynamicRepeated::I64(vs) => ReflectRepeated::push(vs, value),
            DynamicRepeated::F32(vs) => ReflectRepeated::push(vs, value),
            DynamicRepeated::F64(vs) => ReflectRepeated::push(vs, value),
            DynamicRepeated::Bool(vs) => ReflectRepeated::push(vs, value),
            DynamicRepeated::String(vs) => ReflectRepeated::push(vs, value),
            DynamicRepeated::Bytes(vs) => ReflectRepeated::push(vs, value),
            DynamicRepeated::Enum(descriptor, vs) => match value {
                ReflectValueBox::Enum(value_description, v) => {
                    assert_eq!(*descriptor, value_description);
                    vs.push(v);
                }
                _ => panic!("Expected enum value"),
            },
            DynamicRepeated::Message(descriptor, vs) => match value {
                ReflectValueBox::Message(message) => {
                    assert_eq!(*descriptor, message.descriptor_dyn());
                    vs.push(message);
                }
                _ => panic!("Expected message value"),
            },
        }
    }

    fn clear(&mut self) {
        match self {
            DynamicRepeated::U32(vs) => vs.clear(),
            DynamicRepeated::U64(vs) => vs.clear(),
            DynamicRepeated::I32(vs) => vs.clear(),
            DynamicRepeated::I64(vs) => vs.clear(),
            DynamicRepeated::F32(vs) => vs.clear(),
            DynamicRepeated::F64(vs) => vs.clear(),
            DynamicRepeated::Bool(vs) => vs.clear(),
            DynamicRepeated::String(vs) => vs.clear(),
            DynamicRepeated::Bytes(vs) => vs.clear(),
            DynamicRepeated::Enum(_descriptor, vs) => vs.clear(),
            DynamicRepeated::Message(_descriptor, vs) => vs.clear(),
        }
    }

    fn element_type(&self) -> RuntimeTypeBox {
        match self {
            DynamicRepeated::U32(..) => RuntimeTypeBox::U32,
            DynamicRepeated::U64(..) => RuntimeTypeBox::U64,
            DynamicRepeated::I32(..) => RuntimeTypeBox::I32,
            DynamicRepeated::I64(..) => RuntimeTypeBox::I64,
            DynamicRepeated::F32(..) => RuntimeTypeBox::F32,
            DynamicRepeated::F64(..) => RuntimeTypeBox::F64,
            DynamicRepeated::Bool(..) => RuntimeTypeBox::Bool,
            DynamicRepeated::String(..) => RuntimeTypeBox::String,
            DynamicRepeated::Bytes(..) => RuntimeTypeBox::VecU8,
            DynamicRepeated::Enum(descriptor, _vs) => RuntimeTypeBox::Enum(descriptor.clone()),
            DynamicRepeated::Message(descriptor, _vs) => {
                RuntimeTypeBox::Message(descriptor.clone())
            }
        }
    }

    fn data_enum_values(&self) -> &[i32] {
        match self {
            DynamicRepeated::Enum(_descriptor, vs) => &vs,
            _ => panic!("Expected enum value"),
        }
    }

    fn data_bool(&self) -> &[bool] {
        match self {
            DynamicRepeated::Bool(vs) => &vs,
            _ => panic!("Expected bool value"),
        }
    }

    fn data_u32(&self) -> &[u32] {
        match self {
            DynamicRepeated::U32(vs) => &vs,
            _ => panic!("Expected u32 value"),
        }
    }

    fn data_u64(&self) -> &[u64] {
        match self {
            DynamicRepeated::U64(vs) => &vs,
            _ => panic!("Expected u64 value"),
        }
    }

    fn data_i32(&self) -> &[i32] {
        match self {
            DynamicRepeated::I32(vs) => &vs,
            _ => panic!("Expected i32 value"),
        }
    }

    fn data_i64(&self) -> &[i64] {
        match self {
            DynamicRepeated::I64(vs) => &vs,
            _ => panic!("Expected i64 value"),
        }
    }

    fn data_f32(&self) -> &[f32] {
        match self {
            DynamicRepeated::F32(vs) => &vs,
            _ => panic!("Expected f32 value"),
        }
    }

    fn data_f64(&self) -> &[f64] {
        match self {
            DynamicRepeated::F64(vs) => &vs,
            _ => panic!("Expected f64 value"),
        }
    }
}

impl DynamicRepeated {
    pub fn new(elem: RuntimeTypeBox) -> DynamicRepeated {
        match elem {
            RuntimeTypeBox::U32 => DynamicRepeated::U32(Vec::new()),
            RuntimeTypeBox::U64 => DynamicRepeated::U64(Vec::new()),
            RuntimeTypeBox::I32 => DynamicRepeated::I32(Vec::new()),
            RuntimeTypeBox::I64 => DynamicRepeated::I64(Vec::new()),
            RuntimeTypeBox::F32 => DynamicRepeated::F32(Vec::new()),
            RuntimeTypeBox::F64 => DynamicRepeated::F64(Vec::new()),
            RuntimeTypeBox::Bool => DynamicRepeated::Bool(Vec::new()),
            RuntimeTypeBox::String => DynamicRepeated::String(Vec::new()),
            RuntimeTypeBox::VecU8 => DynamicRepeated::Bytes(Vec::new()),
            RuntimeTypeBox::Enum(enum_descriptor) => {
                DynamicRepeated::Enum(enum_descriptor, Vec::new())
            }
            RuntimeTypeBox::Message(message_descriptor) => {
                DynamicRepeated::Message(message_descriptor, Vec::new())
            }
        }
    }
}

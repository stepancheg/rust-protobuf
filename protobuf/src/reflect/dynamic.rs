use crate::cached_size::CachedSize;
use crate::reflect::repeated::{ReflectRepeated, ReflectRepeatedIter};
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueBox;
use crate::reflect::RuntimeFieldType;
use crate::reflect::RuntimeTypeBox;
use crate::reflect::{FieldDescriptor, MessageDescriptor};
use crate::reflect::{ReflectFieldRef, ReflectValueRef};
use crate::Clear;
use crate::CodedInputStream;
use crate::CodedOutputStream;
use crate::Message;
use crate::ProtobufResult;
use crate::UnknownFields;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct DynamicOptional {
    elem: RuntimeTypeBox,
    value: Option<ReflectValueBox>,
}

impl DynamicOptional {
    fn none(elem: RuntimeTypeBox) -> DynamicOptional {
        DynamicOptional { elem, value: None }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DynamicRepeated {
    elem: RuntimeTypeBox,
    vec: Vec<ReflectValueBox>,
}

impl ReflectRepeated for DynamicRepeated {
    fn reflect_iter(&self) -> ReflectRepeatedIter {
        unimplemented!()
    }

    fn len(&self) -> usize {
        self.vec.len()
    }

    fn get(&self, index: usize) -> ReflectValueRef {
        self.vec[index].as_value_ref()
    }

    fn set(&mut self, index: usize, value: ReflectValueBox) {
        unimplemented!()
    }

    fn push(&mut self, value: ReflectValueBox) {
        unimplemented!()
    }

    fn clear(&mut self) {
        self.vec.clear();
    }

    fn element_type(&self) -> RuntimeTypeBox {
        self.elem.clone()
    }
}

impl DynamicRepeated {
    fn new(elem: RuntimeTypeBox) -> DynamicRepeated {
        DynamicRepeated {
            elem,
            vec: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn get(&self, index: usize) -> ReflectValueRef {
        self.vec[index].as_value_ref()
    }

    pub fn element_type(&self) -> RuntimeTypeBox {
        self.elem.clone()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DynamicMap {
    key: RuntimeTypeBox,
    value: RuntimeTypeBox,
    map: HashMap<ReflectValueBox, ReflectValueBox>,
}

impl DynamicMap {
    fn new(key: RuntimeTypeBox, value: RuntimeTypeBox) -> DynamicMap {
        DynamicMap {
            key,
            value,
            map: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum DynamicFieldValue {
    Singular(DynamicOptional),
    Repeated(DynamicRepeated),
    Map(DynamicMap),
}

impl DynamicFieldValue {
    fn as_ref(&self) -> ReflectFieldRef {
        match self {
            DynamicFieldValue::Singular(_v) => unimplemented!(),
            DynamicFieldValue::Repeated(_r) => unimplemented!(),
            DynamicFieldValue::Map(_r) => unimplemented!(),
        }
    }
}

impl DynamicFieldValue {
    fn default_static_for_field(field: &FieldDescriptor) -> &'static DynamicFieldValue {
        unimplemented!()
    }

    fn default_for_field(field: &FieldDescriptor) -> DynamicFieldValue {
        match field.runtime_field_type() {
            RuntimeFieldType::Singular(s) => {
                DynamicFieldValue::Singular(DynamicOptional::none(s.to_box()))
            }
            RuntimeFieldType::Repeated(r) => {
                DynamicFieldValue::Repeated(DynamicRepeated::new(r.to_box()))
            }
            RuntimeFieldType::Map(k, v) => {
                DynamicFieldValue::Map(DynamicMap::new(k.to_box(), v.to_box()))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct DynamicMessage {
    descriptor: MessageDescriptor,
    fields: Box<[DynamicFieldValue]>,
    unknown_fields: UnknownFields,
    cached_size: CachedSize,
}

impl DynamicMessage {
    pub(crate) fn new(descriptor: MessageDescriptor) -> DynamicMessage {
        DynamicMessage {
            descriptor,
            fields: Vec::new().into_boxed_slice(),
            unknown_fields: UnknownFields::new(),
            cached_size: CachedSize::new(),
        }
    }

    fn init_fields(&mut self) {
        if self.fields.is_empty() {
            self.fields = self
                .descriptor
                .fields()
                .into_iter()
                .map(|f| DynamicFieldValue::default_for_field(&f))
                .collect();
        }
    }

    fn get_field<'a>(&'a self, field: &FieldDescriptor) -> ReflectFieldRef<'a> {
        assert!(self.descriptor == field.message_descriptor);
        if self.fields.is_empty() {
            ReflectFieldRef::default_for_field(field)
        } else {
            self.fields[field.index].as_ref()
        }
    }
}

impl ProtobufValue for DynamicMessage {}

impl Clear for DynamicMessage {
    fn clear(&mut self) {
        unimplemented!()
    }
}

impl Message for DynamicMessage {
    fn descriptor(&self) -> MessageDescriptor {
        self.descriptor.clone()
    }

    fn is_initialized(&self) -> bool {
        unimplemented!()
    }

    fn merge_from(&mut self, _is: &mut CodedInputStream) -> ProtobufResult<()> {
        unimplemented!()
    }

    fn write_to_with_cached_sizes(&self, _os: &mut CodedOutputStream) -> ProtobufResult<()> {
        unimplemented!()
    }

    fn compute_size(&self) -> u32 {
        unimplemented!()
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut UnknownFields {
        &mut self.unknown_fields
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        panic!("DynamicMessage cannot be constructed directly")
    }

    fn default_instance() -> &'static Self
    where
        Self: Sized,
    {
        panic!("There's no default instance for dynamic message")
    }
}

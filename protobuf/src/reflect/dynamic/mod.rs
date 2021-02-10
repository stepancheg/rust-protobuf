use crate::cached_size::CachedSize;
use crate::message_dyn::MessageDyn;
use crate::reflect::dynamic::map::DynamicMap;
use crate::reflect::dynamic::optional::DynamicOptional;
use crate::reflect::dynamic::repeated::DynamicRepeated;
use crate::reflect::map::ReflectMap;
use crate::reflect::repeated::ReflectRepeated;
use crate::reflect::value::value_ref::ReflectValueMut;
use crate::reflect::MessageDescriptor;
use crate::reflect::ReflectFieldRef;
use crate::reflect::ReflectMapMut;
use crate::reflect::ReflectMapRef;
use crate::reflect::ReflectRepeatedMut;
use crate::reflect::ReflectRepeatedRef;
use crate::reflect::ReflectValueBox;
use crate::reflect::RuntimeFieldType;
use crate::reflect::{FieldDescriptor, RuntimeTypeBox};
use crate::Clear;
use crate::CodedInputStream;
use crate::CodedOutputStream;
use crate::Message;
use crate::ProtobufResult;
use crate::UnknownFields;
use std::convert::TryInto;

use super::EnumValueDescriptor;

pub(crate) mod map;
pub(crate) mod optional;
pub(crate) mod repeated;

#[derive(Debug, Clone)]
enum DynamicFieldValue {
    Singular(DynamicOptional),
    Repeated(DynamicRepeated),
    Map(DynamicMap),
}

impl DynamicFieldValue {
    fn as_ref(&self) -> ReflectFieldRef {
        match self {
            DynamicFieldValue::Singular(v) => ReflectFieldRef::Optional(v.get()),
            DynamicFieldValue::Repeated(r) => ReflectFieldRef::Repeated(ReflectRepeatedRef::new(r)),
            DynamicFieldValue::Map(m) => ReflectFieldRef::Map(ReflectMapRef::new(m)),
        }
    }

    fn clear(&mut self) {
        match self {
            DynamicFieldValue::Singular(o) => o.clear(),
            DynamicFieldValue::Repeated(r) => r.clear(),
            DynamicFieldValue::Map(m) => m.clear(),
        }
    }
}

impl DynamicFieldValue {
    fn default_for_field(field: &FieldDescriptor) -> DynamicFieldValue {
        match field.runtime_field_type() {
            RuntimeFieldType::Singular(s) => DynamicFieldValue::Singular(DynamicOptional::none(s)),
            RuntimeFieldType::Repeated(r) => DynamicFieldValue::Repeated(DynamicRepeated::new(r)),
            RuntimeFieldType::Map(k, v) => DynamicFieldValue::Map(DynamicMap::new(k, v)),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DynamicMessage {
    pub(crate) descriptor: MessageDescriptor,
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
                .map(|f| DynamicFieldValue::default_for_field(&f))
                .collect();
        }
    }

    pub(crate) fn get_reflect<'a>(&'a self, field: &FieldDescriptor) -> ReflectFieldRef<'a> {
        assert_eq!(self.descriptor, field.message_descriptor);
        if self.fields.is_empty() {
            ReflectFieldRef::default_for_field(field)
        } else {
            self.fields[field.index].as_ref()
        }
    }

    pub fn clear_field(&mut self, field: &FieldDescriptor) {
        assert_eq!(field.message_descriptor, self.descriptor);
        if self.fields.is_empty() {
            return;
        }

        self.fields[field.index].clear();
    }

    fn clear_oneof_group_fields_except(&mut self, field: &FieldDescriptor) {
        if let Some(oneof) = field.containing_oneof() {
            for next in oneof.fields() {
                if &next == field {
                    continue;
                }
                self.clear_field(&next);
            }
        }
    }

    pub(crate) fn mut_singular_field_or_default<'a>(
        &'a mut self,
        field: &FieldDescriptor,
    ) -> ReflectValueMut<'a> {
        assert_eq!(field.message_descriptor, self.descriptor);
        self.init_fields();
        self.clear_oneof_group_fields_except(field);
        // TODO: reset oneof group fields
        match &mut self.fields[field.index] {
            DynamicFieldValue::Singular(f) => f.mut_or_default(),
            _ => panic!("Not a singular field"),
        }
    }

    pub(crate) fn mut_repeated<'a>(
        &'a mut self,
        field: &FieldDescriptor,
    ) -> ReflectRepeatedMut<'a> {
        assert_eq!(self.descriptor, field.message_descriptor);
        self.init_fields();
        // TODO: reset oneof group fields
        match &mut self.fields[field.index] {
            DynamicFieldValue::Repeated(r) => ReflectRepeatedMut::new(r),
            _ => panic!("Not a repeated field: {}", field),
        }
    }

    pub(crate) fn mut_map<'a>(&'a mut self, field: &FieldDescriptor) -> ReflectMapMut<'a> {
        assert_eq!(field.message_descriptor, self.descriptor);
        self.init_fields();
        // TODO: reset oneof group fields
        match &mut self.fields[field.index] {
            DynamicFieldValue::Map(m) => ReflectMapMut::new(m),
            _ => panic!("Not a map field: {}", field),
        }
    }

    pub(crate) fn set_field(&mut self, field: &FieldDescriptor, value: ReflectValueBox) {
        assert_eq!(field.message_descriptor, self.descriptor);
        self.init_fields();
        // TODO: reset oneof group fields
        match &mut self.fields[field.index] {
            DynamicFieldValue::Singular(s) => s.set(value),
            _ => panic!("Not a singular field: {}", field),
        }
    }

    pub fn downcast_ref(message: &dyn MessageDyn) -> &DynamicMessage {
        MessageDyn::downcast_ref(message).unwrap()
    }

    pub fn downcast_mut(message: &mut dyn MessageDyn) -> &mut DynamicMessage {
        MessageDyn::downcast_mut(message).unwrap()
    }
}

impl Clear for DynamicMessage {
    fn clear(&mut self) {
        unimplemented!()
    }
}

// TODO: Dixeran impl this
impl Message for DynamicMessage {
    fn descriptor_by_instance(&self) -> MessageDescriptor {
        self.descriptor.clone()
    }

    fn is_initialized(&self) -> bool {
        unimplemented!()
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) -> ProtobufResult<()> {
        let desc = self.descriptor.clone();

        while !is.eof()? {
            let (field, _) = is.read_tag_unpack()?;
            let field_desc = desc
                .get_field_by_number(field)
                .expect("Invalid field number at decoding");
            match field_desc.runtime_field_type() {
                RuntimeFieldType::Singular(rtb) => {
                    let val = match rtb {
                        RuntimeTypeBox::I32 => ReflectValueBox::from(is.read_int32()?),
                        RuntimeTypeBox::I64 => ReflectValueBox::from(is.read_int64()?),
                        RuntimeTypeBox::U32 => ReflectValueBox::from(is.read_uint32()?),
                        RuntimeTypeBox::U64 => ReflectValueBox::from(is.read_uint64()?),
                        RuntimeTypeBox::F32 => ReflectValueBox::from(is.read_float()?),
                        RuntimeTypeBox::F64 => ReflectValueBox::from(is.read_double()?),
                        RuntimeTypeBox::Bool => ReflectValueBox::from(is.read_bool()?),
                        RuntimeTypeBox::String => ReflectValueBox::from(is.read_string()?),
                        RuntimeTypeBox::VecU8 => ReflectValueBox::from(is.read_bytes()?),
                        RuntimeTypeBox::Enum(enum_desc) => {
                            let enum_num = is.read_int32()?;
                            ReflectValueBox::from(EnumValueDescriptor::new(
                                enum_desc,
                                enum_num.try_into().unwrap(), // FIXME: might unsatisfied
                            ))
                        }
                        RuntimeTypeBox::Message(msg_desc) => {
                            let mut msg_inst = msg_desc.new_instance();
                            is.merge_message(msg_inst.as_mut()).expect("merge sub message failed");
                            ReflectValueBox::from(msg_inst)
                        }
                    };
                    self.set_field(&field_desc, val);
                }
                RuntimeFieldType::Repeated(_) => {}
                RuntimeFieldType::Map(_, _) => {}
            }
        }
        Ok(())
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

use crate::cached_size::CachedSize;
use crate::reflect::dynamic::map::DynamicMap;
use crate::reflect::dynamic::repeated::DynamicRepeated;
use crate::reflect::value::ReflectValueMut;
use crate::reflect::FieldDescriptor;
use crate::reflect::MessageDescriptor;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectFieldRef;
use crate::reflect::ReflectMapMut;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeFieldType;
use crate::reflect::RuntimeTypeBox;
use crate::Clear;
use crate::CodedInputStream;
use crate::CodedOutputStream;
use crate::Message;
use crate::ProtobufResult;
use crate::UnknownFields;

pub(crate) mod map;
pub(crate) mod repeated;

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
    fn default_for_field(field: &FieldDescriptor) -> DynamicFieldValue {
        match field.runtime_field_type() {
            RuntimeFieldType::Singular(s) => DynamicFieldValue::Singular(DynamicOptional::none(s)),
            RuntimeFieldType::Repeated(r) => DynamicFieldValue::Repeated(DynamicRepeated::new(r)),
            RuntimeFieldType::Map(k, v) => DynamicFieldValue::Map(DynamicMap::new(k, v)),
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

    pub(crate) fn get_reflect<'a>(&'a self, field: &FieldDescriptor) -> ReflectFieldRef<'a> {
        assert_eq!(self.descriptor, field.message_descriptor);
        if self.fields.is_empty() {
            ReflectFieldRef::default_for_field(field)
        } else {
            self.fields[field.index].as_ref()
        }
    }

    pub(crate) fn get_singular_field_or_default<'a>(
        &self,
        field: &FieldDescriptor,
    ) -> ReflectValueRef<'a> {
        assert_eq!(field.message_descriptor, self.descriptor);
        unimplemented!(); // TODO
    }

    pub(crate) fn mut_singular_field_or_default<'a>(
        &mut self,
        field: &FieldDescriptor,
    ) -> ReflectValueMut<'a> {
        assert_eq!(field.message_descriptor, self.descriptor);
        unimplemented!(); // TODO
    }

    pub(crate) fn mut_map(&mut self, field: &FieldDescriptor) -> ReflectMapMut {
        assert_eq!(field.message_descriptor, self.descriptor);
        self.init_fields();
        match &mut self.fields[field.index] {
            DynamicFieldValue::Map(m) => ReflectMapMut::new(m),
            _ => panic!("Not a map field: {}", field),
        }
    }

    pub fn downcast_ref(message: &dyn Message) -> &DynamicMessage {
        Message::downcast_ref(message).unwrap()
    }

    pub fn downcast_mut(message: &mut dyn Message) -> &mut DynamicMessage {
        Message::downcast_mut(message).unwrap()
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

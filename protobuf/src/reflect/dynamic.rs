use crate::cached_size::CachedSize;
use crate::reflect::MessageDescriptor;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueBox;
use crate::Clear;
use crate::CodedInputStream;
use crate::CodedOutputStream;
use crate::Message;
use crate::ProtobufResult;
use crate::UnknownFields;
use std::collections::HashMap;

#[derive(Debug)]
enum DynamicFieldValue {
    Singular(ReflectValueBox),
    Repeated(Vec<ReflectValueBox>),
    Map(HashMap<ReflectValueBox, ReflectValueBox>),
}

#[derive(Debug)]
pub struct DynamicMessage {
    descriptor: MessageDescriptor,
    fields: HashMap<String, DynamicFieldValue>,
    unknown_fields: UnknownFields,
    cached_size: CachedSize,
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

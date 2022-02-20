//! Generated messages reflection support.

use std::collections::HashMap;
use std::fmt;
use std::marker;

use crate::descriptor::FileDescriptorProto;
use crate::message_dyn::MessageDyn;
use crate::message_full::MessageFull;
use crate::reflect::acc::FieldAccessor;
use crate::reflect::file::building::FileDescriptorBuilding;
use crate::reflect::file::index::FileIndex;
use crate::reflect::find_message_or_enum::find_message_or_enum;
use crate::reflect::find_message_or_enum::MessageOrEnum;
use crate::reflect::message::index::MessageIndex;

/// Sized to dynamic reflection operations.
pub(crate) trait MessageFactory: Send + Sync + 'static {
    fn new_instance(&self) -> Box<dyn MessageDyn>;
    fn default_instance(&self) -> &dyn MessageDyn;
    fn clone(&self, message: &dyn MessageDyn) -> Box<dyn MessageDyn>;
    fn eq(&self, a: &dyn MessageDyn, b: &dyn MessageDyn) -> bool;
}

impl<'a> fmt::Debug for &'a dyn MessageFactory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MessageFactory").finish()
    }
}

/// The only message factory implementation.
pub(crate) struct MessageFactoryImpl<M>(pub marker::PhantomData<M>);

impl<M> MessageFactory for MessageFactoryImpl<M>
where
    M: 'static + MessageFull + Default + Clone + PartialEq,
{
    fn new_instance(&self) -> Box<dyn MessageDyn> {
        let m: M = Default::default();
        Box::new(m)
    }

    fn default_instance(&self) -> &dyn MessageDyn {
        M::default_instance() as &dyn MessageDyn
    }

    fn clone(&self, message: &dyn MessageDyn) -> Box<dyn MessageDyn> {
        let m: &M = message.downcast_ref().expect("wrong message type");
        Box::new(m.clone())
    }

    fn eq(&self, a: &dyn MessageDyn, b: &dyn MessageDyn) -> bool {
        let a: &M = a.downcast_ref().expect("wrong message type");
        let b: &M = b.downcast_ref().expect("wrong message type");
        a == b
    }
}

#[doc(hidden)]
pub struct GeneratedMessageDescriptorData {
    pub(crate) protobuf_name_to_package: &'static str,
    pub(crate) fields: Vec<FieldAccessor>,
    pub(crate) factory: &'static dyn MessageFactory,
}

impl GeneratedMessageDescriptorData {
    /// Construct a new message descriptor.
    ///
    /// This operation is called from generated code and rarely
    /// need to be called directly.
    ///
    /// This function is not a part of public API.
    #[doc(hidden)]
    pub fn new_2<M: 'static + MessageFull + Default + Clone + PartialEq>(
        protobuf_name_to_package: &'static str,
        _index: u32,
        fields: Vec<FieldAccessor>,
    ) -> GeneratedMessageDescriptorData {
        let factory = &MessageFactoryImpl(marker::PhantomData::<M>);
        GeneratedMessageDescriptorData {
            protobuf_name_to_package,
            fields,
            factory,
        }
    }
}

#[derive(Debug)]
pub(crate) struct NonMapMessageDescriptor {
    pub(crate) factory: &'static dyn MessageFactory,

    pub(crate) fields: Vec<FieldAccessor>,

    pub(crate) index: MessageIndex,
}

#[derive(Debug)]
pub(crate) struct GeneratedMessageDescriptor {
    pub non_map: Option<NonMapMessageDescriptor>,
}

impl GeneratedMessageDescriptor {
    pub fn new_map_entry() -> GeneratedMessageDescriptor {
        GeneratedMessageDescriptor { non_map: None }
    }

    pub(crate) fn new(
        data: GeneratedMessageDescriptorData,
        file_descriptor_proto: &'static FileDescriptorProto,
        _file_index: &FileIndex,
        building: &FileDescriptorBuilding,
    ) -> GeneratedMessageDescriptor {
        let GeneratedMessageDescriptorData {
            protobuf_name_to_package,
            fields,
            factory,
        } = data;

        let (_path_to_package, proto) =
            match find_message_or_enum(file_descriptor_proto, protobuf_name_to_package) {
                Some((path_to_package, MessageOrEnum::Message(m))) => (path_to_package, m),
                Some((_, MessageOrEnum::Enum(_))) => panic!("not a message"),
                None => panic!("not found"),
            };

        let mut field_proto_by_name = HashMap::new();
        for field_proto in &proto.field {
            field_proto_by_name.insert(field_proto.name(), field_proto);
        }

        let index = MessageIndex::index(proto, building);

        GeneratedMessageDescriptor {
            non_map: Some(NonMapMessageDescriptor {
                factory,
                fields,
                index,
            }),
        }
    }

    pub fn non_map(&self) -> &NonMapMessageDescriptor {
        match &self.non_map {
            Some(non_map) => non_map,
            None => panic!("map message"),
        }
    }
}

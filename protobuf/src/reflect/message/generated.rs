//! Generated messages reflection support.

use crate::descriptor::DescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::message::Message;
use crate::message_dyn::MessageDyn;
use crate::reflect::acc::FieldAccessor;
use crate::reflect::find_message_or_enum::find_message_or_enum;
use crate::reflect::find_message_or_enum::MessageOrEnum;
use crate::reflect::message::common::MessageIndices;
use crate::reflect::name::compute_full_name;
use std::collections::HashMap;
use std::marker;

/// Sized to dynamic reflection operations.
pub(crate) trait MessageFactory: Send + Sync + 'static {
    fn new_instance(&self) -> Box<dyn MessageDyn>;
    fn default_instance(&self) -> &dyn MessageDyn;
    fn clone(&self, message: &dyn MessageDyn) -> Box<dyn MessageDyn>;
    fn eq(&self, a: &dyn MessageDyn, b: &dyn MessageDyn) -> bool;
}

/// The only message factory implementation.
pub(crate) struct MessageFactoryImpl<M>(pub marker::PhantomData<M>);

impl<M> MessageFactory for MessageFactoryImpl<M>
where
    M: 'static + Message + Default + Clone + PartialEq,
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
    index: u32,
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
    pub fn new_2<M: 'static + Message + Default + Clone + PartialEq>(
        protobuf_name_to_package: &'static str,
        index: u32,
        fields: Vec<FieldAccessor>,
    ) -> GeneratedMessageDescriptorData {
        let factory = &MessageFactoryImpl(marker::PhantomData::<M>);
        GeneratedMessageDescriptorData {
            index,
            protobuf_name_to_package,
            fields,
            factory,
        }
    }
}

pub(crate) struct GeneratedMessageDescriptor {
    pub(crate) proto: &'static DescriptorProto,

    pub(crate) full_name: String,

    pub(crate) factory: &'static dyn MessageFactory,

    pub(crate) fields: Vec<FieldAccessor>,

    pub indices: MessageIndices,
}

impl GeneratedMessageDescriptor {
    pub fn new(
        data: GeneratedMessageDescriptorData,
        expected_index: u32,
        file_descriptor_proto: &'static FileDescriptorProto,
    ) -> GeneratedMessageDescriptor {
        let GeneratedMessageDescriptorData {
            index,
            protobuf_name_to_package,
            fields,
            factory,
        } = data;

        assert!(expected_index == index);

        let (path_to_package, proto) =
            match find_message_or_enum(file_descriptor_proto, protobuf_name_to_package) {
                Some((path_to_package, MessageOrEnum::Message(m))) => (path_to_package, m),
                Some((_, MessageOrEnum::Enum(_))) => panic!("not a message"),
                None => panic!("not found"),
            };

        let mut field_proto_by_name = HashMap::new();
        for field_proto in &proto.field {
            field_proto_by_name.insert(field_proto.get_name(), field_proto);
        }

        let indices = MessageIndices::index(proto);

        GeneratedMessageDescriptor {
            full_name: compute_full_name(
                file_descriptor_proto.get_package(),
                &path_to_package,
                proto.get_name(),
            ),
            fields,
            indices,
            factory,
            proto,
        }
    }
}

use std::fmt;

use crate::message::Message;

use crate::descriptor::DescriptorProto;
use crate::descriptor::FileDescriptorProto;

use crate::reflect::dynamic::DynamicMessage;
use crate::reflect::file::FileDescriptorImpl;
use crate::reflect::message::common::MessageIndices;
use crate::reflect::message::dynamic::DynamicMessageDescriptor;
use crate::reflect::message::generated::GeneratedMessageDescriptor;
use crate::reflect::reflect_eq::ReflectEq;
use crate::reflect::reflect_eq::ReflectEqMode;
use crate::reflect::FieldDescriptor;
use crate::reflect::FileDescriptor;

pub(crate) mod common;
pub(crate) mod dynamic;
pub(crate) mod generated;
pub(crate) mod message_ref;

/// Dynamic representation of message type.
///
/// Used for reflection.
#[derive(Clone, Eq, PartialEq)]
pub struct MessageDescriptor {
    pub(crate) file_descriptor: FileDescriptor,
    index: usize,
}

impl fmt::Display for MessageDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

impl fmt::Debug for MessageDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MessageDescriptor")
            // TODO: add smth
            .finish()
    }
}

impl MessageDescriptor {
    /// Get underlying `DescriptorProto` object.
    pub fn get_proto(&self) -> &DescriptorProto {
        match self.get_impl() {
            MessageDescriptorImplRef::Generated(g) => &g.proto,
            MessageDescriptorImplRef::Dynamic(d) => d.get_proto(),
        }
    }

    /// Get a message descriptor for given message type
    pub fn for_type<M: Message>() -> MessageDescriptor {
        M::descriptor_static()
    }

    #[doc(hidden)]
    pub fn new_generated_2(file_descriptor: FileDescriptor, index: usize) -> MessageDescriptor {
        MessageDescriptor {
            file_descriptor,
            index,
        }
    }

    pub(crate) fn get_impl(&self) -> MessageDescriptorImplRef {
        match &self.file_descriptor.imp {
            FileDescriptorImpl::Generated(g) => {
                MessageDescriptorImplRef::Generated(&g.messages[self.index])
            }
            FileDescriptorImpl::Dynamic(d) => {
                MessageDescriptorImplRef::Dynamic(&d.messages[self.index])
            }
        }
    }

    /// [`FileDescriptor`] containing this message.
    pub fn file_descriptor(&self) -> &FileDescriptor {
        &self.file_descriptor
    }

    /// `FileDescriptorProto` containg this message type
    pub fn file_descriptor_proto(&self) -> &FileDescriptorProto {
        self.file_descriptor().get_proto()
    }

    /// New empty message
    pub fn new_instance(&self) -> Box<dyn Message> {
        match self.get_impl() {
            MessageDescriptorImplRef::Generated(g) => g.factory.new_instance(),
            MessageDescriptorImplRef::Dynamic(..) => Box::new(DynamicMessage::new(self.clone())),
        }
    }

    /// Shared immutable empty message.
    ///
    /// Returns `None` for dynamic message.
    pub fn default_instance(&self) -> Option<&'static dyn Message> {
        match self.get_impl() {
            MessageDescriptorImplRef::Generated(g) => Some(g.factory.default_instance()),
            MessageDescriptorImplRef::Dynamic(..) => None,
        }
    }

    /// Clone a message
    pub(crate) fn clone_message(&self, message: &dyn Message) -> Box<dyn Message> {
        assert!(&message.descriptor() == self);
        match self.get_impl() {
            MessageDescriptorImplRef::Generated(g) => g.factory.clone(message),
            MessageDescriptorImplRef::Dynamic(..) => {
                let message: &DynamicMessage = Message::downcast_ref(message).unwrap();
                Box::new(message.clone())
            }
        }
    }

    /// Check if two messages equal.
    ///
    /// # Panics
    ///
    /// Is any message has different type than this descriptor.
    pub fn eq(&self, a: &dyn Message, b: &dyn Message) -> bool {
        match self.get_impl() {
            MessageDescriptorImplRef::Generated(g) => g.factory.eq(a, b),
            MessageDescriptorImplRef::Dynamic(..) => unimplemented!(),
        }
    }

    /// Similar to `eq`, but considers `NaN` values equal.
    ///
    /// # Panics
    ///
    /// Is any message has different type than this descriptor.
    pub(crate) fn reflect_eq(
        &self,
        a: &dyn Message,
        b: &dyn Message,
        mode: &ReflectEqMode,
    ) -> bool {
        // Explicitly force panic even if field list is empty
        assert_eq!(self, &a.descriptor(),);
        assert_eq!(self, &b.descriptor(),);

        for field in self.fields() {
            let af = field.get_reflect(a);
            let bf = field.get_reflect(b);
            if !af.reflect_eq(&bf, mode) {
                return false;
            }
        }
        true
    }

    /// Message name as given in `.proto` file
    pub fn name(&self) -> &str {
        self.get_proto().get_name()
    }

    /// Fully qualified protobuf message name
    pub fn full_name(&self) -> &str {
        match self.get_impl() {
            MessageDescriptorImplRef::Generated(g) => &g.full_name,
            MessageDescriptorImplRef::Dynamic(d) => &d.full_name,
        }
    }

    /// Message field descriptors.
    pub fn fields(&self) -> Vec<FieldDescriptor> {
        (0..self.get_indices().fields_len())
            .map(|index| FieldDescriptor {
                message_descriptor: self.clone(),
                index,
            })
            .collect()
    }

    pub(crate) fn get_indices(&self) -> &MessageIndices {
        match self.get_impl() {
            MessageDescriptorImplRef::Generated(g) => &g.indices,
            MessageDescriptorImplRef::Dynamic(d) => &d.indices,
        }
    }

    /// Find message field by protobuf field name
    ///
    /// Note: protobuf field name might be different for Rust field name.
    // TODO: return value, not pointer, pointer is not compatible with dynamic message
    pub fn get_field_by_name<'a>(&'a self, name: &str) -> Option<FieldDescriptor> {
        let &index = self.get_indices().index_by_name.get(name)?;
        Some(FieldDescriptor {
            message_descriptor: self.clone(),
            index,
        })
    }

    /// Find message field by field name or field JSON name
    pub fn get_field_by_name_or_json_name<'a>(&'a self, name: &str) -> Option<FieldDescriptor> {
        let &index = self.get_indices().index_by_name_or_json_name.get(name)?;
        Some(FieldDescriptor {
            message_descriptor: self.clone(),
            index,
        })
    }

    /// Find message field by field name
    pub fn get_field_by_number(&self, number: u32) -> Option<FieldDescriptor> {
        let &index = self.get_indices().index_by_number.get(&number)?;
        Some(FieldDescriptor {
            message_descriptor: self.clone(),
            index,
        })
    }
}

pub(crate) enum MessageDescriptorImplRef<'a> {
    Generated(&'static GeneratedMessageDescriptor),
    Dynamic(&'a DynamicMessageDescriptor),
}

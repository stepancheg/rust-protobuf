use std::fmt;

use crate::message::Message;

use crate::descriptor::DescriptorProto;
use crate::descriptor::FileDescriptorProto;

use crate::reflect::message::generated::GeneratedMessageDescriptor;
use crate::reflect::reflect_eq::ReflectEq;
use crate::reflect::reflect_eq::ReflectEqMode;
use crate::reflect::FieldDescriptor;
use crate::reflect::FileDescriptor;

pub(crate) mod generated;

/// Dynamic representation of message type.
///
/// Used for reflection.
#[derive(Clone, Eq, PartialEq)]
pub struct MessageDescriptor {
    file_descriptor: FileDescriptor,
    index: usize,
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
        self.get_generated().proto
    }

    /// Get a message descriptor for given message type
    pub fn for_type<M: Message>() -> MessageDescriptor {
        M::descriptor_static()
    }

    pub(crate) fn compute_full_name(
        package: &str,
        path_to_package: &str,
        proto: &DescriptorProto,
    ) -> String {
        let mut full_name = package.to_owned();
        if path_to_package.len() != 0 {
            if full_name.len() != 0 {
                full_name.push('.');
            }
            full_name.push_str(path_to_package);
        }
        if full_name.len() != 0 {
            full_name.push('.');
        }
        full_name.push_str(proto.get_name());
        full_name
    }

    #[doc(hidden)]
    pub fn new_generated_2(file_descriptor: FileDescriptor, index: usize) -> MessageDescriptor {
        MessageDescriptor {
            file_descriptor,
            index,
        }
    }

    fn get_generated(&self) -> &GeneratedMessageDescriptor {
        &self.file_descriptor.generated.messages[self.index]
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
        self.get_generated().factory.new_instance()
    }

    /// Shared immutable empty message
    // TODO: figure out what to do with default instance for dynamic message
    pub fn default_instance(&self) -> &'static dyn Message {
        self.get_generated().factory.default_instance()
    }

    /// Clone a message
    pub(crate) fn clone_message(&self, message: &dyn Message) -> Box<dyn Message> {
        self.get_generated().factory.clone(message)
    }

    /// Check if two messages equal.
    ///
    /// # Panics
    ///
    /// Is any message has different type than this descriptor.
    pub fn eq(&self, a: &dyn Message, b: &dyn Message) -> bool {
        self.get_generated().factory.eq(a, b)
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
        &self.get_generated().full_name
    }

    /// Message field descriptors.
    pub fn fields(&self) -> &[FieldDescriptor] {
        &self.get_generated().fields
    }

    /// Find message field by protobuf field name
    ///
    /// Note: protobuf field name might be different for Rust field name.
    // TODO: return value, not pointer, pointer is not compatible with dynamic message
    pub fn get_field_by_name<'a>(&'a self, name: &str) -> Option<&'a FieldDescriptor> {
        let &index = self.get_generated().index_by_name.get(name)?;
        Some(&self.get_generated().fields[index])
    }

    /// Find message field by field name or field JSON name
    pub fn get_field_by_name_or_json_name<'a>(&'a self, name: &str) -> Option<&'a FieldDescriptor> {
        let &index = self.get_generated().index_by_name_or_json_name.get(name)?;
        Some(&self.get_generated().fields[index])
    }

    /// Find message field by field name
    pub fn get_field_by_number(&self, number: u32) -> Option<&FieldDescriptor> {
        let &index = self.get_generated().index_by_number.get(&number)?;
        Some(&self.get_generated().fields[index])
    }
}

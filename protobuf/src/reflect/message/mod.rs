use std::fmt;
use std::io::Read;

use crate::descriptor::DescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::message::Message;
use crate::message_dyn::MessageDyn;
use crate::reflect::dynamic::DynamicMessage;
use crate::reflect::field::FieldDescriptorImpl;
use crate::reflect::file::index::FileIndexMessageEntry;
use crate::reflect::file::FileDescriptorImpl;
use crate::reflect::message::dynamic::DynamicMessageDescriptor;
use crate::reflect::message::generated::GeneratedMessageDescriptor;
use crate::reflect::message::index::MessageIndex;
use crate::reflect::reflect_eq::ReflectEq;
use crate::reflect::reflect_eq::ReflectEqMode;
use crate::reflect::EnumDescriptor;
use crate::reflect::FieldDescriptor;
use crate::reflect::FileDescriptor;
use crate::reflect::OneofDescriptor;
use crate::CodedInputStream;
use crate::Result;

pub(crate) mod dynamic;
pub(crate) mod generated;
pub(crate) mod index;
pub(crate) mod message_ref;
pub(crate) mod path;

/// Dynamic representation of message type.
///
/// Used for reflection.
#[derive(Clone, Eq, PartialEq)]
pub struct MessageDescriptor {
    file_descriptor: FileDescriptor,
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
    pub(crate) fn new(file_descriptor: FileDescriptor, index: usize) -> MessageDescriptor {
        MessageDescriptor {
            file_descriptor,
            index,
        }
    }

    /// Get internal index.
    ///
    /// Only needed for codegen.
    #[doc(hidden)]
    pub fn get_index_in_file_for_codegen(&self) -> usize {
        self.index
    }

    /// Get underlying `DescriptorProto` object.
    pub fn get_proto(&self) -> &DescriptorProto {
        self.file_descriptor.message_proto(self.index)
    }

    /// Message name as specified in `.proto` file.
    pub fn get_name(&self) -> &str {
        self.get_proto().name()
    }

    /// Get enums declared in this message.
    pub fn get_enums(&self) -> Vec<EnumDescriptor> {
        let first_enum_index = self.get_index_entry().first_enum_index;
        self.get_proto()
            .enum_type
            .iter()
            .enumerate()
            .map(|(i, _)| EnumDescriptor::new(self.file_descriptor.clone(), first_enum_index + i))
            .collect()
    }

    fn get_index_entry(&self) -> &FileIndexMessageEntry {
        self.file_descriptor.message_index_entry(self.index)
    }

    /// Get a message descriptor for given message type
    pub fn for_type<M: Message>() -> MessageDescriptor {
        M::descriptor_static()
    }

    #[doc(hidden)]
    pub fn new_generated_2(file_descriptor: FileDescriptor, index: usize) -> MessageDescriptor {
        MessageDescriptor::new(file_descriptor, index)
    }

    /// Messages declared in this messages.
    pub fn nested_messages(&self) -> Vec<MessageDescriptor> {
        self.get_index_entry()
            .nested_messages
            .iter()
            .map(|i| MessageDescriptor::new(self.file_descriptor.clone(), *i))
            .collect()
    }

    /// Get a message containing this message, or `None` if this message is declared at file level.
    pub fn enclosing_message(&self) -> Option<MessageDescriptor> {
        self.get_index_entry()
            .enclosing_message
            .map(|i| MessageDescriptor::new(self.file_descriptor.clone(), i))
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
        self.file_descriptor().proto()
    }

    /// This message descriptor is a map entry.
    pub fn is_map_entry(&self) -> bool {
        self.get_proto().options.get_or_default().map_entry()
    }

    fn assert_not_map_entry(&self) {
        assert!(
            !self.is_map_entry(),
            "message is map entry: {}",
            self.full_name()
        );
    }

    /// New empty message.
    ///
    /// # Panics
    ///
    /// If this message is a map entry message.
    pub fn new_instance(&self) -> Box<dyn MessageDyn> {
        self.assert_not_map_entry();
        match self.get_impl() {
            MessageDescriptorImplRef::Generated(g) => g.non_map().factory.new_instance(),
            MessageDescriptorImplRef::Dynamic(..) => Box::new(DynamicMessage::new(self.clone())),
        }
    }

    /// Shared immutable empty message.
    ///
    /// Returns `None` for dynamic message.
    ///
    /// # Panics
    ///
    /// If this message is a map entry message.
    pub fn default_instance(&self) -> Option<&'static dyn MessageDyn> {
        self.assert_not_map_entry();
        match self.get_impl() {
            MessageDescriptorImplRef::Generated(g) => Some(g.non_map().factory.default_instance()),
            MessageDescriptorImplRef::Dynamic(..) => None,
        }
    }

    /// Clone a message
    pub(crate) fn clone_message(&self, message: &dyn MessageDyn) -> Box<dyn MessageDyn> {
        assert!(&message.descriptor_dyn() == self);
        match self.get_impl() {
            MessageDescriptorImplRef::Generated(g) => g.non_map().factory.clone(message),
            MessageDescriptorImplRef::Dynamic(..) => {
                let message: &DynamicMessage = <dyn MessageDyn>::downcast_ref(message).unwrap();
                Box::new(message.clone())
            }
        }
    }

    /// Check if two messages equal.
    ///
    /// # Panics
    ///
    /// Is any message has different type than this descriptor.
    pub fn eq(&self, a: &dyn MessageDyn, b: &dyn MessageDyn) -> bool {
        match self.get_impl() {
            MessageDescriptorImplRef::Generated(g) => g.non_map().factory.eq(a, b),
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
        a: &dyn MessageDyn,
        b: &dyn MessageDyn,
        mode: &ReflectEqMode,
    ) -> bool {
        // Explicitly force panic even if field list is empty
        assert_eq!(self, &a.descriptor_dyn());
        assert_eq!(self, &b.descriptor_dyn());

        for field in self.fields() {
            let af = field.get_reflect(a);
            let bf = field.get_reflect(b);
            if !af.reflect_eq(&bf, mode) {
                return false;
            }
        }
        true
    }

    pub(crate) fn reflect_eq_maybe_unrelated(
        a: &dyn MessageDyn,
        b: &dyn MessageDyn,
        mode: &ReflectEqMode,
    ) -> bool {
        let ad = a.descriptor_dyn();
        let bd = b.descriptor_dyn();
        ad == bd && ad.reflect_eq(a, b, mode)
    }

    /// Message name as given in `.proto` file
    pub fn name(&self) -> &str {
        self.get_proto().name()
    }

    /// Fully qualified protobuf message name
    pub fn full_name(&self) -> &str {
        &self.get_index_entry().full_name
    }

    /// Name relative to the package where the message is declared.
    pub fn name_to_package(&self) -> &str {
        &self.get_index_entry().name_to_package
    }

    /// Nested oneofs
    pub fn oneofs<'a>(&'a self) -> impl ExactSizeIterator<Item = OneofDescriptor> + 'a {
        self.get_proto()
            .oneof_decl
            .iter()
            .enumerate()
            .map(move |(index, _)| OneofDescriptor {
                message_descriptor: self.clone(),
                index,
            })
    }

    /// Message field descriptors.
    pub fn fields<'a>(&'a self) -> impl ExactSizeIterator<Item = FieldDescriptor> + 'a {
        (0..self.index().fields.len()).map(move |index| FieldDescriptor {
            imp: FieldDescriptorImpl::Field(self.clone(), index),
        })
    }

    /// Extension fields.
    pub fn extensions(&self) -> Vec<FieldDescriptor> {
        (0..self.index().extensions.len())
            .map(move |index| FieldDescriptor {
                imp: FieldDescriptorImpl::ExtensionInMessage(self.clone(), index),
            })
            .collect()
    }

    pub(crate) fn index(&self) -> &MessageIndex {
        match self.get_impl() {
            MessageDescriptorImplRef::Generated(g) => &g.non_map().index,
            MessageDescriptorImplRef::Dynamic(d) => &d.indices,
        }
    }

    pub(crate) fn get_generated_index(&self) -> &'static MessageIndex {
        match self.get_impl() {
            MessageDescriptorImplRef::Generated(g) => &g.non_map().index,
            MessageDescriptorImplRef::Dynamic(_) => panic!("dynamic message: {}", self),
        }
    }

    /// Find message field by protobuf field name
    ///
    /// Note: protobuf field name might be different for Rust field name.
    // TODO: return value, not pointer, pointer is not compatible with dynamic message
    pub fn get_field_by_name<'a>(&'a self, name: &str) -> Option<FieldDescriptor> {
        let &index = self.index().field_index_by_name.get(name)?;
        Some(FieldDescriptor {
            imp: FieldDescriptorImpl::Field(self.clone(), index),
        })
    }

    /// Find message field by field name or field JSON name
    pub fn get_field_by_name_or_json_name<'a>(&'a self, name: &str) -> Option<FieldDescriptor> {
        let &index = self.index().field_index_by_name_or_json_name.get(name)?;
        Some(FieldDescriptor {
            imp: FieldDescriptorImpl::Field(self.clone(), index),
        })
    }

    /// Find message field by field name
    pub fn get_field_by_number(&self, number: u32) -> Option<FieldDescriptor> {
        let &index = self.index().field_index_by_number.get(&number)?;
        Some(FieldDescriptor {
            imp: FieldDescriptorImpl::Field(self.clone(), index),
        })
    }

    /// Parse message from stream.
    pub fn parse_from(&self, is: &mut CodedInputStream) -> Result<Box<dyn MessageDyn>> {
        let mut r = self.new_instance();
        r.merge_from_dyn(is)?;
        r.check_initialized_dyn()?;
        Ok(r)
    }

    /// Parse message from reader.
    /// Parse stops on EOF or when error encountered.
    pub fn parse_from_reader(&self, reader: &mut dyn Read) -> Result<Box<dyn MessageDyn>> {
        let mut is = CodedInputStream::new(reader);
        let r = self.parse_from(&mut is)?;
        is.check_eof()?;
        Ok(r)
    }

    /// Parse message from byte array.
    pub fn parse_from_bytes(&self, bytes: &[u8]) -> Result<Box<dyn MessageDyn>> {
        let mut is = CodedInputStream::from_bytes(bytes);
        let r = self.parse_from(&mut is)?;
        is.check_eof()?;
        Ok(r)
    }
}

pub(crate) enum MessageDescriptorImplRef<'a> {
    Generated(&'static GeneratedMessageDescriptor),
    Dynamic(&'a DynamicMessageDescriptor),
}

#[cfg(test)]
mod test {
    use crate::descriptor::descriptor_proto::ExtensionRange;
    use crate::descriptor::DescriptorProto;
    use crate::Message;

    #[test]
    #[cfg_attr(miri, ignore)] // Too slow on Miri.
    fn nested_messages() {
        assert!(DescriptorProto::descriptor_static()
            .nested_messages()
            .contains(&ExtensionRange::descriptor_static()));
    }

    #[test]
    #[cfg_attr(miri, ignore)] // Too slow on Miri.
    fn enclosing_message() {
        assert_eq!(
            Some(DescriptorProto::descriptor_static()),
            ExtensionRange::descriptor_static().enclosing_message()
        );
        assert_eq!(
            None,
            DescriptorProto::descriptor_static().enclosing_message()
        );
    }
}

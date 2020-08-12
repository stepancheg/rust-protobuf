use crate::descriptor::OneofDescriptorProto;
use crate::reflect::FieldDescriptor;
use crate::reflect::MessageDescriptor;

/// Oneof descriptor.
#[derive(Eq, PartialEq)]
pub struct OneofDescriptor {
    pub(crate) message_descriptor: MessageDescriptor,
    pub(crate) index: usize,
}

impl OneofDescriptor {
    /// `.proto` part associated with this descriptor
    pub fn get_proto(&self) -> &OneofDescriptorProto {
        &self.message_descriptor.get_proto().oneof_decl[self.index]
    }

    /// Fields in this oneof.
    pub fn fields(&self) -> Vec<FieldDescriptor> {
        self.message_descriptor
            .fields()
            .into_iter()
            .filter(|f| f.containing_oneof().as_ref() == Some(self))
            .collect()
    }
}

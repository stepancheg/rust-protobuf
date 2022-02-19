use crate::descriptor::OneofDescriptorProto;
use crate::reflect::FieldDescriptor;
use crate::reflect::MessageDescriptor;

/// Oneof descriptor.
#[derive(Eq, PartialEq, Clone)]
pub struct OneofDescriptor {
    pub(crate) message_descriptor: MessageDescriptor,
    pub(crate) index: usize,
}

impl OneofDescriptor {
    /// `.proto` part associated with this descriptor
    pub fn get_proto(&self) -> &OneofDescriptorProto {
        &self.message_descriptor.get_proto().oneof_decl[self.index]
    }

    /// Oneof name as specified in `.proto` file.
    pub fn get_name(&self) -> &str {
        self.get_proto().name()
    }

    /// Fully qualified name of oneof (fully qualified name of enclosing message
    /// followed by oneof name).
    pub fn full_name(&self) -> String {
        format!(
            "{}.{}",
            self.message_descriptor.full_name(),
            self.get_name()
        )
    }

    /// Fields in this oneof.
    pub fn fields<'a>(&'a self) -> impl Iterator<Item = FieldDescriptor> + 'a {
        self.message_descriptor
            .fields()
            .filter(move |f| f.containing_oneof().as_ref() == Some(self))
    }
}

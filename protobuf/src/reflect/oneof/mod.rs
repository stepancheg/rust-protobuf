use crate::descriptor::OneofDescriptorProto;
use crate::reflect::FieldDescriptor;
use crate::reflect::MessageDescriptor;

/// Oneof descriptor.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct OneofDescriptor {
    pub(crate) message_descriptor: MessageDescriptor,
    pub(crate) index: usize,
}

impl OneofDescriptor {
    /// `.proto` part associated with this descriptor
    pub fn proto(&self) -> &OneofDescriptorProto {
        &self.message_descriptor.proto().oneof_decl[self.index]
    }

    /// Oneof name as specified in `.proto` file.
    pub fn name(&self) -> &str {
        self.proto().name()
    }

    /// This oneof is not present in sources.
    pub fn is_synthetic(&self) -> bool {
        let mut count = 0;
        for field in self.fields() {
            if count > 1 {
                return false;
            }
            if !field.proto().proto3_optional() {
                return false;
            }
            count += 1;
        }
        count == 1
    }

    /// Fully qualified name of oneof (fully qualified name of enclosing message
    /// followed by oneof name).
    pub fn full_name(&self) -> String {
        format!("{}.{}", self.message_descriptor.full_name(), self.name())
    }

    /// Fields in this oneof.
    pub fn fields<'a>(&'a self) -> impl Iterator<Item = FieldDescriptor> + 'a {
        self.message_descriptor
            .fields()
            .filter(move |f| f.containing_oneof_including_synthetic().as_ref() == Some(self))
    }
}

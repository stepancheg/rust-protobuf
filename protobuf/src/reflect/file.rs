use crate::descriptor::FileDescriptorProto;
use crate::reflect::EnumDescriptor;
use crate::reflect::MessageDescriptor;

/// Reflection for objects defined in `.proto` file (messages, enums, etc).
pub struct FileDescriptor {
    file_descriptor_proto: &'static FileDescriptorProto,
    dependencies: Vec<&'static FileDescriptor>,
    messages: Vec<&'static MessageDescriptor>,
    enums: Vec<&'static EnumDescriptor>,
}

impl FileDescriptor {
    /// This function is to be called from generated code.
    pub fn new(
        file_descriptor_proto: &'static FileDescriptorProto,
        dependencies: Vec<&'static FileDescriptor>,
        messages: Vec<&'static MessageDescriptor>,
        enums: Vec<&'static EnumDescriptor>,
    ) -> FileDescriptor {
        FileDescriptor {
            file_descriptor_proto,
            dependencies,
            messages,
            enums,
        }
    }
}

#![allow(dead_code)] // TODO: don't forget to remove

use crate::arc_or_static::ArcOrStatic;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::EnumDescriptor;
use crate::reflect::GeneratedEnumDescriptorData;
use crate::reflect::GeneratedMessageDescriptorData;
use crate::reflect::MessageDescriptor;

/// Reflection for objects defined in `.proto` file (messages, enums, etc).
pub struct FileDescriptor {
    file_descriptor_proto: ArcOrStatic<FileDescriptorProto>,
    dependencies: Vec<ArcOrStatic<FileDescriptor>>,
    messages: Vec<GeneratedMessageDescriptorData>,
    enums: Vec<GeneratedEnumDescriptorData>,
}

impl FileDescriptor {
    /// This function is to be called from generated code.
    pub fn new_generated(
        file_descriptor_proto: &'static FileDescriptorProto,
        dependencies: Vec<&'static FileDescriptor>,
        messages: Vec<GeneratedMessageDescriptorData>,
        enums: Vec<GeneratedEnumDescriptorData>,
    ) -> FileDescriptor {
        FileDescriptor {
            file_descriptor_proto: ArcOrStatic::Static(file_descriptor_proto),
            dependencies: dependencies.into_iter().map(ArcOrStatic::Static).collect(),
            messages,
            enums,
        }
    }

    /// Dynamic message (created from [`FileDescriptorProto`] without generated files.
    pub fn new_dynamic(
        file_descriptor_proto: ArcOrStatic<FileDescriptorProto>,
        dependencies: Vec<ArcOrStatic<FileDescriptor>>,
    ) -> FileDescriptor {
        FileDescriptor {
            file_descriptor_proto,
            dependencies,
            messages: Vec::new(), // TODO
            enums: Vec::new(),    // TODO
        }
    }
}

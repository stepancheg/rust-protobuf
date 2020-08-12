use crate::descriptor::FileDescriptorProto;
use crate::reflect::enums::generated::GeneratedEnumDescriptor;
use crate::reflect::file::index::FileIndex;
use crate::reflect::message::generated::GeneratedMessageDescriptor;
use crate::reflect::{FileDescriptor, GeneratedEnumDescriptorData, GeneratedMessageDescriptorData};

/// Reflection for objects defined in `.proto` file (messages, enums, etc).
#[doc(hidden)]
#[derive(Debug)]
pub struct GeneratedFileDescriptor {
    pub(crate) proto: &'static FileDescriptorProto,
    pub(crate) dependencies: Vec<FileDescriptor>,
    pub(crate) messages: Vec<GeneratedMessageDescriptor>,
    pub(crate) enums: Vec<GeneratedEnumDescriptor>,
    pub(crate) index: FileIndex,
}

impl GeneratedFileDescriptor {
    /// This function is to be called from generated code.
    pub fn new_generated(
        file_descriptor_proto: &'static FileDescriptorProto,
        dependencies: Vec<FileDescriptor>,
        messages: Vec<GeneratedMessageDescriptorData>,
        enums: Vec<GeneratedEnumDescriptorData>,
    ) -> GeneratedFileDescriptor {
        let index = FileIndex::index(file_descriptor_proto);

        let messages = messages
            .into_iter()
            .enumerate()
            .map(|(i, m)| {
                GeneratedMessageDescriptor::new(m, i as u32, file_descriptor_proto, &index)
            })
            .collect();
        let enums = enums
            .into_iter()
            .enumerate()
            .map(|(i, e)| GeneratedEnumDescriptor::new(e, i, file_descriptor_proto))
            .collect();

        GeneratedFileDescriptor {
            proto: file_descriptor_proto,
            dependencies,
            messages,
            enums,
            index,
        }
    }
    /// `.proto` data for this file.
    pub(crate) fn get_proto(&self) -> &FileDescriptorProto {
        &*self.proto
    }
}

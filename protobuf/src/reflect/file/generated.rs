use std::collections::HashMap;

use crate::descriptor::FileDescriptorProto;
use crate::reflect::enums::generated::GeneratedEnumDescriptor;
use crate::reflect::file::common::FileDescriptorCommon;
use crate::reflect::file::index::FileIndex;
use crate::reflect::message::generated::GeneratedMessageDescriptor;
use crate::reflect::oneof::generated::GeneratedOneofDescriptor;
use crate::reflect::FileDescriptor;
use crate::reflect::GeneratedEnumDescriptorData;
use crate::reflect::GeneratedMessageDescriptorData;

/// Reflection for objects defined in `.proto` file (messages, enums, etc).
#[doc(hidden)]
#[derive(Debug)]
pub struct GeneratedFileDescriptor {
    pub(crate) proto: &'static FileDescriptorProto,
    pub(crate) messages: Vec<GeneratedMessageDescriptor>,
    pub(crate) enums: Vec<GeneratedEnumDescriptor>,
    pub(crate) oneofs: Vec<GeneratedOneofDescriptor>,
    pub(crate) common: FileDescriptorCommon,
}

impl GeneratedFileDescriptor {
    /// This function is to be called from generated code.
    pub fn new_generated(
        file_descriptor_proto: &'static FileDescriptorProto,
        dependencies: Vec<FileDescriptor>,
        messages: Vec<GeneratedMessageDescriptorData>,
        enums: Vec<GeneratedEnumDescriptorData>,
    ) -> GeneratedFileDescriptor {
        let index = FileIndex::index(file_descriptor_proto, &dependencies).unwrap();

        let mut messages: HashMap<&str, GeneratedMessageDescriptorData> = messages
            .into_iter()
            .map(|m| (m.protobuf_name_to_package, m))
            .collect();

        let mut enums: HashMap<&str, GeneratedEnumDescriptorData> =
            enums.into_iter().map(|e| (e.name_in_file, e)).collect();

        let messages = index
            .messages
            .iter()
            .map(|message_index| {
                if message_index.map_entry {
                    GeneratedMessageDescriptor::new_map_entry()
                } else {
                    let message = messages
                        .remove(message_index.name_to_package.as_str())
                        .unwrap();
                    GeneratedMessageDescriptor::new(message, file_descriptor_proto, &index).unwrap()
                }
            })
            .collect();

        let enums = index
            .enums
            .iter()
            .map(|enum_index| {
                let en = enums.remove(enum_index.name_to_package.as_str()).unwrap();
                GeneratedEnumDescriptor::new(en, file_descriptor_proto)
            })
            .collect();

        // let oneofs = oneofs
        //     .into_iter()
        //     .enumerate()
        //     .map(|(i, o)| GeneratedOneofDescriptor::new(o, i, file_descriptor_proto))
        //     .collect();
        let oneofs = Vec::new();

        let common = FileDescriptorCommon::new(index, dependencies, file_descriptor_proto).unwrap();

        GeneratedFileDescriptor {
            proto: file_descriptor_proto,
            messages,
            enums,
            oneofs,
            common,
        }
    }
}

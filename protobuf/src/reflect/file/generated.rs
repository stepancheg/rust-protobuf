use std::collections::HashMap;

use crate::descriptor::FileDescriptorProto;
use crate::reflect::enums::generated::GeneratedEnumDescriptor;
use crate::reflect::file::index::FileDescriptorCommon;
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
        let common = FileDescriptorCommon::new(file_descriptor_proto, dependencies).unwrap();

        let mut messages: HashMap<&str, GeneratedMessageDescriptorData> = messages
            .into_iter()
            .map(|m| (m.protobuf_name_to_package, m))
            .collect();

        let mut enums: HashMap<&str, GeneratedEnumDescriptorData> =
            enums.into_iter().map(|e| (e.name_in_file, e)).collect();

        let mut oneofs = Vec::new();
        for oneof in &common.oneofs {
            if oneof.synthetic {
                oneofs.push(GeneratedOneofDescriptor::new_synthetic())
            } else {
                let message = &common.messages[oneof.containing_message];
                let message = messages.get(message.name_to_package.as_str()).unwrap();
                let oneof = &message.oneofs[oneof.index_in_containing_message];
                let oneof = GeneratedOneofDescriptor::new(oneof);
                oneofs.push(oneof);
            }
        }

        let messages = common
            .messages
            .iter()
            .map(|message_index| {
                if message_index.map_entry {
                    GeneratedMessageDescriptor::new_map_entry()
                } else {
                    let message = messages
                        .remove(message_index.name_to_package.as_str())
                        .unwrap();
                    GeneratedMessageDescriptor::new(message, file_descriptor_proto, &common)
                }
            })
            .collect();

        let enums = common
            .enums
            .iter()
            .map(|enum_index| {
                let en = enums.remove(enum_index.name_to_package.as_str()).unwrap();
                GeneratedEnumDescriptor::new(en, file_descriptor_proto)
            })
            .collect();

        GeneratedFileDescriptor {
            proto: file_descriptor_proto,
            messages,
            enums,
            oneofs,
            common,
        }
    }
}

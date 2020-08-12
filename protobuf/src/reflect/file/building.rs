use crate::descriptor::EnumDescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::find_message_or_enum::{find_message_or_enum, MessageOrEnum};
use crate::reflect::name::protobuf_name_starts_with_package;
use crate::reflect::FileDescriptor;
use std::iter;

pub(crate) struct FileDescriptorBuilding<'a> {
    pub file_descriptor: &'a FileDescriptorProto,
    pub deps: &'a [FileDescriptor],
}

impl<'a> FileDescriptorBuilding<'a> {
    fn all_descriptors(&self) -> impl Iterator<Item = &'a FileDescriptorProto> {
        iter::once(self.file_descriptor).chain(self.deps.iter().map(|d| d.get_proto()))
    }

    pub fn find_enum(&self, full_name: &str) -> &'a EnumDescriptorProto {
        assert!(full_name.starts_with("."));

        for file in self.all_descriptors() {
            if let Some(name_to_package) =
                protobuf_name_starts_with_package(full_name, file.get_package())
            {
                if let Some((_, me)) = find_message_or_enum(file, name_to_package) {
                    match me {
                        MessageOrEnum::Enum(e) => return e,
                        MessageOrEnum::Message(_) => panic!("not an enum: {}", full_name),
                    }
                }
            }
        }

        panic!(
            "enum not found: {}, in files: {}",
            full_name,
            self.all_descriptors()
                .map(|d| d.get_name())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}

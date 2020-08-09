#![allow(dead_code)] // TODO: don't forget to remove

use crate::descriptor::FileDescriptorProto;
use crate::reflect::GeneratedFileDescriptor;

pub(crate) mod generated;

/// Reflection for objects defined in `.proto` file (messages, enums, etc).
#[derive(Clone)]
pub struct FileDescriptor {
    pub(crate) generated: &'static GeneratedFileDescriptor,
}

impl FileDescriptor {
    /// This function is called from generated code, it is not stable, and should not be called.
    pub fn new_generated_2(generated: &'static GeneratedFileDescriptor) -> FileDescriptor {
        FileDescriptor { generated }
    }

    /// Dynamic message created from [`FileDescriptorProto`] without generated files.
    pub fn new_dynamic(
        file_descriptor_proto: FileDescriptorProto,
        dependencies: Vec<FileDescriptor>,
    ) -> FileDescriptor {
        unimplemented!()
    }

    /// `.proto` data for this file.
    pub fn get_proto(&self) -> &FileDescriptorProto {
        self.generated.get_proto()
    }
}

impl PartialEq for FileDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.generated as *const GeneratedFileDescriptor
            == other.generated as *const GeneratedFileDescriptor
    }
}

impl Eq for FileDescriptor {}

#[cfg(test)]
mod test {
    use crate::descriptor;

    #[test]
    fn eq() {
        assert!(descriptor::file_descriptor() == descriptor::file_descriptor().clone());
    }
}

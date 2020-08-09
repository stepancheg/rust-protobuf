#![allow(dead_code)] // TODO: don't forget to remove

use crate::descriptor::FileDescriptorProto;
use crate::reflect::GeneratedFileDescriptor;

pub(crate) mod dynamic;
pub(crate) mod generated;

#[derive(Clone)]
pub(crate) enum FileDescriptorImpl {
    Generated(&'static GeneratedFileDescriptor),
}

impl PartialEq for FileDescriptorImpl {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FileDescriptorImpl::Generated(a), FileDescriptorImpl::Generated(b)) => {
                *a as *const GeneratedFileDescriptor == *b as *const GeneratedFileDescriptor
            }
        }
    }
}

impl Eq for FileDescriptorImpl {}

/// Reflection for objects defined in `.proto` file (messages, enums, etc).
#[derive(Clone, PartialEq, Eq)]
pub struct FileDescriptor {
    pub(crate) imp: FileDescriptorImpl,
}

impl FileDescriptor {
    /// This function is called from generated code, it is not stable, and should not be called.
    pub fn new_generated_2(generated: &'static GeneratedFileDescriptor) -> FileDescriptor {
        FileDescriptor {
            imp: FileDescriptorImpl::Generated(generated),
        }
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
        match self.imp {
            FileDescriptorImpl::Generated(g) => g.get_proto(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::descriptor;

    #[test]
    fn eq() {
        assert!(descriptor::file_descriptor() == descriptor::file_descriptor().clone());
    }
}

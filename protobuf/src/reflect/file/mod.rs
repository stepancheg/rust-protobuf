#![allow(dead_code)] // TODO: don't forget to remove

use crate::descriptor::FileDescriptorProto;
use crate::reflect::file::dynamic::DynamicFileDescriptor;
use crate::reflect::GeneratedFileDescriptor;
use std::sync::Arc;

pub(crate) mod dynamic;
pub(crate) mod generated;

#[derive(Clone)]
pub(crate) enum FileDescriptorImpl {
    Generated(&'static GeneratedFileDescriptor),
    Dynamic(Arc<DynamicFileDescriptor>),
}

impl PartialEq for FileDescriptorImpl {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FileDescriptorImpl::Generated(a), FileDescriptorImpl::Generated(b)) => {
                *a as *const GeneratedFileDescriptor == *b as *const GeneratedFileDescriptor
            }
            (FileDescriptorImpl::Dynamic(a), FileDescriptorImpl::Dynamic(b)) => Arc::ptr_eq(a, b),
            _ => false,
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
        proto: FileDescriptorProto,
        dependencies: Vec<FileDescriptor>,
    ) -> FileDescriptor {
        FileDescriptor {
            imp: FileDescriptorImpl::Dynamic(Arc::new(DynamicFileDescriptor::new(
                proto,
                dependencies,
            ))),
        }
    }

    /// `.proto` data for this file.
    pub fn get_proto(&self) -> &FileDescriptorProto {
        match &self.imp {
            FileDescriptorImpl::Generated(g) => &g.proto,
            FileDescriptorImpl::Dynamic(d) => &d.proto,
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

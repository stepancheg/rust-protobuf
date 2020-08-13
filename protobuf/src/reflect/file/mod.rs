use crate::descriptor::DescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::file::dynamic::DynamicFileDescriptor;
use crate::reflect::file::fds::FdsBuilder;
use crate::reflect::file::index::FileIndex;
use crate::reflect::file::index::FileIndexMessageEntry;
use crate::reflect::name::protobuf_name_starts_with_package;
use crate::reflect::MessageDescriptor;
use crate::reflect::{EnumDescriptor, GeneratedFileDescriptor};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::hash::Hasher;
use std::sync::Arc;

pub(crate) mod building;
pub(crate) mod dynamic;
pub(crate) mod fds;
pub(crate) mod generated;
pub(crate) mod index;

#[derive(Clone, Debug)]
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

impl Hash for FileDescriptorImpl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            FileDescriptorImpl::Generated(g) => {
                Hash::hash(&(*g as *const GeneratedFileDescriptor), state)
            }
            FileDescriptorImpl::Dynamic(a) => {
                Hash::hash(&(&**a as *const DynamicFileDescriptor), state)
            }
        }
    }
}

impl Eq for FileDescriptorImpl {}

/// Reflection for objects defined in `.proto` file (messages, enums, etc).
///
/// The object is refcounted: clone is shallow.
///
/// The equality performs pointer comparison: two clones of the same `FileDescriptor`
/// objects are equal, but two `FileDescriptor` objects created from the same `FileDescriptorProto`
/// objects are **not** equal.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FileDescriptor {
    pub(crate) imp: FileDescriptorImpl,
}

impl FileDescriptor {
    fn get_index(&self) -> &FileIndex {
        match &self.imp {
            FileDescriptorImpl::Generated(g) => &g.index,
            FileDescriptorImpl::Dynamic(d) => &d.index,
        }
    }

    pub(crate) fn get_message_index_entry(&self, index: usize) -> &FileIndexMessageEntry {
        &self.get_index().messages[index]
    }

    pub(crate) fn get_message_proto(&self, index: usize) -> &DescriptorProto {
        self.get_message_index_entry(index)
            .path
            .eval(self.get_proto())
            .unwrap()
    }

    /// Get top-level messages.
    pub fn get_messages(&self) -> Vec<MessageDescriptor> {
        self.get_index()
            .top_level_messages
            .iter()
            .map(|i| MessageDescriptor::new(self.clone(), *i))
            .collect()
    }

    /// Get top-level enums.
    pub fn get_enums(&self) -> Vec<EnumDescriptor> {
        self.get_proto()
            .enum_type
            .iter()
            .enumerate()
            .map(|(i, _)| EnumDescriptor::new(self.clone(), i))
            .collect()
    }

    /// Find message by name relative to the package.
    ///
    /// Only search in the current file, not in any dependencies.
    pub fn get_message_by_package_relative_name(&self, name: &str) -> Option<MessageDescriptor> {
        self.get_index()
            .message_by_name_to_package
            .get(name)
            .map(|&index| MessageDescriptor::new(self.clone(), index))
    }

    /// Find message by name relative to the package.
    ///
    /// Only search in the current file, not in any dependencies.
    pub fn get_enum_by_package_relative_name(&self, name: &str) -> Option<EnumDescriptor> {
        self.get_index()
            .enums_by_name_to_package
            .get(name)
            .map(|&index| EnumDescriptor::new(self.clone(), index))
    }

    /// Find message by fully-qualified name.
    ///
    /// Only search in the current file, not in any dependencies.
    pub fn get_message_by_full_name(&self, name: &str) -> Option<MessageDescriptor> {
        if let Some(name_to_package) =
            protobuf_name_starts_with_package(name, self.get_proto().get_package())
        {
            self.get_message_by_package_relative_name(name_to_package)
        } else {
            None
        }
    }

    /// Find enum by name fully-qualified name.
    ///
    /// Only search in the current file, not in any dependencies.
    pub fn get_enum_by_full_name(&self, name: &str) -> Option<EnumDescriptor> {
        if let Some(name_to_package) =
            protobuf_name_starts_with_package(name, self.get_proto().get_package())
        {
            self.get_enum_by_package_relative_name(name_to_package)
        } else {
            None
        }
    }

    /// This function is called from generated code, it is not stable, and should not be called.
    #[doc(hidden)]
    // TODO: rename
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
        // remove undeclared dependencies
        let dependencies: HashMap<_, _> = dependencies
            .iter()
            .map(|d| (d.get_proto().get_name(), d))
            .collect();
        let dependencies: Vec<_> = proto
            .dependency
            .iter()
            .map(|d| dependencies[d.as_str()].clone())
            .collect();

        FileDescriptor {
            imp: FileDescriptorImpl::Dynamic(Arc::new(DynamicFileDescriptor::new(
                proto,
                dependencies,
            ))),
        }
    }

    /// Create a set of file descriptors from individual file descriptors.
    pub fn new_dynamic_fds(protos: Vec<FileDescriptorProto>) -> Vec<FileDescriptor> {
        FdsBuilder::build(protos)
    }

    /// `.proto` data for this file.
    pub fn get_proto(&self) -> &FileDescriptorProto {
        match &self.imp {
            FileDescriptorImpl::Generated(g) => &g.proto,
            FileDescriptorImpl::Dynamic(d) => &d.proto,
        }
    }

    fn get_deps(&self) -> &[FileDescriptor] {
        match &self.imp {
            FileDescriptorImpl::Generated(g) => &g.dependencies,
            FileDescriptorImpl::Dynamic(d) => &d.dependencies,
        }
    }

    /// Subset of dependencies which are public
    pub fn public_deps(&self) -> Vec<FileDescriptor> {
        self.get_proto()
            .public_dependency
            .iter()
            .map(|&i| self.get_deps()[i as usize].clone())
            .collect()
    }

    fn _get_all_files(&self) -> Vec<&FileDescriptor> {
        let mut r = Vec::new();
        let mut visited = HashSet::new();

        let mut stack = Vec::new();
        stack.push(self);
        while let Some(file) = stack.pop() {
            if !visited.insert(file) {
                continue;
            }

            r.push(file);
            stack.extend(file.get_deps());
        }

        r
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

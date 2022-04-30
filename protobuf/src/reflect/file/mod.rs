use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::sync::Arc;

use crate::descriptor::DescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::error::ReflectError;
use crate::reflect::field::FieldDescriptorImpl;
use crate::reflect::file::dynamic::DynamicFileDescriptor;
use crate::reflect::file::fds::build_fds;
use crate::reflect::file::index::EnumIndex;
use crate::reflect::file::index::FileDescriptorCommon;
use crate::reflect::file::index::MessageIndex;
use crate::reflect::name::protobuf_name_starts_with_package;
use crate::reflect::service::ServiceDescriptor;
use crate::reflect::EnumDescriptor;
use crate::reflect::FieldDescriptor;
use crate::reflect::GeneratedFileDescriptor;
use crate::reflect::MessageDescriptor;
use crate::reflect::Syntax;

pub(crate) mod building;
pub(crate) mod dynamic;
pub(crate) mod fds;
pub(crate) mod generated;
pub(crate) mod index;
pub(crate) mod syntax;

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
    pub(crate) fn common(&self) -> &FileDescriptorCommon {
        match &self.imp {
            FileDescriptorImpl::Generated(g) => &g.common,
            FileDescriptorImpl::Dynamic(d) => &d.common,
        }
    }

    pub(crate) fn generated_index(&self) -> &'static FileDescriptorCommon {
        match &self.imp {
            FileDescriptorImpl::Generated(g) => &g.common,
            FileDescriptorImpl::Dynamic(..) => panic!("not generated"),
        }
    }

    pub(crate) fn message_index_entry(&self, index: usize) -> &MessageIndex {
        &self.common().messages[index]
    }

    pub(crate) fn enum_index_entry(&self, index: usize) -> &EnumIndex {
        &self.common().enums[index]
    }

    pub(crate) fn message_proto(&self, index: usize) -> &DescriptorProto {
        // TODO: this should be faster.
        self.message_index_entry(index)
            .path
            .eval(self.proto())
            .unwrap()
    }

    /// Syntax of current file.
    pub fn syntax(&self) -> Syntax {
        Syntax::parse(self.proto().syntax()).unwrap_or(Syntax::Proto2)
    }

    // TODO: return iterator.
    /// Get top-level messages.
    pub fn messages(&self) -> impl Iterator<Item = MessageDescriptor> + '_ {
        self.common()
            .top_level_messages
            .iter()
            .map(|i| MessageDescriptor::new(self.clone(), *i))
    }

    /// Get top-level enums.
    pub fn enums(&self) -> impl Iterator<Item = EnumDescriptor> + '_ {
        self.proto()
            .enum_type
            .iter()
            .enumerate()
            .map(|(i, _)| EnumDescriptor::new(self.clone(), i))
    }

    /// Get services defined in `.proto` file.
    pub fn services(&self) -> impl Iterator<Item = ServiceDescriptor> + '_ {
        self.proto()
            .service
            .iter()
            .enumerate()
            .map(|(i, _)| ServiceDescriptor::new(self.clone(), i))
    }

    /// Extension fields.
    pub fn extensions(&self) -> impl Iterator<Item = FieldDescriptor> + '_ {
        (0..self.common().extensions.len()).map(move |index| FieldDescriptor {
            imp: FieldDescriptorImpl::ExtensionInFile(self.clone(), index),
        })
    }

    /// Find message by name relative to the package.
    ///
    /// Only search in the current file, not in any dependencies.
    pub fn message_by_package_relative_name(&self, name: &str) -> Option<MessageDescriptor> {
        self.common()
            .message_by_name_to_package
            .get(name)
            .map(|&index| MessageDescriptor::new(self.clone(), index))
    }

    /// Find message by name relative to the package.
    ///
    /// Only search in the current file, not in any dependencies.
    pub fn enum_by_package_relative_name(&self, name: &str) -> Option<EnumDescriptor> {
        self.common()
            .enums_by_name_to_package
            .get(name)
            .map(|&index| EnumDescriptor::new(self.clone(), index))
    }

    /// Find message by fully-qualified name.
    ///
    /// Only search in the current file, not in any dependencies.
    pub fn message_by_full_name(&self, name: &str) -> Option<MessageDescriptor> {
        if let Some(name_to_package) =
            protobuf_name_starts_with_package(name, self.proto().package())
        {
            self.message_by_package_relative_name(name_to_package)
        } else {
            None
        }
    }

    /// Find enum by name fully-qualified name.
    ///
    /// Only search in the current file, not in any dependencies.
    pub fn enum_by_full_name(&self, name: &str) -> Option<EnumDescriptor> {
        if let Some(name_to_package) =
            protobuf_name_starts_with_package(name, self.proto().package())
        {
            self.enum_by_package_relative_name(name_to_package)
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
        dependencies: &[FileDescriptor],
    ) -> crate::Result<FileDescriptor> {
        // remove undeclared dependencies
        let dependencies_index: HashMap<_, &FileDescriptor> =
            dependencies.iter().map(|d| (d.proto().name(), d)).collect();

        if dependencies_index.len() != dependencies.len() {
            return Err(ReflectError::NonUniqueDependencies(
                dependencies
                    .iter()
                    .map(|d| d.proto().name())
                    .collect::<Vec<_>>()
                    .join(", "),
            )
            .into());
        }

        let dependencies: Vec<FileDescriptor> = proto
            .dependency
            .iter()
            .map(|d| {
                let dep = dependencies_index.get(d.as_str());
                match dep {
                    Some(dep) => Ok((*dep).clone()),
                    None => Err(ReflectError::DependencyNotFound(
                        d.clone(),
                        proto.name().to_owned(),
                        dependencies
                            .iter()
                            .map(|d| d.proto().name())
                            .collect::<Vec<_>>()
                            .join(", "),
                    )
                    .into()),
                }
            })
            .collect::<crate::Result<Vec<_>>>()?;

        Ok(FileDescriptor {
            imp: FileDescriptorImpl::Dynamic(Arc::new(DynamicFileDescriptor::new(
                proto,
                dependencies,
            )?)),
        })
    }

    /// Create a set of file descriptors from individual file descriptors.
    pub fn new_dynamic_fds(
        protos: Vec<FileDescriptorProto>,
        dependencies: &[FileDescriptor],
    ) -> crate::Result<Vec<FileDescriptor>> {
        build_fds(protos, dependencies)
    }

    /// `.proto` data for this file.
    pub fn proto(&self) -> &FileDescriptorProto {
        match &self.imp {
            FileDescriptorImpl::Generated(g) => &g.proto,
            FileDescriptorImpl::Dynamic(d) => &d.proto,
        }
    }

    /// Direct dependencies of this file.
    pub fn deps(&self) -> &[FileDescriptor] {
        &self.common().dependencies
    }

    /// Subset of dependencies which are public
    pub fn public_deps(&self) -> impl Iterator<Item = FileDescriptor> + '_ {
        self.proto()
            .public_dependency
            .iter()
            .map(|&i| self.deps()[i as usize].clone())
    }

    fn _all_files(&self) -> Vec<&FileDescriptor> {
        let mut r = Vec::new();
        let mut visited = HashSet::new();

        let mut stack = Vec::new();
        stack.push(self);
        while let Some(file) = stack.pop() {
            if !visited.insert(file) {
                continue;
            }

            r.push(file);
            stack.extend(file.deps());
        }

        r
    }
}

#[cfg(test)]
mod test {
    use crate::descriptor;

    #[test]
    #[cfg_attr(miri, ignore)] // TODO: figure out why this test hangs on Miri.
    fn eq() {
        assert!(descriptor::file_descriptor() == descriptor::file_descriptor().clone());
    }
}

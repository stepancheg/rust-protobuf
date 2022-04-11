use std::any::TypeId;
use std::fmt;
use std::fmt::Formatter;
use std::hash::Hash;

use crate::descriptor::EnumDescriptorProto;
use crate::descriptor::EnumValueDescriptorProto;
use crate::enums::Enum;
use crate::reflect::enums::generated::GeneratedEnumDescriptor;
use crate::reflect::file::index::EnumIndex;
use crate::reflect::file::FileDescriptorImpl;
use crate::reflect::FileDescriptor;
use crate::reflect::MessageDescriptor;
use crate::EnumFull;

pub(crate) mod generated;
pub(crate) mod path;

/// Description for enum variant.
///
/// Used in reflection.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct EnumValueDescriptor {
    pub(crate) enum_descriptor: EnumDescriptor,
    pub(crate) index: usize,
}

fn _assert_send_sync() {
    fn _assert_send_sync<T: Send + Sync>() {}
    _assert_send_sync::<EnumValueDescriptor>();
}

impl fmt::Debug for EnumValueDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("EnumValueDescriptor")
            .field("enum_descriptor", &self.enum_descriptor)
            .field("name", &self.name())
            .finish()
    }
}

impl fmt::Display for EnumValueDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

impl EnumValueDescriptor {
    pub(crate) fn new(enum_descriptor: EnumDescriptor, index: usize) -> EnumValueDescriptor {
        EnumValueDescriptor {
            enum_descriptor,
            index,
        }
    }

    /// `.proto` object which declared this value.
    pub fn proto(&self) -> &EnumValueDescriptorProto {
        &self.enum_descriptor.proto().value[self.index as usize]
    }

    /// Name of enum variant as specified in proto file
    pub fn name(&self) -> &str {
        self.proto().name()
    }

    /// Fully qualified enum value name: fully qualified enum name followed by value name.
    pub fn full_name(&self) -> String {
        format!("{}.{}", self.enum_descriptor.full_name(), self.name())
    }

    /// `i32` value of the enum variant
    pub fn value(&self) -> i32 {
        self.proto().number()
    }

    /// Get descriptor of enum holding this value.
    pub fn enum_descriptor(&self) -> &EnumDescriptor {
        &self.enum_descriptor
    }

    /// Convert this value descriptor into proper enum object.
    ///
    /// ```
    /// # use protobuf::well_known_types::NullValue;
    /// # use protobuf::EnumFull;
    /// # use protobuf::reflect::EnumValueDescriptor;
    ///
    /// # if !cfg!(miri) {
    /// # // TODO: Figure out why.
    /// let value: EnumValueDescriptor = NullValue::NULL_VALUE.descriptor();
    /// let null: Option<NullValue> = value.cast();
    /// assert_eq!(Some(NullValue::NULL_VALUE), null);
    /// # }
    /// ```
    pub fn cast<E: EnumFull>(&self) -> Option<E> {
        if self.enum_descriptor != E::enum_descriptor() {
            return None;
        }
        E::from_i32(self.value())
    }
}

/// Dynamic representation of enum type.
///
/// Can be used in reflective operations.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct EnumDescriptor {
    file_descriptor: FileDescriptor,
    index: usize,
}

impl fmt::Display for EnumDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

impl fmt::Debug for EnumDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("EnumDescriptor")
            // TODO
            //.field("full_name", &self.full_name)
            .finish_non_exhaustive()
    }
}

impl EnumDescriptor {
    pub(crate) fn new(file_descriptor: FileDescriptor, index: usize) -> EnumDescriptor {
        EnumDescriptor {
            file_descriptor,
            index,
        }
    }

    /// Get internal index.
    ///
    /// Only needed for codegen.
    #[doc(hidden)]
    pub fn index_in_file_for_codegen(&self) -> usize {
        self.index
    }

    fn get_impl(&self) -> EnumDescriptorImplRef {
        match &self.file_descriptor.imp {
            FileDescriptorImpl::Generated(g) => {
                EnumDescriptorImplRef::Generated(&g.enums[self.index])
            }
            FileDescriptorImpl::Dynamic(..) => EnumDescriptorImplRef::Dynamic,
        }
    }

    /// Descriptor objects which defined this enum.
    pub fn proto(&self) -> &EnumDescriptorProto {
        self.index_entry()
            .enum_path
            .eval(self.file_descriptor.proto())
    }

    /// Enum name as given in `.proto` file
    pub fn name(&self) -> &str {
        self.proto().name()
    }

    fn index_entry(&self) -> &EnumIndex {
        self.file_descriptor.enum_index_entry(self.index)
    }

    /// Fully qualified protobuf name of enum
    pub fn full_name(&self) -> &str {
        &self.index_entry().full_name
    }

    /// Get `EnumDescriptor` object for given enum type
    pub fn for_type<E: EnumFull>() -> EnumDescriptor {
        E::enum_descriptor()
    }

    #[doc(hidden)]
    pub fn new_generated_2(file_descriptor: FileDescriptor, index: usize) -> EnumDescriptor {
        EnumDescriptor {
            file_descriptor,
            index,
        }
    }

    /// Get a message containing this message, or `None` if this message is declared at file level.
    pub fn enclosing_message(&self) -> Option<MessageDescriptor> {
        self.index_entry()
            .enclosing_message
            .map(|i| MessageDescriptor::new(self.file_descriptor.clone(), i))
    }

    /// This enum values
    pub fn values<'a>(&'a self) -> impl Iterator<Item = EnumValueDescriptor> + 'a {
        let value_len = self.proto().value.len();
        (0..value_len).map(move |index| EnumValueDescriptor {
            enum_descriptor: self.clone(),
            index,
        })
    }

    /// First variant (also default in proto3).
    pub fn first_value(&self) -> EnumValueDescriptor {
        self.values().next().unwrap()
    }

    /// Find enum variant by name
    pub fn value_by_name(&self, name: &str) -> Option<EnumValueDescriptor> {
        let index = *self.file_descriptor.index().index.enums[self.index]
            .index_by_name
            .get(name)?;
        Some(EnumValueDescriptor {
            enum_descriptor: self.clone(),
            index,
        })
    }

    /// Find enum variant by number
    pub fn value_by_number(&self, number: i32) -> Option<EnumValueDescriptor> {
        let index = *self.file_descriptor.index().index.enums[self.index]
            .index_by_number
            .get(&number)?;
        Some(self.value_by_index(index))
    }

    /// Get enum variant by index (as declared in `.proto` file).
    pub fn value_by_index(&self, index: usize) -> EnumValueDescriptor {
        EnumValueDescriptor {
            enum_descriptor: self.clone(),
            index,
        }
    }

    /// Default enum value (first variant)
    pub fn default_value(&self) -> EnumValueDescriptor {
        EnumValueDescriptor {
            enum_descriptor: self.clone(),
            index: 0,
        }
    }

    /// Find enum variant by number or return default (first) enum value
    pub fn value_by_number_or_default(&self, number: i32) -> EnumValueDescriptor {
        match self.value_by_number(number) {
            Some(v) => v,
            None => self.default_value(),
        }
    }

    /// Check if this enum descriptor corresponds given enum type
    ///
    /// ```
    /// # use protobuf::EnumFull;
    /// # use protobuf::descriptor::field_descriptor_proto::Label;
    /// # use protobuf::reflect::EnumDescriptor;
    ///
    /// # if !cfg!(miri) {
    /// # // TODO: figure out why
    /// let descriptor: EnumDescriptor = Label::enum_descriptor();
    ///
    /// assert!(descriptor.is::<Label>())
    /// }
    /// ```
    pub fn is<E: Enum>(&self) -> bool {
        match self.get_impl() {
            EnumDescriptorImplRef::Generated(g) => g.type_id == TypeId::of::<E>(),
            EnumDescriptorImplRef::Dynamic => false,
        }
    }
}

enum EnumDescriptorImplRef {
    Generated(&'static GeneratedEnumDescriptor),
    Dynamic,
}

#[cfg(test)]
mod test {
    use crate::descriptor::field_descriptor_proto::Label;
    use crate::descriptor::field_descriptor_proto::Type;
    use crate::descriptor::FieldDescriptorProto;
    use crate::well_known_types::NullValue;
    use crate::EnumFull;
    use crate::MessageFull;

    #[test]
    #[cfg_attr(miri, ignore)] // Too slow on Miri.
    fn enclosing_message() {
        assert_eq!(
            Some(FieldDescriptorProto::descriptor()),
            Type::enum_descriptor().enclosing_message()
        );
        assert_eq!(None, NullValue::enum_descriptor().enclosing_message());
    }

    #[test]
    fn to_string() {
        assert_eq!(
            "google.protobuf.FieldDescriptorProto.Label",
            Label::enum_descriptor().to_string()
        );
        assert_eq!(
            "google.protobuf.FieldDescriptorProto.Label",
            Label::enum_descriptor().full_name()
        );
        assert_eq!(
            "google.protobuf.FieldDescriptorProto.Label.LABEL_REPEATED",
            Label::LABEL_REPEATED.descriptor().to_string()
        );
        assert_eq!(
            "google.protobuf.FieldDescriptorProto.Label.LABEL_REPEATED",
            Label::LABEL_REPEATED.descriptor().full_name()
        );
    }
}

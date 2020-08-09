use std::any::TypeId;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;

use crate::descriptor::EnumDescriptorProto;
use crate::descriptor::EnumValueDescriptorProto;
use crate::enums::ProtobufEnum;
use crate::enums::ProtobufEnumOrUnknown;
use crate::reflect::enums::generated::GeneratedEnumDescriptor;
#[cfg(not(rustc_nightly))]
use crate::reflect::enums::generated::GetEnumDescriptor;
use crate::reflect::file::FileDescriptorImpl;
use crate::reflect::FileDescriptor;
use crate::reflect::ProtobufValue;

pub(crate) mod dynamic;
pub(crate) mod generated;

/// Description for enum variant.
///
/// Used in reflection.
#[derive(Clone, Eq, PartialEq)]
pub struct EnumValueDescriptor {
    pub(crate) enum_descriptor: EnumDescriptor,
    pub(crate) index: usize,
}

impl Hash for EnumValueDescriptor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.value(), state)
    }
}

fn _assert_send_sync() {
    fn _assert_send_sync<T: Send + Sync>() {}
    _assert_send_sync::<EnumValueDescriptor>();
}

impl fmt::Debug for EnumValueDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("EnumValueDescriptor")
            // TODO: add something
            .field("value", &"...")
            .finish()
    }
}

impl EnumValueDescriptor {
    fn get_proto(&self) -> &EnumValueDescriptorProto {
        &self.enum_descriptor.get_proto().value[self.index as usize]
    }

    /// Name of enum variant as specified in proto file
    pub fn name(&self) -> &str {
        self.get_proto().get_name()
    }

    /// `i32` value of the enum variant
    pub fn value(&self) -> i32 {
        self.get_proto().get_number()
    }

    /// Get descriptor of enum holding this value.
    pub fn enum_descriptor(&self) -> &EnumDescriptor {
        &self.enum_descriptor
    }

    /// Convert this value descriptor into proper enum object.
    ///
    /// ```
    /// # use protobuf::well_known_types::NullValue;
    /// # use protobuf::ProtobufEnum;
    /// # use protobuf::reflect::EnumValueDescriptor;
    ///
    /// let value: EnumValueDescriptor = NullValue::NULL_VALUE.descriptor();
    /// let null: Option<NullValue> = value.cast();
    /// assert_eq!(Some(NullValue::NULL_VALUE), null);
    /// ```
    pub fn cast<E: ProtobufEnum>(&self) -> Option<E> {
        self.enum_descriptor()
            .cast_to_protobuf_enum::<E>(self.value())
    }
}

/// Dynamic representation of enum type.
///
/// Can be used in reflective operations.
#[derive(Clone, Eq, PartialEq)]
pub struct EnumDescriptor {
    file_descriptor: FileDescriptor,
    index: u32,
}

impl fmt::Debug for EnumDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("EnumDescriptor")
            // TODO
            //.field("full_name", &self.full_name)
            .field("..", &"..")
            .finish()
    }
}

impl EnumDescriptor {
    fn get_proto(&self) -> &EnumDescriptorProto {
        self.get_generated().proto
    }

    #[deprecated]
    fn get_generated(&self) -> &GeneratedEnumDescriptor {
        match self.file_descriptor.imp {
            FileDescriptorImpl::Generated(g) => &g.enums[self.index as usize],
            _ => unimplemented!("TODO"),
        }
    }

    /// Enum name as given in `.proto` file
    pub fn name(&self) -> &'static str {
        self.get_generated().proto.get_name()
    }

    /// Fully qualified protobuf name of enum
    pub fn full_name(&self) -> &str {
        &self.get_generated().full_name
    }

    /// Get `EnumDescriptor` object for given enum type
    pub fn for_type<E: ProtobufEnum>() -> EnumDescriptor {
        E::enum_descriptor_static()
    }

    /// Separate function to reduce generated code size bloat.
    pub(crate) fn make_indices(
        proto: &EnumDescriptorProto,
    ) -> (HashMap<String, usize>, HashMap<i32, usize>) {
        let mut index_by_name = HashMap::new();
        let mut index_by_number = HashMap::new();
        for (i, v) in proto.value.iter().enumerate() {
            index_by_number.insert(v.get_number(), i);
            index_by_name.insert(v.get_name().to_string(), i);
        }

        (index_by_name, index_by_number)
    }

    #[doc(hidden)]
    pub fn new_generated_2(file_descriptor: FileDescriptor, index: u32) -> EnumDescriptor {
        EnumDescriptor {
            file_descriptor,
            index,
        }
    }

    /// This enum values
    pub fn values(&self) -> Vec<EnumValueDescriptor> {
        (0..self.get_generated().proto.value.len())
            .map(|index| EnumValueDescriptor {
                enum_descriptor: self.clone(),
                index,
            })
            .collect()
    }

    /// Find enum variant by name
    pub fn get_value_by_name<'a>(&'a self, name: &str) -> Option<EnumValueDescriptor> {
        let &index = self.get_generated().index_by_name.get(name)?;
        Some(EnumValueDescriptor {
            enum_descriptor: self.clone(),
            index,
        })
    }

    /// Find enum variant by number
    pub fn get_value_by_number(&self, number: i32) -> Option<EnumValueDescriptor> {
        let &index = self.get_generated().index_by_number.get(&number)?;
        Some(EnumValueDescriptor {
            enum_descriptor: self.clone(),
            index,
        })
    }

    /// Find enum variant by number or return default (first) enum value
    pub fn get_value_by_number_or_default(&self, number: i32) -> EnumValueDescriptor {
        match self.get_value_by_number(number) {
            Some(v) => v,
            None => EnumValueDescriptor {
                enum_descriptor: self.clone(),
                index: 0,
            },
        }
    }

    /// Check if this enum descriptor corresponds given enum type
    ///
    /// ```
    /// # use protobuf::ProtobufEnum;
    /// # use protobuf::descriptor::field_descriptor_proto::Label;
    /// # use protobuf::reflect::EnumDescriptor;
    ///
    /// let descriptor: &EnumDescriptor = Label::enum_descriptor_static();
    ///
    /// assert!(descriptor.is::<Label>())
    /// ```
    pub fn is<E: ProtobufEnum>(&self) -> bool {
        TypeId::of::<E>() == self.get_generated().type_id
    }

    /// Create enum object from given value.
    ///
    /// Type parameter `E` can be either [`ProtobufEnum`](crate::ProtobufEnum)
    /// or [`ProtobufEnumOrUnknown`](crate::ProtobufEnumOrUnknown).
    ///
    /// # Panics
    ///
    /// This operation panics of `E` is `ProtobufEnum` and `value` is unknown.
    pub(crate) fn cast<E: ProtobufValue>(&self, value: i32) -> Option<E> {
        if let Some(e) = self.cast_to_protobuf_enum(value) {
            return Some(e);
        }
        if let Some(e) = self.cast_to_protobuf_enum_or_unknown(value) {
            return Some(e);
        }
        None
    }

    #[cfg(rustc_nightly)]
    fn cast_to_protobuf_enum<E: ProtobufValue>(&self, value: i32) -> Option<E> {
        if TypeId::of::<E>() != self.get_generated().type_id {
            return None;
        }

        Some(<E as cast_impl::CastValueToProtobufEnum>::cast(value))
    }

    #[cfg(not(rustc_nightly))]
    fn cast_to_protobuf_enum<E: ProtobufValue>(&self, value: i32) -> Option<E> {
        if TypeId::of::<E>() != self.get_generated().type_id {
            return None;
        }

        use std::mem;
        unsafe {
            let mut r = mem::MaybeUninit::<E>::uninit();
            self.get_generated()
                .get_descriptor
                .copy_to(value, r.as_mut_ptr() as *mut ());
            Some(r.assume_init())
        }
    }

    #[cfg(rustc_nightly)]
    fn cast_to_protobuf_enum_or_unknown<E: ProtobufValue>(&self, value: i32) -> Option<E> {
        if TypeId::of::<E>() != self.get_generated().enum_or_unknown_type_id {
            return None;
        }

        Some(<E as cast_impl::CastValueToProtobufEnumOrUnknown>::cast(
            value,
        ))
    }

    #[cfg(not(rustc_nightly))]
    fn cast_to_protobuf_enum_or_unknown<E: ProtobufValue>(&self, value: i32) -> Option<E> {
        if TypeId::of::<E>() != self.get_generated().enum_or_unknown_type_id {
            return None;
        }

        use std::mem;
        use std::ptr;
        debug_assert_eq!(mem::size_of::<E>(), mem::size_of::<i32>());
        unsafe {
            // This works because `ProtobufEnumOrUnknown<E>` is `#[repr(transparent)]`
            let mut r = mem::MaybeUninit::<E>::uninit();
            ptr::copy(&value, r.as_mut_ptr() as *mut i32, 1);
            Some(r.assume_init())
        }
    }
}

#[cfg(rustc_nightly)]
mod cast_impl {
    use super::*;

    pub(crate) trait CastValueToProtobufEnumOrUnknown: Sized {
        fn cast(value: i32) -> Self;
    }

    impl<T> CastValueToProtobufEnumOrUnknown for T {
        default fn cast(_value: i32) -> T {
            unreachable!();
        }
    }

    impl<E: ProtobufEnum> CastValueToProtobufEnumOrUnknown for ProtobufEnumOrUnknown<E> {
        fn cast(value: i32) -> ProtobufEnumOrUnknown<E> {
            ProtobufEnumOrUnknown::from_i32(value)
        }
    }

    pub(crate) trait CastValueToProtobufEnum: Sized {
        fn cast(value: i32) -> Self;
    }

    impl<T> CastValueToProtobufEnum for T {
        default fn cast(_value: i32) -> T {
            unreachable!();
        }
    }

    impl<E: ProtobufEnum> CastValueToProtobufEnum for E {
        fn cast(value: i32) -> E {
            E::from_i32(value).expect(&format!("unknown enum value: {}", value))
        }
    }
}

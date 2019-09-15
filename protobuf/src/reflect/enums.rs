use std::any::TypeId;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker;

use crate::descriptor::EnumDescriptorProto;
use crate::descriptor::EnumValueDescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::enums::ProtobufEnum;
use crate::enums::ProtobufEnumOrUnknown;
use crate::reflect::find_message_or_enum::find_message_or_enum;
use crate::reflect::find_message_or_enum::MessageOrEnum;
use crate::reflect::ProtobufValue;

/// Description for enum variant.
///
/// Used in reflection.
#[derive(Clone)]
pub struct EnumValueDescriptor {
    proto: &'static EnumValueDescriptorProto,
    protobuf_value: &'static dyn ProtobufValue,
    enum_descriptor_static: fn() -> &'static EnumDescriptor,
}

impl PartialEq for EnumValueDescriptor {
    fn eq(&self, other: &EnumValueDescriptor) -> bool {
        self.enum_descriptor() == other.enum_descriptor() && self.value() == other.value()
    }
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
            .field("proto", self.proto)
            .field("value", &"...")
            .finish()
    }
}

impl Copy for EnumValueDescriptor {}

impl EnumValueDescriptor {
    /// Name of enum variant as specified in proto file
    pub fn name(&self) -> &'static str {
        self.proto.get_name()
    }

    /// `i32` value of the enum variant
    pub fn value(&self) -> i32 {
        self.proto.get_number()
    }

    /// Convert to generic `ProtobufValue`
    pub fn protobuf_value(&self) -> &'static dyn ProtobufValue {
        self.protobuf_value
    }

    /// Get descriptor of enum holding this value.
    pub fn enum_descriptor(&self) -> &EnumDescriptor {
        (self.enum_descriptor_static)()
    }

    /// Convert this value descriptor into proper enum object.
    ///
    /// ```
    /// # use protobuf::well_known_types::NullValue;
    /// # use protobuf::ProtobufEnum;
    /// # use protobuf::reflect::EnumValueDescriptor;
    ///
    /// let value: &EnumValueDescriptor = NullValue::NULL_VALUE.descriptor();
    /// let null: Option<NullValue> = value.cast();
    /// assert_eq!(Some(NullValue::NULL_VALUE), null);
    /// ```
    pub fn cast<E: ProtobufEnum>(&self) -> Option<E> {
        self.enum_descriptor()
            .cast_to_protobuf_enum::<E>(self.value())
    }
}

trait GetEnumDescriptor: Send + Sync + 'static {
    unsafe fn copy_to(&self, value: i32, dest: *mut ());
}

struct GetDescriptorImpl<E: ProtobufEnum>(marker::PhantomData<E>);

impl<E: ProtobufEnum> GetEnumDescriptor for GetDescriptorImpl<E> {
    unsafe fn copy_to(&self, value: i32, dest: *mut ()) {
        let e = E::from_i32(value).expect("unknown value");
        (&e as *const E).copy_to(dest as *mut E, 1);
    }
}

/// Dynamic representation of enum type.
///
/// Can be used in reflective operations.
pub struct EnumDescriptor {
    full_name: String,
    proto: &'static EnumDescriptorProto,
    values: Vec<EnumValueDescriptor>,
    /// Type id of `<E>`
    type_id: TypeId,
    /// Type id of `<ProtobufEnumOrUnknown<E>>`
    enum_or_unknown_type_id: TypeId,

    #[cfg(not(rustc_nightly))]
    get_descriptor: &'static dyn GetEnumDescriptor,

    index_by_name: HashMap<String, usize>,
    index_by_number: HashMap<i32, usize>,
}

impl fmt::Debug for EnumDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("EnumDescriptor")
            .field("full_name", &self.full_name)
            .field("..", &"..")
            .finish()
    }
}

/// Identity comparison: message descriptor are equal if their addresses are equal
impl PartialEq for EnumDescriptor {
    fn eq(&self, other: &EnumDescriptor) -> bool {
        self as *const EnumDescriptor == other as *const EnumDescriptor
    }
}

impl EnumDescriptor {
    /// Enum name as given in `.proto` file
    pub fn name(&self) -> &'static str {
        self.proto.get_name()
    }

    /// Fully qualified protobuf name of enum
    pub fn full_name(&self) -> &str {
        &self.full_name[..]
    }

    /// Get `EnumDescriptor` object for given enum type
    pub fn for_type<E: ProtobufEnum>() -> &'static EnumDescriptor {
        E::enum_descriptor_static()
    }

    fn compute_full_name(
        package: &str,
        path_to_package: &str,
        proto: &EnumDescriptorProto,
    ) -> String {
        let mut full_name = package.to_owned();
        if path_to_package.len() != 0 {
            if full_name.len() != 0 {
                full_name.push('.');
            }
            full_name.push_str(path_to_package);
        }
        if full_name.len() != 0 {
            full_name.push('.');
        }
        full_name.push_str(proto.get_name());
        full_name
    }

    /// Separate function to reduce generated code size bloat.
    fn make_indices(proto: &EnumDescriptorProto) -> (HashMap<String, usize>, HashMap<i32, usize>) {
        let mut index_by_name = HashMap::new();
        let mut index_by_number = HashMap::new();
        for (i, v) in proto.value.iter().enumerate() {
            index_by_number.insert(v.get_number(), i);
            index_by_name.insert(v.get_name().to_string(), i);
        }

        (index_by_name, index_by_number)
    }

    /// Construct `EnumDescriptor` given enum name and `FileDescriptorProto`.
    ///
    /// This function is called from generated code, and should rarely be called directly.
    ///
    /// This function is not a part of public API.
    #[doc(hidden)]
    pub fn new<E>(name_in_file: &'static str, file: &'static FileDescriptorProto) -> EnumDescriptor
    where
        E: ProtobufEnum,
    {
        let (path_to_package, proto) = match find_message_or_enum(file, name_in_file) {
            (path_to_package, MessageOrEnum::Enum(e)) => (path_to_package, e),
            (_, MessageOrEnum::Message(_)) => panic!("not an enum"),
        };

        let (index_by_name, index_by_number) = EnumDescriptor::make_indices(proto);

        let proto_values = &proto.value;
        let code_values = E::values();
        assert_eq!(proto_values.len(), code_values.len());

        let values = proto_values
            .iter()
            .zip(code_values)
            .map(|(p, c)| EnumValueDescriptor {
                proto: p,
                protobuf_value: c,
                enum_descriptor_static: E::enum_descriptor_static,
            })
            .collect();

        EnumDescriptor {
            full_name: EnumDescriptor::compute_full_name(
                file.get_package(),
                &path_to_package,
                &proto,
            ),
            proto,
            values,
            type_id: TypeId::of::<E>(),
            enum_or_unknown_type_id: TypeId::of::<ProtobufEnumOrUnknown<E>>(),
            #[cfg(not(rustc_nightly))]
            get_descriptor: &GetDescriptorImpl(marker::PhantomData::<E>),
            index_by_name,
            index_by_number,
        }
    }

    /// This enum values
    pub fn values(&self) -> &[EnumValueDescriptor] {
        &self.values
    }

    /// Find enum variant by name
    pub fn get_value_by_name<'a>(&'a self, name: &str) -> Option<&'a EnumValueDescriptor> {
        let &index = self.index_by_name.get(name)?;
        Some(&self.values[index])
    }

    /// Find enum variant by number
    pub fn get_value_by_number(&self, number: i32) -> Option<&EnumValueDescriptor> {
        let &index = self.index_by_number.get(&number)?;
        Some(&self.values[index])
    }

    /// Find enum variant by number or return default (first) enum value
    pub fn get_value_by_number_or_default(&self, number: i32) -> &EnumValueDescriptor {
        match self.get_value_by_number(number) {
            Some(v) => v,
            None => &self.values()[0],
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
        TypeId::of::<E>() == self.type_id
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
        if TypeId::of::<E>() != self.type_id {
            return None;
        }

        Some(<E as cast_impl::CastValueToProtobufEnum>::cast(value))
    }

    #[cfg(not(rustc_nightly))]
    fn cast_to_protobuf_enum<E: ProtobufValue>(&self, value: i32) -> Option<E> {
        if TypeId::of::<E>() != self.type_id {
            return None;
        }

        use std::mem;
        unsafe {
            let mut r = mem::uninitialized();
            self.get_descriptor
                .copy_to(value, &mut r as *mut E as *mut ());
            Some(r)
        }
    }

    #[cfg(rustc_nightly)]
    fn cast_to_protobuf_enum_or_unknown<E: ProtobufValue>(&self, value: i32) -> Option<E> {
        if TypeId::of::<E>() != self.enum_or_unknown_type_id {
            return None;
        }

        Some(<E as cast_impl::CastValueToProtobufEnumOrUnknown>::cast(
            value,
        ))
    }

    #[cfg(not(rustc_nightly))]
    fn cast_to_protobuf_enum_or_unknown<E: ProtobufValue>(&self, value: i32) -> Option<E> {
        if TypeId::of::<E>() != self.enum_or_unknown_type_id {
            return None;
        }

        use std::mem;
        use std::ptr;
        debug_assert_eq!(mem::size_of::<E>(), mem::size_of::<i32>());
        unsafe {
            // This works because `ProtobufEnumOrUnknown<E>` is `#[repr(transparent)]`
            let mut r = mem::uninitialized();
            ptr::copy(&value, &mut r as *mut E as *mut i32, 1);
            Some(r)
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

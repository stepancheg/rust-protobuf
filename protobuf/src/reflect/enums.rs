use crate::descriptor::{EnumDescriptorProto, EnumValueDescriptorProto, FileDescriptorProto};
use crate::descriptorx::find_enum_by_rust_name;
use crate::reflect::ProtobufValue;
use crate::ProtobufEnum;
use std::any::TypeId;
use std::collections::HashMap;
use std::{fmt, marker};

/// Description for enum variant.
///
/// Used in reflection.
#[derive(Clone)]
pub struct EnumValueDescriptor {
    proto: &'static EnumValueDescriptorProto,
    enum_descriptor_static: fn() -> &'static EnumDescriptor,
}

impl PartialEq for EnumValueDescriptor {
    fn eq(&self, other: &EnumValueDescriptor) -> bool {
        self.enum_descriptor() == other.enum_descriptor() && self.value() == other.value()
    }
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

    /// Get descriptor of enum holding this value.
    pub fn enum_descriptor(&self) -> &EnumDescriptor {
        (self.enum_descriptor_static)()
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
    proto: &'static EnumDescriptorProto,
    values: Vec<EnumValueDescriptor>,
    /// Type id of `<E>`
    type_id: TypeId,

    #[cfg(not(rustc_nightly))]
    get_descriptor: &'static dyn GetEnumDescriptor,

    index_by_name: HashMap<String, usize>,
    index_by_number: HashMap<i32, usize>,
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

    /// `EnumDescriptor` for enum type
    pub fn for_type<E: ProtobufEnum>() -> &'static EnumDescriptor {
        E::enum_descriptor_static()
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
    pub fn new<E: ProtobufEnum>(
        rust_name: &'static str,
        file: &'static FileDescriptorProto,
    ) -> EnumDescriptor {
        let proto = find_enum_by_rust_name(file, rust_name);
        let (index_by_name, index_by_number) = EnumDescriptor::make_indices(proto.en);
        let values = proto
            .en
            .get_value()
            .iter()
            .map(|v| EnumValueDescriptor {
                proto: v,
                enum_descriptor_static: E::enum_descriptor_static,
            })
            .collect();
        EnumDescriptor {
            proto: proto.en,
            values,
            type_id: TypeId::of::<E>(),
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

    /// Find enum value by name
    #[deprecated]
    pub fn value_by_name<'a>(&'a self, name: &str) -> &'a EnumValueDescriptor {
        self.get_value_by_name(name).unwrap()
    }

    /// Find enum value by number
    #[deprecated]
    pub fn value_by_number<'a>(&'a self, number: i32) -> &'a EnumValueDescriptor {
        self.get_value_by_number(number).unwrap()
    }

    /// Check if this enum descriptor corresponds given enum type
    ///
    /// ```
    /// # use protobuf::ProtobufEnum;
    /// # use protobuf::descriptor::FieldDescriptorProto_Label;
    /// # use protobuf::reflect::EnumDescriptor;
    ///
    /// let descriptor: &EnumDescriptor = FieldDescriptorProto_Label::enum_descriptor_static();
    ///
    /// assert!(descriptor.is::<FieldDescriptorProto_Label>())
    /// ```
    pub fn is<E: ProtobufEnum>(&self) -> bool {
        TypeId::of::<E>() == self.type_id
    }

    #[cfg(rustc_nightly)]
    pub(crate) fn cast_to_protobuf_enum<E: ProtobufValue>(&self, value: i32) -> Option<E> {
        if TypeId::of::<E>() != self.type_id {
            return None;
        }

        Some(<E as cast_impl::CastValueToProtobufEnum>::cast(value))
    }

    #[cfg(not(rustc_nightly))]
    pub(crate) fn cast_to_protobuf_enum<E: ProtobufValue>(&self, value: i32) -> Option<E> {
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
}

#[cfg(rustc_nightly)]
mod cast_impl {
    use super::*;

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

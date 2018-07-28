//! Reflection implementation for protobuf types.

mod accessor;
mod map;
mod repeated;
mod value;
mod enums;
mod message;
mod field;
pub(crate) mod as_any;
pub(crate) mod transmute_eq;
pub mod types;
pub mod type_dynamic;
pub(crate) mod runtime_types;
pub(crate) mod runtime_type_box;
pub(crate) mod runtime_type_dynamic;

mod reflect_deep_eq;

pub mod rt;

pub use self::value::ProtobufValue;
pub use self::value::ReflectValueRef;
pub use self::value::ReflectValueBox;

pub use self::repeated::ReflectRepeatedRef;
pub use self::repeated::ReflectRepeatedMut;

pub use self::map::ReflectMapRef;
pub use self::map::ReflectMapMut;

pub use self::enums::EnumDescriptor;
pub use self::enums::EnumValueDescriptor;

pub use self::message::MessageDescriptor;

pub use self::field::FieldDescriptor;
pub use self::field::ReflectFieldRef;
pub use self::field::RuntimeFieldType;

pub use self::runtime_type_dynamic::RuntimeTypeDynamic;
pub use self::runtime_type_box::RuntimeTypeBox;

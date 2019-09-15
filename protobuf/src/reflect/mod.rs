//! Reflection implementation for protobuf types.

mod accessor;
mod enums;
mod field;
mod find_message_or_enum;
mod map;
mod message;
mod repeated;
mod runtime_type_box;
mod runtime_type_dynamic;
pub(crate) mod runtime_types;
mod transmute_eq;
mod type_dynamic;
pub mod types;
mod value;

pub(crate) mod reflect_eq;

pub mod rt;

pub use self::value::ProtobufValue;
pub use self::value::ReflectValueBox;
pub use self::value::ReflectValueRef;

pub use self::repeated::ReflectRepeatedMut;
pub use self::repeated::ReflectRepeatedRef;

pub use self::map::ReflectMapMut;
pub use self::map::ReflectMapRef;

pub use self::enums::EnumDescriptor;
pub use self::enums::EnumValueDescriptor;

pub use self::message::MessageDescriptor;

pub use self::field::FieldDescriptor;
pub use self::field::ReflectFieldRef;
pub use self::field::RuntimeFieldType;

pub use self::runtime_type_box::RuntimeTypeBox;
pub use self::runtime_type_dynamic::RuntimeTypeDynamic;

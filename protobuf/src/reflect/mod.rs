//! Reflection implementation for protobuf types.

mod acc;
mod dynamic;
mod enums;
mod field;
mod file;
mod find_message_or_enum;
mod map;
pub(crate) mod message;
mod oneof;
mod repeated;
mod runtime_type_box;
mod type_dynamic;
pub(crate) mod value;

pub mod runtime_types;
pub mod types;

pub(crate) mod reflect_eq;

pub mod rt;

pub(crate) mod name;

pub use self::value::value_box::ReflectValueBox;
pub use self::value::value_ref::ReflectValueRef;
pub use self::value::ProtobufValue;

pub use self::repeated::ReflectRepeatedMut;
pub use self::repeated::ReflectRepeatedRef;

pub use self::map::ReflectMapMut;
pub use self::map::ReflectMapRef;

#[doc(hidden)]
pub use self::enums::generated::GeneratedEnumDescriptorData;
pub use self::enums::EnumDescriptor;
pub use self::enums::EnumValueDescriptor;

#[doc(hidden)]
pub use self::message::generated::GeneratedMessageDescriptorData;
pub use self::message::message_ref::MessageRef;
pub use self::message::MessageDescriptor;

pub use self::field::FieldDescriptor;
pub use self::field::ReflectFieldRef;
pub use self::field::RuntimeFieldType;

pub use self::oneof::OneofDescriptor;

#[doc(hidden)]
pub use self::file::generated::GeneratedFileDescriptor;
pub use self::file::FileDescriptor;

pub use self::runtime_type_box::RuntimeTypeBox;

pub use self::reflect_eq::ReflectEq;
pub use self::reflect_eq::ReflectEqMode;

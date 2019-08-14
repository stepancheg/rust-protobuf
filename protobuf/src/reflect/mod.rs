//! Reflection implementation for protobuf types.

use core::Message;

pub mod accessor;
mod enums;
mod field;
mod map;
mod message;
mod optional;
mod repeated;
mod value;

pub use self::value::ProtobufValue;
pub use self::value::ReflectValueRef;
#[doc(hidden)]
#[deprecated] // deprecated alias
pub use self::value::ReflectValueRef as ProtobufValueRef;

pub use self::enums::EnumDescriptor;
pub use self::enums::EnumValueDescriptor;

pub use self::message::MessageDescriptor;

pub use self::field::FieldDescriptor;
pub use self::field::ReflectFieldRef;

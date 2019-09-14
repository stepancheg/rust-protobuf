//! Reflection implementation for protobuf types.

use crate::core::Message;

mod accessor;
mod enums;
mod field;
mod map;
mod message;
mod optional;
mod repeated;
mod runtime_type_box;
mod runtime_type_dynamic;
pub(crate) mod runtime_types;
mod transmute_eq;
mod type_dynamic;
pub mod types;
mod value;

pub mod rt;

pub use self::value::ProtobufValue;
pub use self::value::ReflectValueBox;
pub use self::value::ReflectValueRef;
#[doc(hidden)]
pub use self::value::ReflectValueRef as ProtobufValueRef;

pub use self::enums::EnumDescriptor;
pub use self::enums::EnumValueDescriptor;

pub use self::message::MessageDescriptor;

pub use self::field::FieldDescriptor;
pub use self::field::ReflectFieldRef;

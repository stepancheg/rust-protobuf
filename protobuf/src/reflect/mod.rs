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

use self::map::ReflectMap;
use self::repeated::ReflectRepeated;

pub use self::value::ProtobufValue;
pub use self::value::ReflectValueRef;
#[doc(hidden)]
#[deprecated] // deprecated alias
pub use self::value::ReflectValueRef as ProtobufValueRef;

pub use self::enums::EnumDescriptor;
pub use self::enums::EnumValueDescriptor;

pub use self::message::MessageDescriptor;

pub use self::field::FieldDescriptor;

/// Dynamic field reference
pub enum ReflectFieldRef<'a> {
    /// Repeated field
    Repeated(&'a ReflectRepeated),
    /// Map field
    Map(&'a ReflectMap),
    /// Optional field
    Optional(Option<ReflectValueRef<'a>>),
}

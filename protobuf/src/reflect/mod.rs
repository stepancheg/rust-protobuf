//! Reflection implementation for protobuf types.

use std::collections::HashMap;
use std::default::Default;
use std::marker;

use core::Message;
use descriptor::DescriptorProto;
use descriptor::FileDescriptorProto;
use descriptorx::find_message_by_rust_name;
use reflect::accessor::FieldAccessor;

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

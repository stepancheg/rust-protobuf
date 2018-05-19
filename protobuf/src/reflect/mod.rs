//! Reflection implementation for protobuf types.

mod accessor;
mod map;
mod repeated;
mod value;
mod optional;
mod enums;
mod message;
pub(crate) mod as_any;
pub mod types;
pub(crate) mod runtime_types;
pub(crate) mod runtime_type_dynamic;

pub mod rt;

pub use self::value::ProtobufValue;
pub use self::value::ReflectValueRef;
pub use self::value::ReflectValueBox;

pub use self::enums::EnumDescriptor;
pub use self::enums::EnumValueDescriptor;

pub use self::message::MessageDescriptor;
pub use self::message::FieldDescriptor;
pub use self::message::ReflectFieldRef;

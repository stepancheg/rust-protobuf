use crate::reflect::EnumDescriptor;
use crate::reflect::MessageDescriptor;

pub enum RuntimeTypeBox {
    I32,
    I64,
    U32,
    U64,
    F32,
    F64,
    Bool,
    String,
    Chars,
    VecU8,
    CarllercheBytes,
    Enum(&'static EnumDescriptor),
    Message(&'static MessageDescriptor),
}

use ::core::*;
use super::*;

pub trait ProtobufValue : 'static {
    fn as_ref(&self) -> ProtobufValueRef {
        panic!("descriptor_static is not implemented for message, \
            LITE_RUNTIME must be used");
    }

    fn is_non_zero(&self) -> bool {
        self.as_ref().is_non_zero()
    }
}

impl ProtobufValue for u32 {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::U32(*self)
    }
}

impl ProtobufValue for u64 {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::U64(*self)
    }
}

impl ProtobufValue for i32 {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::I32(*self)
    }
}

impl ProtobufValue for i64 {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::I64(*self)
    }
}

impl ProtobufValue for f32 {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::F32(*self)
    }
}

impl ProtobufValue for f64 {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::F64(*self)
    }
}

impl ProtobufValue for bool {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::Bool(*self)
    }
}

impl ProtobufValue for String {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::String(*&self)
    }
}

impl ProtobufValue for Vec<u8> {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::Bytes(*&self)
    }
}

// conflicting implementations, so generated code is used instead
/*
impl<E : ProtobufEnum> ProtobufValue for E {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::Enum(self.descriptor())
    }
}

impl<M : Message> ProtobufValue for M {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::Message(self)
    }
}
*/


pub enum ProtobufValueRef<'a> {
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Bool(bool),
    String(&'a str),
    Bytes(&'a [u8]),
    Enum(&'static EnumValueDescriptor),
    Message(&'a Message),
}

impl<'a> ProtobufValueRef<'a> {
    pub fn is_non_zero(&self) -> bool {
        match *self {
            ProtobufValueRef::U32(v) => v != 0,
            ProtobufValueRef::U64(v) => v != 0,
            ProtobufValueRef::I32(v) => v != 0,
            ProtobufValueRef::I64(v) => v != 0,
            ProtobufValueRef::F32(v) => v != 0.,
            ProtobufValueRef::F64(v) => v != 0.,
            ProtobufValueRef::Bool(v) => v,
            ProtobufValueRef::String(v) => !v.is_empty(),
            ProtobufValueRef::Bytes(v) => !v.is_empty(),
            ProtobufValueRef::Enum(v) => v.value() != 0,
            ProtobufValueRef::Message(_) => true,
        }
    }
}

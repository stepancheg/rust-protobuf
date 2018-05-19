use std::any::Any;


#[cfg(feature = "bytes")]
use bytes::Bytes;
#[cfg(feature = "bytes")]
use chars::Chars;

use core::*;
use super::*;
use super::as_any::AsAny;


// Hack against lack of upcasting
pub trait AsProtobufValue {
    fn as_protobuf_value(&self) -> &ProtobufValue;
}

impl<T : ProtobufValue> AsProtobufValue for T {
    fn as_protobuf_value(&self) -> &ProtobufValue {
        self
    }
}


pub trait ProtobufValue : Any + 'static + AsAny + AsProtobufValue {
    fn as_ref(&self) -> ProtobufValueRef {
        unimplemented!()
    }

    fn is_non_zero(&self) -> bool {
        self.as_ref().is_non_zero()
    }

    fn as_ref_copy(&self) -> ProtobufValueRef<'static>
//where Self : Copy // TODO
    {
        match self.as_ref() {
            ProtobufValueRef::Bool(v) => ProtobufValueRef::Bool(v),
            ProtobufValueRef::U32(v) => ProtobufValueRef::U32(v),
            ProtobufValueRef::U64(v) => ProtobufValueRef::U64(v),
            ProtobufValueRef::I32(v) => ProtobufValueRef::I32(v),
            ProtobufValueRef::I64(v) => ProtobufValueRef::I64(v),
            ProtobufValueRef::F32(v) => ProtobufValueRef::F32(v),
            ProtobufValueRef::F64(v) => ProtobufValueRef::F64(v),
            ProtobufValueRef::Enum(v) => ProtobufValueRef::Enum(v),
            ProtobufValueRef::String(..) |
            ProtobufValueRef::Bytes(..) |
            ProtobufValueRef::Message(..) => unreachable!(),
        }
    }

    fn from_value_box(_value: ProtobufValueBox) -> Self where Self : Sized {
        unimplemented!()
    }
}

impl ProtobufValue for u32 {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::U32(*self)
    }

    fn from_value_box(value: ProtobufValueBox) -> Self where Self : Sized {
        if let ProtobufValueBox::U32(v) = value {
            v
        } else {
            panic!("wrong value: {:?}", value);
        }
    }
}

impl ProtobufValue for u64 {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::U64(*self)
    }

    fn from_value_box(value: ProtobufValueBox) -> Self where Self : Sized {
        if let ProtobufValueBox::U64(v) = value {
            v
        } else {
            panic!("wrong value: {:?}", value);
        }
    }
}

impl ProtobufValue for i32 {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::I32(*self)
    }

    fn from_value_box(value: ProtobufValueBox) -> Self where Self : Sized {
        if let ProtobufValueBox::I32(v) = value {
            v
        } else {
            panic!("wrong value: {:?}", value);
        }
    }
}

impl ProtobufValue for i64 {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::I64(*self)
    }

    fn from_value_box(value: ProtobufValueBox) -> Self where Self : Sized {
        if let ProtobufValueBox::I64(v) = value {
            v
        } else {
            panic!("wrong value: {:?}", value);
        }
    }
}

impl ProtobufValue for f32 {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::F32(*self)
    }

    fn from_value_box(value: ProtobufValueBox) -> Self where Self : Sized {
        if let ProtobufValueBox::F32(v) = value {
            v
        } else {
            panic!("wrong value: {:?}", value);
        }
    }
}

impl ProtobufValue for f64 {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::F64(*self)
    }

    fn from_value_box(value: ProtobufValueBox) -> Self where Self : Sized {
        if let ProtobufValueBox::F64(v) = value {
            v
        } else {
            panic!("wrong value: {:?}", value);
        }
    }
}

impl ProtobufValue for bool {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::Bool(*self)
    }

    fn from_value_box(value: ProtobufValueBox) -> Self where Self : Sized {
        if let ProtobufValueBox::Bool(v) = value {
            v
        } else {
            panic!("wrong value: {:?}", value);
        }
    }
}

impl ProtobufValue for String {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::String(*&self)
    }

    fn from_value_box(value: ProtobufValueBox) -> Self where Self : Sized {
        if let ProtobufValueBox::String(v) = value {
            v
        } else {
            panic!("wrong value: {:?}", value);
        }
    }
}

impl ProtobufValue for Vec<u8> {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::Bytes(*&self)
    }

    fn from_value_box(value: ProtobufValueBox) -> Self where Self : Sized {
        if let ProtobufValueBox::Bytes(v) = value {
            v
        } else {
            panic!("wrong value: {:?}", value);
        }
    }
}

#[cfg(feature = "bytes")]
impl ProtobufValue for Bytes {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::Bytes(&*self)
    }

    fn from_value_box(value: ProtobufValueBox) -> Self where Self : Sized {
        if let ProtobufValueBox::Bytes(v) = value {
            v.into()
        } else {
            panic!("wrong value: {:?}", value);
        }
    }
}

#[cfg(feature = "bytes")]
impl ProtobufValue for Chars {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::String(&*self)
    }

    fn from_value_box(value: ProtobufValueBox) -> Self where Self : Sized {
        if let ProtobufValueBox::String(v) = value {
            v.into()
        } else {
            panic!("wrong value: {:?}", value);
        }
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

#[derive(Debug)]
pub enum ProtobufValueBox {
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
    Enum(&'static EnumValueDescriptor),
    Message(Box<Message>),
}

impl ProtobufValueBox {
    pub fn as_value(&self) -> &ProtobufValue {
        match self {
            ProtobufValueBox::U32(v) => v,
            ProtobufValueBox::U64(v) => v,
            ProtobufValueBox::I32(v) => v,
            ProtobufValueBox::I64(v) => v,
            ProtobufValueBox::F32(v) => v,
            ProtobufValueBox::F64(v) => v,
            ProtobufValueBox::Bool(v) => v,
            ProtobufValueBox::String(v) => v,
            ProtobufValueBox::Bytes(v) => v,
            ProtobufValueBox::Enum(_) => unimplemented!(),
            ProtobufValueBox::Message(v) => v.as_protobuf_value(),
        }
    }
}


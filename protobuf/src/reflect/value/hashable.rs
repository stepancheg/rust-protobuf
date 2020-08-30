use crate::reflect::EnumDescriptor;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeTypeBox;
use std::borrow::Borrow;

/// Subset of [`ReflectValueBox`], only hashable types.
#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum ReflectValueBoxHashable {
    /// `u32`
    U32(u32),
    /// `u64`
    U64(u64),
    /// `i32`
    I32(i32),
    /// `i64`
    I64(i64),
    /// `f32`
    Bool(bool),
    /// `string`
    String(String),
    /// `bytes`
    Bytes(Vec<u8>),
    /// `enum`
    Enum(EnumDescriptor, i32),
}

impl ReflectValueBoxHashable {
    /// Try convert.
    pub fn try_from(v: ReflectValueBox) -> Result<ReflectValueBoxHashable, ReflectValueBox> {
        Ok(match v {
            ReflectValueBox::U32(v) => ReflectValueBoxHashable::U32(v),
            ReflectValueBox::U64(v) => ReflectValueBoxHashable::U64(v),
            ReflectValueBox::I32(v) => ReflectValueBoxHashable::I32(v),
            ReflectValueBox::I64(v) => ReflectValueBoxHashable::I64(v),
            ReflectValueBox::Bool(v) => ReflectValueBoxHashable::Bool(v),
            ReflectValueBox::String(v) => ReflectValueBoxHashable::String(v),
            ReflectValueBox::Bytes(v) => ReflectValueBoxHashable::Bytes(v),
            ReflectValueBox::Enum(e, v) => ReflectValueBoxHashable::Enum(e, v),
            v => return Err(v),
        })
    }

    /// Convert or panic if not hashable.
    pub fn from_box(v: ReflectValueBox) -> ReflectValueBoxHashable {
        match Self::try_from(v) {
            Ok(v) => v,
            Err(v) => panic!("value is not hashable: {:?}", v),
        }
    }

    /// Convert.
    pub fn as_value_ref(&self) -> ReflectValueRef {
        match self {
            ReflectValueBoxHashable::I32(v) => ReflectValueRef::I32(*v),
            ReflectValueBoxHashable::U32(v) => ReflectValueRef::U32(*v),
            ReflectValueBoxHashable::U64(v) => ReflectValueRef::U64(*v),
            ReflectValueBoxHashable::I64(v) => ReflectValueRef::I64(*v),
            ReflectValueBoxHashable::Bool(v) => ReflectValueRef::Bool(*v),
            ReflectValueBoxHashable::String(v) => ReflectValueRef::String(v),
            ReflectValueBoxHashable::Bytes(v) => ReflectValueRef::Bytes(v),
            ReflectValueBoxHashable::Enum(e, v) => ReflectValueRef::Enum(e.clone(), *v),
        }
    }

    /// Convert.
    pub fn into_value_box(self) -> ReflectValueBox {
        match self {
            ReflectValueBoxHashable::I32(v) => ReflectValueBox::I32(v),
            ReflectValueBoxHashable::U32(v) => ReflectValueBox::U32(v),
            ReflectValueBoxHashable::U64(v) => ReflectValueBox::U64(v),
            ReflectValueBoxHashable::I64(v) => ReflectValueBox::I64(v),
            ReflectValueBoxHashable::Bool(v) => ReflectValueBox::Bool(v),
            ReflectValueBoxHashable::String(v) => ReflectValueBox::String(v),
            ReflectValueBoxHashable::Bytes(v) => ReflectValueBox::Bytes(v),
            ReflectValueBoxHashable::Enum(e, v) => ReflectValueBox::Enum(e.clone(), v),
        }
    }

    /// Runtime type of this object.
    pub fn get_type(&self) -> RuntimeTypeBox {
        self.as_value_ref().get_type()
    }

    /// Try downcast.
    pub fn downcast<T: ProtobufValue>(self) -> Result<T, Self> {
        self.into_value_box()
            .downcast()
            .map_err(ReflectValueBoxHashable::from_box)
    }
}

// TODO: implement hash consistently
impl Borrow<str> for ReflectValueBoxHashable {
    fn borrow(&self) -> &str {
        match self {
            ReflectValueBoxHashable::String(s) => s.as_str(),
            _ => panic!("not a str"),
        }
    }
}

impl Borrow<[u8]> for ReflectValueBoxHashable {
    fn borrow(&self) -> &[u8] {
        match self {
            ReflectValueBoxHashable::Bytes(s) => s.as_slice(),
            _ => panic!("not a [u8]"),
        }
    }
}

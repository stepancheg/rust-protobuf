use std::any::Any;
use std::hash::Hash;
use std::hash::Hasher;
use std::mem;

#[cfg(feature = "bytes")]
use crate::bytes::Bytes;
#[cfg(feature = "bytes")]
use crate::chars::Chars;

use super::*;
use crate::core::*;
use crate::reflect::reflect_eq::{ReflectEq, ReflectEqMode};
use crate::reflect::transmute_eq::transmute_eq;

/// Type implemented by all protobuf singular types
/// (primitives, string, messages, enums).
///
/// Used for dynamic casting in reflection.
pub trait ProtobufValue: Any + 'static + Send + Sync {}

impl ProtobufValue for u32 {}

impl ProtobufValue for u64 {}

impl ProtobufValue for i32 {}

impl ProtobufValue for i64 {}

impl ProtobufValue for f32 {}

impl ProtobufValue for f64 {}

impl ProtobufValue for bool {}

impl ProtobufValue for String {}

impl ProtobufValue for Vec<u8> {}

#[cfg(feature = "bytes")]
impl ProtobufValue for Bytes {}

#[cfg(feature = "bytes")]
impl ProtobufValue for Chars {}

// conflicting implementations, so generated code is used instead
/*
impl<E : ProtobufEnum> ProtobufValue for E {
}

impl<M : Message> ProtobufValue for M {
}
*/

/// A reference to a value
#[derive(Debug, Clone)]
pub enum ReflectValueRef<'a> {
    /// `u32`
    U32(u32),
    /// `u64`
    U64(u64),
    /// `i32`
    I32(i32),
    /// `i64`
    I64(i64),
    /// `f32`
    F32(f32),
    /// `f64`
    F64(f64),
    /// `bool`
    Bool(bool),
    /// `string`
    String(&'a str),
    /// `bytes`
    Bytes(&'a [u8]),
    /// `enum`
    Enum(&'static EnumDescriptor, i32),
    /// `message`
    Message(&'a dyn Message),
}

impl<'a> ReflectValueRef<'a> {
    /// Value is "non-zero"?
    fn _is_non_zero(&self) -> bool {
        match *self {
            ReflectValueRef::U32(v) => v != 0,
            ReflectValueRef::U64(v) => v != 0,
            ReflectValueRef::I32(v) => v != 0,
            ReflectValueRef::I64(v) => v != 0,
            ReflectValueRef::F32(v) => v != 0.,
            ReflectValueRef::F64(v) => v != 0.,
            ReflectValueRef::Bool(v) => v,
            ReflectValueRef::String(v) => !v.is_empty(),
            ReflectValueRef::Bytes(v) => !v.is_empty(),
            ReflectValueRef::Enum(_d, v) => v != 0,
            ReflectValueRef::Message(_) => true,
        }
    }

    /// Take `i32` value.
    pub fn to_i32(&self) -> Option<i32> {
        match *self {
            ReflectValueRef::I32(v) => Some(v),
            _ => None,
        }
    }

    /// Take `i64` value.
    pub fn to_i64(&self) -> Option<i64> {
        match *self {
            ReflectValueRef::I64(v) => Some(v),
            _ => None,
        }
    }

    /// Take `u32` value.
    pub fn to_u32(&self) -> Option<u32> {
        match *self {
            ReflectValueRef::U32(v) => Some(v),
            _ => None,
        }
    }

    /// Take `u64` value.
    pub fn to_u64(&self) -> Option<u64> {
        match *self {
            ReflectValueRef::U64(v) => Some(v),
            _ => None,
        }
    }

    /// Take `f32` value.
    pub fn to_f32(&self) -> Option<f32> {
        match *self {
            ReflectValueRef::F32(v) => Some(v),
            _ => None,
        }
    }

    /// Take `f64` value.
    pub fn to_f64(&self) -> Option<f64> {
        match *self {
            ReflectValueRef::F64(v) => Some(v),
            _ => None,
        }
    }

    /// Take `bool` value.
    pub fn to_bool(&self) -> Option<bool> {
        match *self {
            ReflectValueRef::Bool(v) => Some(v),
            _ => None,
        }
    }

    /// Take `str` value.
    pub fn to_str(&self) -> Option<&str> {
        match *self {
            ReflectValueRef::String(v) => Some(v),
            _ => None,
        }
    }

    /// Take `[u8]` value.
    pub fn to_bytes(&self) -> Option<&[u8]> {
        match *self {
            ReflectValueRef::Bytes(v) => Some(v),
            _ => None,
        }
    }

    /// Take message value.
    pub fn to_message(&self) -> Option<&'a dyn Message> {
        match *self {
            ReflectValueRef::Message(m) => Some(m),
            _ => None,
        }
    }

    /// Clone to a box
    pub fn to_box(&self) -> ReflectValueBox {
        match *self {
            ReflectValueRef::U32(v) => ReflectValueBox::U32(v),
            ReflectValueRef::U64(v) => ReflectValueBox::U64(v),
            ReflectValueRef::I32(v) => ReflectValueBox::I32(v),
            ReflectValueRef::I64(v) => ReflectValueBox::I64(v),
            ReflectValueRef::F32(v) => ReflectValueBox::F32(v),
            ReflectValueRef::F64(v) => ReflectValueBox::F64(v),
            ReflectValueRef::Bool(v) => ReflectValueBox::Bool(v),
            ReflectValueRef::String(v) => ReflectValueBox::String(v.to_owned()),
            ReflectValueRef::Bytes(v) => ReflectValueBox::Bytes(v.to_owned()),
            ReflectValueRef::Enum(d, v) => ReflectValueBox::Enum(d, v),
            ReflectValueRef::Message(v) => ReflectValueBox::Message(v.clone_box()),
        }
    }

    /// Convert a value to arbitrary value.
    pub fn downcast_clone<V: ProtobufValue>(&self) -> Result<V, Self> {
        self.to_box().downcast().map_err(|_| self.clone())
    }
}

impl<'a> ReflectEq for ReflectValueRef<'a> {
    fn reflect_eq(&self, that: &Self, mode: &ReflectEqMode) -> bool {
        use super::ReflectValueRef::*;
        match (self, that) {
            (U32(a), U32(b)) => a == b,
            (U64(a), U64(b)) => a == b,
            (I32(a), I32(b)) => a == b,
            (I64(a), I64(b)) => a == b,
            (F32(a), F32(b)) => {
                if a.is_nan() || b.is_nan() {
                    a.is_nan() == b.is_nan() && mode.nan_equal
                } else {
                    a == b
                }
            }
            (F64(a), F64(b)) => {
                if a.is_nan() || b.is_nan() {
                    a.is_nan() == b.is_nan() && mode.nan_equal
                } else {
                    a == b
                }
            }
            (Bool(a), Bool(b)) => a == b,
            (String(a), String(b)) => a == b,
            (Bytes(a), Bytes(b)) => a == b,
            (Enum(ad, a), Enum(bd, b)) => ad == bd && a == b,
            (Message(a), Message(b)) => {
                let ad = a.descriptor();
                let bd = b.descriptor();
                ad == bd && ad.reflect_eq(*a, *b, mode)
            }
            _ => unreachable!(),
        }
    }
}

pub enum ReflectValueMut<'a> {
    Message(&'a mut dyn Message),
}

/// Owner value of any elementary type
#[derive(Debug, Clone)]
pub enum ReflectValueBox {
    /// `u32`
    U32(u32),
    /// `u64`
    U64(u64),
    /// `i32`
    I32(i32),
    /// `i64`
    I64(i64),
    /// `f32`
    F32(f32),
    /// `f64`
    F64(f64),
    /// `bool`
    Bool(bool),
    /// `string`
    String(String),
    /// `bytes`
    Bytes(Vec<u8>),
    /// `enum`
    Enum(&'static EnumDescriptor, i32),
    /// `message`
    Message(Box<dyn Message>),
}

impl From<u32> for ReflectValueBox {
    fn from(v: u32) -> Self {
        ReflectValueBox::U32(v)
    }
}

impl From<u64> for ReflectValueBox {
    fn from(v: u64) -> Self {
        ReflectValueBox::U64(v)
    }
}

impl From<i32> for ReflectValueBox {
    fn from(v: i32) -> Self {
        ReflectValueBox::I32(v)
    }
}

impl From<i64> for ReflectValueBox {
    fn from(v: i64) -> Self {
        ReflectValueBox::I64(v)
    }
}

impl From<f32> for ReflectValueBox {
    fn from(v: f32) -> Self {
        ReflectValueBox::F32(v)
    }
}

impl From<f64> for ReflectValueBox {
    fn from(v: f64) -> Self {
        ReflectValueBox::F64(v)
    }
}

impl From<bool> for ReflectValueBox {
    fn from(v: bool) -> Self {
        ReflectValueBox::Bool(v)
    }
}

impl From<String> for ReflectValueBox {
    fn from(v: String) -> Self {
        ReflectValueBox::String(v)
    }
}

impl From<Vec<u8>> for ReflectValueBox {
    fn from(v: Vec<u8>) -> Self {
        ReflectValueBox::Bytes(v)
    }
}

impl From<&'static EnumValueDescriptor> for ReflectValueBox {
    fn from(v: &'static EnumValueDescriptor) -> Self {
        ReflectValueBox::Enum(v.enum_descriptor(), v.value())
    }
}

impl From<Box<dyn Message>> for ReflectValueBox {
    fn from(v: Box<dyn Message>) -> Self {
        ReflectValueBox::Message(v)
    }
}

fn _assert_value_box_send_sync() {
    fn _assert_send_sync<T: Send + Sync>() {}
    _assert_send_sync::<ReflectValueBox>();
}

#[cfg(not(feature = "bytes"))]
type VecU8OrBytes = Vec<u8>;
#[cfg(feature = "bytes")]
type VecU8OrBytes = Vec<u8>;
#[cfg(not(feature = "bytes"))]
type StringOrChars = String;
#[cfg(feature = "bytes")]
type StringOrChars = Chars;

impl ReflectValueBox {
    /// As ref
    pub fn as_value_ref(&self) -> ReflectValueRef {
        use std::ops::Deref;
        match *self {
            ReflectValueBox::U32(v) => ReflectValueRef::U32(v),
            ReflectValueBox::U64(v) => ReflectValueRef::U64(v),
            ReflectValueBox::I32(v) => ReflectValueRef::I32(v),
            ReflectValueBox::I64(v) => ReflectValueRef::I64(v),
            ReflectValueBox::F32(v) => ReflectValueRef::F32(v),
            ReflectValueBox::F64(v) => ReflectValueRef::F64(v),
            ReflectValueBox::Bool(v) => ReflectValueRef::Bool(v),
            ReflectValueBox::String(ref v) => ReflectValueRef::String(v.as_str()),
            ReflectValueBox::Bytes(ref v) => ReflectValueRef::Bytes(v.as_slice()),
            ReflectValueBox::Enum(d, v) => ReflectValueRef::Enum(d, v),
            ReflectValueBox::Message(ref v) => ReflectValueRef::Message(v.deref()),
        }
    }

    /// Downcast to real typed value.
    ///
    /// For `enum` `V` can be either `V: ProtobufEnum` or `V: ProtobufEnumOrUnknown<E>`.
    pub fn downcast<V: ProtobufValue>(self) -> Result<V, Self> {
        match self {
            ReflectValueBox::U32(v) => transmute_eq(v).map_err(ReflectValueBox::U32),
            ReflectValueBox::U64(v) => transmute_eq(v).map_err(ReflectValueBox::U64),
            ReflectValueBox::I32(v) => transmute_eq(v).map_err(ReflectValueBox::I32),
            ReflectValueBox::I64(v) => transmute_eq(v).map_err(ReflectValueBox::I64),
            ReflectValueBox::F32(v) => transmute_eq(v).map_err(ReflectValueBox::F32),
            ReflectValueBox::F64(v) => transmute_eq(v).map_err(ReflectValueBox::F64),
            ReflectValueBox::Bool(v) => transmute_eq(v).map_err(ReflectValueBox::Bool),
            ReflectValueBox::String(v) => transmute_eq::<String, _>(v)
                .or_else(|v: String| transmute_eq::<StringOrChars, _>(v.into()))
                .map_err(|v: StringOrChars| ReflectValueBox::String(v.into())),
            ReflectValueBox::Bytes(v) => transmute_eq::<Vec<u8>, _>(v)
                .or_else(|v: Vec<u8>| transmute_eq::<VecU8OrBytes, _>(v.into()))
                .map_err(|v: VecU8OrBytes| ReflectValueBox::Bytes(v.into())),
            ReflectValueBox::Enum(d, e) => d.cast(e).ok_or(ReflectValueBox::Enum(d, e)),
            ReflectValueBox::Message(m) => m
                .downcast_box::<V>()
                .map(|m| *m)
                .map_err(ReflectValueBox::Message),
        }
    }
}

impl<'a> PartialEq for ReflectValueRef<'a> {
    fn eq(&self, other: &ReflectValueRef) -> bool {
        use self::ReflectValueRef::*;
        match (self, other) {
            (U32(a), U32(b)) => a == b,
            (U64(a), U64(b)) => a == b,
            (I32(a), I32(b)) => a == b,
            (I64(a), I64(b)) => a == b,
            // should probably NaN == NaN here
            (F32(a), F32(b)) => a == b,
            (F64(a), F64(b)) => a == b,
            (Bool(a), Bool(b)) => a == b,
            (String(a), String(b)) => a == b,
            (Bytes(a), Bytes(b)) => a == b,
            (Enum(da, a), Enum(db, b)) => da == db && a == b,
            (Message(a), Message(b)) => {
                use std::ops::Deref;
                a.descriptor() == b.descriptor() && a.descriptor().eq(a.deref(), b.deref())
            }
            _ => false,
        }
    }
}

impl<'a> PartialEq for ReflectValueBox {
    fn eq(&self, other: &Self) -> bool {
        self.as_value_ref() == other.as_value_ref()
    }
}

impl<'a> PartialEq<ReflectValueRef<'a>> for ReflectValueBox {
    fn eq(&self, other: &ReflectValueRef) -> bool {
        self.as_value_ref() == *other
    }
}

impl<'a> PartialEq<ReflectValueBox> for ReflectValueRef<'a> {
    fn eq(&self, other: &ReflectValueBox) -> bool {
        *self == other.as_value_ref()
    }
}

// Panics if contained type is not hashable
impl<'a> Hash for ReflectValueRef<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use self::ReflectValueRef::*;
        Hash::hash(&mem::discriminant(self), state);
        match self {
            U32(v) => Hash::hash(&v, state),
            U64(v) => Hash::hash(&v, state),
            I32(v) => Hash::hash(&v, state),
            I64(v) => Hash::hash(&v, state),
            Bool(v) => Hash::hash(&v, state),
            String(v) => Hash::hash(&v, state),
            Bytes(v) => Hash::hash(&v, state),
            Enum(_d, v) => Hash::hash(v, state),
            F32(_) | F64(_) | Message(_) => panic!("not hashable: {:?}", self),
        }
    }
}

// Panics if contained type is not hashable
impl Hash for ReflectValueBox {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.as_value_ref(), state)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reflect_value_box_downcast_primitive() {
        assert_eq!(Ok(10), ReflectValueBox::U32(10).downcast::<u32>());
        assert_eq!(
            Err(ReflectValueBox::I32(10)),
            ReflectValueBox::I32(10).downcast::<u32>()
        );
    }

    #[test]
    fn reflect_value_box_downcast_string() {
        assert_eq!(
            Ok("aa".to_owned()),
            ReflectValueBox::String("aa".to_owned()).downcast::<String>()
        );
        assert_eq!(
            Err(ReflectValueBox::String("aa".to_owned())),
            ReflectValueBox::String("aa".to_owned()).downcast::<u32>()
        );
        assert_eq!(
            Err(ReflectValueBox::Bool(false)),
            ReflectValueBox::Bool(false).downcast::<String>()
        );
    }

    #[test]
    fn reflect_value_box_downcast_chars() {
        assert_eq!(
            Ok(StringOrChars::from("aa".to_owned())),
            ReflectValueBox::String("aa".to_owned()).downcast::<StringOrChars>()
        );
        assert_eq!(
            Err(ReflectValueBox::String("aa".to_owned())),
            ReflectValueBox::String("aa".to_owned()).downcast::<u32>()
        );
        assert_eq!(
            Err(ReflectValueBox::Bool(false)),
            ReflectValueBox::Bool(false).downcast::<StringOrChars>()
        );
    }
}

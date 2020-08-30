use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeTypeBox;
use std::hash::Hash;
use std::hash::Hasher;

/// Subset of [`ReflectValueRef`], only hashable types.
#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum ReflectValueRefHashable<'a> {
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
    String(&'a str),
}

/// Subset of [`ReflectValueBox`], only hashable types.
#[derive(Eq, Debug, Clone)]
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
}

impl PartialEq for ReflectValueBoxHashable {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Hash for ReflectValueBoxHashable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.as_ref(), state)
    }
}

impl<'a> ReflectValueRefHashable<'a> {
    /// Try convert.
    pub fn try_from_value_ref<'b>(
        v: ReflectValueRef<'b>,
    ) -> Result<ReflectValueRefHashable<'b>, ReflectValueRef<'b>> {
        Ok(match v {
            ReflectValueRef::U32(v) => ReflectValueRefHashable::U32(v),
            ReflectValueRef::U64(v) => ReflectValueRefHashable::U64(v),
            ReflectValueRef::I32(v) => ReflectValueRefHashable::I32(v),
            ReflectValueRef::I64(v) => ReflectValueRefHashable::I64(v),
            ReflectValueRef::Bool(v) => ReflectValueRefHashable::Bool(v),
            ReflectValueRef::String(v) => ReflectValueRefHashable::String(v),
            v => return Err(v),
        })
    }
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
    pub fn as_ref(&self) -> ReflectValueRefHashable {
        match self {
            ReflectValueBoxHashable::I32(v) => ReflectValueRefHashable::I32(*v),
            ReflectValueBoxHashable::U32(v) => ReflectValueRefHashable::U32(*v),
            ReflectValueBoxHashable::U64(v) => ReflectValueRefHashable::U64(*v),
            ReflectValueBoxHashable::I64(v) => ReflectValueRefHashable::I64(*v),
            ReflectValueBoxHashable::Bool(v) => ReflectValueRefHashable::Bool(*v),
            ReflectValueBoxHashable::String(v) => ReflectValueRefHashable::String(v),
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

    /// Convert.
    pub fn to_ref(&self) -> ReflectValueRefHashable {
        match self {
            ReflectValueBoxHashable::I32(v) => ReflectValueRefHashable::I32(*v),
            ReflectValueBoxHashable::U32(v) => ReflectValueRefHashable::U32(*v),
            ReflectValueBoxHashable::U64(v) => ReflectValueRefHashable::U64(*v),
            ReflectValueBoxHashable::I64(v) => ReflectValueRefHashable::I64(*v),
            ReflectValueBoxHashable::Bool(v) => ReflectValueRefHashable::Bool(*v),
            ReflectValueBoxHashable::String(v) => ReflectValueRefHashable::String(v),
        }
    }
}

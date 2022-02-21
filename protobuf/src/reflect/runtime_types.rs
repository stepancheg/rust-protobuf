//! Implementations of `RuntimeType` for all types.

use std::collections::HashMap;
use std::fmt;
use std::marker;
use std::mem;

#[cfg(feature = "bytes")]
use bytes::Bytes;

#[cfg(feature = "bytes")]
use crate::chars::Chars;
use crate::enum_or_unknown::EnumOrUnknown;
use crate::message_full::MessageFull;
use crate::reflect::runtime_type_box::RuntimeTypeBox;
use crate::reflect::value::value_ref::ReflectValueMut;
use crate::reflect::MessageRef;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::EnumFull;

/// `RuntimeType` is not implemented by all protobuf types directly
/// because it's not possible to implement `RuntimeType` for all `Message`
/// implementations at once: each `Message` implementation has to reimplement
/// all the methods again. With current strategy there's only implementation
/// for all messages, which is `RuntimeTypeMessage`.
///
/// The downside is that we have to explicitly specify type parameters
/// in a lot of places.
pub trait RuntimeType: fmt::Debug + Send + Sync + Sized + 'static {
    /// Actual value for this type.
    type Value: ProtobufValue + Clone + Sized + fmt::Debug + Default;

    /// "Box" version of type type.
    fn runtime_type_box() -> RuntimeTypeBox;

    /// Default value for this type.
    fn default_value_ref() -> ReflectValueRef<'static>;

    /// Construct a value from given reflective value.
    ///
    /// # Panics
    ///
    /// If reflective value is of incompatible type.
    fn from_value_box(value_box: ReflectValueBox) -> Result<Self::Value, ReflectValueBox>;

    /// Convert a value into a refletive box value.
    fn into_value_box(value: Self::Value) -> ReflectValueBox;

    /// Convert a value into a ref value if possible.
    ///
    /// # Panics
    ///
    /// For message and enum.
    // TODO: move the operation into a separate trait
    fn into_static_value_ref(value: Self::Value) -> ReflectValueRef<'static> {
        panic!("value {:?} cannot be converted to static ref", value)
    }

    /// Pointer to a dynamic reference.
    fn as_ref(value: &Self::Value) -> ReflectValueRef;
    /// Mutable pointer to a dynamic mutable reference.
    fn as_mut(value: &mut Self::Value) -> ReflectValueMut;

    /// Value is non-default?
    fn is_non_zero(value: &Self::Value) -> bool;

    /// Write the value.
    fn set_from_value_box(target: &mut Self::Value, value_box: ReflectValueBox) {
        *target = Self::from_value_box(value_box).expect("wrong type");
    }

    /// Cast values to enum values.
    ///
    /// # Panics
    ///
    /// If self is not an enum.
    fn cast_to_enum_values(values: &[Self::Value]) -> &[i32] {
        let _ = values;
        panic!("not enum")
    }
}

/// Runtime type which can be dereferenced.
pub trait RuntimeTypeWithDeref: RuntimeType {
    /// Deref target.
    type DerefTarget: ?Sized;

    /// Deref.
    // TODO: rename to `deref`
    fn defef_as_ref(value: &Self::DerefTarget) -> ReflectValueRef;
}

/// Object wrapper can be used to query hashmap.
pub enum RefOrValue<'a, Q: ?Sized, K> {
    /// A reference
    Ref(&'a Q),
    /// A value
    Value(K),
}

/// Types which can be hashmap keys.
pub trait RuntimeTypeHashable: RuntimeType {
    /// Query hash map with a given key.
    fn hash_map_get<'a, V>(map: &'a HashMap<Self::Value, V>, key: ReflectValueRef)
        -> Option<&'a V>;
}

/// Implementation for `f32`
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeF32;
/// Implementation for `f64`
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeF64;
/// Implementation for `i32`
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeI32;
/// Implementation for `f32`
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeI64;
/// Implementation for `u32`
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeU32;
/// Implementation for `u64`
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeU64;
/// Implementation for `bool`
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeBool;
/// Implementation for `String`
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeString;
/// Implementation for `Vec<u8>`
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeVecU8;

/// Implementation for [`Bytes`].
#[cfg(feature = "bytes")]
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeTokioBytes;
/// Implementation for [`Chars`].
#[cfg(feature = "bytes")]
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeTokioChars;

/// Implementation for enum.
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeEnumOrUnknown<E: EnumFull>(marker::PhantomData<E>);
/// Implementation for [`MessageFull`].
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeMessage<M: MessageFull>(marker::PhantomData<M>);

impl RuntimeType for RuntimeTypeF32 {
    type Value = f32;

    fn runtime_type_box() -> RuntimeTypeBox
    where
        Self: Sized,
    {
        RuntimeTypeBox::F32
    }

    fn default_value_ref() -> ReflectValueRef<'static> {
        ReflectValueRef::F32(0.0)
    }

    fn from_value_box(value_box: ReflectValueBox) -> Result<f32, ReflectValueBox> {
        match value_box {
            ReflectValueBox::F32(v) => Ok(v),
            b => Err(b),
        }
    }
    fn into_value_box(value: f32) -> ReflectValueBox {
        ReflectValueBox::F32(value)
    }

    fn into_static_value_ref(value: f32) -> ReflectValueRef<'static> {
        ReflectValueRef::F32(value)
    }
    fn as_ref(value: &f32) -> ReflectValueRef {
        ReflectValueRef::F32(*value)
    }

    fn as_mut(_value: &mut Self::Value) -> ReflectValueMut {
        unimplemented!()
    }

    fn is_non_zero(value: &f32) -> bool {
        *value != 0.0
    }
}

impl RuntimeType for RuntimeTypeF64 {
    type Value = f64;

    fn default_value_ref() -> ReflectValueRef<'static> {
        ReflectValueRef::F64(0.0)
    }

    fn runtime_type_box() -> RuntimeTypeBox
    where
        Self: Sized,
    {
        RuntimeTypeBox::F64
    }

    fn from_value_box(value_box: ReflectValueBox) -> Result<f64, ReflectValueBox> {
        match value_box {
            ReflectValueBox::F64(v) => Ok(v),
            b => Err(b),
        }
    }

    fn into_value_box(value: f64) -> ReflectValueBox {
        ReflectValueBox::F64(value)
    }

    fn into_static_value_ref(value: f64) -> ReflectValueRef<'static> {
        ReflectValueRef::F64(value)
    }

    fn as_ref(value: &f64) -> ReflectValueRef {
        ReflectValueRef::F64(*value)
    }

    fn is_non_zero(value: &f64) -> bool {
        *value != 0.0
    }

    fn as_mut(_value: &mut Self::Value) -> ReflectValueMut {
        unimplemented!()
    }
}

impl RuntimeType for RuntimeTypeI32 {
    type Value = i32;

    fn default_value_ref() -> ReflectValueRef<'static> {
        ReflectValueRef::I32(0)
    }

    fn runtime_type_box() -> RuntimeTypeBox
    where
        Self: Sized,
    {
        RuntimeTypeBox::I32
    }

    fn from_value_box(value_box: ReflectValueBox) -> Result<i32, ReflectValueBox> {
        match value_box {
            ReflectValueBox::I32(v) => Ok(v),
            b => Err(b),
        }
    }

    fn into_value_box(value: i32) -> ReflectValueBox {
        ReflectValueBox::I32(value)
    }

    fn into_static_value_ref(value: i32) -> ReflectValueRef<'static> {
        ReflectValueRef::I32(value)
    }

    fn as_ref(value: &i32) -> ReflectValueRef {
        ReflectValueRef::I32(*value)
    }

    fn is_non_zero(value: &i32) -> bool {
        *value != 0
    }

    fn as_mut(_value: &mut Self::Value) -> ReflectValueMut {
        unimplemented!()
    }
}
impl RuntimeTypeHashable for RuntimeTypeI32 {
    fn hash_map_get<'a, V>(map: &'a HashMap<i32, V>, key: ReflectValueRef) -> Option<&'a V> {
        match key {
            ReflectValueRef::I32(i) => map.get(&i),
            _ => None,
        }
    }
}

impl RuntimeType for RuntimeTypeI64 {
    type Value = i64;

    fn default_value_ref() -> ReflectValueRef<'static> {
        ReflectValueRef::I64(0)
    }

    fn runtime_type_box() -> RuntimeTypeBox
    where
        Self: Sized,
    {
        RuntimeTypeBox::I64
    }

    fn from_value_box(value_box: ReflectValueBox) -> Result<i64, ReflectValueBox> {
        match value_box {
            ReflectValueBox::I64(v) => Ok(v),
            b => Err(b),
        }
    }

    fn into_value_box(value: i64) -> ReflectValueBox {
        ReflectValueBox::I64(value)
    }

    fn into_static_value_ref(value: i64) -> ReflectValueRef<'static> {
        ReflectValueRef::I64(value)
    }

    fn as_ref(value: &i64) -> ReflectValueRef {
        ReflectValueRef::I64(*value)
    }

    fn is_non_zero(value: &i64) -> bool {
        *value != 0
    }

    fn as_mut(_value: &mut Self::Value) -> ReflectValueMut {
        unimplemented!()
    }
}
impl RuntimeTypeHashable for RuntimeTypeI64 {
    fn hash_map_get<'a, V>(map: &'a HashMap<i64, V>, key: ReflectValueRef) -> Option<&'a V> {
        match key {
            ReflectValueRef::I64(i) => map.get(&i),
            _ => None,
        }
    }
}

impl RuntimeType for RuntimeTypeU32 {
    type Value = u32;

    fn runtime_type_box() -> RuntimeTypeBox
    where
        Self: Sized,
    {
        RuntimeTypeBox::U32
    }

    fn default_value_ref() -> ReflectValueRef<'static> {
        ReflectValueRef::U32(0)
    }

    fn from_value_box(value_box: ReflectValueBox) -> Result<u32, ReflectValueBox> {
        match value_box {
            ReflectValueBox::U32(v) => Ok(v),
            b => Err(b),
        }
    }

    fn into_value_box(value: u32) -> ReflectValueBox {
        ReflectValueBox::U32(value)
    }

    fn into_static_value_ref(value: u32) -> ReflectValueRef<'static> {
        ReflectValueRef::U32(value)
    }

    fn as_ref(value: &u32) -> ReflectValueRef {
        ReflectValueRef::U32(*value)
    }

    fn as_mut(_value: &mut Self::Value) -> ReflectValueMut {
        unimplemented!()
    }

    fn is_non_zero(value: &u32) -> bool {
        *value != 0
    }
}
impl RuntimeTypeHashable for RuntimeTypeU32 {
    fn hash_map_get<'a, V>(map: &'a HashMap<u32, V>, key: ReflectValueRef) -> Option<&'a V> {
        match key {
            ReflectValueRef::U32(i) => map.get(&i),
            _ => None,
        }
    }
}

impl RuntimeType for RuntimeTypeU64 {
    type Value = u64;

    fn default_value_ref() -> ReflectValueRef<'static> {
        ReflectValueRef::U64(0)
    }

    fn runtime_type_box() -> RuntimeTypeBox
    where
        Self: Sized,
    {
        RuntimeTypeBox::U64
    }

    fn from_value_box(value_box: ReflectValueBox) -> Result<u64, ReflectValueBox> {
        match value_box {
            ReflectValueBox::U64(v) => Ok(v),
            b => Err(b),
        }
    }

    fn into_value_box(value: u64) -> ReflectValueBox {
        ReflectValueBox::U64(value)
    }

    fn into_static_value_ref(value: u64) -> ReflectValueRef<'static> {
        ReflectValueRef::U64(value)
    }

    fn as_ref(value: &u64) -> ReflectValueRef {
        ReflectValueRef::U64(*value)
    }

    fn is_non_zero(value: &u64) -> bool {
        *value != 0
    }

    fn as_mut(_value: &mut Self::Value) -> ReflectValueMut {
        unimplemented!()
    }
}
impl RuntimeTypeHashable for RuntimeTypeU64 {
    fn hash_map_get<'a, V>(map: &'a HashMap<u64, V>, key: ReflectValueRef) -> Option<&'a V> {
        match key {
            ReflectValueRef::U64(i) => map.get(&i),
            _ => None,
        }
    }
}

impl RuntimeType for RuntimeTypeBool {
    type Value = bool;

    fn default_value_ref() -> ReflectValueRef<'static> {
        ReflectValueRef::Bool(false)
    }

    fn runtime_type_box() -> RuntimeTypeBox
    where
        Self: Sized,
    {
        RuntimeTypeBox::Bool
    }

    fn from_value_box(value_box: ReflectValueBox) -> Result<bool, ReflectValueBox> {
        match value_box {
            ReflectValueBox::Bool(v) => Ok(v),
            b => Err(b),
        }
    }

    fn into_value_box(value: bool) -> ReflectValueBox {
        ReflectValueBox::Bool(value)
    }

    fn into_static_value_ref(value: bool) -> ReflectValueRef<'static> {
        ReflectValueRef::Bool(value)
    }

    fn as_ref(value: &bool) -> ReflectValueRef {
        ReflectValueRef::Bool(*value)
    }

    fn is_non_zero(value: &bool) -> bool {
        *value
    }

    fn as_mut(_value: &mut Self::Value) -> ReflectValueMut {
        unimplemented!()
    }
}
impl RuntimeTypeHashable for RuntimeTypeBool {
    fn hash_map_get<'a, V>(map: &'a HashMap<bool, V>, key: ReflectValueRef) -> Option<&'a V> {
        match key {
            ReflectValueRef::Bool(i) => map.get(&i),
            _ => None,
        }
    }
}

impl RuntimeType for RuntimeTypeString {
    type Value = String;

    fn runtime_type_box() -> RuntimeTypeBox
    where
        Self: Sized,
    {
        RuntimeTypeBox::String
    }

    fn default_value_ref() -> ReflectValueRef<'static> {
        ReflectValueRef::String("")
    }

    fn from_value_box(value_box: ReflectValueBox) -> Result<String, ReflectValueBox> {
        match value_box {
            ReflectValueBox::String(v) => Ok(v),
            b => Err(b),
        }
    }

    fn into_value_box(value: String) -> ReflectValueBox {
        ReflectValueBox::String(value)
    }

    fn as_ref(value: &String) -> ReflectValueRef {
        ReflectValueRef::String(&*value)
    }

    fn as_mut(_value: &mut Self::Value) -> ReflectValueMut {
        unimplemented!()
    }

    fn is_non_zero(value: &String) -> bool {
        !value.is_empty()
    }
}
impl RuntimeTypeWithDeref for RuntimeTypeString {
    type DerefTarget = str;

    fn defef_as_ref(value: &str) -> ReflectValueRef {
        ReflectValueRef::String(value)
    }
}
impl RuntimeTypeHashable for RuntimeTypeString {
    fn hash_map_get<'a, V>(map: &'a HashMap<String, V>, key: ReflectValueRef) -> Option<&'a V> {
        match key {
            ReflectValueRef::String(s) => map.get(*&s),
            _ => None,
        }
    }
}

impl RuntimeType for RuntimeTypeVecU8 {
    type Value = Vec<u8>;

    fn runtime_type_box() -> RuntimeTypeBox
    where
        Self: Sized,
    {
        RuntimeTypeBox::VecU8
    }

    fn default_value_ref() -> ReflectValueRef<'static> {
        ReflectValueRef::Bytes(b"")
    }

    fn from_value_box(value_box: ReflectValueBox) -> Result<Vec<u8>, ReflectValueBox> {
        match value_box {
            ReflectValueBox::Bytes(v) => Ok(v),
            b => Err(b),
        }
    }

    fn into_value_box(value: Vec<u8>) -> ReflectValueBox {
        ReflectValueBox::Bytes(value)
    }

    fn as_ref(value: &Vec<u8>) -> ReflectValueRef {
        ReflectValueRef::Bytes(value.as_slice())
    }

    fn as_mut(_value: &mut Self::Value) -> ReflectValueMut {
        unimplemented!()
    }

    fn is_non_zero(value: &Vec<u8>) -> bool {
        !value.is_empty()
    }
}
impl RuntimeTypeWithDeref for RuntimeTypeVecU8 {
    type DerefTarget = [u8];

    fn defef_as_ref(value: &[u8]) -> ReflectValueRef {
        ReflectValueRef::Bytes(value)
    }
}

#[cfg(feature = "bytes")]
impl RuntimeType for RuntimeTypeTokioBytes {
    type Value = Bytes;

    fn default_value_ref() -> ReflectValueRef<'static> {
        ReflectValueRef::Bytes(b"")
    }

    fn runtime_type_box() -> RuntimeTypeBox
    where
        Self: Sized,
    {
        RuntimeTypeBox::VecU8
    }

    fn from_value_box(value_box: ReflectValueBox) -> Result<Bytes, ReflectValueBox> {
        match value_box {
            ReflectValueBox::Bytes(v) => Ok(v.into()),
            b => Err(b),
        }
    }

    fn into_value_box(value: Bytes) -> ReflectValueBox {
        // TODO: copies here
        ReflectValueBox::Bytes(value.as_ref().to_owned())
    }

    fn as_ref(value: &Bytes) -> ReflectValueRef {
        ReflectValueRef::Bytes(value.as_ref())
    }

    fn is_non_zero(value: &Bytes) -> bool {
        !value.is_empty()
    }

    fn as_mut(_value: &mut Self::Value) -> ReflectValueMut {
        unimplemented!()
    }
}
#[cfg(feature = "bytes")]
impl RuntimeTypeWithDeref for RuntimeTypeTokioBytes {
    type DerefTarget = [u8];

    fn defef_as_ref(value: &[u8]) -> ReflectValueRef {
        ReflectValueRef::Bytes(value)
    }
}

#[cfg(feature = "bytes")]
impl RuntimeType for RuntimeTypeTokioChars {
    type Value = Chars;

    fn default_value_ref() -> ReflectValueRef<'static> {
        ReflectValueRef::String("")
    }

    fn runtime_type_box() -> RuntimeTypeBox
    where
        Self: Sized,
    {
        RuntimeTypeBox::String
    }

    fn from_value_box(value_box: ReflectValueBox) -> Result<Chars, ReflectValueBox> {
        match value_box {
            ReflectValueBox::String(v) => Ok(v.into()),
            b => Err(b),
        }
    }

    fn into_value_box(value: Chars) -> ReflectValueBox {
        ReflectValueBox::String(value.into())
    }

    fn as_ref(value: &Chars) -> ReflectValueRef {
        ReflectValueRef::String(value.as_ref())
    }

    fn is_non_zero(value: &Chars) -> bool {
        !value.is_empty()
    }

    fn as_mut(_value: &mut Self::Value) -> ReflectValueMut {
        unimplemented!()
    }
}
#[cfg(feature = "bytes")]
impl RuntimeTypeWithDeref for RuntimeTypeTokioChars {
    type DerefTarget = str;

    fn defef_as_ref(value: &str) -> ReflectValueRef {
        ReflectValueRef::String(value)
    }
}
#[cfg(feature = "bytes")]
impl RuntimeTypeHashable for RuntimeTypeTokioChars {
    fn hash_map_get<'a, V>(map: &'a HashMap<Chars, V>, key: ReflectValueRef) -> Option<&'a V> {
        match key {
            ReflectValueRef::String(s) => map.get(&*s),
            _ => None,
        }
    }
}

impl<E> RuntimeType for RuntimeTypeEnumOrUnknown<E>
where
    E: EnumFull + fmt::Debug,
{
    type Value = EnumOrUnknown<E>;

    fn runtime_type_box() -> RuntimeTypeBox
    where
        Self: Sized,
    {
        RuntimeTypeBox::Enum(E::enum_descriptor_static())
    }

    fn default_value_ref() -> ReflectValueRef<'static> {
        ReflectValueRef::Enum(
            E::enum_descriptor_static(),
            E::enum_descriptor_static().first_value().value(),
        )
    }

    fn from_value_box(value_box: ReflectValueBox) -> Result<EnumOrUnknown<E>, ReflectValueBox> {
        match value_box {
            ReflectValueBox::Enum(d, v) if d == E::enum_descriptor_static() => {
                Ok(EnumOrUnknown::from_i32(v))
            }
            b => Err(b),
        }
    }

    fn into_value_box(value: EnumOrUnknown<E>) -> ReflectValueBox {
        ReflectValueBox::Enum(E::enum_descriptor_static(), value.value())
    }

    fn into_static_value_ref(value: EnumOrUnknown<E>) -> ReflectValueRef<'static> {
        ReflectValueRef::Enum(E::enum_descriptor_static(), value.value())
    }

    fn as_ref(value: &EnumOrUnknown<E>) -> ReflectValueRef {
        ReflectValueRef::Enum(E::enum_descriptor_static(), value.value())
    }

    fn as_mut(_value: &mut Self::Value) -> ReflectValueMut {
        unimplemented!()
    }

    fn is_non_zero(value: &EnumOrUnknown<E>) -> bool {
        value.value() != 0
    }

    fn cast_to_enum_values(values: &[EnumOrUnknown<E>]) -> &[i32] {
        assert_eq!(mem::size_of::<i32>(), mem::size_of::<EnumOrUnknown<E>>());
        // SAFETY: `ProtobufEnumOrUnknown<E>` is transparent as `i32`.
        unsafe { mem::transmute(values) }
    }
}

impl<M> RuntimeType for RuntimeTypeMessage<M>
where
    M: MessageFull + ProtobufValue + Clone + Default,
{
    type Value = M;

    fn runtime_type_box() -> RuntimeTypeBox
    where
        Self: Sized,
    {
        RuntimeTypeBox::Message(M::descriptor_static())
    }

    fn default_value_ref() -> ReflectValueRef<'static> {
        ReflectValueRef::Message(MessageRef::new(M::default_instance()))
    }

    fn from_value_box(value_box: ReflectValueBox) -> Result<M, ReflectValueBox> {
        match value_box {
            ReflectValueBox::Message(v) => v
                .downcast_box()
                .map(|v| *v)
                .map_err(ReflectValueBox::Message),
            b => Err(b),
        }
    }

    fn into_value_box(value: M) -> ReflectValueBox {
        ReflectValueBox::Message(Box::new(value))
    }
    fn as_ref(value: &M) -> ReflectValueRef {
        ReflectValueRef::Message(MessageRef::new(value))
    }

    fn as_mut(value: &mut M) -> ReflectValueMut {
        ReflectValueMut::Message(value)
    }

    fn is_non_zero(_value: &M) -> bool {
        true
    }
}

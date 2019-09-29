//! Implementations of `RuntimeType` for all types.

use std::fmt;
use std::marker;

#[cfg(feature = "bytes")]
use bytes::Bytes;

use crate::reflect::runtime_type_box::RuntimeTypeBox;
use crate::reflect::runtime_type_dynamic::RuntimeTypeDynamic;
use crate::reflect::runtime_type_dynamic::RuntimeTypeDynamicImpl;
use crate::reflect::value::ReflectValueMut;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;

#[cfg(feature = "bytes")]
use crate::chars::Chars;
use crate::core::Message;
use crate::enums::ProtobufEnum;
use crate::enums::ProtobufEnumOrUnknown;

/// `RuntimeType` is not implemented by all protobuf types directly
/// because it's not possible to implement `RuntimeType` for all `Message`
/// implementations at once: each `Message` implementation has to reimplement
/// all the methods again. With current strategy there's only implementation
/// for all messages, which is `RuntimeTypeMessage`.
///
/// The downside is that we have to explicitly specify type parameters
/// in a lot of places.
pub trait RuntimeType: fmt::Debug + Send + Sync + 'static {
    type Value: ProtobufValue + Clone + Sized + fmt::Debug + Default;

    fn dynamic() -> &'static dyn RuntimeTypeDynamic
    where
        Self: Sized,
    {
        &RuntimeTypeDynamicImpl::<Self>(marker::PhantomData)
    }

    fn runtime_type_box() -> RuntimeTypeBox
    where
        Self: Sized;

    fn default_value_ref() -> ReflectValueRef<'static>;

    fn from_value_box(value_box: ReflectValueBox) -> Self::Value;

    fn into_value_box(value: Self::Value) -> ReflectValueBox;

    // TODO: move the operation into a separate trait
    fn into_static_value_ref(value: Self::Value) -> ReflectValueRef<'static> {
        panic!("value {:?} cannot be converted to static ref", value)
    }

    fn as_ref(value: &Self::Value) -> ReflectValueRef;
    fn as_mut(value: &mut Self::Value) -> ReflectValueMut;

    fn is_non_zero(value: &Self::Value) -> bool;

    fn set_from_value_box(target: &mut Self::Value, value_box: ReflectValueBox) {
        *target = Self::from_value_box(value_box);
    }
}

pub trait RuntimeTypeWithDeref: RuntimeType {
    type DerefTarget: ?Sized;

    // TODO: rename to `deref`
    fn defef_as_ref(value: &Self::DerefTarget) -> ReflectValueRef;
}

#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeF32;
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeF64;
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeI32;
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeI64;
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeU32;
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeU64;
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeBool;
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeString;
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeVecU8;
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeChars;

#[cfg(feature = "bytes")]
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeCarllercheBytes;
#[cfg(feature = "bytes")]
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeCarllercheChars;

#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeEnum<E: ProtobufEnum>(marker::PhantomData<E>);
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeEnumOrUnknown<E: ProtobufEnum>(marker::PhantomData<E>);
#[derive(Debug, Copy, Clone)]
pub struct RuntimeTypeMessage<M: Message>(marker::PhantomData<M>);

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

    fn from_value_box(value_box: ReflectValueBox) -> f32 {
        match value_box {
            ReflectValueBox::F32(v) => v,
            _ => panic!("wrong type"),
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

    fn from_value_box(value_box: ReflectValueBox) -> f64 {
        match value_box {
            ReflectValueBox::F64(v) => v,
            _ => panic!("wrong type"),
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

    fn from_value_box(value_box: ReflectValueBox) -> i32 {
        match value_box {
            ReflectValueBox::I32(v) => v,
            _ => panic!("wrong type"),
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

    fn from_value_box(value_box: ReflectValueBox) -> i64 {
        match value_box {
            ReflectValueBox::I64(v) => v,
            _ => panic!("wrong type"),
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

    fn from_value_box(value_box: ReflectValueBox) -> u32 {
        match value_box {
            ReflectValueBox::U32(v) => v,
            _ => panic!("wrong type"),
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

    fn from_value_box(value_box: ReflectValueBox) -> u64 {
        match value_box {
            ReflectValueBox::U64(v) => v,
            _ => panic!("wrong type"),
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

    fn from_value_box(value_box: ReflectValueBox) -> bool {
        match value_box {
            ReflectValueBox::Bool(v) => v,
            _ => panic!("wrong type"),
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

    fn from_value_box(value_box: ReflectValueBox) -> String {
        match value_box {
            ReflectValueBox::String(v) => v,
            _ => panic!("wrong type"),
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

    fn from_value_box(value_box: ReflectValueBox) -> Vec<u8> {
        match value_box {
            ReflectValueBox::Bytes(v) => v,
            _ => panic!("wrong type"),
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
impl RuntimeType for RuntimeTypeCarllercheBytes {
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

    fn from_value_box(value_box: ReflectValueBox) -> Bytes {
        match value_box {
            ReflectValueBox::Bytes(v) => v.into(),
            _ => panic!("wrong type"),
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
impl RuntimeTypeWithDeref for RuntimeTypeCarllercheBytes {
    type DerefTarget = [u8];

    fn defef_as_ref(value: &[u8]) -> ReflectValueRef {
        ReflectValueRef::Bytes(value)
    }
}

#[cfg(feature = "bytes")]
impl RuntimeType for RuntimeTypeCarllercheChars {
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

    fn from_value_box(value_box: ReflectValueBox) -> Chars {
        match value_box {
            ReflectValueBox::String(v) => v.into(),
            _ => panic!("wrong type"),
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
impl RuntimeTypeWithDeref for RuntimeTypeCarllercheChars {
    type DerefTarget = str;

    fn defef_as_ref(value: &str) -> ReflectValueRef {
        ReflectValueRef::String(value)
    }
}

impl<E> RuntimeType for RuntimeTypeEnum<E>
where
    E: ProtobufEnum + ProtobufValue + fmt::Debug,
{
    type Value = E;

    fn runtime_type_box() -> RuntimeTypeBox
    where
        Self: Sized,
    {
        RuntimeTypeBox::Enum(E::enum_descriptor_static())
    }

    fn default_value_ref() -> ReflectValueRef<'static> {
        ReflectValueRef::Enum(
            E::enum_descriptor_static(),
            E::enum_descriptor_static().values()[0].value(),
        )
    }

    fn from_value_box(value_box: ReflectValueBox) -> E {
        match value_box {
            // TODO: panic
            ReflectValueBox::Enum(_d, v) => E::from_i32(v).expect("unknown enum value"),
            _ => panic!("wrong type"),
        }
    }

    fn into_value_box(value: E) -> ReflectValueBox {
        ReflectValueBox::Enum(E::enum_descriptor_static(), value.value())
    }

    fn into_static_value_ref(value: E) -> ReflectValueRef<'static> {
        ReflectValueRef::Enum(E::enum_descriptor_static(), value.value())
    }

    fn as_ref(value: &E) -> ReflectValueRef {
        ReflectValueRef::Enum(E::enum_descriptor_static(), value.value())
    }

    fn as_mut(_value: &mut Self::Value) -> ReflectValueMut {
        unimplemented!()
    }

    fn is_non_zero(value: &E) -> bool {
        value.value() != 0
    }
}

impl<E> RuntimeType for RuntimeTypeEnumOrUnknown<E>
where
    E: ProtobufEnum + ProtobufValue + fmt::Debug,
{
    type Value = ProtobufEnumOrUnknown<E>;

    fn runtime_type_box() -> RuntimeTypeBox
    where
        Self: Sized,
    {
        RuntimeTypeBox::Enum(E::enum_descriptor_static())
    }

    fn default_value_ref() -> ReflectValueRef<'static> {
        ReflectValueRef::Enum(
            E::enum_descriptor_static(),
            E::enum_descriptor_static().values()[0].value(),
        )
    }

    fn from_value_box(value_box: ReflectValueBox) -> ProtobufEnumOrUnknown<E> {
        match value_box {
            ReflectValueBox::Enum(_d, v) => ProtobufEnumOrUnknown::from_i32(v),
            _ => panic!("wrong type"),
        }
    }

    fn into_value_box(value: ProtobufEnumOrUnknown<E>) -> ReflectValueBox {
        ReflectValueBox::Enum(E::enum_descriptor_static(), value.value())
    }

    fn into_static_value_ref(value: ProtobufEnumOrUnknown<E>) -> ReflectValueRef<'static> {
        ReflectValueRef::Enum(E::enum_descriptor_static(), value.value())
    }

    fn as_ref(value: &ProtobufEnumOrUnknown<E>) -> ReflectValueRef {
        ReflectValueRef::Enum(E::enum_descriptor_static(), value.value())
    }

    fn as_mut(_value: &mut Self::Value) -> ReflectValueMut {
        unimplemented!()
    }

    fn is_non_zero(value: &ProtobufEnumOrUnknown<E>) -> bool {
        value.value() != 0
    }
}

impl<M> RuntimeType for RuntimeTypeMessage<M>
where
    M: Message + Clone + ProtobufValue + Default,
{
    type Value = M;

    fn runtime_type_box() -> RuntimeTypeBox
    where
        Self: Sized,
    {
        RuntimeTypeBox::Message(M::descriptor_static())
    }

    fn default_value_ref() -> ReflectValueRef<'static> {
        ReflectValueRef::Message(M::descriptor_static().default_instance())
    }

    fn from_value_box(value_box: ReflectValueBox) -> M {
        match value_box {
            ReflectValueBox::Message(v) => *v.downcast_box().expect("wrong message type"),
            _ => panic!("wrong type"),
        }
    }

    fn into_value_box(value: M) -> ReflectValueBox {
        ReflectValueBox::Message(Box::new(value))
    }
    fn as_ref(value: &M) -> ReflectValueRef {
        ReflectValueRef::Message(value)
    }

    fn as_mut(value: &mut M) -> ReflectValueMut {
        ReflectValueMut::Message(value)
    }

    fn is_non_zero(_value: &M) -> bool {
        true
    }
}

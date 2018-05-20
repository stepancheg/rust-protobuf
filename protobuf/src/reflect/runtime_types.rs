use ProtobufEnum;
use std::marker;
use Message;
use reflect::ReflectValueBox;
#[cfg(feature = "bytes")]
use bytes::Bytes;
#[cfg(feature = "bytes")]
use Chars;
use reflect::ProtobufValue;
use reflect::EnumDescriptor;
use reflect::MessageDescriptor;
use reflect::ReflectValueRef;
use reflect::runtime_type_dynamic::RuntimeTypeDynamic;
use reflect::runtime_type_dynamic::RuntimeTypeDynamicImpl;


/// `RuntimeType` is not implemented by all protobuf types directly
/// because it's not possible to implement `RuntimeType` for all `Message`
/// implementations at once: each `Message` implementation has to reimplement
/// all the methods again. With current strategy there's only implementation
/// for all messages, which is `RuntimeTypeMessage`.
///
/// The downside is that we have to explicitly specify type parameters
/// in a lot of places.
pub trait RuntimeType : Send + Sync + 'static {
    type Value : ProtobufValue + Clone + Sized + 'static;

    fn dynamic() -> &'static RuntimeTypeDynamic;

    /// Get enum descriptor for this type, panics if not enum
    fn enum_descriptor() -> &'static EnumDescriptor {
        panic!("not an enum");
    }

    /// Get enum descriptor for this type, panics if not enum
    fn message_descriptor() -> &'static MessageDescriptor {
        panic!("not an message");
    }

    fn from_value_box(value_box: ReflectValueBox) -> Self::Value;

    fn into_value_box(value: Self::Value) -> ReflectValueBox;

    fn into_static_value_ref(_value: Self::Value) -> ReflectValueRef<'static> {
        panic!("type cannot be converted to static ref")
    }

    fn as_ref(value: &Self::Value) -> ReflectValueRef;

    fn set_from_value_box(target: &mut Self::Value, value_box: ReflectValueBox) {
        *target = Self::from_value_box(value_box);
    }
}

#[derive(Copy, Clone)]
pub struct RuntimeTypeF32;
#[derive(Copy, Clone)]
pub struct RuntimeTypeF64;
#[derive(Copy, Clone)]
pub struct RuntimeTypeI32;
#[derive(Copy, Clone)]
pub struct RuntimeTypeI64;
#[derive(Copy, Clone)]
pub struct RuntimeTypeU32;
#[derive(Copy, Clone)]
pub struct RuntimeTypeU64;
#[derive(Copy, Clone)]
pub struct RuntimeTypeBool;
#[derive(Copy, Clone)]
pub struct RuntimeTypeString;
#[derive(Copy, Clone)]
pub struct RuntimeTypeVecU8;
#[derive(Copy, Clone)]
pub struct RuntimeTypeChars;

#[cfg(feature = "bytes")]
#[derive(Copy, Clone)]
pub struct RuntimeTypeCarllercheBytes;
#[cfg(feature = "bytes")]
#[derive(Copy, Clone)]
pub struct RuntimeTypeCarllercheChars;

#[derive(Copy, Clone)]
pub struct RuntimeTypeEnum<E : ProtobufEnum>(marker::PhantomData<E>);
#[derive(Copy, Clone)]
pub struct RuntimeTypeMessage<M : Message>(marker::PhantomData<M>);

#[derive(Copy, Clone)]
pub struct RuntimeTypeUnreachable;

static DYNAMIC_F32: RuntimeTypeDynamicImpl<RuntimeTypeF32> = RuntimeTypeDynamicImpl(marker::PhantomData);

impl RuntimeType for RuntimeTypeF32 {
    type Value = f32;

    fn dynamic() -> &'static RuntimeTypeDynamic {
        &DYNAMIC_F32
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
}

static DYNAMIC_F64: RuntimeTypeDynamicImpl<RuntimeTypeF64> = RuntimeTypeDynamicImpl(marker::PhantomData);

impl RuntimeType for RuntimeTypeF64 {
    type Value = f64;

    fn dynamic() -> &'static RuntimeTypeDynamic {
        &DYNAMIC_F64
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
}

static DYNAMIC_I32: RuntimeTypeDynamicImpl<RuntimeTypeI32> = RuntimeTypeDynamicImpl(marker::PhantomData);

impl RuntimeType for RuntimeTypeI32 {
    type Value = i32;

    fn dynamic() -> &'static RuntimeTypeDynamic {
        &DYNAMIC_I32
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
}

static DYNAMIC_I64: RuntimeTypeDynamicImpl<RuntimeTypeI64> = RuntimeTypeDynamicImpl(marker::PhantomData);

impl RuntimeType for RuntimeTypeI64 {
    type Value = i64;

    fn dynamic() -> &'static RuntimeTypeDynamic {
        &DYNAMIC_I64
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
}

static DYNAMIC_U32: RuntimeTypeDynamicImpl<RuntimeTypeU32> = RuntimeTypeDynamicImpl(marker::PhantomData);

impl RuntimeType for RuntimeTypeU32 {
    type Value = u32;

    fn dynamic() -> &'static RuntimeTypeDynamic {
        &DYNAMIC_U32
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
}

static DYNAMIC_U64: RuntimeTypeDynamicImpl<RuntimeTypeU64> = RuntimeTypeDynamicImpl(marker::PhantomData);

impl RuntimeType for RuntimeTypeU64 {
    type Value = u64;

    fn dynamic() -> &'static RuntimeTypeDynamic {
        &DYNAMIC_U64
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
}

static DYNAMIC_BOOL: RuntimeTypeDynamicImpl<RuntimeTypeBool> = RuntimeTypeDynamicImpl(marker::PhantomData);

impl RuntimeType for RuntimeTypeBool {
    type Value = bool;

    fn dynamic() -> &'static RuntimeTypeDynamic {
        &DYNAMIC_BOOL
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
}

static DYNAMIC_STRING: RuntimeTypeDynamicImpl<RuntimeTypeString> = RuntimeTypeDynamicImpl(marker::PhantomData);

impl RuntimeType for RuntimeTypeString {
    type Value = String;

    fn dynamic() -> &'static RuntimeTypeDynamic {
        &DYNAMIC_STRING
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
}

static DYNAMIC_VEC_U8: RuntimeTypeDynamicImpl<RuntimeTypeVecU8> = RuntimeTypeDynamicImpl(marker::PhantomData);

impl RuntimeType for RuntimeTypeVecU8 {
    type Value = Vec<u8>;

    fn dynamic() -> &'static RuntimeTypeDynamic {
        &DYNAMIC_VEC_U8
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
}

#[cfg(feature = "bytes")]
static DYNAMIC_CARLLERCHE_BYTES: RuntimeTypeDynamicImpl<RuntimeTypeCarllercheBytes> = RuntimeTypeDynamicImpl(marker::PhantomData);

#[cfg(feature = "bytes")]
impl RuntimeType for RuntimeTypeCarllercheBytes {
    type Value = Bytes;

    fn dynamic() -> &'static RuntimeTypeDynamic {
        &DYNAMIC_CARLLERCHE_BYTES
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
}

#[cfg(feature = "bytes")]
static DYNAMIC_CARLLERCHE_CHARS: RuntimeTypeDynamicImpl<RuntimeTypeCarllercheChars> = RuntimeTypeDynamicImpl(marker::PhantomData);

#[cfg(feature = "bytes")]
impl RuntimeType for RuntimeTypeCarllercheChars {
    type Value = Chars;

    fn dynamic() -> &'static RuntimeTypeDynamic {
        &DYNAMIC_CARLLERCHE_CHARS
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
}

impl<E> RuntimeType for RuntimeTypeEnum<E>
    where E : ProtobufEnum + ProtobufValue
{
    type Value = E;

    fn dynamic() -> &'static RuntimeTypeDynamic {
        Self::enum_descriptor().dynamic()
    }

    fn enum_descriptor() -> &'static EnumDescriptor {
        E::enum_descriptor_static()
    }

    fn from_value_box(value_box: ReflectValueBox) -> E {
        match value_box {
            ReflectValueBox::Enum(v) => E::from_i32(v.value()).expect("unknown enum value"),
            _ => panic!("wrong type"),
        }
    }

    fn into_value_box(value: E) -> ReflectValueBox {
        ReflectValueBox::Enum(value.descriptor())
    }

    fn as_ref(value: &E) -> ReflectValueRef {
        ReflectValueRef::Enum(value.descriptor())
    }
}

impl<M> RuntimeType for RuntimeTypeMessage<M>
    where M : Message + Clone + ProtobufValue
{
    type Value = M;

    fn dynamic() -> &'static RuntimeTypeDynamic {
        Self::message_descriptor().dynamic()
    }

    fn message_descriptor() -> &'static MessageDescriptor {
        M::descriptor_static()
    }

    fn from_value_box(value_box: ReflectValueBox) -> M {
        match value_box {
            ReflectValueBox::Message(v) => {
                *v.into_any_box().downcast().expect("wrong message type")
            },
            _ => panic!("wrong type"),
        }
    }

    fn into_value_box(value: M) -> ReflectValueBox {
        ReflectValueBox::Message(Box::new(value))
    }
    fn as_ref(value: &M) -> ReflectValueRef {
        ReflectValueRef::Message(value)
    }
}

impl RuntimeType for RuntimeTypeUnreachable {
    type Value = u32;

    fn dynamic() -> &'static RuntimeTypeDynamic {
        unreachable!()
    }

    fn from_value_box(_value_box: ReflectValueBox) -> u32 {
        unreachable!()
    }

    fn into_value_box(_value: u32) -> ReflectValueBox {
        unreachable!()
    }

    fn as_ref(_value: &u32) -> ReflectValueRef {
        unreachable!()
    }
}

use crate::core::message_down_cast;
use crate::reflect::accessor::{AccessorKind, FieldAccessor};
use crate::reflect::optional::ReflectOptional;
use crate::reflect::repeated::ReflectRepeatedRef;
use crate::reflect::runtime_type_dynamic::RuntimeTypeDynamic;
use crate::reflect::runtime_types::RuntimeType;
use crate::reflect::runtime_types::RuntimeTypeBool;
use crate::reflect::runtime_types::RuntimeTypeEnum;
use crate::reflect::runtime_types::RuntimeTypeF32;
use crate::reflect::runtime_types::RuntimeTypeF64;
use crate::reflect::runtime_types::RuntimeTypeI32;
use crate::reflect::runtime_types::RuntimeTypeI64;
use crate::reflect::runtime_types::RuntimeTypeMessage;
use crate::reflect::runtime_types::RuntimeTypeString;
use crate::reflect::runtime_types::RuntimeTypeU32;
use crate::reflect::runtime_types::RuntimeTypeU64;
use crate::reflect::runtime_types::RuntimeTypeVecU8;
use crate::reflect::types::ProtobufType;
use crate::reflect::EnumValueDescriptor;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueRef;
use crate::Message;
use crate::ProtobufEnum;
use crate::SingularField;
use crate::SingularPtrField;
use std::fmt;

/// This trait should not be used directly, use `FieldDescriptor` instead
pub(crate) trait SingularFieldAccessor: Send + Sync + 'static {
    fn get_field<'a>(&self, m: &'a dyn Message) -> Option<ReflectValueRef<'a>>;
    fn has_field_generic(&self, m: &dyn Message) -> bool;
    // TODO: should it return default value or panic on unset field?
    fn get_message_generic<'a>(&self, m: &'a dyn Message) -> &'a dyn Message;
    fn get_enum_generic(&self, m: &dyn Message) -> &'static EnumValueDescriptor;
    fn get_str_generic<'a>(&self, m: &'a dyn Message) -> &'a str;
    fn get_bytes_generic<'a>(&self, m: &'a dyn Message) -> &'a [u8];
    fn get_u32_generic(&self, m: &dyn Message) -> u32;
    fn get_u64_generic(&self, m: &dyn Message) -> u64;
    fn get_i32_generic(&self, m: &dyn Message) -> i32;
    fn get_i64_generic(&self, m: &dyn Message) -> i64;
    fn get_bool_generic(&self, m: &dyn Message) -> bool;
    fn get_f32_generic(&self, m: &dyn Message) -> f32;
    fn get_f64_generic(&self, m: &dyn Message) -> f64;
}

pub(crate) struct SingularFieldAccessorHolder {
    pub accessor: Box<dyn SingularFieldAccessor>,
}

trait GetSingularMessage<M>: Send + Sync + 'static {
    fn get_message<'a>(&self, m: &'a M) -> &'a dyn Message;
}

struct GetSingularMessageImpl<M, N> {
    get: for<'a> fn(&'a M) -> &'a N,
}

impl<M: Message, N: Message + 'static> GetSingularMessage<M> for GetSingularMessageImpl<M, N> {
    fn get_message<'a>(&self, m: &'a M) -> &'a dyn Message {
        (self.get)(m)
    }
}

trait GetSingularEnum<M>: Send + Sync + 'static {
    fn get_enum(&self, m: &M) -> &'static EnumValueDescriptor;
}

struct GetSingularEnumImpl<M, E> {
    get: fn(&M) -> E,
}

impl<M: Message, E: ProtobufEnum> GetSingularEnum<M> for GetSingularEnumImpl<M, E> {
    fn get_enum(&self, m: &M) -> &'static EnumValueDescriptor {
        (self.get)(m).descriptor()
    }
}

trait GetRepeatedMessage<M> {
    fn len_field(&self, m: &M) -> usize;
    fn get_message_item<'a>(&self, m: &'a M, index: usize) -> &'a dyn Message;
    fn reflect_repeated_message<'a>(&self, m: &'a M) -> ReflectRepeatedRef<'a>;
}

trait GetRepeatedEnum<M: Message + 'static> {
    fn len_field(&self, m: &M) -> usize;
    fn get_enum_item(&self, m: &M, index: usize) -> &'static EnumValueDescriptor;
    fn reflect_repeated_enum<'a>(&self, m: &'a M) -> ReflectRepeatedRef<'a>;
}

trait GetSetCopyFns<M>: Send + Sync + 'static {
    fn get_field<'a>(&self, m: &'a M) -> ReflectValueRef<'a>;
}

struct GetSetCopyFnsImpl<M, V: ProtobufValue + Copy> {
    get: fn(&M) -> V,
    _set: fn(&mut M, V),
}

impl<M: Send + Sync + 'static, V: ProtobufValue + Copy> GetSetCopyFns<M>
    for GetSetCopyFnsImpl<M, V>
{
    fn get_field<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        (&(self.get)(m) as &dyn ProtobufValue).as_ref_copy()
    }
}

enum SingularGetSet<M> {
    Copy(Box<dyn GetSetCopyFns<M>>),
    String(for<'a> fn(&'a M) -> &'a str, fn(&mut M, String)),
    Bytes(for<'a> fn(&'a M) -> &'a [u8], fn(&mut M, Vec<u8>)),
    Enum(Box<dyn GetSingularEnum<M> + 'static>),
    Message(Box<dyn GetSingularMessage<M> + 'static>),
}

impl<M: Message + 'static> SingularGetSet<M> {
    fn get_ref<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        match self {
            &SingularGetSet::Copy(ref copy) => copy.get_field(m),
            &SingularGetSet::String(get, _) => ReflectValueRef::String(get(m)),
            &SingularGetSet::Bytes(get, _) => ReflectValueRef::Bytes(get(m)),
            &SingularGetSet::Enum(ref get) => ReflectValueRef::Enum(get.get_enum(m)),
            &SingularGetSet::Message(ref get) => ReflectValueRef::Message(get.get_message(m)),
        }
    }
}

trait FieldAccessor2<M, R: ?Sized>: Send + Sync + 'static
where
    M: Message + Send + Sync + 'static,
{
    fn get_field<'a>(&self, _: &'a M) -> &'a R;
    fn mut_field<'a>(&self, _: &'a mut M) -> &'a mut R;
}

struct MessageGetMut<M, L>
where
    M: Message + 'static,
{
    get_field: for<'a> fn(&'a M) -> &'a L,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut L,
}

enum FieldAccessorFunctions<M> {
    // up to 1.0.24 optional or required
    SingularHasGetSet {
        has: fn(&M) -> bool,
        get_set: SingularGetSet<M>,
    },
    // protobuf 3 simple field
    Simple(Box<dyn FieldAccessor2<M, dyn ProtobufValue>>),
    // optional, required or message
    Optional(Box<dyn FieldAccessor2<M, dyn ReflectOptional>>),
}

impl<M> fmt::Debug for FieldAccessorFunctions<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &FieldAccessorFunctions::SingularHasGetSet { .. } => {
                write!(f, "SingularHasGetSet {{ .. }}")
            }
            &FieldAccessorFunctions::Simple(..) => write!(f, "Simple(..)"),
            &FieldAccessorFunctions::Optional(..) => write!(f, "Optional(..)"),
        }
    }
}

struct FieldAccessorImpl<M> {
    fns: FieldAccessorFunctions<M>,
    runtime_type: &'static dyn RuntimeTypeDynamic,
}

impl<M: Message + Send + Sync + 'static> FieldAccessorImpl<M> {
    fn get_value_option<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        match self.fns {
            FieldAccessorFunctions::Simple(ref a) => Some(a.get_field(m).as_ref()),
            FieldAccessorFunctions::Optional(ref a) => {
                a.get_field(m).to_option().map(|v| v.as_ref())
            }
            FieldAccessorFunctions::SingularHasGetSet {
                ref has,
                ref get_set,
            } => {
                if !has(m) {
                    None
                } else {
                    Some(get_set.get_ref(m))
                }
            }
        }
    }
}

impl<M: Message + 'static> SingularFieldAccessor for FieldAccessorImpl<M> {
    fn get_field<'a>(&self, m: &'a dyn Message) -> Option<ReflectValueRef<'a>> {
        match self.fns {
            FieldAccessorFunctions::Optional(ref accessor2) => accessor2
                .get_field(message_down_cast(m))
                .to_option()
                .map(|v| v.as_ref()),
            FieldAccessorFunctions::Simple(ref accessor2) => {
                let v = accessor2.get_field(message_down_cast(m));
                if v.is_non_zero() {
                    Some(v.as_ref())
                } else {
                    None
                }
            }
            FieldAccessorFunctions::SingularHasGetSet {
                ref has,
                ref get_set,
            } => {
                if has(message_down_cast(m)) {
                    Some(get_set.get_ref(message_down_cast(m)))
                } else {
                    None
                }
            }
        }
    }

    fn has_field_generic(&self, m: &dyn Message) -> bool {
        match self.fns {
            FieldAccessorFunctions::SingularHasGetSet { has, .. } => has(message_down_cast(m)),
            FieldAccessorFunctions::Optional(ref a) => {
                a.get_field(message_down_cast(m)).to_option().is_some()
            }
            FieldAccessorFunctions::Simple(ref a) => {
                a.get_field(message_down_cast(m)).is_non_zero()
            }
        }
    }

    fn get_message_generic<'a>(&self, m: &'a dyn Message) -> &'a dyn Message {
        match self.fns {
            FieldAccessorFunctions::SingularHasGetSet {
                get_set: SingularGetSet::Message(ref get),
                ..
            } => get.get_message(message_down_cast(m)),
            FieldAccessorFunctions::Optional(ref t) => {
                match t
                    .get_field(message_down_cast(m))
                    .to_option()
                    .expect("field unset")
                    .as_ref()
                {
                    ReflectValueRef::Message(m) => m,
                    _ => panic!("not a message"),
                }
            }
            ref fns => panic!("unknown accessor type: {:?}", fns),
        }
    }

    fn get_enum_generic(&self, m: &dyn Message) -> &'static EnumValueDescriptor {
        match self.fns {
            FieldAccessorFunctions::SingularHasGetSet {
                get_set: SingularGetSet::Enum(ref get),
                ..
            } => get.get_enum(message_down_cast(m)),
            _ => panic!(),
        }
    }

    fn get_str_generic<'a>(&self, m: &'a dyn Message) -> &'a str {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::String(v)) => v,
            Some(_) => panic!("wrong type"),
            None => "", // TODO: check type
        }
    }

    fn get_bytes_generic<'a>(&self, m: &'a dyn Message) -> &'a [u8] {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::Bytes(v)) => v,
            Some(_) => panic!("wrong type"),
            None => b"", // TODO: check type
        }
    }

    fn get_u32_generic(&self, m: &dyn Message) -> u32 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::U32(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0, // TODO: check type
        }
    }

    fn get_u64_generic(&self, m: &dyn Message) -> u64 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::U64(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0, // TODO: check type
        }
    }

    fn get_i32_generic(&self, m: &dyn Message) -> i32 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::I32(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0, // TODO: check type
        }
    }

    fn get_i64_generic(&self, m: &dyn Message) -> i64 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::I64(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0, // TODO: check type
        }
    }

    fn get_bool_generic(&self, m: &dyn Message) -> bool {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::Bool(v)) => v,
            Some(_) => panic!("wrong type"),
            None => false, // TODO: check type
        }
    }

    fn get_f32_generic(&self, m: &dyn Message) -> f32 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::F32(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0.0, // TODO: check type
        }
    }

    fn get_f64_generic(&self, m: &dyn Message) -> f64 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::F64(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0.0, // TODO: check type
        }
    }
}

// singular

fn set_panic<A, B>(_: &mut A, _: B) {
    panic!()
}

// TODO: make_singular_xxx_accessor are used only for oneof fields
// oneof codegen should be changed

pub fn make_singular_u32_accessor<M: Message + 'static>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: fn(&M) -> u32,
) -> FieldAccessor {
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(FieldAccessorImpl {
                fns: FieldAccessorFunctions::SingularHasGetSet {
                    has,
                    get_set: SingularGetSet::Copy(Box::new(GetSetCopyFnsImpl {
                        get,
                        _set: set_panic,
                    })),
                },
                runtime_type: RuntimeTypeU32::dynamic(),
            }),
        }),
    }
}

pub fn make_singular_i32_accessor<M: Message + 'static>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: fn(&M) -> i32,
) -> FieldAccessor {
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(FieldAccessorImpl {
                fns: FieldAccessorFunctions::SingularHasGetSet {
                    has,
                    get_set: SingularGetSet::Copy(Box::new(GetSetCopyFnsImpl {
                        get,
                        _set: set_panic,
                    })),
                },
                runtime_type: RuntimeTypeI32::dynamic(),
            }),
        }),
    }
}

pub fn make_singular_u64_accessor<M: Message + 'static>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: fn(&M) -> u64,
) -> FieldAccessor {
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(FieldAccessorImpl {
                fns: FieldAccessorFunctions::SingularHasGetSet {
                    has,
                    get_set: SingularGetSet::Copy(Box::new(GetSetCopyFnsImpl {
                        get,
                        _set: set_panic,
                    })),
                },
                runtime_type: RuntimeTypeU64::dynamic(),
            }),
        }),
    }
}

pub fn make_singular_i64_accessor<M: Message + 'static>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: fn(&M) -> i64,
) -> FieldAccessor {
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(FieldAccessorImpl {
                fns: FieldAccessorFunctions::SingularHasGetSet {
                    has,
                    get_set: SingularGetSet::Copy(Box::new(GetSetCopyFnsImpl {
                        get,
                        _set: set_panic,
                    })),
                },
                runtime_type: RuntimeTypeI64::dynamic(),
            }),
        }),
    }
}

pub fn make_singular_f32_accessor<M: Message + 'static>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: fn(&M) -> f32,
) -> FieldAccessor {
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(FieldAccessorImpl {
                fns: FieldAccessorFunctions::SingularHasGetSet {
                    has,
                    get_set: SingularGetSet::Copy(Box::new(GetSetCopyFnsImpl {
                        get,
                        _set: set_panic,
                    })),
                },
                runtime_type: RuntimeTypeF32::dynamic(),
            }),
        }),
    }
}

pub fn make_singular_f64_accessor<M: Message + 'static>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: fn(&M) -> f64,
) -> FieldAccessor {
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(FieldAccessorImpl {
                fns: FieldAccessorFunctions::SingularHasGetSet {
                    has,
                    get_set: SingularGetSet::Copy(Box::new(GetSetCopyFnsImpl {
                        get,
                        _set: set_panic,
                    })),
                },
                runtime_type: RuntimeTypeF64::dynamic(),
            }),
        }),
    }
}

pub fn make_singular_bool_accessor<M: Message + 'static>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: fn(&M) -> bool,
) -> FieldAccessor {
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(FieldAccessorImpl {
                fns: FieldAccessorFunctions::SingularHasGetSet {
                    has,
                    get_set: SingularGetSet::Copy(Box::new(GetSetCopyFnsImpl {
                        get,
                        _set: set_panic,
                    })),
                },
                runtime_type: RuntimeTypeBool::dynamic(),
            }),
        }),
    }
}

pub fn make_singular_enum_accessor<M: Message + 'static, E: ProtobufEnum + 'static>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: fn(&M) -> E,
) -> FieldAccessor {
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(FieldAccessorImpl {
                fns: FieldAccessorFunctions::SingularHasGetSet {
                    has,
                    get_set: SingularGetSet::Enum(Box::new(GetSingularEnumImpl { get: get })),
                },
                runtime_type: RuntimeTypeEnum::<E>::dynamic(),
            }),
        }),
    }
}

pub fn make_singular_string_accessor<M: Message + 'static>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: for<'a> fn(&'a M) -> &'a str,
) -> FieldAccessor {
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(FieldAccessorImpl {
                fns: FieldAccessorFunctions::SingularHasGetSet {
                    has,
                    get_set: SingularGetSet::String(get, set_panic),
                },
                runtime_type: RuntimeTypeString::dynamic(),
            }),
        }),
    }
}

pub fn make_singular_bytes_accessor<M: Message + 'static>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: for<'a> fn(&'a M) -> &'a [u8],
) -> FieldAccessor {
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(FieldAccessorImpl {
                fns: FieldAccessorFunctions::SingularHasGetSet {
                    has,
                    get_set: SingularGetSet::Bytes(get, set_panic),
                },
                runtime_type: RuntimeTypeVecU8::dynamic(),
            }),
        }),
    }
}

pub fn make_singular_message_accessor<
    M: Message + 'static,
    F: Message + Clone + Default + 'static,
>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: for<'a> fn(&'a M) -> &'a F,
) -> FieldAccessor {
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(FieldAccessorImpl {
                fns: FieldAccessorFunctions::SingularHasGetSet {
                    has,
                    get_set: SingularGetSet::Message(Box::new(GetSingularMessageImpl { get: get })),
                },
                runtime_type: RuntimeTypeMessage::<F>::dynamic(),
            }),
        }),
    }
}

impl<M, V> FieldAccessor2<M, dyn ReflectOptional> for MessageGetMut<M, Option<V>>
where
    M: Message + 'static,
    V: ProtobufValue + Clone + 'static,
{
    fn get_field<'a>(&self, m: &'a M) -> &'a dyn ReflectOptional {
        (self.get_field)(m) as &dyn ReflectOptional
    }

    fn mut_field<'a>(&self, m: &'a mut M) -> &'a mut dyn ReflectOptional {
        (self.mut_field)(m) as &mut dyn ReflectOptional
    }
}

pub fn make_option_accessor<M, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a Option<<V::RuntimeType as RuntimeType>::Value>,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut Option<<V::RuntimeType as RuntimeType>::Value>,
) -> FieldAccessor
where
    M: Message + 'static,
    V: ProtobufType + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(FieldAccessorImpl {
                fns: FieldAccessorFunctions::Optional(Box::new(MessageGetMut::<
                    M,
                    Option<<V::RuntimeType as RuntimeType>::Value>,
                > {
                    get_field,
                    mut_field,
                })),
                runtime_type: <V::RuntimeType as RuntimeType>::dynamic(),
            }),
        }),
    }
}

impl<M, V> FieldAccessor2<M, dyn ReflectOptional> for MessageGetMut<M, SingularField<V>>
where
    M: Message + 'static,
    V: ProtobufValue + Clone + 'static,
{
    fn get_field<'a>(&self, m: &'a M) -> &'a dyn ReflectOptional {
        (self.get_field)(m) as &dyn ReflectOptional
    }

    fn mut_field<'a>(&self, m: &'a mut M) -> &'a mut dyn ReflectOptional {
        (self.mut_field)(m) as &mut dyn ReflectOptional
    }
}

pub fn make_singular_field_accessor<M, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a SingularField<<V::RuntimeType as RuntimeType>::Value>,
    mut_field: for<'a> fn(
        &'a mut M,
    ) -> &'a mut SingularField<<V::RuntimeType as RuntimeType>::Value>,
) -> FieldAccessor
where
    M: Message + 'static,
    V: ProtobufType + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(FieldAccessorImpl {
                fns: FieldAccessorFunctions::Optional(Box::new(MessageGetMut::<
                    M,
                    SingularField<<V::RuntimeType as RuntimeType>::Value>,
                > {
                    get_field,
                    mut_field,
                })),
                runtime_type: <V::RuntimeType as RuntimeType>::dynamic(),
            }),
        }),
    }
}

impl<M, V> FieldAccessor2<M, dyn ReflectOptional> for MessageGetMut<M, SingularPtrField<V>>
where
    M: Message + 'static,
    V: ProtobufValue + Clone + 'static,
{
    fn get_field<'a>(&self, m: &'a M) -> &'a dyn ReflectOptional {
        (self.get_field)(m) as &dyn ReflectOptional
    }

    fn mut_field<'a>(&self, m: &'a mut M) -> &'a mut dyn ReflectOptional {
        (self.mut_field)(m) as &mut dyn ReflectOptional
    }
}

pub fn make_singular_ptr_field_accessor<M, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a SingularPtrField<<V::RuntimeType as RuntimeType>::Value>,
    mut_field: for<'a> fn(
        &'a mut M,
    ) -> &'a mut SingularPtrField<<V::RuntimeType as RuntimeType>::Value>,
) -> FieldAccessor
where
    M: Message + 'static,
    V: ProtobufType + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(FieldAccessorImpl {
                fns: FieldAccessorFunctions::Optional(Box::new(MessageGetMut::<
                    M,
                    SingularPtrField<<V::RuntimeType as RuntimeType>::Value>,
                > {
                    get_field,
                    mut_field,
                })),
                runtime_type: <V::RuntimeType as RuntimeType>::dynamic(),
            }),
        }),
    }
}

impl<M, V> FieldAccessor2<M, dyn ProtobufValue> for MessageGetMut<M, V>
where
    M: Message + 'static,
    V: ProtobufValue + Clone + 'static,
{
    fn get_field<'a>(&self, m: &'a M) -> &'a dyn ProtobufValue {
        (self.get_field)(m) as &dyn ProtobufValue
    }

    fn mut_field<'a>(&self, m: &'a mut M) -> &'a mut dyn ProtobufValue {
        (self.mut_field)(m) as &mut dyn ProtobufValue
    }
}

pub fn make_simple_field_accessor<M, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a <V::RuntimeType as RuntimeType>::Value,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut <V::RuntimeType as RuntimeType>::Value,
) -> FieldAccessor
where
    M: Message + 'static,
    V: ProtobufType + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(FieldAccessorImpl {
                fns: FieldAccessorFunctions::Simple(Box::new(MessageGetMut::<
                    M,
                    <V::RuntimeType as RuntimeType>::Value,
                > {
                    get_field,
                    mut_field,
                })),
                runtime_type: <V::RuntimeType as RuntimeType>::dynamic(),
            }),
        }),
    }
}

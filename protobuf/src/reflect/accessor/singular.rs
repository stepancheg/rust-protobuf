use std::fmt;
use std::mem;

use Message;
use reflect::EnumDescriptor;
use reflect::MessageDescriptor;
use reflect::EnumValueDescriptor;
use reflect::ReflectValueRef;
use reflect::ReflectValueBox;
use reflect::runtime_types::RuntimeType;
use reflect::types::ProtobufType;
use core::message_down_cast;
use reflect::accessor::FieldAccessor;
use reflect::accessor::AccessorKind;
use reflect::types::ProtobufTypeString;
use reflect::types::ProtobufTypeBytes;
use reflect::types::ProtobufTypeMessage;
use reflect::ProtobufValue;
use core::message_down_cast_mut;
use singular::OptionLike;
use reflect::runtime_types::RuntimeTypeWithDeref;


/// This trait should not be used directly, use `FieldDescriptor` instead
pub(crate) trait SingularFieldAccessor : 'static {
    /// Return enum descriptor for enum field, panics if field type is not enum.
    fn enum_descriptor(&self) -> &'static EnumDescriptor;
    /// Return message descriptor for message field, panics if field type is not message.
    fn message_descriptor(&self) -> &'static MessageDescriptor;

    fn has_field_generic(&self, m: &Message) -> bool;
    // TODO: should it return default value or panic on unset field?
    fn get_message_generic<'a>(&self, m: &'a Message) -> Option<&'a Message>;
    fn mut_message_generic<'a>(&self, m: &'a mut Message) -> &'a mut Message;
    fn get_enum_generic(&self, m: &Message) -> &'static EnumValueDescriptor;
    fn get_str_generic<'a>(&self, m: &'a Message) -> &'a str;
    fn get_bytes_generic<'a>(&self, m: &'a Message) -> &'a [u8];
    fn get_u32_generic(&self, m: &Message) -> u32;
    fn get_u64_generic(&self, m: &Message) -> u64;
    fn get_i32_generic(&self, m: &Message) -> i32;
    fn get_i64_generic(&self, m: &Message) -> i64;
    fn get_bool_generic(&self, m: &Message) -> bool;
    fn get_f32_generic(&self, m: &Message) -> f32;
    fn get_f64_generic(&self, m: &Message) -> f64;

    fn get_reflect<'a>(&self, m: &'a Message) -> Option<ReflectValueRef<'a>>;

    fn get_singular_field_or_default<'a>(&self, m: &'a Message) -> ReflectValueRef<'a>;
    fn set_singular_field(&self, m: &mut Message, value: ReflectValueBox);
}

trait GetMutSetSingularMessage<M> {
    fn get_message<'a>(&self, m: &'a M) -> &'a Message;
    fn mut_message<'a>(&self, m: &'a mut M) -> &'a mut Message;
    fn set_message(&self, m: &mut M, field: Box<Message>);
}

struct GetMutSetSingularMessageImpl<M, F> {
    get_field: for<'a> fn(&'a M) -> &'a F,
    set_field: fn(&mut M, F),
    mut_field: for<'a> fn(&'a mut M) -> &'a mut F,
}

impl<M : Message, F: Message + 'static> GetMutSetSingularMessage<M> for GetMutSetSingularMessageImpl<M, F> {
    fn get_message<'a>(&self, m: &'a M) -> &'a Message {
        (self.get_field)(m)
    }

    fn mut_message<'a>(&self, m: &'a mut M) -> &'a mut Message {
        (self.mut_field)(m)
    }

    fn set_message(&self, m: &mut M, mut field: Box<Message>) {
        let field = field.as_any_mut().downcast_mut().expect("wrong message type");
        (self.set_field)(m, mem::replace(field, F::new()));
    }
}


trait GetSingularEnum<M> {
    fn get_enum(&self, m: &M) -> &'static EnumValueDescriptor;
}

struct GetSetCopyFnsImpl<M, V : RuntimeType> {
    get: fn(&M) -> V::Value,
    set: fn(&mut M, V::Value),
}

enum SingularGetSet<M, V>
    where V : RuntimeType
{
    Copy(GetSetCopyFnsImpl<M, V>),
    String(for<'a> fn(&'a M) -> &'a str, fn(&mut M, String)),
    Bytes(for<'a> fn(&'a M) -> &'a [u8], fn(&mut M, Vec<u8>)),
    Message(Box<GetMutSetSingularMessage<M> + 'static>),
}

impl<M, V> SingularGetSet<M, V>
    where M : Message + 'static, V : RuntimeType
{
    fn get_ref<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        match self {
            &SingularGetSet::Copy(ref copy) => V::into_static_value_ref((copy.get)(m)),
            &SingularGetSet::String(get, _) => ReflectValueRef::String(get(m)),
            &SingularGetSet::Bytes(get, _) => ReflectValueRef::Bytes(get(m)),
            &SingularGetSet::Message(ref get) => ReflectValueRef::Message(get.get_message(m)),
        }
    }

    fn set_singular_field(&self, m: &mut M, value: ReflectValueBox) {
        match self {
            SingularGetSet::Copy(copy) => (copy.set)(m, V::from_value_box(value)),
            SingularGetSet::String(_, set) => {
                match value {
                    ReflectValueBox::String(s) => set(m, s),
                    _ => panic!("wrong type"),
                }
            }
            SingularGetSet::Bytes(_, set) => {
                match value {
                    ReflectValueBox::Bytes(b) => set(m, b),
                    _ => panic!("wrong type"),
                }
            }
            SingularGetSet::Message(ref fns) => {
                match value {
                    ReflectValueBox::Message(f) => fns.set_message(m, f),
                    _ => panic!("wrong type"),
                }
            }
        }
    }
}

trait GetMut<M, R : ?Sized>
    where
        M : Message + 'static,
{
    fn get_field<'a>(&self, message: &'a M) -> &'a R;
    fn mut_field<'a>(&self, message: &'a mut M) -> &'a mut R;
}

struct GetMutImpl<M, L>
    where
        M : Message + 'static,
{
    get_field: for<'a> fn(&'a M) -> &'a L,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut L,
}

impl<M, V, O> GetMut<M, OptionLike<V>> for GetMutImpl<M, O>
    where
        M : Message + 'static,
        V : ProtobufValue + Clone + 'static,
        O : OptionLike<V> + 'static,
{
    fn get_field<'a>(&self, m: &'a M) -> &'a (OptionLike<V> + 'static) {
        (self.get_field)(m) as &OptionLike<V>
    }

    fn mut_field<'a>(&self, m: &'a mut M) -> &'a mut (OptionLike<V> + 'static) {
        (self.mut_field)(m) as &mut OptionLike<V>
    }
}


impl<M, V> GetMut<M, ProtobufValue> for GetMutImpl<M, V>
    where
        M : Message + 'static,
        V : ProtobufValue + Clone + 'static,
{
    fn get_field<'a>(&self, m: &'a M) -> &'a ProtobufValue {
        (self.get_field)(m) as &ProtobufValue
    }

    fn mut_field<'a>(&self, m: &'a mut M) -> &'a mut ProtobufValue {
        (self.mut_field)(m) as &mut ProtobufValue
    }
}


trait GetOrDefault<M> {
    fn get_or_default<'a>(&self, m: &'a M) -> ReflectValueRef<'a>;
}

struct GetOrDefaultCopy<M, V>
    where
        M : Message,
        V : RuntimeType,
{
    get: fn(&M) -> V::Value,
}

struct GetOrDefaultRef<M, V>
    where
        M : Message,
        V : RuntimeTypeWithDeref,
{
    get: for<'a> fn(&'a M) -> &'a V::DerefTarget,
}

impl<M, V> GetOrDefault<M> for GetOrDefaultCopy<M, V>
    where
        M : Message,
        V : RuntimeType,
{
    fn get_or_default<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        V::into_static_value_ref((self.get)(m))
    }
}

impl<M, V> GetOrDefault<M> for GetOrDefaultRef<M, V>
    where
        M : Message,
        V : RuntimeTypeWithDeref,
{
    fn get_or_default<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        V::defef_as_ref((self.get)(m))
    }
}



enum FieldAccessorFunctions<M, V>
    where M : Message + 'static, V : ProtobufType
{
    // still used for optional fields
    SingularHasGetSet {
        has: fn(&M) -> bool,
        get_set: SingularGetSet<M, V::RuntimeType>,
    },
    // protobuf 3 simple field
    FieldPointer(GetMutImpl<M, <V::RuntimeType as RuntimeType>::Value>),
    // optional, required or message
    Optional(
        Box<GetMut<M, OptionLike<<V::RuntimeType as RuntimeType>::Value>> + 'static>,
        Option<Box<GetOrDefault<M>>>),
}

impl<M, V> fmt::Debug for FieldAccessorFunctions<M, V>
    where M : Message, V : ProtobufType
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &FieldAccessorFunctions::SingularHasGetSet { .. } =>
                write!(f, "SingularHasGetSet {{ .. }}"),
            &FieldAccessorFunctions::FieldPointer(..) =>
                write!(f, "FieldPointer(..)"),
            &FieldAccessorFunctions::Optional(..) =>
                write!(f, "Optional(..)"),
        }
    }
}


struct SingularFieldAccessorImpl<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    fns: FieldAccessorFunctions<M, V>,
}

impl<M, V> SingularFieldAccessorImpl<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    fn get_value_option<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        match self.fns {
            FieldAccessorFunctions::Optional(ref accessor, ..) => {
                accessor
                    .get_field(m)
                    .as_option_ref()
                    .map(V::RuntimeType::as_ref)
            }
            FieldAccessorFunctions::FieldPointer(ref accessor2) => {
                let v = V::RuntimeType::as_ref((accessor2.get_field)(m));
                if v.is_non_zero() {
                    Some(v)
                } else {
                    None
                }
            }
            FieldAccessorFunctions::SingularHasGetSet {
                ref has,
                ref get_set,
            } => {
                if has(m) {
                    Some(get_set.get_ref(m))
                } else {
                    None
                }
            }
        }
    }
}

impl<M, V> SingularFieldAccessor for SingularFieldAccessorImpl<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    fn enum_descriptor(&self) -> &'static EnumDescriptor {
        V::RuntimeType::enum_descriptor()
    }

    fn message_descriptor(&self) -> &'static MessageDescriptor {
        V::RuntimeType::message_descriptor()
    }

    fn has_field_generic(&self, m: &Message) -> bool {
        let m = message_down_cast(m);
        match self.fns {
            FieldAccessorFunctions::SingularHasGetSet { has, .. } => has(m),
            FieldAccessorFunctions::Optional(ref a, ..) => {
                a.get_field(m).as_option_ref().is_some()
            }
            FieldAccessorFunctions::FieldPointer(ref a) => {
                V::RuntimeType::as_ref((a.get_field)(m)).is_non_zero()
            }
        }
    }

    fn get_message_generic<'a>(&self, m: &'a Message) -> Option<&'a Message> {
        let m = message_down_cast(m);
        match self.fns {
            FieldAccessorFunctions::SingularHasGetSet {
                has,
                get_set: SingularGetSet::Message(ref get), ..
            } => {
                if has(m) {
                    Some(get.get_message(m))
                } else {
                    None
                }
            },
            FieldAccessorFunctions::Optional(ref t, ..) => {
                t.get_field(m).as_option_ref().map(V::RuntimeType::as_ref).map(|v| match v {
                    ReflectValueRef::Message(m) => m,
                    _ => panic!("not a message"),
                })
            }
            ref fns => panic!("unknown accessor type: {:?}", fns),
        }
    }

    fn mut_message_generic<'a>(&self, m: &'a mut Message) -> &'a mut Message {
        let _m: &mut M = message_down_cast_mut(m);
        unimplemented!()
    }

    fn get_enum_generic(&self, m: &Message) -> &'static EnumValueDescriptor {
        let m = message_down_cast(m);
        match self.fns {
            FieldAccessorFunctions::SingularHasGetSet {
                get_set: SingularGetSet::Copy(ref get), ..
            } => {
                match V::RuntimeType::into_value_box((get.get)(m)) {
                    ReflectValueBox::Enum(e) => e,
                    _ => panic!("not an enum"),
                }
            },
            _ => panic!(),
        }
    }

    fn get_str_generic<'a>(&self, m: &'a Message) -> &'a str {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::String(v)) => v,
            Some(_) => panic!("wrong type"),
            None => "", // TODO: check type
        }
    }

    fn get_bytes_generic<'a>(&self, m: &'a Message) -> &'a [u8] {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::Bytes(v)) => v,
            Some(_) => panic!("wrong type"),
            None => b"", // TODO: check type
        }
    }

    fn get_u32_generic(&self, m: &Message) -> u32 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::U32(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0, // TODO: check type
        }
    }

    fn get_u64_generic(&self, m: &Message) -> u64 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::U64(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0, // TODO: check type
        }
    }

    fn get_i32_generic(&self, m: &Message) -> i32 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::I32(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0, // TODO: check type
        }
    }

    fn get_i64_generic(&self, m: &Message) -> i64 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::I64(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0, // TODO: check type
        }
    }

    fn get_bool_generic(&self, m: &Message) -> bool {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::Bool(v)) => v,
            Some(_) => panic!("wrong type"),
            None => false, // TODO: check type
        }
    }

    fn get_f32_generic(&self, m: &Message) -> f32 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::F32(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0.0, // TODO: check type
        }
    }

    fn get_f64_generic(&self, m: &Message) -> f64 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ReflectValueRef::F64(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0.0, // TODO: check type
        }
    }

    fn get_reflect<'a>(&self, m: &'a Message) -> Option<ReflectValueRef<'a>> {
        let m = message_down_cast(m);
        self.get_value_option(m)
    }

    fn get_singular_field_or_default<'a>(&self, m: &'a Message) -> ReflectValueRef<'a> {
        let m: &M = message_down_cast(m);
        match self.fns {
            FieldAccessorFunctions::SingularHasGetSet { ref get_set, .. } => {
                get_set.get_ref(m)
            }
            FieldAccessorFunctions::FieldPointer(ref fns) => {
                V::RuntimeType::as_ref((fns.get_field)(m))
            }
            FieldAccessorFunctions::Optional(_, Some(ref get_or_default)) => {
                get_or_default.get_or_default(m)
            }
            FieldAccessorFunctions::Optional(ref fns, None) => {
                match fns.get_field(m).as_option_ref() {
                    Some(v) => V::RuntimeType::as_ref(v),
                    None => V::RuntimeType::default_value_ref(),
                }
            }
        }
    }

    fn set_singular_field(&self, m: &mut Message, value: ReflectValueBox) {
        let m: &mut M = m.as_any_mut().downcast_mut().expect("wrong_type");
        match self.fns {
            FieldAccessorFunctions::SingularHasGetSet { ref get_set, .. } => {
                get_set.set_singular_field(m, value)
            }
            FieldAccessorFunctions::FieldPointer(ref fns) => {
                V::RuntimeType::set_from_value_box((fns.mut_field)(m), value);
            }
            FieldAccessorFunctions::Optional(ref fns, ..) => {
                fns.mut_field(m).set_value(V::RuntimeType::from_value_box(value));
            }
        }
    }
}

// TODO: make_singular_xxx_accessor are used only for oneof fields
// oneof codegen should be changed

pub fn make_singular_copy_has_get_set_accessor<M, V>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: fn(&M) -> <V::RuntimeType as RuntimeType>::Value,
    set: fn(&mut M, <V::RuntimeType as RuntimeType>::Value),
) -> FieldAccessor
    where
        M : Message + 'static,
        V : ProtobufType + 'static,
        <V::RuntimeType as RuntimeType>::Value : Copy,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(Box::new(SingularFieldAccessorImpl::<M, V> {
            fns: FieldAccessorFunctions::SingularHasGetSet {
                has,
                get_set: SingularGetSet::Copy(GetSetCopyFnsImpl { get, set }),
            },
        }))
    }
}

pub fn make_singular_string_has_get_set_accessor<M : Message + 'static>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: for<'a> fn(&'a M) -> &'a str,
    set: fn(&mut M, String),
) -> FieldAccessor {
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(Box::new(SingularFieldAccessorImpl::<M, ProtobufTypeString> {
            fns: FieldAccessorFunctions::SingularHasGetSet {
                has: has,
                get_set: SingularGetSet::String(get, set),
            },
        }))
    }
}

pub fn make_singular_bytes_has_get_set_accessor<M : Message + 'static>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: for<'a> fn(&'a M) -> &'a [u8],
    set: fn(&mut M, Vec<u8>),
) -> FieldAccessor {
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(Box::new(SingularFieldAccessorImpl::<M, ProtobufTypeBytes> {
            fns: FieldAccessorFunctions::SingularHasGetSet {
                has,
                get_set: SingularGetSet::Bytes(get, set),
            },
        }))
    }
}

pub fn make_singular_message_has_get_mut_set_accessor<M, F>(
    name: &'static str,
    has_field: fn(&M) -> bool,
    get_field: for<'a> fn(&'a M) -> &'a F,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut F,
    set_field: fn(&mut M, F),
) -> FieldAccessor
    where M : Message + 'static, F : Message + Clone + 'static
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(Box::new(SingularFieldAccessorImpl::<M, ProtobufTypeMessage<F>> {
            fns: FieldAccessorFunctions::SingularHasGetSet {
                has: has_field,
                get_set: SingularGetSet::Message(Box::new(GetMutSetSingularMessageImpl {
                    get_field,
                    set_field,
                    mut_field,
                })),
            },
        }))
    }
}

pub fn make_option_accessor<M, V, O>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a O,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut O,
) -> FieldAccessor
    where
        M : Message + 'static,
        V : ProtobufType + 'static,
        O : OptionLike<<V::RuntimeType as RuntimeType>::Value> + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(Box::new(SingularFieldAccessorImpl::<M, V> {
            fns: FieldAccessorFunctions::Optional(Box::new(GetMutImpl::<M, O> {
                get_field,
                mut_field,
            }), None),
        }))
    }
}

pub fn make_option_get_copy_accessor<M, V, O>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a O,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut O,
    get_value: fn(&M) -> <V::RuntimeType as RuntimeType>::Value,
) -> FieldAccessor
    where
        M : Message + 'static,
        V : ProtobufType + 'static,
        O : OptionLike<<V::RuntimeType as RuntimeType>::Value> + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(Box::new(SingularFieldAccessorImpl::<M, V> {
            fns: FieldAccessorFunctions::Optional(
                Box::new(GetMutImpl::<M, O> {
                    get_field,
                    mut_field,
                }),
                Some(Box::new(GetOrDefaultCopy::<M, V::RuntimeType> { get: get_value }))),
        }))
    }
}

pub fn make_option_get_ref_accessor<M, V, O>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a O,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut O,
    get_value: for<'a> fn(&'a M) -> &'a <V::RuntimeType as RuntimeTypeWithDeref>::DerefTarget,
) -> FieldAccessor
    where
        M : Message + 'static,
        V : ProtobufType + 'static,
        V::RuntimeType : RuntimeTypeWithDeref,
        O : OptionLike<<V::RuntimeType as RuntimeType>::Value> + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(Box::new(SingularFieldAccessorImpl::<M, V> {
            fns: FieldAccessorFunctions::Optional(
                Box::new(GetMutImpl::<M, O> {
                    get_field,
                    mut_field,
                }),
                Some(Box::new(GetOrDefaultRef::<M, V::RuntimeType> { get: get_value }))),
        }))
    }
}

pub fn make_simple_field_accessor<M, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a <V::RuntimeType as RuntimeType>::Value,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut <V::RuntimeType as RuntimeType>::Value,
) -> FieldAccessor
    where
        M : Message + 'static,
        V : ProtobufType + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(Box::new(SingularFieldAccessorImpl::<M, V> {
            fns: FieldAccessorFunctions::FieldPointer(GetMutImpl {
                get_field,
                mut_field,
            }),
        }))
    }
}

use std::hash::Hash;
use std::collections::HashMap;

use core::Message;
use core::ProtobufEnum;
use core::message_down_cast;
use reflect::EnumValueDescriptor;
use types::*;

use repeated::RepeatedField;
use singular::SingularField;
use singular::SingularPtrField;

use super::map::ReflectMap;
use super::repeated::ReflectRepeated;
use super::repeated::ReflectRepeatedRef;
use super::repeated::ReflectRepeatedEnum;
use super::repeated::ReflectRepeatedEnumImpl;
use super::repeated::ReflectRepeatedMessage;
use super::repeated::ReflectRepeatedMessageImpl;
use super::optional::ReflectOptional;
use super::value::ProtobufValue;
use super::value::ProtobufValueRef;
use super::ReflectFieldRef;


/// this trait should not be used directly, use `FieldDescriptor` instead
pub trait FieldAccessor {
    fn name_generic(&self) -> &'static str;
    fn has_field_generic(&self, m: &Message) -> bool;
    fn len_field_generic(&self, m: &Message) -> usize;
    fn get_message_generic<'a>(&self, m: &'a Message) -> &'a Message;
    fn get_rep_message_item_generic<'a>(&self, m: &'a Message, index: usize) -> &'a Message;
    fn get_enum_generic(&self, m: &Message) -> &'static EnumValueDescriptor;
    fn get_rep_enum_item_generic(&self, m: &Message, index: usize) -> &'static EnumValueDescriptor;
    fn get_str_generic<'a>(&self, m: &'a Message) -> &'a str;
    fn get_rep_str_generic<'a>(&self, m: &'a Message) -> &'a [String];
    fn get_bytes_generic<'a>(&self, m: &'a Message) -> &'a [u8];
    fn get_rep_bytes_generic<'a>(&self, m: &'a Message) -> &'a [Vec<u8>];
    fn get_u32_generic(&self, m: &Message) -> u32;
    fn get_rep_u32_generic<'a>(&self, m: &'a Message) -> &'a [u32];
    fn get_u64_generic(&self, m: &Message) -> u64;
    fn get_rep_u64_generic<'a>(&self, m: &'a Message) -> &'a [u64];
    fn get_i32_generic(&self, m: &Message) -> i32;
    fn get_rep_i32_generic<'a>(&self, m: &'a Message) -> &'a [i32];
    fn get_i64_generic(&self, m: &Message) -> i64;
    fn get_rep_i64_generic<'a>(&self, m: &'a Message) -> &'a [i64];
    fn get_bool_generic(&self, m: &Message) -> bool;
    fn get_rep_bool_generic<'a>(&self, m: &'a Message) -> &'a [bool];
    fn get_f32_generic(&self, m: &Message) -> f32;
    fn get_rep_f32_generic<'a>(&self, m: &'a Message) -> &'a [f32];
    fn get_f64_generic(&self, m: &Message) -> f64;
    fn get_rep_f64_generic<'a>(&self, m: &'a Message) -> &'a [f64];

    fn get_reflect<'a>(&self, m: &'a Message) -> ReflectFieldRef<'a>;
}


trait GetSingularMessage<M> {
    fn get_message<'a>(&self, m: &'a M) -> &'a Message;
}

struct GetSingularMessageImpl<M, N> {
    get: for<'a> fn(&'a M) -> &'a N,
}

impl<M : Message, N : Message + 'static> GetSingularMessage<M> for GetSingularMessageImpl<M, N> {
    fn get_message<'a>(&self, m: &'a M) -> &'a Message {
        (self.get)(m)
    }
}


trait GetSingularEnum<M> {
    fn get_enum(&self, m: &M) -> &'static EnumValueDescriptor;
}

struct GetSingularEnumImpl<M, E> {
    get: fn(&M) -> E,
}

impl<M : Message, E : ProtobufEnum> GetSingularEnum<M> for GetSingularEnumImpl<M, E> {
    fn get_enum(&self, m: &M) -> &'static EnumValueDescriptor {
        (self.get)(m).descriptor()
    }
}


trait GetRepeatedMessage<M> {
    fn len_field(&self, m: &M) -> usize;
    fn get_message_item<'a>(&self, m: &'a M, index: usize) -> &'a Message;
    fn reflect_repeated_message<'a>(&self, m: &'a M) -> Box<ReflectRepeatedMessage<'a> + 'a>;
}

struct GetRepeatedMessageImpl<M, N> {
    get: for<'a> fn(&'a M) -> &'a [N],
}

impl<M : Message, N : Message + 'static> GetRepeatedMessage<M> for GetRepeatedMessageImpl<M, N> {
    fn len_field(&self, m: &M) -> usize {
        (self.get)(m).len()
    }

    fn get_message_item<'a>(&self, m: &'a M, index: usize) -> &'a Message {
        &(self.get)(m)[index]
    }

    fn reflect_repeated_message<'a>(&self, m: &'a M) -> Box<ReflectRepeatedMessage<'a> + 'a> {
        Box::new(ReflectRepeatedMessageImpl {
            slice: (self.get)(m)
        })
    }
}


trait GetRepeatedEnum<M : Message + 'static> {
    fn len_field(&self, m: &M) -> usize;
    fn get_enum_item(&self, m: &M, index: usize) -> &'static EnumValueDescriptor;
    fn reflect_repeated_enum<'a>(&self, m: &'a M) -> Box<ReflectRepeatedEnum<'a> + 'a>;
}

struct GetRepeatedEnumImpl<M : Message + 'static, E : ProtobufEnum + 'static> {
    get: for<'a> fn(&'a M) -> &'a [E],
}

impl<M : Message, E : ProtobufEnum + 'static> GetRepeatedEnum<M> for GetRepeatedEnumImpl<M, E> {
    fn len_field(&self, m: &M) -> usize {
        (self.get)(m).len()
    }

    fn get_enum_item(&self, m: &M, index: usize) -> &'static EnumValueDescriptor {
        (self.get)(m)[index].descriptor()
    }

    fn reflect_repeated_enum<'a>(&self, m: &'a M) -> Box<ReflectRepeatedEnum<'a> + 'a> {
        Box::new(ReflectRepeatedEnumImpl {
            slice: (self.get)(m)
        })
    }
}




trait GetSetCopyFns<M> {
    fn get_field<'a>(&self, m: &'a M) -> ProtobufValueRef<'a>;
}

struct GetSetCopyFnsImpl<M, V : ProtobufValue + Copy> {
    get: fn(&M) -> V,
    _set: fn(&mut M, V),
}

impl<M, V : ProtobufValue + Copy> GetSetCopyFns<M> for GetSetCopyFnsImpl<M, V> {
    fn get_field<'a>(&self, m: &'a M) -> ProtobufValueRef<'a> {
        (&(self.get)(m) as &ProtobufValue).as_ref_copy()
    }
}


enum SingularGetSet<M> {
    Copy(Box<GetSetCopyFns<M>>),
    String(for<'a> fn(&'a M) -> &'a str, fn(&mut M, String)),
    Bytes(for<'a> fn(&'a M) -> &'a [u8], fn(&mut M, Vec<u8>)),
    Enum(Box<GetSingularEnum<M> + 'static>),
    Message(Box<GetSingularMessage<M> + 'static>),
}

impl<M : Message + 'static> SingularGetSet<M> {
    fn get_ref<'a>(&self, m: &'a M) -> ProtobufValueRef<'a> {
        match self {
            &SingularGetSet::Copy(ref copy) => copy.get_field(m),
            &SingularGetSet::String(get, _) => ProtobufValueRef::String(get(m)),
            &SingularGetSet::Bytes(get, _) => ProtobufValueRef::Bytes(get(m)),
            &SingularGetSet::Enum(ref get) => ProtobufValueRef::Enum(get.get_enum(m)),
            &SingularGetSet::Message(ref get) => ProtobufValueRef::Message(get.get_message(m)),
        }
    }
}

// for rust-protobuf up to 1.0.24
enum RepeatedOldGet<M> {
    U32(for<'a> fn(&'a M) -> &'a [u32]),
    U64(for<'a> fn(&'a M) -> &'a [u64]),
    I32(for<'a> fn(&'a M) -> &'a [i32]),
    I64(for<'a> fn(&'a M) -> &'a [i64]),
    F32(for<'a> fn(&'a M) -> &'a [f32]),
    F64(for<'a> fn(&'a M) -> &'a [f64]),
    Bool(for<'a> fn(&'a M) -> &'a [bool]),
    String(for<'a> fn(&'a M) -> &'a [String]),
    Bytes(for<'a> fn(&'a M) -> &'a [Vec<u8>]),
    Enum(Box<GetRepeatedEnum<M> + 'static>),
    Message(Box<GetRepeatedMessage<M> + 'static>),
}

impl<M : Message> RepeatedOldGet<M> {
    fn len_field(&self, m: &M) -> usize {
        match *self {
            RepeatedOldGet::U32(get) => get(m).len(),
            RepeatedOldGet::U64(get) => get(m).len(),
            RepeatedOldGet::I32(get) => get(m).len(),
            RepeatedOldGet::I64(get) => get(m).len(),
            RepeatedOldGet::F32(get) => get(m).len(),
            RepeatedOldGet::F64(get) => get(m).len(),
            RepeatedOldGet::Bool(get) => get(m).len(),
            RepeatedOldGet::String(get) => get(m).len(),
            RepeatedOldGet::Bytes(get) => get(m).len(),
            RepeatedOldGet::Enum(ref get) => get.len_field(m),
            RepeatedOldGet::Message(ref get) => get.len_field(m),
        }
    }

    fn get_repeated<'a>(&self, m: &'a M) -> ReflectRepeatedRef<'a> {
        match *self {
            RepeatedOldGet::U32(get) => ReflectRepeatedRef::U32(get(m)),
            RepeatedOldGet::U64(get) => ReflectRepeatedRef::U64(get(m)),
            RepeatedOldGet::I32(get) => ReflectRepeatedRef::I32(get(m)),
            RepeatedOldGet::I64(get) => ReflectRepeatedRef::I64(get(m)),
            RepeatedOldGet::F32(get) => ReflectRepeatedRef::F32(get(m)),
            RepeatedOldGet::F64(get) => ReflectRepeatedRef::F64(get(m)),
            RepeatedOldGet::Bool(get) => ReflectRepeatedRef::Bool(get(m)),
            RepeatedOldGet::String(get) => ReflectRepeatedRef::String(get(m)),
            RepeatedOldGet::Bytes(get) => ReflectRepeatedRef::Bytes(get(m)),
            RepeatedOldGet::Enum(ref get) => ReflectRepeatedRef::Enum(get.reflect_repeated_enum(m)),
            RepeatedOldGet::Message(ref get) => ReflectRepeatedRef::Message(get.reflect_repeated_message(m)),
        }
    }
}

trait FieldAccessor2<M, R : ?Sized>
    where
        M : Message + 'static,
{
    fn get_field<'a>(&self, &'a M) -> &'a R;
    fn mut_field<'a>(&self, &'a mut M) -> &'a mut R;
}

struct MessageGetMut<M, L>
    where
        M : Message + 'static,
{
    get_field: for<'a> fn(&'a M) -> &'a L,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut L,
}


enum FieldAccessorFunctions<M> {
    // up to 1.0.24 optional or required
    SingularHasGetSet { has: fn(&M) -> bool, get_set: SingularGetSet<M> },
    // up to 1.0.24 repeated
    RepeatedOld(RepeatedOldGet<M>),
    // protobuf 3 simple field
    Simple(Box<FieldAccessor2<M, ProtobufValue>>),
    // optional, required or message
    Optional(Box<FieldAccessor2<M, ReflectOptional>>),
    // repeated
    Repeated(Box<FieldAccessor2<M, ReflectRepeated>>),
    // protobuf 3 map
    Map(Box<FieldAccessor2<M, ReflectMap>>),
}


struct FieldAccessorImpl<M> {
    name: &'static str,
    fns: FieldAccessorFunctions<M>,
}

impl<M : Message> FieldAccessorImpl<M> {
    fn get_value_option<'a>(&self, m: &'a M) -> Option<ProtobufValueRef<'a>> {
        match self.fns {
            FieldAccessorFunctions::Repeated(..) |
            FieldAccessorFunctions::RepeatedOld(..) |
            FieldAccessorFunctions::Map(..) => panic!("repeated"),
            FieldAccessorFunctions::Simple(ref a) => {
                Some(a.get_field(m).as_ref())
            },
            FieldAccessorFunctions::Optional(ref a) => {
                a.get_field(m).to_option().map(|v| v.as_ref())
            },
            FieldAccessorFunctions::SingularHasGetSet { ref has, ref get_set } => {
                if !has(m) {
                    None
                } else {
                    Some(get_set.get_ref(m))
                }
            }
        }
    }
}

impl<M : Message + 'static> FieldAccessor for FieldAccessorImpl<M> {
    fn name_generic(&self) -> &'static str {
        self.name
    }

    fn has_field_generic(&self, m: &Message) -> bool {
        match self.fns {
            FieldAccessorFunctions::SingularHasGetSet { has, .. } => has(message_down_cast(m)),
            FieldAccessorFunctions::Optional(ref a) => a.get_field(message_down_cast(m)).to_option().is_some(),
            FieldAccessorFunctions::Simple(ref a) => a.get_field(message_down_cast(m)).is_non_zero(),
            _ => panic!(),
        }
    }

    fn len_field_generic(&self, m: &Message) -> usize {
        match self.fns {
            FieldAccessorFunctions::RepeatedOld(ref r) => r.len_field(message_down_cast(m)),
            FieldAccessorFunctions::Repeated(ref a) => a.get_field(message_down_cast(m)).len(),
            FieldAccessorFunctions::Map(ref a) => a.get_field(message_down_cast(m)).len(),
            _ => panic!("not repeated"),
        }
    }

    fn get_message_generic<'a>(&self, m: &'a Message) -> &'a Message {
        match self.fns {
            FieldAccessorFunctions::SingularHasGetSet { get_set: SingularGetSet::Message(ref get), .. } => {
                get.get_message(message_down_cast(m))
            }
            _ => panic!(),
        }
    }

    fn get_enum_generic(&self, m: &Message) -> &'static EnumValueDescriptor {
        match self.fns {
            FieldAccessorFunctions::SingularHasGetSet { get_set: SingularGetSet::Enum(ref get), .. } =>
                get.get_enum(message_down_cast(m)),
            _ => panic!(),
        }
    }

    fn get_str_generic<'a>(&self, m: &'a Message) -> &'a str {
        match self.get_value_option(message_down_cast(m)) {
            Some(ProtobufValueRef::String(v)) => v,
            Some(_) => panic!("wrong type"),
            None => "", // TODO: check type
        }
    }

    fn get_bytes_generic<'a>(&self, m: &'a Message) -> &'a [u8] {
        match self.get_value_option(message_down_cast(m)) {
            Some(ProtobufValueRef::Bytes(v)) => v,
            Some(_) => panic!("wrong type"),
            None => b"", // TODO: check type
        }
    }

    fn get_u32_generic(&self, m: &Message) -> u32 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ProtobufValueRef::U32(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0, // TODO: check type
        }
    }

    fn get_u64_generic(&self, m: &Message) -> u64 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ProtobufValueRef::U64(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0, // TODO: check type
        }
    }

    fn get_i32_generic(&self, m: &Message) -> i32 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ProtobufValueRef::I32(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0, // TODO: check type
        }
    }

    fn get_i64_generic(&self, m: &Message) -> i64 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ProtobufValueRef::I64(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0, // TODO: check type
        }
    }

    fn get_f32_generic(&self, m: &Message) -> f32 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ProtobufValueRef::F32(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0.0, // TODO: check type
        }
    }

    fn get_f64_generic(&self, m: &Message) -> f64 {
        match self.get_value_option(message_down_cast(m)) {
            Some(ProtobufValueRef::F64(v)) => v,
            Some(_) => panic!("wrong type"),
            None => 0.0, // TODO: check type
        }
    }

    fn get_bool_generic(&self, m: &Message) -> bool {
        match self.get_value_option(message_down_cast(m)) {
            Some(ProtobufValueRef::Bool(v)) => v,
            Some(_) => panic!("wrong type"),
            None => false, // TODO: check type
        }
    }

    fn get_rep_message_item_generic<'a>(&self, m: &'a Message, index: usize) -> &'a Message {
        match self.fns {
            FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::Message(ref get)) =>
                get.get_message_item(message_down_cast(m), index),
            _ => panic!(),
        }
    }

    fn get_rep_enum_item_generic(&self, m: &Message, index: usize) -> &'static EnumValueDescriptor {
        match self.fns {
            FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::Enum(ref get)) =>
                get.get_enum_item(message_down_cast(m), index),
            _ => panic!(),
        }
    }

    fn get_rep_str_generic<'a>(&self, m: &'a Message) -> &'a [String] {
        match self.fns {
            FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::String(get)) => get(message_down_cast(m)),
            _ => panic!(),
        }
    }

    fn get_rep_bytes_generic<'a>(&self, m: &'a Message) -> &'a [Vec<u8>] {
        match self.fns {
            FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::Bytes(get)) => get(message_down_cast(m)),
            _ => panic!(),
        }
    }

    fn get_rep_u32_generic<'a>(&self, m: &'a Message) -> &'a [u32] {
        match self.fns {
            FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::U32(get)) => get(message_down_cast(m)),
            _ => panic!(),
        }
    }

    fn get_rep_u64_generic<'a>(&self, m: &'a Message) -> &'a [u64] {
        match self.fns {
            FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::U64(get)) => get(message_down_cast(m)),
            _ => panic!(),
        }
    }

    fn get_rep_i32_generic<'a>(&self, m: &'a Message) -> &'a [i32] {
        match self.fns {
            FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::I32(get)) => get(message_down_cast(m)),
            _ => panic!(),
        }
    }

    fn get_rep_i64_generic<'a>(&self, m: &'a Message) -> &'a [i64] {
        match self.fns {
            FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::I64(get)) => get(message_down_cast(m)),
            _ => panic!(),
        }
    }

    fn get_rep_f32_generic<'a>(&self, m: &'a Message) -> &'a [f32] {
        match self.fns {
            FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::F32(get)) => get(message_down_cast(m)),
            _ => panic!(),
        }
    }

    fn get_rep_f64_generic<'a>(&self, m: &'a Message) -> &'a [f64] {
        match self.fns {
            FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::F64(get)) => get(message_down_cast(m)),
            _ => panic!(),
        }
    }

    fn get_rep_bool_generic<'a>(&self, m: &'a Message) -> &'a [bool] {
        match self.fns {
            FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::Bool(get)) => get(message_down_cast(m)),
            _ => panic!(),
        }
    }

    fn get_reflect<'a>(&self, m: &'a Message) -> ReflectFieldRef<'a> {
        match self.fns {
            FieldAccessorFunctions::Repeated(ref accessor2) => {
                ReflectFieldRef::Repeated(accessor2.get_field(message_down_cast(m)))
            }
            FieldAccessorFunctions::Map(ref accessor2) => {
                ReflectFieldRef::Map(accessor2.get_field(message_down_cast(m)))
            }
            FieldAccessorFunctions::Optional(ref accessor2) => {
                ReflectFieldRef::Optional(accessor2.get_field(message_down_cast(m)).to_option().map(|v| v.as_ref()))
            }
            FieldAccessorFunctions::Simple(ref accessor2) => {
                ReflectFieldRef::Optional({
                    let v = accessor2.get_field(message_down_cast(m));
                    if v.is_non_zero() {
                        Some(v.as_ref())
                    } else {
                        None
                    }
                })
            }
            FieldAccessorFunctions::SingularHasGetSet { ref has, ref get_set } => {
                ReflectFieldRef::Optional(
                    if has(message_down_cast(m)) {
                        Some(get_set.get_ref(message_down_cast(m)))
                    } else {
                        None
                    }
                )
            }
            FieldAccessorFunctions::RepeatedOld(ref get) => {
                ReflectFieldRef::RepeatedOld(get.get_repeated(message_down_cast(m)))
            }
        }
    }
}


// singular

fn set_panic<A, B>(_: &mut A, _: B) {
    panic!()
}

pub fn make_singular_u32_accessor<M : Message + 'static>(
        name: &'static str,
        has: fn(&M) -> bool,
        get: fn(&M) -> u32,
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::SingularHasGetSet {
            has: has,
            get_set: SingularGetSet::Copy(Box::new(GetSetCopyFnsImpl {
                get: get,
                _set: set_panic,
            })),
        },
    })
}

pub fn make_singular_i32_accessor<M : Message + 'static>(
        name: &'static str,
        has: fn(&M) -> bool,
        get: fn(&M) -> i32,
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::SingularHasGetSet {
            has: has,
            get_set: SingularGetSet::Copy(Box::new(GetSetCopyFnsImpl {
                get: get,
                _set: set_panic,
            })),
        },
    })
}

pub fn make_singular_u64_accessor<M : Message + 'static>(
        name: &'static str,
        has: fn(&M) -> bool,
        get: fn(&M) -> u64,
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::SingularHasGetSet {
            has: has,
            get_set: SingularGetSet::Copy(Box::new(GetSetCopyFnsImpl {
                get: get,
                _set: set_panic,
            })),
        },
    })
}

pub fn make_singular_i64_accessor<M : Message + 'static>(
        name: &'static str,
        has: fn(&M) -> bool,
        get: fn(&M) -> i64,
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::SingularHasGetSet {
            has: has,
            get_set: SingularGetSet::Copy(Box::new(GetSetCopyFnsImpl {
                get: get,
                _set: set_panic,
            })),
        },
    })
}

pub fn make_singular_f32_accessor<M : Message + 'static>(
        name: &'static str,
        has: fn(&M) -> bool,
        get: fn(&M) -> f32,
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::SingularHasGetSet {
            has: has,
            get_set: SingularGetSet::Copy(Box::new(GetSetCopyFnsImpl {
                get: get,
                _set: set_panic,
            })),
        },
    })
}

pub fn make_singular_f64_accessor<M : Message + 'static>(
        name: &'static str,
        has: fn(&M) -> bool,
        get: fn(&M) -> f64,
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::SingularHasGetSet {
            has: has,
            get_set: SingularGetSet::Copy(Box::new(GetSetCopyFnsImpl {
                get: get,
                _set: set_panic,
            })),
        },
    })
}

pub fn make_singular_bool_accessor<M : Message + 'static>(
        name: &'static str,
        has: fn(&M) -> bool,
        get: fn(&M) -> bool,
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::SingularHasGetSet {
            has: has,
            get_set: SingularGetSet::Copy(Box::new(GetSetCopyFnsImpl {
                get: get,
                _set: set_panic,
            })),
        },
    })
}

pub fn make_singular_enum_accessor<M : Message + 'static, E : ProtobufEnum + 'static>(
        name: &'static str,
        has: fn(&M) -> bool,
        get: fn(&M) -> E,
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::SingularHasGetSet {
            has: has,
            get_set: SingularGetSet::Enum(
                Box::new(GetSingularEnumImpl { get: get }),
            ),
        },
    })
}

pub fn make_singular_string_accessor<M : Message + 'static>(
        name: &'static str,
        has: fn(&M) -> bool,
        get: for<'a> fn(&'a M) -> &'a str,
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::SingularHasGetSet {
            has: has,
            get_set: SingularGetSet::String(get, set_panic),
        },
    })
}

pub fn make_singular_bytes_accessor<M : Message + 'static>(
        name: &'static str,
        has: fn(&M) -> bool,
        get: for<'a> fn(&'a M) -> &'a [u8],
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::SingularHasGetSet {
            has: has,
            get_set: SingularGetSet::Bytes(get, set_panic),
        },
    })
}

pub fn make_singular_message_accessor<M : Message + 'static, F : Message + 'static>(
        name: &'static str,
        has: fn(&M) -> bool,
        get: for<'a> fn(&'a M) -> &'a F,
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::SingularHasGetSet {
            has: has,
            get_set: SingularGetSet::Message(
                Box::new(GetSingularMessageImpl { get: get }),
            ),
        },
    })
}

// repeated

#[deprecated]
pub fn make_repeated_u32_accessor<M : Message + 'static>(
        name: &'static str,
        get: for<'a> fn(&'a M) -> &'a [u32],
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::U32(get)),
    })
}

#[deprecated]
pub fn make_repeated_i32_accessor<M : Message + 'static>(
        name: &'static str,
        get: for<'a> fn(&'a M) -> &'a [i32],
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::I32(get)),
    })
}

#[deprecated]
pub fn make_repeated_u64_accessor<M : Message + 'static>(
        name: &'static str,
        get: for<'a> fn(&'a M) -> &'a [u64],
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::U64(get)),
    })
}

#[deprecated]
pub fn make_repeated_i64_accessor<M : Message + 'static>(
        name: &'static str,
        get: for<'a> fn(&'a M) -> &'a [i64],
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::I64(get)),
    })
}

#[deprecated]
pub fn make_repeated_f32_accessor<M : Message + 'static>(
        name: &'static str,
        get: for<'a> fn(&'a M) -> &'a [f32],
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::F32(get)),
    })
}

#[deprecated]
pub fn make_repeated_f64_accessor<M : Message + 'static>(
        name: &'static str,
        get: for<'a> fn(&'a M) -> &'a [f64],
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::F64(get)),
    })
}

#[deprecated]
pub fn make_repeated_bool_accessor<M : Message + 'static>(
        name: &'static str,
        get: for<'a> fn(&'a M) -> &'a [bool],
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::Bool(get)),
    })
}

#[deprecated]
pub fn make_repeated_string_accessor<M : Message + 'static>(
        name: &'static str,
        get: for<'a> fn(&'a M) -> &'a [String],
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::String(get)),
    })
}

#[deprecated]
pub fn make_repeated_bytes_accessor<M : Message + 'static>(
        name: &'static str,
        get: for<'a> fn(&'a M) -> &'a [Vec<u8>],
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::Bytes(get)),
    })
}

#[deprecated]
pub fn make_repeated_enum_accessor<M : Message + 'static, E : ProtobufEnum + 'static>(
        name: &'static str,
        get: for<'a> fn(&'a M) -> &'a [E],
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::Enum(
            Box::new(GetRepeatedEnumImpl { get: get }),
        )),
    })
}

#[deprecated]
pub fn make_repeated_message_accessor<M : Message + 'static, F : Message + 'static>(
        name: &'static str,
        get: for<'a> fn(&'a M) -> &'a [F],
    ) -> Box<FieldAccessor + 'static>
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::RepeatedOld(RepeatedOldGet::Message(
            Box::new(GetRepeatedMessageImpl { get: get }),
        )),
    })
}



impl<M, V> FieldAccessor2<M, ReflectRepeated> for MessageGetMut<M, Vec<V>>
    where
        M : Message + 'static,
        V : ProtobufValue + 'static,
{
    fn get_field<'a>(&self, m: &'a M) -> &'a ReflectRepeated {
        (self.get_field)(m) as &ReflectRepeated
    }

    fn mut_field<'a>(&self, m: &'a mut M) -> &'a mut ReflectRepeated {
        (self.mut_field)(m) as &mut ReflectRepeated
    }
}



pub fn make_vec_accessor<M, V>(
    name: &'static str,
    get_vec: for<'a> fn(&'a M) -> &'a Vec<V::Value>,
    mut_vec: for<'a> fn(&'a mut M) -> &'a mut Vec<V::Value>)
        -> Box<FieldAccessor + 'static>
    where
        M : Message + 'static,
        V : ProtobufType + 'static,
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::Repeated(Box::new(MessageGetMut::<M, Vec<V::Value>> {
            get_field: get_vec,
            mut_field: mut_vec,
        })),
    })
}


impl<M, V> FieldAccessor2<M, ReflectRepeated> for MessageGetMut<M, RepeatedField<V>>
    where
        M : Message + 'static,
        V : ProtobufValue + 'static,
{
    fn get_field<'a>(&self, m: &'a M) -> &'a ReflectRepeated {
        (self.get_field)(m) as &ReflectRepeated
    }

    fn mut_field<'a>(&self, m: &'a mut M) -> &'a mut ReflectRepeated {
        (self.mut_field)(m) as &mut ReflectRepeated
    }
}


pub fn make_repeated_field_accessor<M, V>(
    name: &'static str,
    get_vec: for<'a> fn(&'a M) -> &'a RepeatedField<V::Value>,
    mut_vec: for<'a> fn(&'a mut M) -> &'a mut RepeatedField<V::Value>)
        -> Box<FieldAccessor + 'static>
    where
        M : Message + 'static,
        V : ProtobufType + 'static,
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::Repeated(Box::new(MessageGetMut::<M, RepeatedField<V::Value>> {
            get_field: get_vec,
            mut_field: mut_vec,
        })),
    })
}

impl<M, V> FieldAccessor2<M, ReflectOptional> for MessageGetMut<M, Option<V>>
    where
        M : Message + 'static,
        V : ProtobufValue + Clone + 'static,
{
    fn get_field<'a>(&self, m: &'a M) -> &'a ReflectOptional {
        (self.get_field)(m) as &ReflectOptional
    }

    fn mut_field<'a>(&self, m: &'a mut M) -> &'a mut ReflectOptional {
        (self.mut_field)(m) as &mut ReflectOptional
    }
}

//#[deprecated]
pub fn make_option_accessor<M, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a Option<V::Value>,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut Option<V::Value>)
        -> Box<FieldAccessor + 'static>
where
    M : Message + 'static,
    V : ProtobufType + 'static,
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::Optional(Box::new(MessageGetMut::<M, Option<V::Value>> {
            get_field: get_field,
            mut_field: mut_field,
        }))
    })
}


pub fn make_has_get_set_clear_accessor<M, V>(
    _name: &'static str,
    _has_value: for<'a> fn(&'a M) -> bool,
    _get_value: for<'a> fn(&'a M) -> &'a V::Value,
    _set_value: for<'a> fn(&'a M, V::Value),
    _clr_value: for<'a> fn(&'a M))
        -> Box<FieldAccessor + 'static>
where
    M : Message + 'static,
    V : ProtobufType + 'static,
{
    unimplemented!()
}


pub fn make_has_get_mut_clear_accessor<M, V>(
    _name: &'static str,
    _has_value: for<'a> fn(&'a M) -> bool,
    _get_value: for<'a> fn(&'a M) -> &'a V::Value,
    _mut_value: for<'a> fn(&'a M) -> &'a mut V::Value,
    _clr_value: for<'a> fn(&'a M))
        -> Box<FieldAccessor + 'static>
where
    M : Message + 'static,
    V : ProtobufType + 'static,
{
    unimplemented!()
}


impl<M, V> FieldAccessor2<M, ReflectOptional> for MessageGetMut<M, SingularField<V>>
    where
        M : Message + 'static,
        V : ProtobufValue + Clone + 'static,
{
    fn get_field<'a>(&self, m: &'a M) -> &'a ReflectOptional {
        (self.get_field)(m) as &ReflectOptional
    }

    fn mut_field<'a>(&self, m: &'a mut M) -> &'a mut ReflectOptional {
        (self.mut_field)(m) as &mut ReflectOptional
    }
}

pub fn make_singular_field_accessor<M, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a SingularField<V::Value>,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut SingularField<V::Value>)
        -> Box<FieldAccessor + 'static>
where
    M : Message + 'static,
    V : ProtobufType + 'static,
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::Optional(Box::new(MessageGetMut::<M, SingularField<V::Value>> {
            get_field: get_field,
            mut_field: mut_field,
        }))
    })
}

impl<M, V> FieldAccessor2<M, ReflectOptional> for MessageGetMut<M, SingularPtrField<V>>
    where
        M : Message + 'static,
        V : ProtobufValue + Clone + 'static,
{
    fn get_field<'a>(&self, m: &'a M) -> &'a ReflectOptional {
        (self.get_field)(m) as &ReflectOptional
    }

    fn mut_field<'a>(&self, m: &'a mut M) -> &'a mut ReflectOptional {
        (self.mut_field)(m) as &mut ReflectOptional
    }
}

pub fn make_singular_ptr_field_accessor<M, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a SingularPtrField<V::Value>,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut SingularPtrField<V::Value>)
        -> Box<FieldAccessor + 'static>
where
    M : Message + 'static,
    V : ProtobufType + 'static,
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::Optional(Box::new(MessageGetMut::<M, SingularPtrField<V::Value>> {
            get_field: get_field,
            mut_field: mut_field,
        }))
    })
}

impl<M, V> FieldAccessor2<M, ProtobufValue> for MessageGetMut<M, V>
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

pub fn make_simple_field_accessor<M, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a V::Value,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut V::Value)
        -> Box<FieldAccessor + 'static>
where
    M : Message + 'static,
    V : ProtobufType + 'static,
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::Simple(Box::new(MessageGetMut::<M, V::Value> {
            get_field: get_field,
            mut_field: mut_field,
        }))
    })
}


impl<M, K, V> FieldAccessor2<M, ReflectMap> for MessageGetMut<M, HashMap<K, V>>
    where
        M : Message + 'static,
        K : ProtobufValue + 'static,
        V : ProtobufValue + 'static,
        K : Hash + Eq,
{
    fn get_field<'a>(&self, m: &'a M) -> &'a ReflectMap {
        (self.get_field)(m) as &ReflectMap
    }

    fn mut_field<'a>(&self, m: &'a mut M) -> &'a mut ReflectMap {
        (self.mut_field)(m) as &mut ReflectMap
    }
}


pub fn make_map_accessor<M, K, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a HashMap<K::Value, V::Value>,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut HashMap<K::Value, V::Value>)
        -> Box<FieldAccessor + 'static>
where
    M : Message + 'static,
    K : ProtobufType + 'static,
    V : ProtobufType + 'static,
    <K as ProtobufType>::Value : Hash + Eq,
{
    Box::new(FieldAccessorImpl {
        name: name,
        fns: FieldAccessorFunctions::Map(Box::new(MessageGetMut::<M, HashMap<K::Value, V::Value>> {
            get_field: get_field,
            mut_field: mut_field,
        })),
    })
}


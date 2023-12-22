use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::hash::Hash;

use crate::message_dyn::MessageDyn;
use crate::message_full::MessageFull;
use crate::reflect::acc::v2::AccessorV2;
use crate::reflect::acc::FieldAccessor;
use crate::reflect::map::ReflectMapMut;
use crate::reflect::map::ReflectMapRef;
use crate::reflect::runtime_types::RuntimeTypeMapKey;
use crate::reflect::runtime_types::RuntimeTypeTrait;
use crate::reflect::ProtobufValue;
use crate::reflect::RuntimeType;

pub(crate) trait MapFieldAccessor: Send + Sync + 'static {
    fn get_reflect<'a>(&self, m: &'a dyn MessageDyn) -> ReflectMapRef<'a>;
    fn mut_reflect<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectMapMut<'a>;
    fn element_type(&self) -> (RuntimeType, RuntimeType);
}

pub(crate) struct MapFieldAccessorHolder {
    pub accessor: Box<dyn MapFieldAccessor>,
}

impl<'a> fmt::Debug for MapFieldAccessorHolder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MapFieldAccessorHolder").finish()
    }
}

struct MapFieldAccessorImpl<M, T>
where
    M: MessageFull,
{
    get_field: fn(&M) -> &T,
    mut_field: fn(&mut M) -> &mut T,
}

impl<M, K, V> MapFieldAccessor for MapFieldAccessorImpl<M, HashMap<K, V>>
where
    M: MessageFull,
    K: ProtobufValue + Eq + Hash,
    K::RuntimeType: RuntimeTypeMapKey,
    V: ProtobufValue,
{
    fn get_reflect<'a>(&self, m: &'a dyn MessageDyn) -> ReflectMapRef<'a> {
        let m = m.downcast_ref().unwrap();
        let map = (self.get_field)(m);
        ReflectMapRef::new(map)
    }

    fn mut_reflect<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectMapMut<'a> {
        let m = m.downcast_mut().unwrap();
        let map = (self.mut_field)(m);
        ReflectMapMut::new(map)
    }

    fn element_type(&self) -> (RuntimeType, RuntimeType) {
        (
            K::RuntimeType::runtime_type_box(),
            V::RuntimeType::runtime_type_box(),
        )
    }
}

impl<M, K, V> MapFieldAccessor for MapFieldAccessorImpl<M, BTreeMap<K, V>>
where
    M: MessageFull,
    K: ProtobufValue + Ord,
    K::RuntimeType: RuntimeTypeMapKey,
    V: ProtobufValue,
{
    fn get_reflect<'a>(&self, m: &'a dyn MessageDyn) -> ReflectMapRef<'a> {
        let m = m.downcast_ref().unwrap();
        let map = (self.get_field)(m);
        ReflectMapRef::new(map)
    }

    fn mut_reflect<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectMapMut<'a> {
        let m = m.downcast_mut().unwrap();
        let map = (self.mut_field)(m);
        ReflectMapMut::new(map)
    }

    fn element_type(&self) -> (RuntimeType, RuntimeType) {
        (
            K::RuntimeType::runtime_type_box(),
            V::RuntimeType::runtime_type_box(),
        )
    }
}

/// Make accessor for map field
pub fn make_map_simpler_accessor<M, T>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a T,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut T,
) -> FieldAccessor
where
    M: MessageFull,
    MapFieldAccessorImpl<M, T>: MapFieldAccessor,
{
    FieldAccessor::new(
        name,
        AccessorV2::Map(MapFieldAccessorHolder {
            accessor: Box::new(MapFieldAccessorImpl::<M, T> {
                get_field,
                mut_field,
            }),
        }),
    )
}

use std::collections::HashMap;
use std::hash::Hash;

use crate::core::Message;

use crate::reflect::acc::v2::AccessorV2;
use crate::reflect::acc::FieldAccessor;
use crate::reflect::map::ReflectMapMut;
use crate::reflect::map::ReflectMapRef;
use crate::reflect::type_dynamic::ProtobufTypeDynamic;
use crate::reflect::types::ProtobufType;

pub(crate) trait MapFieldAccessor: Send + Sync + 'static {
    fn get_reflect<'a>(&self, m: &'a dyn Message) -> ReflectMapRef<'a>;
    fn mut_reflect<'a>(&self, m: &'a mut dyn Message) -> ReflectMapMut<'a>;
}

pub(crate) struct MapFieldAccessorHolder {
    pub accessor: Box<dyn MapFieldAccessor>,
    pub key_type: &'static dyn ProtobufTypeDynamic,
    pub value_type: &'static dyn ProtobufTypeDynamic,
}

struct MapFieldAccessorImpl<M, K, V>
where
    M: Message,
    K: ProtobufType,
    V: ProtobufType,
{
    get_field: fn(&M) -> &HashMap<K::ProtobufValue, V::ProtobufValue>,
    mut_field: fn(&mut M) -> &mut HashMap<K::ProtobufValue, V::ProtobufValue>,
}

impl<M, K, V> MapFieldAccessor for MapFieldAccessorImpl<M, K, V>
where
    M: Message,
    K: ProtobufType,
    V: ProtobufType,
    K::ProtobufValue: Eq + Hash,
{
    fn get_reflect<'a>(&self, m: &'a dyn Message) -> ReflectMapRef<'a> {
        let m = m.downcast_ref().unwrap();
        let map = (self.get_field)(m);
        ReflectMapRef { map }
    }

    fn mut_reflect<'a>(&self, m: &'a mut dyn Message) -> ReflectMapMut<'a> {
        let m = m.downcast_mut().unwrap();
        let map = (self.mut_field)(m);
        ReflectMapMut { map }
    }
}

/// Make accessor for map field
pub fn make_map_accessor<M, K, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a HashMap<K::ProtobufValue, V::ProtobufValue>,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut HashMap<K::ProtobufValue, V::ProtobufValue>,
) -> FieldAccessor
where
    M: Message + 'static,
    K: ProtobufType + 'static,
    V: ProtobufType + 'static,
    K::ProtobufValue: Hash + Eq,
{
    FieldAccessor::new_v2(
        name,
        AccessorV2::Map(MapFieldAccessorHolder {
            accessor: Box::new(MapFieldAccessorImpl::<M, K, V> {
                get_field,
                mut_field,
            }),
            key_type: K::dynamic(),
            value_type: V::dynamic(),
        }),
    )
}

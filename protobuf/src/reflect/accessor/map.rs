use std::collections::HashMap;
use std::hash::Hash;

use Message;
use core::message_down_cast;

use reflect::runtime_types::RuntimeType;
use reflect::accessor::FieldAccessor;
use reflect::types::ProtobufType;
use reflect::accessor::AccessorKind;
use reflect::map::ReflectMapRef;


pub(crate) trait MapFieldAccessor : 'static {
    fn len_field_generic(&self, m: &Message) -> usize;
    fn get_reflect<'a>(&self, m: &'a Message) -> ReflectMapRef<'a>;
}

struct MapFieldAccessorImpl<M, K, V>
    where
        M : Message,
        K : RuntimeType,
        V : RuntimeType,
{
    get_field: fn(&M) -> &HashMap<K::Value, V::Value>,
    _mut_field: fn(&mut M) -> &mut HashMap<K::Value, V::Value>,
}

impl<M, K, V> MapFieldAccessor for MapFieldAccessorImpl<M, K, V>
    where
        M : Message,
        K : RuntimeType,
        V : RuntimeType,
        K::Value : Eq + Hash
{
    fn len_field_generic(&self, m: &Message) -> usize {
        let m = message_down_cast(m);
        let map = (self.get_field)(m);
        map.len()
    }

    fn get_reflect<'a>(&self, m: &'a Message) -> ReflectMapRef<'a> {
        let m = message_down_cast(m);
        let map = (self.get_field)(m);
        ReflectMapRef {
            map,
            key_dynamic: K::dynamic(),
            value_dynamic: V::dynamic(),
        }
    }
}


pub fn make_map_accessor<M, K, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M)
        -> &'a HashMap<<K::RuntimeType as RuntimeType>::Value, <V::RuntimeType as RuntimeType>::Value>,
    mut_field: for<'a> fn(&'a mut M)
        -> &'a mut HashMap<<K::RuntimeType as RuntimeType>::Value, <V::RuntimeType as RuntimeType>::Value>,
) -> FieldAccessor
    where
        M : Message + 'static,
        K : ProtobufType + 'static,
        V : ProtobufType + 'static,
        <K::RuntimeType as RuntimeType>::Value : Hash + Eq,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Map(Box::new(MapFieldAccessorImpl::<M, K::RuntimeType, V::RuntimeType> {
            get_field,
            _mut_field: mut_field,
        })),
    }
}

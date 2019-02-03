use std::collections::HashMap;
use std::hash::Hash;

use Message;

use reflect::accessor::AccessorKind;
use reflect::accessor::FieldAccessor;
use reflect::map::ReflectMapMut;
use reflect::map::ReflectMapRef;
use reflect::runtime_types::RuntimeType;
use reflect::type_dynamic::ProtobufTypeDynamic;
use reflect::types::ProtobufType;

pub(crate) trait MapFieldAccessor: Send + Sync + 'static {
    fn get_reflect<'a>(&self, m: &'a Message) -> ReflectMapRef<'a>;
    fn mut_reflect<'a>(&self, m: &'a mut Message) -> ReflectMapMut<'a>;
}

pub(crate) struct MapFieldAccessorHolder {
    pub accessor: Box<MapFieldAccessor>,
    pub key_type: &'static ProtobufTypeDynamic,
    pub value_type: &'static ProtobufTypeDynamic,
}

struct MapFieldAccessorImpl<M, K, V>
where
    M: Message,
    K: ProtobufType,
    V: ProtobufType,
{
    get_field: fn(
        &M
    ) -> &HashMap<
        <K::RuntimeType as RuntimeType>::Value,
        <V::RuntimeType as RuntimeType>::Value,
    >,
    mut_field: fn(
        &mut M
    ) -> &mut HashMap<
        <K::RuntimeType as RuntimeType>::Value,
        <V::RuntimeType as RuntimeType>::Value,
    >,
}

impl<M, K, V> MapFieldAccessor for MapFieldAccessorImpl<M, K, V>
where
    M: Message,
    K: ProtobufType,
    V: ProtobufType,
    <K::RuntimeType as RuntimeType>::Value: Eq + Hash,
{
    fn get_reflect<'a>(&self, m: &'a Message) -> ReflectMapRef<'a> {
        let m = m.downcast_ref().unwrap();
        let map = (self.get_field)(m);
        ReflectMapRef {
            map,
            key_dynamic: K::RuntimeType::dynamic(),
            value_dynamic: V::RuntimeType::dynamic(),
        }
    }

    fn mut_reflect<'a>(&self, m: &'a mut Message) -> ReflectMapMut<'a> {
        let m = m.downcast_mut().unwrap();
        let map = (self.mut_field)(m);
        ReflectMapMut {
            map,
            key_dynamic: K::RuntimeType::dynamic(),
            value_dynamic: V::RuntimeType::dynamic(),
        }
    }
}

pub fn make_map_accessor<M, K, V>(
    name: &'static str,
    get_field: for<'a> fn(
        &'a M
    ) -> &'a HashMap<
        <K::RuntimeType as RuntimeType>::Value,
        <V::RuntimeType as RuntimeType>::Value,
    >,
    mut_field: for<'a> fn(
        &'a mut M
    ) -> &'a mut HashMap<
        <K::RuntimeType as RuntimeType>::Value,
        <V::RuntimeType as RuntimeType>::Value,
    >,
) -> FieldAccessor
where
    M: Message + 'static,
    K: ProtobufType + 'static,
    V: ProtobufType + 'static,
    <K::RuntimeType as RuntimeType>::Value: Hash + Eq,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Map(MapFieldAccessorHolder {
            accessor: Box::new(MapFieldAccessorImpl::<M, K, V> {
                get_field,
                mut_field,
            }),
            key_type: K::dynamic(),
            value_type: V::dynamic(),
        }),
    }
}

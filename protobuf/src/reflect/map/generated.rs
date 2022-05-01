use std::collections::hash_map;
use std::collections::HashMap;
use std::hash::Hash;

use crate::reflect::map::ReflectMap;
use crate::reflect::map::ReflectMapIter;
use crate::reflect::map::ReflectMapIterTrait;
use crate::reflect::runtime_types::RuntimeTypeHashable;
use crate::reflect::runtime_types::RuntimeTypeTrait;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeType;

impl<K, V> ReflectMap for HashMap<K, V>
where
    K: ProtobufValue + Eq + Hash,
    V: ProtobufValue,
    K::RuntimeType: RuntimeTypeHashable,
{
    fn reflect_iter<'a>(&'a self) -> ReflectMapIter<'a> {
        ReflectMapIter::new(GeneratedMapIterImpl::<'a, K, V> { iter: self.iter() })
    }

    fn len(&self) -> usize {
        HashMap::len(self)
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn get<'a>(&'a self, key: ReflectValueRef) -> Option<ReflectValueRef<'a>> {
        <K::RuntimeType as RuntimeTypeHashable>::hash_map_get(self, key).map(V::RuntimeType::as_ref)
    }

    fn insert(&mut self, key: ReflectValueBox, value: ReflectValueBox) {
        let key: K = key.downcast().expect("wrong key type");
        let value: V = value.downcast().expect("wrong value type");
        self.insert(key, value);
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn key_type(&self) -> RuntimeType {
        K::RuntimeType::runtime_type_box()
    }

    fn value_type(&self) -> RuntimeType {
        V::RuntimeType::runtime_type_box()
    }
}

struct GeneratedMapIterImpl<'a, K: Eq + Hash + 'static, V: 'static> {
    iter: hash_map::Iter<'a, K, V>,
}

impl<'a, K: ProtobufValue + Eq + Hash, V: ProtobufValue> ReflectMapIterTrait<'a>
    for GeneratedMapIterImpl<'a, K, V>
{
    fn next(&mut self) -> Option<(ReflectValueRef<'a>, ReflectValueRef<'a>)> {
        match self.iter.next() {
            Some((k, v)) => Some((K::RuntimeType::as_ref(k), V::RuntimeType::as_ref(v))),
            None => None,
        }
    }

    fn key_type(&self) -> RuntimeType {
        K::RuntimeType::runtime_type_box()
    }

    fn value_type(&self) -> RuntimeType {
        V::RuntimeType::runtime_type_box()
    }
}

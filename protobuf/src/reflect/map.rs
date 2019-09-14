use std::collections::hash_map;
use std::collections::HashMap;
use std::hash::Hash;

use crate::reflect::value::ProtobufValue;
use crate::reflect::ReflectValueRef;
use crate::reflect::value::ReflectValueBox;

/// Implemented for `HashMap` with appropriate keys and values
pub trait ReflectMap: 'static {
    fn reflect_iter(&self) -> ReflectMapIter;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn get(&self, key: ReflectValueRef) -> Option<&dyn ProtobufValue>;

    fn insert(&mut self, key: ReflectValueBox, value: ReflectValueBox);

    fn clear(&mut self);
}

impl<K: ProtobufValue + Eq + Hash + 'static, V: ProtobufValue + 'static> ReflectMap
    for HashMap<K, V>
{
    fn reflect_iter<'a>(&'a self) -> ReflectMapIter<'a> {
        ReflectMapIter {
            imp: Box::new(ReflectMapIterImpl::<'a, K, V> { iter: self.iter() }),
        }
    }

    fn len(&self) -> usize {
        HashMap::len(self)
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn get(&self, key: ReflectValueRef) -> Option<&dyn ProtobufValue> {
        // TODO: malloc for string or bytes
        let key: K = key.to_box().downcast().expect("wrong key type");
        self.get(&key).map(|v| v as &dyn ProtobufValue)
    }

    fn insert(&mut self, key: ReflectValueBox, value: ReflectValueBox) {
        let key: K = key.downcast().expect("wrong key type");
        let value: V = value.downcast().expect("wrong value type");
        self.insert(key, value);
    }

    fn clear(&mut self) {
        self.clear();
    }
}

trait ReflectMapIterTrait<'a> {
    fn next(&mut self) -> Option<(&'a dyn ProtobufValue, &'a dyn ProtobufValue)>;
}

struct ReflectMapIterImpl<'a, K: Eq + Hash + 'static, V: 'static> {
    iter: hash_map::Iter<'a, K, V>,
}

impl<'a, K: ProtobufValue + Eq + Hash + 'static, V: ProtobufValue + 'static> ReflectMapIterTrait<'a>
    for ReflectMapIterImpl<'a, K, V>
{
    fn next(&mut self) -> Option<(&'a dyn ProtobufValue, &'a dyn ProtobufValue)> {
        match self.iter.next() {
            Some((k, v)) => Some((k as &dyn ProtobufValue, v as &dyn ProtobufValue)),
            None => None,
        }
    }
}

pub struct ReflectMapIter<'a> {
    imp: Box<dyn ReflectMapIterTrait<'a> + 'a>,
}

impl<'a> Iterator for ReflectMapIter<'a> {
    type Item = (&'a dyn ProtobufValue, &'a dyn ProtobufValue);

    fn next(&mut self) -> Option<(&'a dyn ProtobufValue, &'a dyn ProtobufValue)> {
        self.imp.next()
    }
}

impl<'a> IntoIterator for &'a dyn ReflectMap {
    type Item = (&'a dyn ProtobufValue, &'a dyn ProtobufValue);
    type IntoIter = ReflectMapIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.reflect_iter()
    }
}

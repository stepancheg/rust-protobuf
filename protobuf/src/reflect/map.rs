use std::collections::hash_map;
use std::collections::HashMap;
use std::hash::Hash;

use crate::reflect::reflect_eq::ReflectEq;
use crate::reflect::reflect_eq::ReflectEqMode;
use crate::reflect::runtime_types::RuntimeTypeHashable;
use crate::reflect::value::hashable::ReflectValueBoxHashable;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeTypeBox;

/// Implemented for `HashMap` with appropriate keys and values
pub(crate) trait ReflectMap: Send + Sync + 'static {
    fn reflect_iter(&self) -> ReflectMapIter;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn get<'a>(&'a self, key: ReflectValueRef) -> Option<ReflectValueRef<'a>>;

    fn insert(&mut self, key: ReflectValueBoxHashable, value: ReflectValueBox);

    fn clear(&mut self);

    fn key_type(&self) -> RuntimeTypeBox;

    fn value_type(&self) -> RuntimeTypeBox;
}

impl<K, V> ReflectMap for HashMap<K, V>
where
    K: ProtobufValue + Eq + Hash,
    V: ProtobufValue,
    K::RuntimeType: RuntimeTypeHashable,
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

    fn get<'a>(&'a self, key: ReflectValueRef) -> Option<ReflectValueRef<'a>> {
        <K::RuntimeType as RuntimeTypeHashable>::hash_map_get(self, key).map(V::as_ref)
    }

    fn insert(&mut self, key: ReflectValueBoxHashable, value: ReflectValueBox) {
        let key: K = key.downcast().expect("wrong key type");
        let value: V = value.downcast().expect("wrong value type");
        self.insert(key, value);
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn key_type(&self) -> RuntimeTypeBox {
        K::runtime_type_box()
    }

    fn value_type(&self) -> RuntimeTypeBox {
        V::runtime_type_box()
    }
}

trait ReflectMapIterTrait<'a> {
    fn next(&mut self) -> Option<(ReflectValueRef<'a>, ReflectValueRef<'a>)>;
    fn key_type(&self) -> RuntimeTypeBox;
    fn value_type(&self) -> RuntimeTypeBox;
}

struct ReflectMapIterImpl<'a, K: Eq + Hash + 'static, V: 'static> {
    iter: hash_map::Iter<'a, K, V>,
}

impl<'a, K: ProtobufValue + Eq + Hash, V: ProtobufValue> ReflectMapIterTrait<'a>
    for ReflectMapIterImpl<'a, K, V>
{
    fn next(&mut self) -> Option<(ReflectValueRef<'a>, ReflectValueRef<'a>)> {
        match self.iter.next() {
            Some((k, v)) => Some((K::as_ref(k), V::as_ref(v))),
            None => None,
        }
    }

    fn key_type(&self) -> RuntimeTypeBox {
        K::runtime_type_box()
    }

    fn value_type(&self) -> RuntimeTypeBox {
        V::runtime_type_box()
    }
}

pub struct ReflectMapIter<'a> {
    imp: Box<dyn ReflectMapIterTrait<'a> + 'a>,
}

impl<'a> Iterator for ReflectMapIter<'a> {
    type Item = (ReflectValueRef<'a>, ReflectValueRef<'a>);

    fn next(&mut self) -> Option<(ReflectValueRef<'a>, ReflectValueRef<'a>)> {
        self.imp.next()
    }
}

impl<'a> IntoIterator for &'a dyn ReflectMap {
    type Item = (ReflectValueRef<'a>, ReflectValueRef<'a>);
    type IntoIter = ReflectMapIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.reflect_iter()
    }
}

/// Dynamic reference to `map` field
#[derive(Copy, Clone)]
pub struct ReflectMapRef<'a> {
    map: &'a dyn ReflectMap,
}

/// Dynamic mutable reference to `map` field
pub struct ReflectMapMut<'a> {
    map: &'a mut dyn ReflectMap,
}

impl<'a> ReflectMapRef<'a> {
    pub(crate) fn new(map: &'a dyn ReflectMap) -> ReflectMapRef<'a> {
        ReflectMapRef { map }
    }

    /// Size of the map
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Is map empty?
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Find a value by given key.
    pub fn get(&self, key: ReflectValueRef) -> Option<ReflectValueRef> {
        self.map.get(key)
    }

    /// Map key type
    pub fn key_type(&self) -> RuntimeTypeBox {
        self.map.key_type()
    }

    /// Map value type
    pub fn value_type(&self) -> RuntimeTypeBox {
        self.map.value_type()
    }
}

impl<'a> ReflectEq for ReflectMapRef<'a> {
    fn reflect_eq(&self, that: &Self, mode: &ReflectEqMode) -> bool {
        let len = self.len();

        if len != that.len() {
            return false;
        }

        for (k, va) in self {
            let vb = match that.get(k) {
                Some(v) => v,
                None => return false,
            };

            if !va.reflect_eq(&vb, mode) {
                return false;
            }
        }

        true
    }
}

impl<'a> ReflectMapMut<'a> {
    pub(crate) fn new(map: &'a mut dyn ReflectMap) -> ReflectMapMut<'a> {
        ReflectMapMut { map }
    }

    fn as_ref(&'a self) -> ReflectMapRef<'a> {
        ReflectMapRef { map: self.map }
    }

    /// Map key type
    pub fn key_type(&self) -> RuntimeTypeBox {
        self.map.key_type()
    }

    /// Map value type
    pub fn value_type(&self) -> RuntimeTypeBox {
        self.map.value_type()
    }

    /// Number of map entries
    pub fn len(&self) -> usize {
        self.as_ref().len()
    }

    /// Is this map empty?
    pub fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    /// Find a value for given key
    pub fn get(&self, key: ReflectValueRef) -> Option<ReflectValueRef> {
        self.map.get(key)
    }

    /// Insert a value into the map
    pub fn insert(&mut self, key: ReflectValueBoxHashable, value: ReflectValueBox) {
        self.map.insert(key, value)
    }

    /// Clear
    pub fn clear(&mut self) {
        self.map.clear();
    }
}

/// Iterator over map
pub struct ReflectMapRefIter<'a> {
    iter: ReflectMapIter<'a>,
}

impl<'a> ReflectMapRefIter<'a> {
    fn _key_type(&self) -> RuntimeTypeBox {
        self.iter.imp.key_type()
    }

    fn _value_type(&self) -> RuntimeTypeBox {
        self.iter.imp.value_type()
    }
}

impl<'a> Iterator for ReflectMapRefIter<'a> {
    type Item = (ReflectValueRef<'a>, ReflectValueRef<'a>);

    fn next(&mut self) -> Option<(ReflectValueRef<'a>, ReflectValueRef<'a>)> {
        self.iter.next()
    }
}

impl<'a, 'b> IntoIterator for &'b ReflectMapRef<'a> {
    type Item = (ReflectValueRef<'a>, ReflectValueRef<'a>);
    type IntoIter = ReflectMapRefIter<'a>;

    fn into_iter(self) -> ReflectMapRefIter<'a> {
        ReflectMapRefIter {
            iter: self.map.reflect_iter(),
        }
    }
}

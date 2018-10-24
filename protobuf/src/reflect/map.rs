use std::collections::hash_map;
use std::collections::HashMap;
use std::hash::Hash;

use super::value::ProtobufValue;
use reflect::reflect_deep_eq::ReflectDeepEq;
use reflect::runtime_type_dynamic::RuntimeTypeDynamic;
use reflect::ReflectValueBox;
use reflect::ReflectValueRef;

/// Implemented for `HashMap` with appropriate keys and values
pub(crate) trait ReflectMap: Send + Sync + 'static {
    fn reflect_iter(&self) -> ReflectMapIter;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn get(&self, key: ReflectValueRef) -> Option<&ProtobufValue>;

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

    fn get(&self, key: ReflectValueRef) -> Option<&ProtobufValue> {
        // TODO: malloc for string or bytes
        let key: K = key.to_box().downcast().expect("wrong key type");
        self.get(&key).map(|v| v as &ProtobufValue)
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
    fn next(&mut self) -> Option<(&'a ProtobufValue, &'a ProtobufValue)>;
}

struct ReflectMapIterImpl<'a, K: Eq + Hash + 'static, V: 'static> {
    iter: hash_map::Iter<'a, K, V>,
}

impl<'a, K: ProtobufValue + Eq + Hash + 'static, V: ProtobufValue + 'static> ReflectMapIterTrait<'a>
    for ReflectMapIterImpl<'a, K, V>
{
    fn next(&mut self) -> Option<(&'a ProtobufValue, &'a ProtobufValue)> {
        match self.iter.next() {
            Some((k, v)) => Some((k as &ProtobufValue, v as &ProtobufValue)),
            None => None,
        }
    }
}

pub struct ReflectMapIter<'a> {
    imp: Box<ReflectMapIterTrait<'a> + 'a>,
}

impl<'a> Iterator for ReflectMapIter<'a> {
    type Item = (&'a ProtobufValue, &'a ProtobufValue);

    fn next(&mut self) -> Option<(&'a ProtobufValue, &'a ProtobufValue)> {
        self.imp.next()
    }
}

impl<'a> IntoIterator for &'a ReflectMap {
    type Item = (&'a ProtobufValue, &'a ProtobufValue);
    type IntoIter = ReflectMapIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.reflect_iter()
    }
}

#[derive(Copy, Clone)]
pub struct ReflectMapRef<'a> {
    pub(crate) map: &'a ReflectMap,
    pub(crate) key_dynamic: &'a RuntimeTypeDynamic,
    pub(crate) value_dynamic: &'a RuntimeTypeDynamic,
}

pub struct ReflectMapMut<'a> {
    pub(crate) map: &'a mut ReflectMap,
    pub(crate) key_dynamic: &'a RuntimeTypeDynamic,
    pub(crate) value_dynamic: &'a RuntimeTypeDynamic,
}

impl<'a> ReflectMapRef<'a> {
    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn get(&self, key: ReflectValueRef) -> Option<ReflectValueRef> {
        self.map
            .get(key)
            .map(|v| self.value_dynamic.value_to_ref(v))
    }

    pub fn key_type(&self) -> &RuntimeTypeDynamic {
        self.key_dynamic
    }

    pub fn value_type(&self) -> &RuntimeTypeDynamic {
        self.value_dynamic
    }
}

impl<'a> ReflectDeepEq for ReflectMapRef<'a> {
    fn reflect_deep_eq(&self, that: &Self) -> bool {
        let len = self.len();

        if len != that.len() {
            return false;
        }

        for (k, va) in self {
            let vb = match that.get(k) {
                Some(v) => v,
                None => return false,
            };

            if !va.reflect_deep_eq(&vb) {
                return false;
            }
        }

        true
    }
}

impl<'a> ReflectMapMut<'a> {
    fn as_ref(&'a self) -> ReflectMapRef<'a> {
        ReflectMapRef {
            map: self.map,
            key_dynamic: self.key_dynamic,
            value_dynamic: self.value_dynamic,
        }
    }

    pub fn key_type(&self) -> &RuntimeTypeDynamic {
        self.key_dynamic
    }

    pub fn value_type(&self) -> &RuntimeTypeDynamic {
        self.value_dynamic
    }

    pub fn len(&self) -> usize {
        self.as_ref().len()
    }

    pub fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    pub fn get(&self, key: ReflectValueRef) -> Option<ReflectValueRef> {
        self.map
            .get(key)
            .map(|v| self.value_dynamic.value_to_ref(v))
    }

    pub fn insert(&mut self, key: ReflectValueBox, value: ReflectValueBox) {
        self.map.insert(key, value)
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

pub struct ReflectMapRefIter<'a> {
    iter: ReflectMapIter<'a>,
    key_dynamic: &'a RuntimeTypeDynamic,
    value_dynamic: &'a RuntimeTypeDynamic,
}

impl<'a> Iterator for ReflectMapRefIter<'a> {
    type Item = (ReflectValueRef<'a>, ReflectValueRef<'a>);

    fn next(&mut self) -> Option<(ReflectValueRef<'a>, ReflectValueRef<'a>)> {
        self.iter.next().map(|(k, v)| {
            (
                self.key_dynamic.value_to_ref(k),
                self.value_dynamic.value_to_ref(v),
            )
        })
    }
}

impl<'a, 'b> IntoIterator for &'b ReflectMapRef<'a> {
    type Item = (ReflectValueRef<'a>, ReflectValueRef<'a>);
    type IntoIter = ReflectMapRefIter<'a>;

    fn into_iter(self) -> ReflectMapRefIter<'a> {
        ReflectMapRefIter {
            iter: self.map.reflect_iter(),
            key_dynamic: self.key_dynamic,
            value_dynamic: self.value_dynamic,
        }
    }
}

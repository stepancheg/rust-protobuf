use std::hash::Hash;
use std::collections::HashMap;
use std::collections::hash_map;

use super::value::ProtobufValue;
use reflect::runtime_type_dynamic::RuntimeTypeDynamic;
use reflect::ReflectValueRef;


/// Implemented for `HashMap` with appropriate keys and values
pub(crate) trait ReflectMap : Send + Sync + 'static {
    fn reflect_iter(&self) -> ReflectMapIter;

    fn len(&self) -> usize;
}

impl<K : ProtobufValue + Eq + Hash + 'static, V : ProtobufValue + 'static> ReflectMap
    for HashMap<K, V> {
    fn reflect_iter<'a>(&'a self) -> ReflectMapIter<'a> {
        ReflectMapIter { imp: Box::new(ReflectMapIterImpl::<'a, K, V> { iter: self.iter() }) }
    }

    fn len(&self) -> usize {
        HashMap::len(self)
    }
}


trait ReflectMapIterTrait<'a> {
    fn next(&mut self) -> Option<(&'a ProtobufValue, &'a ProtobufValue)>;
}

struct ReflectMapIterImpl<'a, K : Eq + Hash + 'static, V : 'static> {
    iter: hash_map::Iter<'a, K, V>,
}

impl<
    'a,
    K : ProtobufValue + Eq + Hash + 'static,
    V : ProtobufValue + 'static,
> ReflectMapIterTrait<'a> for ReflectMapIterImpl<'a, K, V> {
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

pub struct ReflectMapRefIter<'a> {
    iter: ReflectMapIter<'a>,
    key_dynamic: &'a RuntimeTypeDynamic,
    value_dynamic: &'a RuntimeTypeDynamic,
}

impl<'a> Iterator for ReflectMapRefIter<'a> {
    type Item = (ReflectValueRef<'a>, ReflectValueRef<'a>);

    fn next(&mut self) -> Option<(ReflectValueRef<'a>, ReflectValueRef<'a>)> {
        self.iter.next().map(|(k, v)| {
            (self.key_dynamic.value_to_ref(k), self.value_dynamic.value_to_ref(v))
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

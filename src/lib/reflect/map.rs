use std::hash::Hash;
use std::collections::HashMap;
use std::collections::hash_map;

use super::value::ProtobufValue;


/// Implemented for `HashMap` with appropriate keys and values
pub trait ReflectMap : 'static {
    fn reflect_iter(&self) -> ReflectMapIter;
}

impl<K : ProtobufValue + Eq + Hash + 'static, V : ProtobufValue + 'static> ReflectMap for HashMap<K, V> {
    fn reflect_iter<'a>(&'a self) -> ReflectMapIter<'a> {
        ReflectMapIter {
            imp: Box::new(ReflectMapIterImpl::<'a, K, V> {
                iter: self.iter()
            })
        }
    }
}


trait ReflectMapIterTrait<'a> {
    fn next(&mut self) -> Option<(&'a ProtobufValue, &'a ProtobufValue)>;
}

struct ReflectMapIterImpl<'a, K : Eq + Hash + 'static, V : 'static> {
    iter: hash_map::Iter<'a, K, V>,
}

impl<'a, K : ProtobufValue + Eq + Hash + 'static, V : ProtobufValue + 'static> ReflectMapIterTrait<'a> for ReflectMapIterImpl<'a, K, V> {
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

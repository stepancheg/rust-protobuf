use std::hash::Hash;
use std::collections::HashMap;
use std::collections::hash_map;

use super::value::ProtobufValue;
use super::value::ProtobufValueRef;


/// Implemented for `HashMap` with appropriate keys and values
pub trait ProtobufMap : 'static {
    fn map_iter(&self) -> ProtobufMapIter;
}

impl<K : ProtobufValue + Eq + Hash + 'static, V : ProtobufValue + 'static> ProtobufMap for HashMap<K, V> {
    fn map_iter<'a>(&'a self) -> ProtobufMapIter<'a> {
        ProtobufMapIter {
            imp: Box::new(ProtobufMapIterImpl::<'a, K, V> {
                iter: self.iter()
            })
        }
    }
}


trait ProtobufMapIterTrait<'a> {
    fn next(&mut self) -> Option<(&'a ProtobufValue, &'a ProtobufValue)>;
}

struct ProtobufMapIterImpl<'a, K : Eq + Hash + 'static, V : 'static> {
    iter: hash_map::Iter<'a, K, V>,
}

impl<'a, K : ProtobufValue + Eq + Hash + 'static, V : ProtobufValue + 'static> ProtobufMapIterTrait<'a> for ProtobufMapIterImpl<'a, K, V> {
    fn next(&mut self) -> Option<(&'a ProtobufValue, &'a ProtobufValue)> {
        match self.iter.next() {
            Some((k, v)) => Some((k as &ProtobufValue, v as &ProtobufValue)),
            None => None,
        }
    }
}

pub struct ProtobufMapIter<'a> {
    imp: Box<ProtobufMapIterTrait<'a> + 'a>,
}

impl<'a> Iterator for ProtobufMapIter<'a> {
    type Item = (&'a ProtobufValue, &'a ProtobufValue);

    fn next(&mut self) -> Option<(&'a ProtobufValue, &'a ProtobufValue)> {
        self.imp.next()
    }
}

use std::slice;

use super::value::ProtobufValue;

use repeated::RepeatedField;

pub trait ReflectRepeated : 'static {
    fn reflect_iter(&self) -> ReflectRepeatedIter;
    fn len(&self) -> usize;
}

impl<V : ProtobufValue + 'static> ReflectRepeated for Vec<V> {
    fn reflect_iter<'a>(&'a self) -> ReflectRepeatedIter<'a> {
        ReflectRepeatedIter {
            imp: Box::new(ReflectRepeatedIterImplSlice::<'a, V> {
                iter: self.iter()
            })
        }
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }
}

// useless
impl<V : ProtobufValue + 'static> ReflectRepeated for [V] {
    fn reflect_iter<'a>(&'a self) -> ReflectRepeatedIter<'a> {
        ReflectRepeatedIter {
            imp: Box::new(ReflectRepeatedIterImplSlice::<'a, V> {
                iter: self.iter()
            })
        }
    }

    fn len(&self) -> usize {
        <[_]>::len(self)
    }
}

impl<V : ProtobufValue + 'static> ReflectRepeated for RepeatedField<V> {
    fn reflect_iter<'a>(&'a self) -> ReflectRepeatedIter<'a> {
        ReflectRepeatedIter {
            imp: Box::new(ReflectRepeatedIterImplSlice::<'a, V> {
                iter: self.iter()
            })
        }
    }

    fn len(&self) -> usize {
        RepeatedField::len(self)
    }
}

trait ReflectRepeatedIterTrait<'a> {
    fn next(&mut self) -> Option<&'a ProtobufValue>;
}

struct ReflectRepeatedIterImplSlice<'a, V : ProtobufValue + 'static> {
    iter: slice::Iter<'a, V>,
}

impl<'a, V : ProtobufValue + 'static> ReflectRepeatedIterTrait<'a> for ReflectRepeatedIterImplSlice<'a, V> {
    fn next(&mut self) -> Option<&'a ProtobufValue> {
        self.iter.next().map(|v| v as &ProtobufValue)
    }
}

pub struct ReflectRepeatedIter<'a> {
    imp: Box<ReflectRepeatedIterTrait<'a> + 'a>,
}

impl<'a> Iterator for ReflectRepeatedIter<'a> {
    type Item = &'a ProtobufValue;

    fn next(&mut self) -> Option<Self::Item> {
        self.imp.next()
    }
}

impl<'a> IntoIterator for &'a ReflectRepeated {
    type IntoIter = ReflectRepeatedIter<'a>;
    type Item = &'a ProtobufValue;

    fn into_iter(self) -> Self::IntoIter {
        self.reflect_iter()
    }
}

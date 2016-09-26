use std::slice;

use core::ProtobufEnum;
use core::Message;

use super::value::ProtobufValue;

use repeated::RepeatedField;

pub trait ReflectRepeated : 'static {
    fn reflect_iter(&self) -> ReflectRepeatedIter;
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> &ProtobufValue;
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

    fn get(&self, index: usize) -> &ProtobufValue {
        &self[index]
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

    fn get(&self, index: usize) -> &ProtobufValue {
        &self[index]
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

    fn get(&self, index: usize) -> &ProtobufValue {
        &self[index]
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

pub trait ReflectRepeatedEnum<'a> {
    fn len(&self) -> usize;

    fn get(&self, index: usize) -> &'a ProtobufValue;
}

pub struct ReflectRepeatedEnumImpl<'a, E : ProtobufEnum> {
    slice: &'a [E],
}

impl<'a, E : ProtobufEnum + ProtobufValue + 'static> ReflectRepeatedEnum<'a> for ReflectRepeatedEnumImpl<'a, E> {
    fn len(&self) -> usize {
        self.slice.len()
    }

    fn get(&self, index: usize) -> &'a ProtobufValue {
        &self.slice[index]
    }
}

pub trait ReflectRepeatedMessage<'a> {
    fn len(&self) -> usize;

    fn get(&self, index: usize) -> &'a ProtobufValue;
}

pub struct ReflectRepeatedMessageImpl<'a, M : Message> {
    slice: &'a [M],
}

impl<'a, M : Message + ProtobufValue> ReflectRepeatedMessage<'a> for ReflectRepeatedMessageImpl<'a, M> {
    fn len(&self) -> usize {
        self.slice.len()
    }

    fn get(&self, index: usize) -> &'a ProtobufValue {
        &self.slice[index]
    }
}

pub enum ReflectRepeatedRef<'a> {
    Generic(&'a ReflectRepeated),
    U32(&'a [u32]),
    U64(&'a [u64]),
    I32(&'a [i32]),
    I64(&'a [i64]),
    F32(&'a [f32]),
    F64(&'a [f64]),
    Bool(&'a [bool]),
    String(&'a [String]),
    Bytes(&'a [Vec<u8>]),
    Enum(Box<ReflectRepeatedEnum<'a>>),
    Message(Box<ReflectRepeatedMessage<'a>>),
}

impl<'a> ReflectRepeatedRef<'a> {
    fn len(&self) -> usize {
        match *self {
            ReflectRepeatedRef::Generic(ref r) => r.len(),
            ReflectRepeatedRef::U32(ref r) => r.len(),
            ReflectRepeatedRef::U64(ref r) => r.len(),
            ReflectRepeatedRef::I32(ref r) => r.len(),
            ReflectRepeatedRef::I64(ref r) => r.len(),
            ReflectRepeatedRef::F32(ref r) => r.len(),
            ReflectRepeatedRef::F64(ref r) => r.len(),
            ReflectRepeatedRef::Bool(ref r) => r.len(),
            ReflectRepeatedRef::String(ref r) => r.len(),
            ReflectRepeatedRef::Bytes(ref r) => r.len(),
            ReflectRepeatedRef::Enum(ref r) => r.len(),
            ReflectRepeatedRef::Message(ref r) => r.len(),
        }
    }

    fn get(&self, index: usize) -> &'a ProtobufValue {
        match *self {
            ReflectRepeatedRef::Generic(ref r) => r.get(index),
            ReflectRepeatedRef::U32(ref r) => &r[index] as &ProtobufValue,
            ReflectRepeatedRef::U64(ref r) => &r[index] as &ProtobufValue,
            ReflectRepeatedRef::I32(ref r) => &r[index] as &ProtobufValue,
            ReflectRepeatedRef::I64(ref r) => &r[index] as &ProtobufValue,
            ReflectRepeatedRef::F32(ref r) => &r[index] as &ProtobufValue,
            ReflectRepeatedRef::F64(ref r) => &r[index] as &ProtobufValue,
            ReflectRepeatedRef::Bool(ref r) => &r[index] as &ProtobufValue,
            ReflectRepeatedRef::String(ref r) => &r[index] as &ProtobufValue,
            ReflectRepeatedRef::Bytes(ref r) => &r[index] as &ProtobufValue,
            ReflectRepeatedRef::Enum(ref r) => r.get(index),
            ReflectRepeatedRef::Message(ref r) => r.get(index),
        }
    }
}

pub struct ReflectRepeatedRefIter<'a> {
    repeated: &'a ReflectRepeatedRef<'a>,
    pos: usize,
}

impl<'a> Iterator for ReflectRepeatedRefIter<'a> {
    type Item = &'a ProtobufValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.repeated.len() {
            let pos = self.pos;
            self.pos += 1;
            Some(self.repeated.get(pos))
        } else {
            None
        }
    }

}

impl<'a> IntoIterator for &'a ReflectRepeatedRef<'a> {
    type IntoIter = ReflectRepeatedRefIter<'a>;
    type Item = &'a ProtobufValue;

    fn into_iter(self) -> Self::IntoIter {
        ReflectRepeatedRefIter {
            repeated: self,
            pos: 0,
        }
    }
}

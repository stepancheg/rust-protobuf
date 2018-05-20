use std::slice;

use super::value::ProtobufValue;
use super::value::ReflectValueRef;

use repeated::RepeatedField;
use reflect::runtime_type_dynamic::RuntimeTypeDynamic;


pub(crate) trait ReflectRepeated : Sync + 'static {
    fn reflect_iter(&self) -> ReflectRepeatedIter;
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> &ProtobufValue;
}

impl<V : ProtobufValue + 'static> ReflectRepeated for Vec<V> {
    fn reflect_iter<'a>(&'a self) -> ReflectRepeatedIter<'a> {
        ReflectRepeatedIter {
            imp: Box::new(ReflectRepeatedIterImplSlice::<'a, V> { iter: self.iter() }),
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
            imp: Box::new(ReflectRepeatedIterImplSlice::<'a, V> { iter: self.iter() }),
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
            imp: Box::new(ReflectRepeatedIterImplSlice::<'a, V> { iter: self.iter() }),
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

impl<'a, V : ProtobufValue + 'static> ReflectRepeatedIterTrait<'a>
    for ReflectRepeatedIterImplSlice<'a, V> {
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

    fn get(&self, index: usize) -> ReflectValueRef<'a>;
}

pub trait ReflectRepeatedMessage<'a> {
    fn len(&self) -> usize;

    fn get(&self, index: usize) -> ReflectValueRef<'a>;
}

enum ReflectRepeatedRefUnused<'a> {
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
    Enum(Box<ReflectRepeatedEnum<'a> + 'a>),
    Message(Box<ReflectRepeatedMessage<'a> + 'a>),
}

impl<'a> ReflectRepeatedRefUnused<'a> {
    fn len(&self) -> usize {
        match *self {
            ReflectRepeatedRefUnused::Generic(ref r) => r.len(),
            ReflectRepeatedRefUnused::U32(ref r) => r.len(),
            ReflectRepeatedRefUnused::U64(ref r) => r.len(),
            ReflectRepeatedRefUnused::I32(ref r) => r.len(),
            ReflectRepeatedRefUnused::I64(ref r) => r.len(),
            ReflectRepeatedRefUnused::F32(ref r) => r.len(),
            ReflectRepeatedRefUnused::F64(ref r) => r.len(),
            ReflectRepeatedRefUnused::Bool(ref r) => r.len(),
            ReflectRepeatedRefUnused::String(ref r) => r.len(),
            ReflectRepeatedRefUnused::Bytes(ref r) => r.len(),
            ReflectRepeatedRefUnused::Enum(ref r) => r.len(),
            ReflectRepeatedRefUnused::Message(ref r) => r.len(),
        }
    }

    fn get(&self, index: usize) -> ReflectValueRef<'a> {
        match *self {
            ReflectRepeatedRefUnused::Generic(_) => unimplemented!(),
            ReflectRepeatedRefUnused::U32(ref r) => ReflectValueRef::U32(r[index]),
            ReflectRepeatedRefUnused::U64(ref r) => ReflectValueRef::U64(r[index]),
            ReflectRepeatedRefUnused::I32(ref r) => ReflectValueRef::I32(r[index]),
            ReflectRepeatedRefUnused::I64(ref r) => ReflectValueRef::I64(r[index]),
            ReflectRepeatedRefUnused::F32(ref r) => ReflectValueRef::F32(r[index]),
            ReflectRepeatedRefUnused::F64(ref r) => ReflectValueRef::F64(r[index]),
            ReflectRepeatedRefUnused::Bool(ref r) => ReflectValueRef::Bool(r[index]),
            ReflectRepeatedRefUnused::String(ref r) => ReflectValueRef::String(&r[index]),
            ReflectRepeatedRefUnused::Bytes(ref r) => ReflectValueRef::Bytes(&r[index]),
            ReflectRepeatedRefUnused::Enum(ref r) => r.get(index),
            ReflectRepeatedRefUnused::Message(ref r) => r.get(index),
        }
    }
}

pub struct ReflectRepeatedRefUnusedIter<'a> {
    repeated: &'a ReflectRepeatedRefUnused<'a>,
    pos: usize,
}

impl<'a> Iterator for ReflectRepeatedRefUnusedIter<'a> {
    type Item = ReflectValueRef<'a>;

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

impl<'a> IntoIterator for &'a ReflectRepeatedRefUnused<'a> {
    type Item = ReflectValueRef<'a>;
    type IntoIter = ReflectRepeatedRefUnusedIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ReflectRepeatedRefUnusedIter {
            repeated: self,
            pos: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct ReflectRepeatedRef<'a> {
    pub(crate) repeated: &'a ReflectRepeated,
    pub(crate) dynamic: &'static RuntimeTypeDynamic,
}

impl<'a> ReflectRepeatedRef<'a> {
    pub fn len(&self) -> usize {
        self.repeated.len()
    }

    pub fn get(&self, index: usize) -> ReflectValueRef<'a> {
        self.dynamic.value_to_ref(self.repeated.get(index))
    }
}

pub struct ReflectRepeatedRefIter<'a> {
    repeated: ReflectRepeatedRef<'a>,
    index: usize,
}

impl<'a> Iterator for ReflectRepeatedRefIter<'a> {
    type Item = ReflectValueRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        if index != self.repeated.len() {
            let r = self.repeated.get(index);
            self.index += 1;
            Some(r)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a ReflectRepeatedRef<'a> {
    type Item = ReflectValueRef<'a>;
    type IntoIter = ReflectRepeatedRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ReflectRepeatedRefIter {
            repeated: *self,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for ReflectRepeatedRef<'a> {
    type Item = ReflectValueRef<'a>;
    type IntoIter = ReflectRepeatedRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ReflectRepeatedRefIter {
            repeated: self,
            index: 0,
        }
    }
}

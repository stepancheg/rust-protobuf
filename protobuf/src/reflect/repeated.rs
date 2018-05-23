use std::slice;

use super::value::ProtobufValue;
use super::value::ReflectValueRef;

use repeated::RepeatedField;
use reflect::runtime_type_dynamic::RuntimeTypeDynamic;
use reflect::ReflectValueBox;
use std::fmt;


pub(crate) trait ReflectRepeated : Sync + 'static + fmt::Debug {
    fn reflect_iter(&self) -> ReflectRepeatedIter;
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> &ProtobufValue;
    fn set(&mut self, index: usize, value: ReflectValueBox);
    fn push(&mut self, value: ReflectValueBox);
}

impl<V : ProtobufValue + fmt::Debug + 'static> ReflectRepeated for Vec<V> {
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

    fn set(&mut self, index: usize, value: ReflectValueBox) {
        let value = value.downcast().expect("wrong type");
        self[index] = value;
    }

    fn push(&mut self, value: ReflectValueBox) {
        let value = value.downcast().expect("wrong type");
        self.push(value)
    }
}

// useless
impl<V : ProtobufValue + fmt::Debug + 'static> ReflectRepeated for [V] {
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

    fn set(&mut self, index: usize, value: ReflectValueBox) {
        let value = value.downcast().expect("wrong type");
        self[index] = value;
    }

    fn push(&mut self, _value: ReflectValueBox) {
        panic!("push is not possible for [V]")
    }
}

impl<V : ProtobufValue + fmt::Debug + 'static> ReflectRepeated for RepeatedField<V> {
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

    fn set(&mut self, index: usize, value: ReflectValueBox) {
        let value = value.downcast().expect("wrong type");
        self[index] = value;
    }

    fn push(&mut self, value: ReflectValueBox) {
        let value = value.downcast().expect("wrong type");
        self.push(value)
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
    type Item = &'a ProtobufValue;
    type IntoIter = ReflectRepeatedIter<'a>;

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

pub struct ReflectRepeatedMut<'a> {
    pub(crate) repeated: &'a mut ReflectRepeated,
    pub(crate) dynamic: &'static RuntimeTypeDynamic,
}

impl<'a> ReflectRepeatedRef<'a> {
    pub fn len(&self) -> usize {
        self.repeated.len()
    }

    // TODO: replace with index
    pub fn get(&self, index: usize) -> ReflectValueRef<'a> {
        self.dynamic.value_to_ref(self.repeated.get(index))
    }
}

impl<'a> PartialEq for ReflectRepeatedRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for i in 0..self.len() {
            if self.get(i) != other.get(i) {
                return false;
            }
        }

        return true;
    }
}

impl<'a> PartialEq<[ReflectValueBox]> for ReflectRepeatedRef<'a> {
    fn eq(&self, other: &[ReflectValueBox]) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for i in 0..self.len() {
            if self.get(i) != other[i] {
                return false;
            }
        }

        return true;
    }
}

impl<'a> PartialEq<ReflectRepeatedRef<'a>> for [ReflectValueBox] {
    fn eq(&self, other: &ReflectRepeatedRef) -> bool {
        other == self
    }
}

impl<'a> PartialEq<Vec<ReflectValueBox>> for ReflectRepeatedRef<'a> {
    fn eq(&self, other: &Vec<ReflectValueBox>) -> bool {
        self == other.as_slice()
    }
}

impl<'a> PartialEq<ReflectRepeatedRef<'a>> for Vec<ReflectValueBox> {
    fn eq(&self, other: &ReflectRepeatedRef) -> bool {
        self.as_slice() == other
    }
}


impl<'a> ReflectRepeatedMut<'a> {
    fn as_ref(&'a self) -> ReflectRepeatedRef<'a> {
        ReflectRepeatedRef {
            repeated: self.repeated,
            dynamic: self.dynamic,
        }
    }

    pub fn len(&self) -> usize {
        self.repeated.len()
    }

    pub fn get(&'a self, index: usize) -> ReflectValueRef<'a> {
        self.dynamic.value_to_ref(self.repeated.get(index))
    }

    pub fn set(&mut self, index: usize, value: ReflectValueBox) {
        self.repeated.set(index, value);
    }

    pub fn push(&mut self, value: ReflectValueBox) {
        self.repeated.push(value);
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

impl<'a> IntoIterator for &'a ReflectRepeatedMut<'a> {
    type Item = ReflectValueRef<'a>;
    type IntoIter = ReflectRepeatedRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().into_iter()
    }
}


impl<'a> fmt::Debug for ReflectRepeatedRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.repeated, f)
    }
}

impl<'a> fmt::Debug for ReflectRepeatedMut<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.repeated, f)
    }
}

impl<'a> PartialEq for ReflectRepeatedMut<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<'a> PartialEq<ReflectRepeatedRef<'a>> for ReflectRepeatedMut<'a> {
    fn eq(&self, other: &ReflectRepeatedRef) -> bool {
        PartialEq::eq(&self.as_ref(), other)
    }
}

impl<'a> PartialEq<[ReflectValueBox]> for ReflectRepeatedMut<'a> {
    fn eq(&self, other: &[ReflectValueBox]) -> bool {
        PartialEq::eq(&self.as_ref(), other)
    }
}

impl<'a> PartialEq<ReflectRepeatedMut<'a>> for [ReflectValueBox] {
    fn eq(&self, other: &ReflectRepeatedMut) -> bool {
        PartialEq::eq(self, &other.as_ref())
    }
}

impl<'a> PartialEq<Vec<ReflectValueBox>> for ReflectRepeatedMut<'a> {
    fn eq(&self, other: &Vec<ReflectValueBox>) -> bool {
        self == other.as_slice()
    }
}

impl<'a> PartialEq<ReflectRepeatedMut<'a>> for Vec<ReflectValueBox> {
    fn eq(&self, other: &ReflectRepeatedMut) -> bool {
        self.as_slice() == other
    }
}

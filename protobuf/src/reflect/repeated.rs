use std::fmt;
use std::slice;

use crate::reflect::dynamic::repeated::DynamicRepeated;
use crate::reflect::reflect_eq::ReflectEq;
use crate::reflect::reflect_eq::ReflectEqMode;
use crate::reflect::value::value_ref::ReflectValueRef;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueBox;
use crate::reflect::RuntimeTypeBox;

pub(crate) trait ReflectRepeated: Sync + 'static + fmt::Debug {
    fn reflect_iter(&self) -> ReflectRepeatedIter;
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> ReflectValueRef;
    fn set(&mut self, index: usize, value: ReflectValueBox);
    fn push(&mut self, value: ReflectValueBox);
    fn clear(&mut self);
    fn element_type(&self) -> RuntimeTypeBox;
}

impl<V: ProtobufValue> ReflectRepeated for Vec<V> {
    fn reflect_iter<'a>(&'a self) -> ReflectRepeatedIter<'a> {
        ReflectRepeatedIter {
            imp: Box::new(ReflectRepeatedIterImplSlice::<'a, V> { iter: self.iter() }),
        }
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn get(&self, index: usize) -> ReflectValueRef {
        V::as_ref(&self[index])
    }

    fn set(&mut self, index: usize, value: ReflectValueBox) {
        let value = value.downcast().expect("wrong type");
        self[index] = value;
    }

    fn push(&mut self, value: ReflectValueBox) {
        let value = value.downcast().expect("wrong type");
        self.push(value)
    }

    fn clear(&mut self) {
        self.clear()
    }

    fn element_type(&self) -> RuntimeTypeBox {
        V::runtime_type_box()
    }
}

// useless
impl<V: ProtobufValue> ReflectRepeated for [V] {
    fn reflect_iter<'a>(&'a self) -> ReflectRepeatedIter<'a> {
        ReflectRepeatedIter {
            imp: Box::new(ReflectRepeatedIterImplSlice::<'a, V> { iter: self.iter() }),
        }
    }

    fn len(&self) -> usize {
        <[_]>::len(self)
    }

    fn get(&self, index: usize) -> ReflectValueRef {
        V::as_ref(&self[index])
    }

    fn set(&mut self, index: usize, value: ReflectValueBox) {
        let value = value.downcast().expect("wrong type");
        self[index] = value;
    }

    fn push(&mut self, _value: ReflectValueBox) {
        panic!("push is not possible for [V]");
    }

    fn clear(&mut self) {
        panic!("clear is not possible for [V]");
    }

    fn element_type(&self) -> RuntimeTypeBox {
        V::runtime_type_box()
    }
}

trait ReflectRepeatedIterTrait<'a> {
    fn next(&mut self) -> Option<ReflectValueRef<'a>>;
}

struct ReflectRepeatedIterImplSlice<'a, V: ProtobufValue + 'static> {
    iter: slice::Iter<'a, V>,
}

impl<'a, V: ProtobufValue + 'static> ReflectRepeatedIterTrait<'a>
    for ReflectRepeatedIterImplSlice<'a, V>
{
    fn next(&mut self) -> Option<ReflectValueRef<'a>> {
        self.iter.next().map(ProtobufValue::as_ref)
    }
}

pub struct ReflectRepeatedIter<'a> {
    imp: Box<dyn ReflectRepeatedIterTrait<'a> + 'a>,
}

impl<'a> Iterator for ReflectRepeatedIter<'a> {
    type Item = ReflectValueRef<'a>;

    fn next(&mut self) -> Option<ReflectValueRef<'a>> {
        self.imp.next()
    }
}

impl<'a> IntoIterator for &'a dyn ReflectRepeated {
    type Item = ReflectValueRef<'a>;
    type IntoIter = ReflectRepeatedIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.reflect_iter()
    }
}

#[derive(Clone)]
enum ReflectRepeatedRefImpl<'a> {
    Generated(&'a dyn ReflectRepeated),
    DynamicEmpty(DynamicRepeated),
}

impl<'a> fmt::Debug for ReflectRepeatedRefImpl<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReflectRepeatedRefImpl::Generated(r) => fmt::Debug::fmt(r, f),
            ReflectRepeatedRefImpl::DynamicEmpty(r) => fmt::Debug::fmt(r, f),
        }
    }
}

/// Dynamic reference to repeated field
#[derive(Clone)]
pub struct ReflectRepeatedRef<'a> {
    imp: ReflectRepeatedRefImpl<'a>,
}

/// Dynamic mutable reference to repeated field
pub struct ReflectRepeatedMut<'a> {
    repeated: &'a mut dyn ReflectRepeated,
}

impl<'a> ReflectRepeatedRef<'a> {
    pub(crate) fn new(repeated: &'a dyn ReflectRepeated) -> ReflectRepeatedRef<'a> {
        ReflectRepeatedRef {
            imp: ReflectRepeatedRefImpl::Generated(repeated),
        }
    }

    pub(crate) fn new_empty(elem: RuntimeTypeBox) -> ReflectRepeatedRef<'static> {
        ReflectRepeatedRef {
            imp: ReflectRepeatedRefImpl::DynamicEmpty(DynamicRepeated::new(elem)),
        }
    }

    /// Number of elements in repeated field
    pub fn len(&self) -> usize {
        match &self.imp {
            ReflectRepeatedRefImpl::Generated(g) => g.len(),
            ReflectRepeatedRefImpl::DynamicEmpty(d) => d.len(),
        }
    }

    /// Repeated field is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get item by index
    // TODO: replace with index
    pub fn get(&self, index: usize) -> ReflectValueRef<'a> {
        match &self.imp {
            ReflectRepeatedRefImpl::Generated(r) => r.get(index),
            ReflectRepeatedRefImpl::DynamicEmpty(..) => panic!("empty"),
        }
    }

    /// Runtime type of element
    pub fn element_type(&self) -> RuntimeTypeBox {
        match &self.imp {
            ReflectRepeatedRefImpl::Generated(r) => r.element_type(),
            ReflectRepeatedRefImpl::DynamicEmpty(r) => r.element_type(),
        }
    }
}

impl<'a> ReflectEq for ReflectRepeatedRef<'a> {
    fn reflect_eq(&self, that: &Self, mode: &ReflectEqMode) -> bool {
        let len = self.len();

        if len != that.len() {
            return false;
        }

        for i in 0..len {
            let a = self.get(i);
            let b = that.get(i);
            if !a.reflect_eq(&b, mode) {
                return false;
            }
        }

        true
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
    pub(crate) fn new(repeated: &'a mut dyn ReflectRepeated) -> ReflectRepeatedMut<'a> {
        ReflectRepeatedMut { repeated }
    }

    fn as_ref(&'a self) -> ReflectRepeatedRef<'a> {
        ReflectRepeatedRef::new(self.repeated)
    }

    /// Number of elements in repeated field
    pub fn len(&self) -> usize {
        self.repeated.len()
    }

    /// Self-explanatory
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get an item by index
    ///
    /// Note: return immutable reference.
    pub fn get(&'a self, index: usize) -> ReflectValueRef<'a> {
        self.repeated.get(index)
    }

    /// Runtime type of element
    pub fn element_type(&self) -> RuntimeTypeBox {
        self.repeated.element_type()
    }

    /// Set a value at given index.
    ///
    /// # Panics
    ///
    /// If index if out of range or value type does not match container element type
    pub fn set(&mut self, index: usize, value: ReflectValueBox) {
        self.repeated.set(index, value);
    }

    /// Push an item to repeated field.
    ///
    /// # Panics
    ///
    /// If index if out of range or value type does not match container element type
    pub fn push(&mut self, value: ReflectValueBox) {
        self.repeated.push(value);
    }

    /// Self-explanatory
    pub fn clear(&mut self) {
        self.repeated.clear();
    }
}

/// Iterator over repeated field.
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
            repeated: self.clone(),
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
        fmt::Debug::fmt(&self.imp, f)
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

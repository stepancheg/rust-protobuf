use std::any::TypeId;
use std::fmt;
use std::mem;
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
    /// Set element at index.
    ///
    /// # Panics
    ///
    /// * if index is out of bounds
    /// * if the element type does not match the collection element type
    fn set(&mut self, index: usize, value: ReflectValueBox);
    /// Append element.
    ///
    /// # Panics
    ///
    /// * if the element type does not match the collection element type
    fn push(&mut self, value: ReflectValueBox);
    fn clear(&mut self);
    /// Get the collection element type.
    fn element_type(&self) -> RuntimeTypeBox;

    /// Get array data for enum elements.
    ///
    /// # Panics
    ///
    /// * if the element type is not an enum
    fn data_enum_values(&self) -> &[i32];

    /// Get array data if the element type is bool.
    fn data_bool(&self) -> &[bool];
    /// Get array data if the element type is i32.
    fn data_i32(&self) -> &[i32];
    /// Get array data if the element type is u32.
    fn data_u32(&self) -> &[u32];
    /// Get array data if the element type is i64.
    fn data_i64(&self) -> &[i64];
    /// Get array data if the element type is u64.
    fn data_u64(&self) -> &[u64];
    /// Get array data if the element type is f32.
    fn data_f32(&self) -> &[f32];
    /// Get array data if the element type is f64.
    fn data_f64(&self) -> &[f64];
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

    fn data_enum_values(&self) -> &[i32] {
        self.as_slice().data_enum_values()
    }

    fn data_bool(&self) -> &[bool] {
        self.as_slice().data_bool()
    }

    fn data_i32(&self) -> &[i32] {
        self.as_slice().data_i32()
    }

    fn data_u32(&self) -> &[u32] {
        self.as_slice().data_u32()
    }

    fn data_i64(&self) -> &[i64] {
        self.as_slice().data_i64()
    }

    fn data_u64(&self) -> &[u64] {
        self.as_slice().data_u64()
    }

    fn data_f32(&self) -> &[f32] {
        self.as_slice().data_f32()
    }

    fn data_f64(&self) -> &[f64] {
        self.as_slice().data_f64()
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

    fn data_enum_values(&self) -> &[i32] {
        V::cast_to_enum_values(&self)
    }

    fn data_bool(&self) -> &[bool] {
        if TypeId::of::<Self>() == TypeId::of::<[bool]>() {
            // Safety: we've just checked types
            unsafe { mem::transmute(self) }
        } else {
            panic!("not bool");
        }
    }

    fn data_i32(&self) -> &[i32] {
        if TypeId::of::<Self>() == TypeId::of::<[i32]>() {
            // Safety: we've just checked types
            unsafe { mem::transmute(self) }
        } else {
            panic!("not i32");
        }
    }

    fn data_u32(&self) -> &[u32] {
        if TypeId::of::<Self>() == TypeId::of::<[u32]>() {
            // Safety: we've just checked types
            unsafe { mem::transmute(self) }
        } else {
            panic!("not u32");
        }
    }

    fn data_i64(&self) -> &[i64] {
        if TypeId::of::<Self>() == TypeId::of::<[i64]>() {
            // Safety: we've just checked types
            unsafe { mem::transmute(self) }
        } else {
            panic!("not i64");
        }
    }

    fn data_u64(&self) -> &[u64] {
        if TypeId::of::<Self>() == TypeId::of::<[u64]>() {
            // Safety: we've just checked types
            unsafe { mem::transmute(self) }
        } else {
            panic!("not u64");
        }
    }

    fn data_f32(&self) -> &[f32] {
        if TypeId::of::<Self>() == TypeId::of::<[f32]>() {
            // Safety: we've just checked types
            unsafe { mem::transmute(self) }
        } else {
            panic!("not f32");
        }
    }

    fn data_f64(&self) -> &[f64] {
        if TypeId::of::<Self>() == TypeId::of::<[f64]>() {
            // Safety: we've just checked types
            unsafe { mem::transmute(self) }
        } else {
            panic!("not f64");
        }
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

    pub(crate) fn data_enum_values(&self) -> &[i32] {
        match &self.imp {
            ReflectRepeatedRefImpl::Generated(r) => r.data_enum_values(),
            ReflectRepeatedRefImpl::DynamicEmpty(r) => r.data_enum_values(),
        }
    }

    pub(crate) fn data_bool(&self) -> &[bool] {
        match &self.imp {
            ReflectRepeatedRefImpl::Generated(r) => r.data_bool(),
            ReflectRepeatedRefImpl::DynamicEmpty(r) => r.data_bool(),
        }
    }

    pub(crate) fn data_u32(&self) -> &[u32] {
        match &self.imp {
            ReflectRepeatedRefImpl::Generated(r) => r.data_u32(),
            ReflectRepeatedRefImpl::DynamicEmpty(r) => r.data_u32(),
        }
    }

    pub(crate) fn data_i32(&self) -> &[i32] {
        match &self.imp {
            ReflectRepeatedRefImpl::Generated(r) => r.data_i32(),
            ReflectRepeatedRefImpl::DynamicEmpty(r) => r.data_i32(),
        }
    }

    pub(crate) fn data_u64(&self) -> &[u64] {
        match &self.imp {
            ReflectRepeatedRefImpl::Generated(r) => r.data_u64(),
            ReflectRepeatedRefImpl::DynamicEmpty(r) => r.data_u64(),
        }
    }

    pub(crate) fn data_i64(&self) -> &[i64] {
        match &self.imp {
            ReflectRepeatedRefImpl::Generated(r) => r.data_i64(),
            ReflectRepeatedRefImpl::DynamicEmpty(r) => r.data_i64(),
        }
    }

    pub(crate) fn data_f32(&self) -> &[f32] {
        match &self.imp {
            ReflectRepeatedRefImpl::Generated(r) => r.data_f32(),
            ReflectRepeatedRefImpl::DynamicEmpty(r) => r.data_f32(),
        }
    }

    pub(crate) fn data_f64(&self) -> &[f64] {
        match &self.imp {
            ReflectRepeatedRefImpl::Generated(r) => r.data_f64(),
            ReflectRepeatedRefImpl::DynamicEmpty(r) => r.data_f64(),
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

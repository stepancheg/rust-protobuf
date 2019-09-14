use std::{fmt, slice};

use super::value::ProtobufValue;
use super::value::ReflectValueRef;

use crate::reflect::runtime_type_dynamic::RuntimeTypeDynamic;
use crate::reflect::ReflectValueBox;
use crate::repeated::RepeatedField;

pub trait ReflectRepeated: Sync + 'static + fmt::Debug {
    fn reflect_iter(&self) -> ReflectRepeatedIter;
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> &dyn ProtobufValue;
    fn set(&mut self, index: usize, value: ReflectValueBox);
    fn push(&mut self, value: ReflectValueBox);
    fn clear(&mut self);
}

impl<V: ProtobufValue + fmt::Debug + 'static> ReflectRepeated for Vec<V> {
    fn reflect_iter<'a>(&'a self) -> ReflectRepeatedIter<'a> {
        ReflectRepeatedIter {
            imp: Box::new(ReflectRepeatedIterImplSlice::<'a, V> { iter: self.iter() }),
        }
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn get(&self, index: usize) -> &dyn ProtobufValue {
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

    fn clear(&mut self) {
        self.clear()
    }
}

// useless
impl<V: ProtobufValue + fmt::Debug + 'static> ReflectRepeated for [V] {
    fn reflect_iter<'a>(&'a self) -> ReflectRepeatedIter<'a> {
        ReflectRepeatedIter {
            imp: Box::new(ReflectRepeatedIterImplSlice::<'a, V> { iter: self.iter() }),
        }
    }

    fn len(&self) -> usize {
        <[_]>::len(self)
    }

    fn get(&self, index: usize) -> &dyn ProtobufValue {
        &self[index]
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
}

impl<V: ProtobufValue + fmt::Debug + 'static> ReflectRepeated for RepeatedField<V> {
    fn reflect_iter<'a>(&'a self) -> ReflectRepeatedIter<'a> {
        ReflectRepeatedIter {
            imp: Box::new(ReflectRepeatedIterImplSlice::<'a, V> { iter: self.iter() }),
        }
    }

    fn len(&self) -> usize {
        RepeatedField::len(self)
    }

    fn get(&self, index: usize) -> &dyn ProtobufValue {
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

    fn clear(&mut self) {
        self.clear()
    }
}

trait ReflectRepeatedIterTrait<'a> {
    fn next(&mut self) -> Option<&'a dyn ProtobufValue>;
}

struct ReflectRepeatedIterImplSlice<'a, V: ProtobufValue + 'static> {
    iter: slice::Iter<'a, V>,
}

impl<'a, V: ProtobufValue + 'static> ReflectRepeatedIterTrait<'a>
    for ReflectRepeatedIterImplSlice<'a, V>
{
    fn next(&mut self) -> Option<&'a dyn ProtobufValue> {
        self.iter.next().map(|v| v as &dyn ProtobufValue)
    }
}

pub struct ReflectRepeatedIter<'a> {
    imp: Box<dyn ReflectRepeatedIterTrait<'a> + 'a>,
}

impl<'a> Iterator for ReflectRepeatedIter<'a> {
    type Item = &'a dyn ProtobufValue;

    fn next(&mut self) -> Option<Self::Item> {
        self.imp.next()
    }
}

impl<'a> IntoIterator for &'a dyn ReflectRepeated {
    type Item = &'a dyn ProtobufValue;
    type IntoIter = ReflectRepeatedIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.reflect_iter()
    }
}

/// Dynamic reference to repeated field
#[derive(Copy, Clone)]
pub struct ReflectRepeatedRef<'a> {
    pub(crate) repeated: &'a dyn ReflectRepeated,
    pub(crate) dynamic: &'static dyn RuntimeTypeDynamic,
}

/// Dynamic mutable reference to repeated field
pub struct ReflectRepeatedMut<'a> {
    pub(crate) repeated: &'a mut dyn ReflectRepeated,
    pub(crate) dynamic: &'static dyn RuntimeTypeDynamic,
}

impl<'a> ReflectRepeatedRef<'a> {
    /// Number of elements in repeated field
    pub fn len(&self) -> usize {
        self.repeated.len()
    }

    /// Repeated field is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get item by index
    // TODO: replace with index
    pub fn get(&self, index: usize) -> ReflectValueRef<'a> {
        self.dynamic.value_to_ref(self.repeated.get(index))
    }

    /// Runtime type of element
    pub fn element_type(&self) -> &dyn RuntimeTypeDynamic {
        self.dynamic
    }
}

impl<'a> ReflectRepeatedMut<'a> {
    fn as_ref(&'a self) -> ReflectRepeatedRef<'a> {
        ReflectRepeatedRef {
            repeated: self.repeated,
            dynamic: self.dynamic,
        }
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

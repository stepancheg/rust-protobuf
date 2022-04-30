use std::fmt;

use crate::reflect::map::ReflectMapIter;
use crate::reflect::map::ReflectMapIterTrait;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeTypeBox;

#[derive(Clone)]
pub(crate) struct DynamicEmptyMap {
    key_type: RuntimeTypeBox,
    value_type: RuntimeTypeBox,
}

impl DynamicEmptyMap {
    pub(crate) fn new(key_type: RuntimeTypeBox, value_type: RuntimeTypeBox) -> DynamicEmptyMap {
        Self {
            key_type,
            value_type,
        }
    }

    pub(crate) fn len(&self) -> usize {
        0
    }

    pub(crate) fn is_empty(&self) -> bool {
        true
    }

    pub(crate) fn get<'a>(&'a self, _key: ReflectValueRef) -> Option<ReflectValueRef<'a>> {
        None
    }

    pub(crate) fn key_type(&self) -> RuntimeTypeBox {
        self.key_type.clone()
    }

    pub(crate) fn value_type(&self) -> RuntimeTypeBox {
        self.value_type.clone()
    }

    pub(crate) fn reflect_iter(&self) -> ReflectMapIter {
        ReflectMapIter::new(DynamicEmptyMapIter {
            key_type: self.key_type.clone(),
            value_type: self.value_type.clone(),
        })
    }
}

impl fmt::Debug for DynamicEmptyMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().finish()
    }
}

struct DynamicEmptyMapIter {
    key_type: RuntimeTypeBox,
    value_type: RuntimeTypeBox,
}

impl<'a> ReflectMapIterTrait<'a> for DynamicEmptyMapIter {
    fn next(&mut self) -> Option<(ReflectValueRef<'a>, ReflectValueRef<'a>)> {
        None
    }

    fn key_type(&self) -> RuntimeTypeBox {
        self.key_type.clone()
    }

    fn value_type(&self) -> RuntimeTypeBox {
        self.value_type.clone()
    }
}

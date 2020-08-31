use crate::reflect::map::ReflectMap;
use crate::reflect::map::ReflectMapIter;
use crate::reflect::value::hashable::ReflectValueBoxHashable;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeTypeBox;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(crate) struct DynamicMap {
    key: RuntimeTypeBox,
    value: RuntimeTypeBox,
    map: HashMap<ReflectValueBoxHashable, ReflectValueBox>,
}

impl DynamicMap {
    pub fn new(key: RuntimeTypeBox, value: RuntimeTypeBox) -> DynamicMap {
        DynamicMap {
            key,
            value,
            map: HashMap::new(),
        }
    }
}

impl ReflectMap for DynamicMap {
    fn reflect_iter(&self) -> ReflectMapIter {
        // TODO
        unimplemented!()
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn get<'a>(&'a self, key: ReflectValueRef) -> Option<ReflectValueRef<'a>> {
        if key.get_type() != self.key {
            return None;
        }

        match key {
            ReflectValueRef::String(s) => self.map.get(s),
            key => self
                .map
                .get(&ReflectValueBoxHashable::from_box(key.to_box())),
        }
        .map(ReflectValueBox::as_value_ref)
    }

    fn insert(&mut self, key: ReflectValueBoxHashable, value: ReflectValueBox) {
        assert!(key.get_type() == self.key);
        assert!(value.get_type() == self.value);
        self.map.insert(key, value);
    }

    fn clear(&mut self) {
        self.map.clear()
    }

    fn key_type(&self) -> RuntimeTypeBox {
        self.key.clone()
    }

    fn value_type(&self) -> RuntimeTypeBox {
        self.value.clone()
    }
}

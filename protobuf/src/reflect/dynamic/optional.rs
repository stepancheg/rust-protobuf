use crate::reflect::value::ReflectValueMut;
use crate::reflect::{ReflectValueBox, RuntimeTypeBox};

#[derive(Debug, Clone)]
pub(crate) struct DynamicOptional {
    elem: RuntimeTypeBox,
    value: Option<ReflectValueBox>,
}

impl DynamicOptional {
    pub fn none(elem: RuntimeTypeBox) -> DynamicOptional {
        DynamicOptional { elem, value: None }
    }

    pub fn mut_or_default(&mut self) -> ReflectValueMut {
        if let None = self.value {
            self.value = Some(self.elem.default_value_ref().to_box());
        }
        self.value.as_mut().unwrap().as_value_mut()
    }

    pub fn clear(&mut self) {
        self.value = None;
    }
}

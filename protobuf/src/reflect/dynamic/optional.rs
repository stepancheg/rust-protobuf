use crate::reflect::value::value_ref::ReflectValueMut;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeTypeBox;

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

    pub fn get(&self) -> Option<ReflectValueRef> {
        self.value.as_ref().map(ReflectValueBox::as_value_ref)
    }

    pub fn set(&mut self, value: ReflectValueBox) {
        assert_eq!(value.get_type(), self.elem);
        self.value = Some(value);
    }
}

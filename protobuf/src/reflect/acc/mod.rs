#![doc(hidden)]

use crate::reflect::acc::v1::FieldAccessorTrait;
use crate::reflect::{EnumValueDescriptor, ReflectFieldRef};
use crate::Message;

pub(crate) mod v1;

pub(crate) enum Accessor {
    V1(Box<dyn FieldAccessorTrait + 'static>),
}

/// Accessor object is constructed in generated code.
/// Should not be used directly.
pub struct FieldAccessor {
    //pub(crate) name: &'static str,
    pub(crate) accessor: Accessor,
}

impl FieldAccessor {
    pub(crate) fn new_v1<F: FieldAccessorTrait>(f: F) -> FieldAccessor {
        FieldAccessor {
            accessor: Accessor::V1(Box::new(f)),
        }
    }

    pub(crate) fn name_generic(&self) -> &str {
        match &self.accessor {
            Accessor::V1(a) => a.name_generic(),
        }
    }

    pub(crate) fn has_field_generic(&self, m: &dyn Message) -> bool {
        match &self.accessor {
            Accessor::V1(a) => a.has_field_generic(m),
        }
    }

    pub(crate) fn len_field_generic(&self, m: &dyn Message) -> usize {
        match &self.accessor {
            Accessor::V1(a) => a.len_field_generic(m),
        }
    }
    pub(crate) fn get_message_generic<'a>(&self, m: &'a dyn Message) -> &'a dyn Message {
        match &self.accessor {
            Accessor::V1(a) => a.get_message_generic(m),
        }
    }
    pub(crate) fn get_enum_generic(&self, m: &dyn Message) -> &'static EnumValueDescriptor {
        match &self.accessor {
            Accessor::V1(a) => a.get_enum_generic(m),
        }
    }
    pub(crate) fn get_str_generic<'a>(&self, m: &'a dyn Message) -> &'a str {
        match &self.accessor {
            Accessor::V1(a) => a.get_str_generic(m),
        }
    }
    pub(crate) fn get_bytes_generic<'a>(&self, m: &'a dyn Message) -> &'a [u8] {
        match &self.accessor {
            Accessor::V1(a) => a.get_bytes_generic(m),
        }
    }
    pub(crate) fn get_u32_generic(&self, m: &dyn Message) -> u32 {
        match &self.accessor {
            Accessor::V1(a) => a.get_u32_generic(m),
        }
    }
    pub(crate) fn get_u64_generic(&self, m: &dyn Message) -> u64 {
        match &self.accessor {
            Accessor::V1(a) => a.get_u64_generic(m),
        }
    }
    pub(crate) fn get_i32_generic(&self, m: &dyn Message) -> i32 {
        match &self.accessor {
            Accessor::V1(a) => a.get_i32_generic(m),
        }
    }
    pub(crate) fn get_i64_generic(&self, m: &dyn Message) -> i64 {
        match &self.accessor {
            Accessor::V1(a) => a.get_i64_generic(m),
        }
    }
    pub(crate) fn get_bool_generic(&self, m: &dyn Message) -> bool {
        match &self.accessor {
            Accessor::V1(a) => a.get_bool_generic(m),
        }
    }
    pub(crate) fn get_f32_generic(&self, m: &dyn Message) -> f32 {
        match &self.accessor {
            Accessor::V1(a) => a.get_f32_generic(m),
        }
    }
    pub(crate) fn get_f64_generic(&self, m: &dyn Message) -> f64 {
        match &self.accessor {
            Accessor::V1(a) => a.get_f64_generic(m),
        }
    }

    pub(crate) fn get_reflect<'a>(&self, m: &'a dyn Message) -> ReflectFieldRef<'a> {
        match &self.accessor {
            Accessor::V1(a) => a.get_reflect(m),
        }
    }
}

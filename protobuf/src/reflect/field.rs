use crate::descriptor::FieldDescriptorProto;
use crate::descriptor::FieldDescriptorProto_Label;
use crate::reflect::accessor::AccessorKind;
use crate::reflect::accessor::FieldAccessor;
use crate::reflect::map::ReflectMapRef;
use crate::reflect::repeated::ReflectRepeatedRef;
use crate::reflect::EnumValueDescriptor;
use crate::reflect::ReflectValueRef;
use crate::Message;

/// Reference to a value stored in a field, optional, repeated or map.
// TODO: implement Eq
pub enum ReflectFieldRef<'a> {
    /// Singular field, optional or required in proto3 and just plain field in proto3
    Optional(Option<ReflectValueRef<'a>>),
    /// Repeated field
    Repeated(ReflectRepeatedRef<'a>),
    /// Map field
    Map(ReflectMapRef<'a>),
}

/// Field descriptor.
///
/// Can be used for runtime reflection.
pub struct FieldDescriptor {
    proto: &'static FieldDescriptorProto,
    accessor: FieldAccessor,
}

impl FieldDescriptor {
    pub(crate) fn new(
        accessor: FieldAccessor,
        proto: &'static FieldDescriptorProto,
    ) -> FieldDescriptor {
        assert_eq!(proto.get_name(), accessor.name);
        FieldDescriptor { proto, accessor }
    }

    /// Get `.proto` description of field
    pub fn proto(&self) -> &'static FieldDescriptorProto {
        self.proto
    }

    /// Field name as specified in `.proto` file
    pub fn name(&self) -> &'static str {
        self.proto.get_name()
    }

    /// If this field repeated?
    pub fn is_repeated(&self) -> bool {
        self.proto.get_label() == FieldDescriptorProto_Label::LABEL_REPEATED
    }

    /// Check if field is set in given message.
    ///
    /// For repeated field or map field return `true` if
    /// collection is not empty.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type.
    pub fn has_field(&self, m: &dyn Message) -> bool {
        match &self.accessor.accessor {
            AccessorKind::Old(a) => a.has_field_generic(m),
            AccessorKind::Repeated(a) => !a.accessor.get_reflect(m).is_empty(),
            AccessorKind::Map(a) => !a.accessor.get_reflect(m).is_empty(),
        }
    }

    /// Return length of repeated field.
    ///
    /// For singualar field return `1` if field is set and `0` otherwise.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type.
    pub fn len_field(&self, m: &dyn Message) -> usize {
        match &self.accessor.accessor {
            AccessorKind::Old(a) => if a.has_field_generic(m) { 1 } else { 0 },
            AccessorKind::Repeated(a) => a.accessor.get_reflect(m).len(),
            AccessorKind::Map(a) => a.accessor.get_reflect(m).len(),
        }
    }

    /// Get message field or default instance if field is unset.
    ///
    /// # Panics
    /// If this field belongs to a different message type or
    /// field type is not message.
    pub fn get_message<'a>(&self, m: &'a dyn Message) -> &'a dyn Message {
        match &self.accessor.accessor {
            AccessorKind::Old(a) => a.get_message_generic(m),
            AccessorKind::Repeated(..) | AccessorKind::Map(..) => panic!("not a singular field"),
        }
    }

    /// Get `enum` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type
    /// or field type is not singular `enum`.
    pub fn get_enum(&self, m: &dyn Message) -> &'static EnumValueDescriptor {
        match &self.accessor.accessor {
            AccessorKind::Old(a) => a.get_enum_generic(m),
            AccessorKind::Repeated(..) | AccessorKind::Map(..) => panic!("not a singular field"),
        }
    }

    /// Get `string` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type
    /// or field type is not singular `string`.
    pub fn get_str<'a>(&self, m: &'a dyn Message) -> &'a str {
        match &self.accessor.accessor {
            AccessorKind::Old(a) => a.get_str_generic(m),
            AccessorKind::Repeated(..) | AccessorKind::Map(..) => panic!("not a singular field"),
        }
    }

    /// Get `bytes` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type
    /// or field type is not singular `bytes`.
    pub fn get_bytes<'a>(&self, m: &'a dyn Message) -> &'a [u8] {
        match &self.accessor.accessor {
            AccessorKind::Old(a) => a.get_bytes_generic(m),
            AccessorKind::Repeated(..) | AccessorKind::Map(..) => panic!("not a singular field"),
        }
    }

    /// Get `u32` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type
    /// or field type is not singular `u32`.
    pub fn get_u32(&self, m: &dyn Message) -> u32 {
        match &self.accessor.accessor {
            AccessorKind::Old(a) => a.get_u32_generic(m),
            AccessorKind::Repeated(..) | AccessorKind::Map(..) => panic!("not a singular field"),
        }
    }

    /// Get `u64` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type
    /// or field type is not singular `u64`.
    pub fn get_u64(&self, m: &dyn Message) -> u64 {
        match &self.accessor.accessor {
            AccessorKind::Old(a) => a.get_u64_generic(m),
            AccessorKind::Repeated(..) | AccessorKind::Map(..) => panic!("not a singular field"),
        }
    }

    /// Get `i32` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type
    /// or field type is not singular `i32`.
    pub fn get_i32(&self, m: &dyn Message) -> i32 {
        match &self.accessor.accessor {
            AccessorKind::Old(a) => a.get_i32_generic(m),
            AccessorKind::Repeated(..) | AccessorKind::Map(..) => panic!("not a singular field"),
        }
    }

    /// Get `i64` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type
    /// or field type is not singular `i64`.
    pub fn get_i64(&self, m: &dyn Message) -> i64 {
        match &self.accessor.accessor {
            AccessorKind::Old(a) => a.get_i64_generic(m),
            AccessorKind::Repeated(..) | AccessorKind::Map(..) => panic!("not a singular field"),
        }
    }

    /// Get `bool` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or
    /// field type is not singular `bool`.
    pub fn get_bool(&self, m: &dyn Message) -> bool {
        match &self.accessor.accessor {
            AccessorKind::Old(a) => a.get_bool_generic(m),
            AccessorKind::Repeated(..) | AccessorKind::Map(..) => panic!("not a singular field"),
        }
    }

    /// Get `float` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or
    /// field type is not singular `float`.
    pub fn get_f32(&self, m: &dyn Message) -> f32 {
        match &self.accessor.accessor {
            AccessorKind::Old(a) => a.get_f32_generic(m),
            AccessorKind::Repeated(..) | AccessorKind::Map(..) => panic!("not a singular field"),
        }
    }

    /// Get `double` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type
    /// or field type is not singular `double`.
    pub fn get_f64(&self, m: &dyn Message) -> f64 {
        match &self.accessor.accessor {
            AccessorKind::Old(a) => a.get_f64_generic(m),
            AccessorKind::Repeated(..) | AccessorKind::Map(..) => panic!("not a singular field"),
        }
    }

    /// Get field of any type.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type.
    pub fn get_reflect<'a>(&self, m: &'a dyn Message) -> ReflectFieldRef<'a> {
        match &self.accessor.accessor {
            AccessorKind::Old(a) => a.get_reflect(m),
            AccessorKind::Repeated(a) => ReflectFieldRef::Repeated(a.accessor.get_reflect(m)),
            AccessorKind::Map(a) => ReflectFieldRef::Map(a.accessor.get_reflect(m)),
        }
    }
}

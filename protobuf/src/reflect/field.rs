use crate::descriptor::FieldDescriptorProto;
use crate::descriptor::FieldDescriptorProto_Label;
use crate::reflect::accessor::map::MapFieldAccessorHolder;
use crate::reflect::accessor::repeated::RepeatedFieldAccessorHolder;
use crate::reflect::accessor::singular::SingularFieldAccessorHolder;
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
            AccessorKind::Singular(a) => a.accessor.get_field(m).is_some(),
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
    #[deprecated]
    pub fn len_field(&self, m: &dyn Message) -> usize {
        match &self.accessor.accessor {
            AccessorKind::Singular(a) => {
                if a.accessor.get_field(m).is_some() {
                    1
                } else {
                    0
                }
            }
            AccessorKind::Repeated(a) => a.accessor.get_reflect(m).len(),
            AccessorKind::Map(a) => a.accessor.get_reflect(m).len(),
        }
    }

    // accessors

    fn singular(&self) -> &SingularFieldAccessorHolder {
        match self.accessor.accessor {
            AccessorKind::Singular(ref a) => a,
            _ => panic!("not a singular field: {}", self.name()),
        }
    }

    fn repeated(&self) -> &RepeatedFieldAccessorHolder {
        match self.accessor.accessor {
            AccessorKind::Repeated(ref a) => a,
            _ => panic!("not a repeated field: {}", self.name()),
        }
    }

    fn map(&self) -> &MapFieldAccessorHolder {
        match self.accessor.accessor {
            AccessorKind::Map(ref a) => a,
            _ => panic!("not a map field: {}", self.name()),
        }
    }

    /// Get message field or default instance if field is unset.
    ///
    /// # Panics
    /// If this field belongs to a different message type or
    /// field type is not message.
    #[deprecated]
    pub fn get_message<'a>(&self, m: &'a dyn Message) -> &'a dyn Message {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::Message(m) => m,
            _ => panic!("not message"),
        }
    }

    /// Get `enum` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type
    /// or field type is not singular `enum`.
    #[deprecated]
    pub fn get_enum(&self, m: &dyn Message) -> &'static EnumValueDescriptor {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::Enum(v) => v,
            _ => panic!("not enum"),
        }
    }

    /// Get `string` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type
    /// or field type is not singular `string`.
    #[deprecated]
    pub fn get_str<'a>(&self, m: &'a dyn Message) -> &'a str {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::String(v) => v,
            _ => panic!("not string"),
        }
    }

    /// Get `bytes` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type
    /// or field type is not singular `bytes`.
    #[deprecated]
    pub fn get_bytes<'a>(&self, m: &'a dyn Message) -> &'a [u8] {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::Bytes(v) => v,
            _ => panic!("not bytes"),
        }
    }

    /// Get `u32` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type
    /// or field type is not singular `u32`.
    #[deprecated]
    pub fn get_u32(&self, m: &dyn Message) -> u32 {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::U32(v) => v,
            _ => panic!("not u32"),
        }
    }

    /// Get `u64` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type
    /// or field type is not singular `u64`.
    #[deprecated]
    pub fn get_u64(&self, m: &dyn Message) -> u64 {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::U64(v) => v,
            _ => panic!("not u64"),
        }
    }

    /// Get `i32` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type
    /// or field type is not singular `i32`.
    #[deprecated]
    pub fn get_i32(&self, m: &dyn Message) -> i32 {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::I32(v) => v,
            _ => panic!("not i32"),
        }
    }

    /// Get `i64` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type
    /// or field type is not singular `i64`.
    #[deprecated]
    pub fn get_i64(&self, m: &dyn Message) -> i64 {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::I64(v) => v,
            _ => panic!("not i64"),
        }
    }

    /// Get `bool` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or
    /// field type is not singular `bool`.
    #[deprecated]
    pub fn get_bool(&self, m: &dyn Message) -> bool {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::Bool(v) => v,
            _ => panic!("not bool"),
        }
    }

    /// Get `float` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or
    /// field type is not singular `float`.
    #[deprecated]
    pub fn get_f32(&self, m: &dyn Message) -> f32 {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::F32(v) => v,
            _ => panic!("not f32"),
        }
    }

    /// Get `double` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type
    /// or field type is not singular `double`.
    #[deprecated]
    pub fn get_f64(&self, m: &dyn Message) -> f64 {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::F64(v) => v,
            _ => panic!("not f64"),
        }
    }

    /// Get singular field value.
    ///
    /// Return field default value if field is unset.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or fields is not singular.
    pub fn get_singular_field_or_default<'a>(&self, m: &'a dyn Message) -> ReflectValueRef<'a> {
        self.singular().accessor.get_field_or_default(m)
    }

    /// Get field of any type.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type.
    pub fn get_reflect<'a>(&self, m: &'a dyn Message) -> ReflectFieldRef<'a> {
        match &self.accessor.accessor {
            AccessorKind::Singular(a) => ReflectFieldRef::Optional(a.accessor.get_field(m)),
            AccessorKind::Repeated(a) => ReflectFieldRef::Repeated(a.accessor.get_reflect(m)),
            AccessorKind::Map(a) => ReflectFieldRef::Map(a.accessor.get_reflect(m)),
        }
    }
}

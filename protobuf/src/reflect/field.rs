use crate::core::Message;
use crate::descriptor::field_descriptor_proto;
use crate::descriptor::FieldDescriptorProto;
use crate::json::json_name;
use crate::reflect::accessor::map::MapFieldAccessorHolder;
use crate::reflect::accessor::repeated::RepeatedFieldAccessorHolder;
use crate::reflect::accessor::singular::SingularFieldAccessorHolder;
use crate::reflect::accessor::AccessorKind;
use crate::reflect::accessor::FieldAccessor;
use crate::reflect::map::ReflectMapMut;
use crate::reflect::map::ReflectMapRef;
use crate::reflect::reflect_eq::{ReflectEq, ReflectEqMode};
use crate::reflect::repeated::ReflectRepeatedMut;
use crate::reflect::repeated::ReflectRepeatedRef;
use crate::reflect::value::ReflectValueMut;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeTypeDynamic;

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

impl<'a> ReflectEq for ReflectFieldRef<'a> {
    fn reflect_eq(&self, that: &Self, mode: &ReflectEqMode) -> bool {
        match (self, that) {
            (ReflectFieldRef::Optional(a), ReflectFieldRef::Optional(b)) => match (a, b) {
                (Some(av), Some(bv)) => av.reflect_eq(&bv, mode),
                (None, None) => true,
                _ => false,
            },
            (ReflectFieldRef::Repeated(a), ReflectFieldRef::Repeated(b)) => a.reflect_eq(b, mode),
            (ReflectFieldRef::Map(a), ReflectFieldRef::Map(b)) => a.reflect_eq(b, mode),
            _ => false,
        }
    }
}

/// Reflective representation of field type
pub enum RuntimeFieldType {
    /// Singular field (required, optional for proto2 or singular for proto3)
    Singular(&'static dyn RuntimeTypeDynamic),
    /// Repeated field
    Repeated(&'static dyn RuntimeTypeDynamic),
    /// Map field
    Map(
        &'static dyn RuntimeTypeDynamic,
        &'static dyn RuntimeTypeDynamic,
    ),
}

fn _assert_sync<'a>() {
    fn _assert_send_sync<T: Sync>() {}
    _assert_send_sync::<ReflectFieldRef<'a>>();
}

/// Field descriptor.
///
/// Can be used for runtime reflection.
pub struct FieldDescriptor {
    proto: &'static FieldDescriptorProto,
    accessor: FieldAccessor,
    json_name: String,
}

impl FieldDescriptor {
    pub(crate) fn new(
        accessor: FieldAccessor,
        proto: &'static FieldDescriptorProto,
    ) -> FieldDescriptor {
        assert_eq!(proto.get_name(), accessor.name);
        let json_name = if !proto.get_json_name().is_empty() {
            proto.get_json_name().to_string()
        } else {
            json_name(proto.get_name())
        };
        FieldDescriptor {
            proto,
            accessor,
            // probably could be lazy-init
            json_name,
        }
    }

    /// Get `.proto` description of field
    pub fn proto(&self) -> &'static FieldDescriptorProto {
        self.proto
    }

    /// Field name as specified in `.proto` file
    pub fn name(&self) -> &str {
        self.accessor.name
    }

    /// JSON field name.
    ///
    /// Can be different from `.proto` field name.
    ///
    /// See [JSON mapping][json] for details.
    ///
    /// [json]: https://developers.google.com/protocol-buffers/docs/proto3#json
    pub fn json_name(&self) -> &str {
        &self.json_name
    }

    /// If this field repeated?
    pub fn is_repeated(&self) -> bool {
        self.proto.get_label() == field_descriptor_proto::Label::LABEL_REPEATED
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
        match self.accessor.accessor {
            AccessorKind::Singular(ref a) => a.accessor.get_field(m).is_some(),
            AccessorKind::Repeated(ref a) => a.accessor.get_reflect(m).len() != 0,
            AccessorKind::Map(ref a) => a.accessor.get_reflect(m).len() != 0,
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
    pub fn get_message<'a>(&self, m: &'a dyn Message) -> &'a dyn Message {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::Message(m) => m,
            _ => panic!("not message"),
        }
    }

    /// Get a mutable reference to a message field.
    /// Initialize field with default message if unset.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or
    /// field type is not singular message.
    pub fn mut_message<'a>(&self, m: &'a mut dyn Message) -> &'a mut dyn Message {
        match self.mut_singular_field_or_default(m) {
            ReflectValueMut::Message(m) => m,
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

    // Not public because it is not implemented for all types
    fn mut_singular_field_or_default<'a>(&self, m: &'a mut dyn Message) -> ReflectValueMut<'a> {
        self.singular().accessor.mut_field_or_default(m)
    }

    /// Runtime representation of singular field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or field is not singular.
    pub fn singular_runtime_type(&self) -> &dyn RuntimeTypeDynamic {
        self.singular().element_type.runtime_type()
    }

    /// Set singular field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or
    /// field is not singular or value is of different type.
    pub fn set_singular_field(&self, m: &mut dyn Message, value: ReflectValueBox) {
        self.singular().accessor.set_field(m, value)
    }

    /// Dynamic representation of field type.
    pub fn runtime_field_type(&self) -> RuntimeFieldType {
        use self::AccessorKind::*;
        match self.accessor.accessor {
            Singular(ref a) => RuntimeFieldType::Singular(a.element_type.runtime_type()),
            Repeated(ref a) => RuntimeFieldType::Repeated(a.element_type.runtime_type()),
            Map(ref a) => {
                RuntimeFieldType::Map(a.key_type.runtime_type(), a.value_type.runtime_type())
            }
        }
    }

    /// Get field of any type.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type.
    pub fn get_reflect<'a>(&self, m: &'a dyn Message) -> ReflectFieldRef<'a> {
        use self::AccessorKind::*;
        match self.accessor.accessor {
            Singular(ref a) => ReflectFieldRef::Optional(a.accessor.get_field(m)),
            Repeated(ref a) => ReflectFieldRef::Repeated(a.accessor.get_reflect(m)),
            Map(ref a) => ReflectFieldRef::Map(a.accessor.get_reflect(m)),
        }
    }

    // repeated

    /// Get repeated field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or field is not repeated.
    pub fn get_repeated<'a>(&self, m: &'a dyn Message) -> ReflectRepeatedRef<'a> {
        self.repeated().accessor.get_reflect(m)
    }

    /// Get a mutable reference to `repeated` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or field is not `repeated`.
    pub fn mut_repeated<'a>(&self, m: &'a mut dyn Message) -> ReflectRepeatedMut<'a> {
        self.repeated().accessor.mut_reflect(m)
    }

    // map

    /// Get `map` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or field is not `map`.
    pub fn get_map<'a>(&self, m: &'a dyn Message) -> ReflectMapRef<'a> {
        self.map().accessor.get_reflect(m)
    }

    /// Get a mutable reference to `map` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or field is not `map`.
    pub fn mut_map<'a>(&self, m: &'a mut dyn Message) -> ReflectMapMut<'a> {
        self.map().accessor.mut_reflect(m)
    }
}

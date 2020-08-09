use crate::descriptor::field_descriptor_proto;
use crate::descriptor::FieldDescriptorProto;
use crate::message::Message;
use crate::reflect::acc::v2::map::MapFieldAccessorHolder;
use crate::reflect::acc::v2::repeated::RepeatedFieldAccessorHolder;
use crate::reflect::acc::v2::singular::SingularFieldAccessorHolder;
use crate::reflect::acc::v2::AccessorV2;
use crate::reflect::acc::FieldAccessorImpl;
use crate::reflect::map::ReflectMapMut;
use crate::reflect::map::ReflectMapRef;
use crate::reflect::reflect_eq::ReflectEq;
use crate::reflect::reflect_eq::ReflectEqMode;
use crate::reflect::repeated::ReflectRepeatedMut;
use crate::reflect::repeated::ReflectRepeatedRef;
use crate::reflect::value::MessageRef;
use crate::reflect::value::ReflectValueMut;
use crate::reflect::MessageDescriptor;
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

impl<'a> ReflectFieldRef<'a> {
    pub(crate) fn default_for_field(field: &FieldDescriptor) -> ReflectFieldRef<'a> {
        match field.runtime_field_type() {
            RuntimeFieldType::Singular(_) => ReflectFieldRef::Optional(None),
            RuntimeFieldType::Repeated(..) => unimplemented!(),
            RuntimeFieldType::Map(..) => unimplemented!(),
        }
    }
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
    pub(crate) message_descriptor: MessageDescriptor,
    pub(crate) index: usize,
}

impl FieldDescriptor {
    /// Get `.proto` description of field
    pub fn proto(&self) -> &FieldDescriptorProto {
        &self.message_descriptor.get_proto().field[self.index]
    }

    /// Field name as specified in `.proto` file
    pub fn name(&self) -> &str {
        // TODO: slow for dynamic
        self.proto().get_name()
    }

    /// JSON field name.
    ///
    /// Can be different from `.proto` field name.
    ///
    /// See [JSON mapping][json] for details.
    ///
    /// [json]: https://developers.google.com/protocol-buffers/docs/proto3#json
    pub fn json_name(&self) -> &str {
        &self.message_descriptor.get_indices().json_names[self.index]
    }

    /// If this field repeated?
    pub fn is_repeated(&self) -> bool {
        self.proto().get_label() == field_descriptor_proto::Label::LABEL_REPEATED
    }

    fn get_accessor(&self) -> &FieldAccessorImpl {
        self.message_descriptor.get_accessor(self.index)
    }

    /// If this field a map field?
    pub fn is_map(&self) -> bool {
        match self.get_accessor() {
            FieldAccessorImpl::V2(AccessorV2::Map(..)) => true,
            FieldAccessorImpl::V2(..) => false,
            FieldAccessorImpl::Dynamic => unimplemented!(), // TODO
        }
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
        match self.get_reflect(m) {
            ReflectFieldRef::Optional(Some(..)) => true,
            ReflectFieldRef::Optional(None) => false,
            ReflectFieldRef::Repeated(r) => !r.is_empty(),
            ReflectFieldRef::Map(m) => !m.is_empty(),
        }
    }

    // accessors

    fn singular(&self) -> &SingularFieldAccessorHolder {
        match self.get_accessor() {
            FieldAccessorImpl::V2(AccessorV2::Singular(ref a)) => a,
            FieldAccessorImpl::V2(..) => panic!("not a singular field: {}", self.name()),
            FieldAccessorImpl::Dynamic => unimplemented!(), // TODO
        }
    }

    fn repeated(&self) -> &RepeatedFieldAccessorHolder {
        match self.get_accessor() {
            FieldAccessorImpl::V2(AccessorV2::Repeated(ref a)) => a,
            FieldAccessorImpl::V2(..) => panic!("not a repeated field: {}", self.name()),
            FieldAccessorImpl::Dynamic => unimplemented!(), // TODO
        }
    }

    fn map(&self) -> &MapFieldAccessorHolder {
        match self.get_accessor() {
            FieldAccessorImpl::V2(AccessorV2::Map(ref a)) => a,
            FieldAccessorImpl::V2(..) => panic!("not a map field: {}", self.name()),
            FieldAccessorImpl::Dynamic => unimplemented!(), // TODO
        }
    }

    /// Get message field or default instance if field is unset.
    ///
    /// # Panics
    /// If this field belongs to a different message type or
    /// field type is not message.
    pub fn get_message<'a>(&self, m: &'a dyn Message) -> MessageRef<'a> {
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
        self.singular().element_type
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
        use self::AccessorV2::*;
        match self.get_accessor() {
            FieldAccessorImpl::V2(Singular(ref a)) => RuntimeFieldType::Singular(a.element_type),
            FieldAccessorImpl::V2(Repeated(ref a)) => RuntimeFieldType::Repeated(a.element_type),
            FieldAccessorImpl::V2(Map(ref a)) => RuntimeFieldType::Map(a.key_type, a.value_type),
            FieldAccessorImpl::Dynamic => unimplemented!(), // TODO
        }
    }

    /// Get field of any type.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type.
    pub fn get_reflect<'a>(&self, m: &'a dyn Message) -> ReflectFieldRef<'a> {
        use self::AccessorV2::*;
        match self.get_accessor() {
            FieldAccessorImpl::V2(Singular(ref a)) => {
                ReflectFieldRef::Optional(a.accessor.get_field(m))
            }
            FieldAccessorImpl::V2(Repeated(ref a)) => {
                ReflectFieldRef::Repeated(a.accessor.get_reflect(m))
            }
            FieldAccessorImpl::V2(Map(ref a)) => ReflectFieldRef::Map(a.accessor.get_reflect(m)),
            FieldAccessorImpl::Dynamic => unimplemented!(), // TODO
        }
    }

    // repeated

    /// Get repeated field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or field is not repeated.
    pub fn get_repeated<'a>(&self, m: &'a dyn Message) -> ReflectRepeatedRef<'a> {
        match self.get_reflect(m) {
            ReflectFieldRef::Repeated(r) => r,
            _ => panic!("not a repeated field"),
        }
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
        match self.get_reflect(m) {
            ReflectFieldRef::Map(m) => m,
            _ => panic!("not a map field"),
        }
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

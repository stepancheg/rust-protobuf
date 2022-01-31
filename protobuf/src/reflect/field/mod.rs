use std::fmt;

use crate::descriptor::field_descriptor_proto;
use crate::descriptor::FieldDescriptorProto;
use crate::message_dyn::MessageDyn;
use crate::reflect::acc::v2::map::MapFieldAccessorHolder;
use crate::reflect::acc::v2::repeated::RepeatedFieldAccessorHolder;
use crate::reflect::acc::v2::singular::SingularFieldAccessorHolder;
use crate::reflect::acc::v2::AccessorV2;
use crate::reflect::acc::GeneratedFieldAccessor;
use crate::reflect::dynamic::DynamicMessage;
use crate::reflect::field::dynamic::DynamicFieldDescriptorRef;
use crate::reflect::field::index::FieldIndex;
use crate::reflect::field::runtime_field_type::RuntimeFieldType;
use crate::reflect::map::ReflectMapMut;
use crate::reflect::map::ReflectMapRef;
use crate::reflect::message::message_ref::MessageRef;
use crate::reflect::message::MessageDescriptorImplRef;
use crate::reflect::oneof::OneofDescriptor;
use crate::reflect::reflect_eq::ReflectEq;
use crate::reflect::reflect_eq::ReflectEqMode;
use crate::reflect::repeated::ReflectRepeatedMut;
use crate::reflect::repeated::ReflectRepeatedRef;
use crate::reflect::value::value_ref::ReflectValueMut;
use crate::reflect::MessageDescriptor;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeTypeBox;

pub(crate) mod dynamic;
pub(crate) mod index;
pub(crate) mod runtime_field_type;

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
            RuntimeFieldType::Repeated(elem) => {
                ReflectFieldRef::Repeated(ReflectRepeatedRef::new_empty(elem))
            }
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

fn _assert_sync<'a>() {
    fn _assert_send_sync<T: Sync>() {}
    _assert_send_sync::<ReflectFieldRef<'a>>();
}

/// Field descriptor.
///
/// Can be used for runtime reflection.
#[derive(Eq, PartialEq, Clone)]
pub struct FieldDescriptor {
    pub(crate) message_descriptor: MessageDescriptor,
    pub(crate) index: usize,
}

impl fmt::Display for FieldDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.message_descriptor, self.get_name())
    }
}

impl FieldDescriptor {
    /// Get `.proto` description of field
    pub fn get_proto(&self) -> &FieldDescriptorProto {
        &self.message_descriptor.get_proto().field[self.index]
    }

    /// Field name as specified in `.proto` file
    pub fn get_name(&self) -> &str {
        // TODO: slow for dynamic
        self.get_proto().get_name()
    }

    /// Oneof descriptor containing this field.
    pub fn containing_oneof(&self) -> Option<OneofDescriptor> {
        let proto = self.get_proto();
        if proto.has_oneof_index() {
            Some(OneofDescriptor {
                message_descriptor: self.message_descriptor.clone(),
                index: proto.get_oneof_index() as usize,
            })
        } else {
            None
        }
    }

    fn get_index(&self) -> &FieldIndex {
        &self.message_descriptor.get_index().fields[self.index]
    }

    /// JSON field name.
    ///
    /// Can be different from `.proto` field name.
    ///
    /// See [JSON mapping][json] for details.
    ///
    /// [json]: https://developers.google.com/protocol-buffers/docs/proto3#json
    pub fn json_name(&self) -> &str {
        &self.get_index().json_name
    }

    /// If this field is optional or required.
    pub fn is_singular(&self) -> bool {
        match self.get_proto().get_label() {
            field_descriptor_proto::Label::LABEL_REQUIRED => true,
            field_descriptor_proto::Label::LABEL_OPTIONAL => true,
            field_descriptor_proto::Label::LABEL_REPEATED => false,
        }
    }

    /// Is this field required.
    pub fn is_required(&self) -> bool {
        self.get_proto().get_label() == field_descriptor_proto::Label::LABEL_REQUIRED
    }

    /// If this field repeated or map?
    pub fn is_repeated_or_map(&self) -> bool {
        self.get_proto().get_label() == field_descriptor_proto::Label::LABEL_REPEATED
    }

    /// Is this field repeated, but not map field?
    pub fn is_repeated(&self) -> bool {
        match self.runtime_field_type() {
            RuntimeFieldType::Repeated(..) => true,
            _ => false,
        }
    }

    fn get_impl(&self) -> FieldDescriptorImplRef {
        match self.message_descriptor.get_impl() {
            MessageDescriptorImplRef::Generated(g) => {
                FieldDescriptorImplRef::Generated(&g.non_map().fields[self.index].accessor)
            }
            MessageDescriptorImplRef::Dynamic(_) => {
                FieldDescriptorImplRef::Dynamic(DynamicFieldDescriptorRef { field: self })
            }
        }
    }

    /// If this field a map field?
    pub fn is_map(&self) -> bool {
        match self.runtime_field_type() {
            RuntimeFieldType::Map(..) => true,
            _ => false,
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
    pub fn has_field(&self, m: &dyn MessageDyn) -> bool {
        match self.get_reflect(m) {
            ReflectFieldRef::Optional(Some(..)) => true,
            ReflectFieldRef::Optional(None) => false,
            ReflectFieldRef::Repeated(r) => !r.is_empty(),
            ReflectFieldRef::Map(m) => !m.is_empty(),
        }
    }

    // accessors

    fn singular(&self) -> SingularFieldAccessorRef {
        match self.get_impl() {
            FieldDescriptorImplRef::Generated(GeneratedFieldAccessor::V2(
                AccessorV2::Singular(ref a),
            )) => SingularFieldAccessorRef::Generated(a),
            FieldDescriptorImplRef::Generated(GeneratedFieldAccessor::V2(..)) => {
                panic!("not a singular field: {}", self)
            }
            FieldDescriptorImplRef::Dynamic(d) => SingularFieldAccessorRef::Dynamic(d),
        }
    }

    fn repeated(&self) -> RepeatedFieldAccessorRef {
        match self.get_impl() {
            FieldDescriptorImplRef::Generated(GeneratedFieldAccessor::V2(
                AccessorV2::Repeated(ref a),
            )) => RepeatedFieldAccessorRef::Generated(a),
            FieldDescriptorImplRef::Generated(GeneratedFieldAccessor::V2(..)) => {
                panic!("not a repeated field: {}", self)
            }
            FieldDescriptorImplRef::Dynamic(d) => RepeatedFieldAccessorRef::Dynamic(d),
        }
    }

    fn map(&self) -> MapFieldAccessorRef {
        match self.get_impl() {
            FieldDescriptorImplRef::Generated(GeneratedFieldAccessor::V2(AccessorV2::Map(
                ref a,
            ))) => MapFieldAccessorRef::Generated(a),
            FieldDescriptorImplRef::Generated(GeneratedFieldAccessor::V2(..)) => {
                panic!("not a map field: {}", self)
            }
            FieldDescriptorImplRef::Dynamic(d) => MapFieldAccessorRef::Dynamic(d),
        }
    }

    /// Get message field or default instance if field is unset.
    ///
    /// # Panics
    /// If this field belongs to a different message type or
    /// field type is not message.
    pub fn get_message<'a>(&self, m: &'a dyn MessageDyn) -> MessageRef<'a> {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::Message(m) => m,
            _ => panic!("not message field: {}", self),
        }
    }

    /// Get a mutable reference to a message field.
    /// Initialize field with default message if unset.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or
    /// field type is not singular message.
    pub fn mut_message<'a>(&self, m: &'a mut dyn MessageDyn) -> &'a mut dyn MessageDyn {
        match self.mut_singular_field_or_default(m) {
            ReflectValueMut::Message(m) => m,
        }
    }

    /// Default value.
    ///
    /// # Panics
    ///
    /// If field is not singular.
    pub fn singular_default_value(&self) -> ReflectValueRef {
        self.get_index().default_value(self)
    }

    /// Get singular field value.
    ///
    /// Return field default value if field is unset.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or fields is not singular.
    pub fn get_singular_field_or_default<'a>(&self, m: &'a dyn MessageDyn) -> ReflectValueRef<'a> {
        match self.get_singular(m) {
            Some(m) => m,
            None => {
                let message_index = match self.singular() {
                    SingularFieldAccessorRef::Generated(..) => {
                        self.message_descriptor.get_generated_index()
                    }
                    SingularFieldAccessorRef::Dynamic(..) => {
                        DynamicMessage::downcast_ref(m).descriptor.get_index()
                    }
                };
                message_index.fields[self.index].default_value(self)
            }
        }
    }

    // Not public because it is not implemented for all types
    fn mut_singular_field_or_default<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectValueMut<'a> {
        match self.singular() {
            SingularFieldAccessorRef::Generated(g) => g.accessor.mut_field_or_default(m),
            SingularFieldAccessorRef::Dynamic(..) => {
                DynamicMessage::downcast_mut(m).mut_singular_field_or_default(self)
            }
        }
    }

    /// Runtime representation of singular field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or field is not singular.
    pub fn singular_runtime_type(&self) -> RuntimeTypeBox {
        match self.runtime_field_type() {
            RuntimeFieldType::Singular(s) => s,
            _ => panic!("Not a singular field: {}", self),
        }
    }

    /// Set singular field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or
    /// field is not singular or value is of different type.
    pub fn set_singular_field(&self, m: &mut dyn MessageDyn, value: ReflectValueBox) {
        match self.singular() {
            SingularFieldAccessorRef::Generated(g) => g.accessor.set_field(m, value),
            SingularFieldAccessorRef::Dynamic(d) => d.set_field(m, value),
        }
    }

    /// Dynamic representation of field type.
    pub fn runtime_field_type(&self) -> RuntimeFieldType {
        self.get_index().field_type.resolve(self)
    }

    /// Get field of any type.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type.
    pub fn get_reflect<'a>(&self, m: &'a dyn MessageDyn) -> ReflectFieldRef<'a> {
        match self.get_impl() {
            FieldDescriptorImplRef::Generated(g) => g.get_reflect(m),
            FieldDescriptorImplRef::Dynamic(d) => d.get_reflect(m),
        }
    }

    /// Get singular field value.
    ///
    /// Return `None` if field is unset.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or fields is not singular.
    pub fn get_singular<'a>(&self, m: &'a dyn MessageDyn) -> Option<ReflectValueRef<'a>> {
        match self.get_reflect(m) {
            ReflectFieldRef::Optional(o) => o,
            _ => panic!("not a singular field"),
        }
    }

    // repeated

    /// Get repeated field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or field is not repeated.
    pub fn get_repeated<'a>(&self, m: &'a dyn MessageDyn) -> ReflectRepeatedRef<'a> {
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
    pub fn mut_repeated<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectRepeatedMut<'a> {
        match self.repeated() {
            RepeatedFieldAccessorRef::Generated(g) => g.accessor.mut_repeated(m),
            RepeatedFieldAccessorRef::Dynamic(d) => d.mut_repeated(m),
        }
    }

    // map

    /// Get `map` field.
    ///
    /// # Panics
    ///
    /// If this field belongs to a different message type or field is not `map`.
    pub fn get_map<'a>(&self, m: &'a dyn MessageDyn) -> ReflectMapRef<'a> {
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
    pub fn mut_map<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectMapMut<'a> {
        match self.map() {
            MapFieldAccessorRef::Generated(g) => g.accessor.mut_reflect(m),
            MapFieldAccessorRef::Dynamic(d) => d.mut_map(m),
        }
    }
}

enum SingularFieldAccessorRef<'a> {
    Generated(&'a SingularFieldAccessorHolder),
    Dynamic(DynamicFieldDescriptorRef<'a>),
}

enum RepeatedFieldAccessorRef<'a> {
    Generated(&'a RepeatedFieldAccessorHolder),
    Dynamic(DynamicFieldDescriptorRef<'a>),
}

enum MapFieldAccessorRef<'a> {
    Generated(&'a MapFieldAccessorHolder),
    Dynamic(DynamicFieldDescriptorRef<'a>),
}

pub(crate) enum FieldDescriptorImplRef<'a> {
    Generated(&'static GeneratedFieldAccessor),
    Dynamic(DynamicFieldDescriptorRef<'a>),
}

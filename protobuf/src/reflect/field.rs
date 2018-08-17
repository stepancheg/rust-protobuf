use descriptor::FieldDescriptorProto;
use reflect::accessor::FieldAccessor;
use descriptor::FieldDescriptorProto_Label;
use reflect::EnumDescriptor;
use reflect::accessor::AccessorKind;
use reflect::MessageDescriptor;
use reflect::reflect_deep_eq::ReflectDeepEq;
use Message;
use reflect::accessor::singular::SingularFieldAccessor;
use reflect::accessor::repeated::RepeatedFieldAccessor;
use reflect::accessor::map::MapFieldAccessor;
use reflect::EnumValueDescriptor;
use reflect::ReflectValueRef;
use reflect::RuntimeTypeDynamic;
use reflect::ReflectValueBox;
use reflect::repeated::ReflectRepeatedRef;
use reflect::repeated::ReflectRepeatedMut;
use reflect::map::ReflectMapRef;
use reflect::map::ReflectMapMut;
use json::json_name;


/// Reference to a value stored in a field, optional, repeated or map.
pub enum ReflectFieldRef<'a> {
    /// Singular field, optional or required in proto3 and just plain field in proto3
    Optional(Option<ReflectValueRef<'a>>),
    /// Repeated field
    Repeated(ReflectRepeatedRef<'a>),
    /// Map field
    Map(ReflectMapRef<'a>),
}

impl<'a> ReflectDeepEq for ReflectFieldRef<'a> {
    fn reflect_deep_eq(&self, that: &Self) -> bool {
        match (self, that) {
            (ReflectFieldRef::Optional(a), ReflectFieldRef::Optional(b)) => {
                match (a, b) {
                    (Some(av), Some(bv)) => av.reflect_deep_eq(&bv),
                    (None, None) => true,
                    _ => false,
                }
            }
            (ReflectFieldRef::Repeated(a), ReflectFieldRef::Repeated(b)) => {
                a.reflect_deep_eq(b)
            }
            (ReflectFieldRef::Map(a), ReflectFieldRef::Map(b)) => {
                a.reflect_deep_eq(b)
            }
            _ => unreachable!(),
        }
    }
}

pub enum RuntimeFieldType {
    Singular(&'static RuntimeTypeDynamic),
    Repeated(&'static RuntimeTypeDynamic),
    Map(&'static RuntimeTypeDynamic, &'static RuntimeTypeDynamic),
}

fn _assert_sync<'a>() {
    fn _assert_send_sync<T : Sync>() {}
    _assert_send_sync::<ReflectFieldRef<'a>>();
}


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
        FieldDescriptor {
            proto,
            accessor,
            // probably could be lazy-init
            json_name: json_name(proto.get_name()),
        }
    }

    pub fn proto(&self) -> &'static FieldDescriptorProto {
        self.proto
    }

    pub fn name(&self) -> &str {
        self.accessor.name
    }

    pub fn json_name(&self) -> &str {
        &self.json_name
    }

    pub fn is_repeated(&self) -> bool {
        self.proto.get_label() == FieldDescriptorProto_Label::LABEL_REPEATED
    }

    /// Return enum descriptor for enum field, panics if field type is not enum.
    pub fn enum_descriptor(&self) -> &'static EnumDescriptor {
        match self.accessor.accessor {
            AccessorKind::Singular(ref a) => a.protobuf_type().runtime_type().enum_descriptor(),
            AccessorKind::Repeated(ref a) => a.element_protobuf_type().runtime_type().enum_descriptor(),
            _ => panic!("not a singular or repeated field"),
        }
    }

    /// Return enum descriptor for message field, panics if field type is not message.
    pub fn message_descriptor(&self) -> &'static MessageDescriptor {
        match self.accessor.accessor {
            AccessorKind::Singular(ref a) => a.protobuf_type().runtime_type().message_descriptor(),
            AccessorKind::Repeated(ref a) => a.element_protobuf_type().runtime_type().message_descriptor(),
            _ => panic!("not a singular or repeated field"),
        }
    }

    pub fn has_field(&self, m: &Message) -> bool {
        match self.accessor.accessor {
            AccessorKind::Singular(ref a) => a.get_reflect(m).is_some(),
            AccessorKind::Repeated(ref a) => a.get_reflect(m).len() != 0,
            AccessorKind::Map(ref a) => a.len_field_generic(m) != 0,
        }
    }

    pub fn len_field(&self, m: &Message) -> usize {
        match self.accessor.accessor {
            AccessorKind::Singular(ref a) => if a.get_reflect(m).is_some() { 1 } else { 0 },
            AccessorKind::Repeated(ref a) => a.get_reflect(m).len(),
            AccessorKind::Map(ref a) => a.len_field_generic(m),
        }
    }

    // accessors

    fn singular(&self) -> &SingularFieldAccessor {
        match self.accessor.accessor {
            AccessorKind::Singular(ref a) => &**a,
            _ => panic!("not a singular field: {}", self.name()),
        }
    }

    fn repeated(&self) -> &RepeatedFieldAccessor {
        match self.accessor.accessor {
            AccessorKind::Repeated(ref a) => &**a,
            _ => panic!("not a repeated field: {}", self.name()),
        }
    }

    fn map(&self) -> &MapFieldAccessor {
        match self.accessor.accessor {
            AccessorKind::Map(ref a) => &**a,
            _ => panic!("not a map field: {}", self.name()),
        }
    }

    /// Get message field or default instance if field is unset.
    /// Panic if field type is not message.
    pub fn get_message<'a>(&self, m: &'a Message) -> &'a Message {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::Message(m) => m,
            _ => panic!("not message"),
        }
    }

    /// Not implemented
    pub fn mut_message<'a>(&self, _m: &'a mut Message) -> &'a mut Message {
        unimplemented!()
    }

    pub fn get_enum(&self, m: &Message) -> &'static EnumValueDescriptor {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::Enum(v) => v,
            _ => panic!("not enum"),
        }
    }

    pub fn get_str<'a>(&self, m: &'a Message) -> &'a str {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::String(v) => v,
            _ => panic!("not string"),
        }
    }

    pub fn get_bytes<'a>(&self, m: &'a Message) -> &'a [u8] {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::Bytes(v) => v,
            _ => panic!("not bytes"),
        }
    }

    pub fn get_u32(&self, m: &Message) -> u32 {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::U32(v) => v,
            _ => panic!("not u32"),
        }
    }

    pub fn get_u64(&self, m: &Message) -> u64 {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::U64(v) => v,
            _ => panic!("not u64"),
        }
    }

    pub fn get_i32(&self, m: &Message) -> i32 {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::I32(v) => v,
            _ => panic!("not i32"),
        }
    }

    pub fn get_i64(&self, m: &Message) -> i64 {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::I64(v) => v,
            _ => panic!("not i64"),
        }
    }

    pub fn get_bool(&self, m: &Message) -> bool {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::Bool(v) => v,
            _ => panic!("not bool"),
        }
    }

    pub fn get_f32(&self, m: &Message) -> f32 {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::F32(v) => v,
            _ => panic!("not f32"),
        }
    }

    pub fn get_f64(&self, m: &Message) -> f64 {
        match self.get_singular_field_or_default(m) {
            ReflectValueRef::F64(v) => v,
            _ => panic!("not f64"),
        }
    }

    pub fn get_singular_field_or_default<'a>(&self, m: &'a Message) -> ReflectValueRef<'a> {
        self.singular().get_singular_field_or_default(m)
    }

    pub fn singular_runtime_type(&self) -> &RuntimeTypeDynamic {
        self.singular().protobuf_type().runtime_type()
    }

    pub fn set_singular_field(&self, m: &mut Message, value: ReflectValueBox) {
        self.singular().set_singular_field(m, value)
    }

    pub fn runtime_field_type(&self) -> RuntimeFieldType {
        use self::AccessorKind::*;
        match self.accessor.accessor {
            Singular(ref a) => {
                RuntimeFieldType::Singular(a.protobuf_type().runtime_type())
            },
            Repeated(ref a) => {
                let element_protobuf_type = a.element_protobuf_type();
                RuntimeFieldType::Repeated(element_protobuf_type.runtime_type())
            },
            Map(ref a) => {
                let (k, v) = a.entry_type();
                RuntimeFieldType::Map(
                    k.runtime_type(),
                    v.runtime_type())
            }
        }
    }

    pub fn get_reflect<'a>(&self, m: &'a Message) -> ReflectFieldRef<'a> {
        use self::AccessorKind::*;
        match self.accessor.accessor {
            Singular(ref a) => ReflectFieldRef::Optional(a.get_reflect(m)),
            Repeated(ref a) => ReflectFieldRef::Repeated(a.get_reflect(m)),
            Map(ref a) => ReflectFieldRef::Map(a.get_reflect(m)),
        }
    }

    // repeated

    pub fn get_repeated<'a>(&self, m: &'a Message) -> ReflectRepeatedRef<'a> {
        self.repeated().get_reflect(m)
    }

    pub fn mut_repeated<'a>(&self, m: &'a mut Message) -> ReflectRepeatedMut<'a> {
        self.repeated().mut_reflect(m)
    }

    // map

    pub fn get_map<'a>(&self, m: &'a Message) -> ReflectMapRef<'a> {
        self.map().get_reflect(m)
    }

    pub fn mut_map<'a>(&self, m: &'a mut Message) -> ReflectMapMut<'a> {
        self.map().mut_reflect(m)
    }

}

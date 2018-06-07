use descriptor::FieldDescriptorProto;
use reflect::accessor::FieldAccessor;
use descriptor::FieldDescriptorProto_Label;
use reflect::EnumDescriptor;
use reflect::accessor::AccessorKind;
use reflect::MessageDescriptor;
use Message;
use reflect::accessor::singular::SingularFieldAccessor;
use reflect::accessor::repeated::RepeatedFieldAccessor;
use reflect::accessor::map::MapFieldAccessor;
use reflect::EnumValueDescriptor;
use descriptor::FieldDescriptorProto_Type;
use reflect::ReflectValueRef;
use reflect::RuntimeTypeDynamic;
use reflect::ReflectValueBox;
use reflect::repeated::ReflectRepeatedRef;
use reflect::repeated::ReflectRepeatedMut;
use reflect::map::ReflectMapRef;
use reflect::map::ReflectMapMut;


/// Reference to a value stored in a field, optional, repeated or map.
pub enum ReflectFieldRef<'a> {
    /// Singular field, optional or required in proto3 and just plain field in proto3
    Optional(Option<ReflectValueRef<'a>>),
    /// Repeated field
    Repeated(ReflectRepeatedRef<'a>),
    /// Map field
    Map(ReflectMapRef<'a>),
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
        }
    }

    pub fn proto(&self) -> &'static FieldDescriptorProto {
        self.proto
    }

    pub fn name(&self) -> &str {
        self.accessor.name
    }

    pub fn json_name(&self) -> &str {
        // TODO: Message field names are mapped to lowerCamelCase and become JSON object keys.
        self.name()
    }

    pub fn is_repeated(&self) -> bool {
        self.proto.get_label().unwrap() == FieldDescriptorProto_Label::LABEL_REPEATED
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
            AccessorKind::Singular(ref a) => a.has_field_generic(m),
            AccessorKind::Repeated(ref a) => a.len_field_generic(m) != 0,
            AccessorKind::Map(ref a) => a.len_field_generic(m) != 0,
        }
    }

    pub fn len_field(&self, m: &Message) -> usize {
        match self.accessor.accessor {
            AccessorKind::Singular(ref a) => if a.has_field_generic(m) { 1 } else { 0 },
            AccessorKind::Repeated(ref a) => a.len_field_generic(m),
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
        match self.singular().get_message_generic(m) {
            Some(m) => m,
            None => self.message_descriptor().default_instance(),
        }
    }

    pub fn mut_message<'a>(&self, m: &'a mut Message) -> &'a mut Message {
        self.singular().mut_message_generic(m)
    }

    pub fn get_enum(&self, m: &Message) -> &'static EnumValueDescriptor {
        assert_eq!(FieldDescriptorProto_Type::TYPE_ENUM, self.proto.get_field_type().unwrap());
        self.singular().get_enum_generic(m)
    }

    pub fn get_str<'a>(&self, m: &'a Message) -> &'a str {
        assert_eq!(FieldDescriptorProto_Type::TYPE_STRING, self.proto.get_field_type().unwrap());
        self.singular().get_str_generic(m)
    }

    pub fn get_bytes<'a>(&self, m: &'a Message) -> &'a [u8] {
        assert_eq!(FieldDescriptorProto_Type::TYPE_BYTES, self.proto.get_field_type().unwrap());
        self.singular().get_bytes_generic(m)
    }

    pub fn get_u32(&self, m: &Message) -> u32 {
        self.singular().get_u32_generic(m)
    }

    pub fn get_u64(&self, m: &Message) -> u64 {
        self.singular().get_u64_generic(m)
    }

    pub fn get_i32(&self, m: &Message) -> i32 {
        self.singular().get_i32_generic(m)
    }

    pub fn get_i64(&self, m: &Message) -> i64 {
        self.singular().get_i64_generic(m)
    }

    pub fn get_bool(&self, m: &Message) -> bool {
        assert_eq!(FieldDescriptorProto_Type::TYPE_BOOL, self.proto.get_field_type().unwrap());
        self.singular().get_bool_generic(m)
    }

    pub fn get_f32(&self, m: &Message) -> f32 {
        self.singular().get_f32_generic(m)
    }

    pub fn get_f64(&self, m: &Message) -> f64 {
        self.singular().get_f64_generic(m)
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

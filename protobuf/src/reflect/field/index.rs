use crate::descriptor::field_descriptor_proto;
use crate::descriptor::field_descriptor_proto::Type;
use crate::descriptor::FieldDescriptorProto;
use crate::json::json_name;
use crate::reflect::file::building::FileDescriptorBuilding;
use crate::reflect::protobuf_type_box::ProtobufTypeBox;
use crate::reflect::EnumDescriptor;
use crate::reflect::EnumValueDescriptor;
use crate::reflect::FieldDescriptor;
use crate::reflect::MessageDescriptor;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeFieldType;
use crate::reflect::RuntimeTypeBox;

#[derive(Debug)]
pub(crate) enum ForwardProtobufTypeBox {
    ProtobufTypeBox(ProtobufTypeBox),
    CurrentFileEnum(usize),
    CurrentFileMessage(usize),
}

impl ForwardProtobufTypeBox {
    pub(crate) fn message(message: MessageDescriptor) -> ForwardProtobufTypeBox {
        ForwardProtobufTypeBox::ProtobufTypeBox(ProtobufTypeBox::message(message))
    }

    pub(crate) fn enumeration(enumeration: EnumDescriptor) -> ForwardProtobufTypeBox {
        ForwardProtobufTypeBox::ProtobufTypeBox(ProtobufTypeBox::enumeration(enumeration))
    }

    pub(crate) fn from_proto_type(t: Type) -> ForwardProtobufTypeBox {
        ForwardProtobufTypeBox::ProtobufTypeBox(ProtobufTypeBox::from_proto_type(t))
    }

    fn resolve(&self, field: &FieldDescriptor) -> ProtobufTypeBox {
        match self {
            ForwardProtobufTypeBox::ProtobufTypeBox(t) => t.clone(),
            ForwardProtobufTypeBox::CurrentFileMessage(m) => ProtobufTypeBox::message(
                MessageDescriptor::new(field.message_descriptor.file_descriptor().clone(), *m),
            ),
            ForwardProtobufTypeBox::CurrentFileEnum(m) => ProtobufTypeBox::enumeration(
                EnumDescriptor::new(field.message_descriptor.file_descriptor().clone(), *m),
            ),
        }
    }
}

#[derive(Debug)]
pub(crate) enum ForwardRuntimeFieldType {
    Singular(ForwardProtobufTypeBox),
    Repeated(ForwardProtobufTypeBox),
    Map(ForwardProtobufTypeBox, ForwardProtobufTypeBox),
}

impl ForwardRuntimeFieldType {
    pub fn resolve(&self, field: &FieldDescriptor) -> RuntimeFieldType {
        match self {
            ForwardRuntimeFieldType::Singular(t) => {
                RuntimeFieldType::Singular(t.resolve(field).into_runtime())
            }
            ForwardRuntimeFieldType::Repeated(t) => {
                RuntimeFieldType::Repeated(t.resolve(field).into_runtime())
            }
            ForwardRuntimeFieldType::Map(k, v) => RuntimeFieldType::Map(
                k.resolve(field).into_runtime(),
                v.resolve(field).into_runtime(),
            ),
        }
    }
}

#[derive(Debug)]
pub(crate) enum FieldDefaultValue {
    ReflectValueBox(ReflectValueBox),
    Enum(usize),
}

#[derive(Debug)]
pub(crate) struct FieldIndex {
    pub(crate) json_name: String,
    pub(crate) field_type: ForwardRuntimeFieldType,
    pub(crate) default_value: Option<FieldDefaultValue>,
}

impl FieldIndex {
    fn enum_default_value(
        field: &FieldDescriptorProto,
        building: &FileDescriptorBuilding,
    ) -> FieldDefaultValue {
        let en = building.find_enum(field.get_type_name());
        let (n, _) = match en
            .value
            .iter()
            .enumerate()
            .find(|(_n, v)| v.get_name() == field.get_default_value())
        {
            Some(v) => v,
            None => panic!(
                "enum value not found a default value: {}",
                field.get_default_value()
            ),
        };
        FieldDefaultValue::Enum(n)
    }

    fn parse_default_value(
        field: &FieldDescriptorProto,
        building: &FileDescriptorBuilding,
    ) -> FieldDefaultValue {
        FieldDefaultValue::ReflectValueBox(match field.get_field_type() {
            t @ field_descriptor_proto::Type::TYPE_GROUP
            | t @ field_descriptor_proto::Type::TYPE_MESSAGE => {
                panic!("{:?} cannot have a default value", t)
            }
            field_descriptor_proto::Type::TYPE_ENUM => {
                return Self::enum_default_value(field, building)
            }
            t => RuntimeTypeBox::from_proto_type(t)
                .parse_proto_default_value(field.get_default_value()),
        })
    }

    pub fn index(field: &FieldDescriptorProto, building: &FileDescriptorBuilding) -> FieldIndex {
        let default_value = if field.has_default_value() {
            Some(Self::parse_default_value(field, building))
        } else {
            None
        };

        let json_name = if !field.get_json_name().is_empty() {
            field.get_json_name().to_owned()
        } else {
            json_name(field.get_name())
        };

        FieldIndex {
            default_value,
            json_name,
            field_type: building.resolve_field_type(field),
        }
    }

    pub(crate) fn default_value<'a>(&'a self, field: &FieldDescriptor) -> ReflectValueRef<'a> {
        match &self.default_value {
            Some(FieldDefaultValue::ReflectValueBox(v)) => v.as_value_ref(),
            Some(FieldDefaultValue::Enum(v)) => match field.singular_runtime_type() {
                RuntimeTypeBox::Enum(e) => {
                    let ev = EnumValueDescriptor::new(e.clone(), *v);
                    ReflectValueRef::from(ev)
                }
                t => panic!("wrong type {:?} for default value enum", t),
            },
            None => field.singular_runtime_type().default_value_ref(),
        }
    }
}

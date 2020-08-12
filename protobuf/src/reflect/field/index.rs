use crate::descriptor::field_descriptor_proto;
use crate::descriptor::FieldDescriptorProto;
use crate::json::json_name;
use crate::reflect::file::building::FileDescriptorBuilding;
use crate::reflect::{ReflectValueBox, RuntimeTypeBox};

#[derive(Debug)]
pub(crate) enum FieldDefaultValue {
    ReflectValueBox(ReflectValueBox),
    Enum(usize),
}

#[derive(Debug)]
pub(crate) struct FieldIndex {
    pub(crate) json_name: String,
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

    fn default_value(
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
            Some(Self::default_value(field, building))
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
        }
    }
}

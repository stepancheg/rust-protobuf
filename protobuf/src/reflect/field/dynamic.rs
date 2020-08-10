use crate::descriptor::field_descriptor_proto;
use crate::descriptor::field_descriptor_proto::Type;
use crate::descriptor::FieldDescriptorProto;
use crate::reflect::find_message_or_enum::MessageOrEnum;
use crate::reflect::message::dynamic::DynamicMessageDescriptor;
use crate::reflect::FileDescriptor;
use crate::reflect::RuntimeFieldType;
use crate::reflect::RuntimeTypeBox;

pub(crate) struct DynamicFieldDescriptorRef<'a> {
    pub(crate) file: &'a FileDescriptor,
    pub(crate) message: &'a DynamicMessageDescriptor,
    pub(crate) index: usize,
}

impl<'a> DynamicFieldDescriptorRef<'a> {
    fn get_proto(&self) -> &FieldDescriptorProto {
        &self.message.get_proto().field[self.index]
    }

    fn element_type(&self) -> RuntimeTypeBox {
        match self.get_proto().get_field_type() {
            field_descriptor_proto::Type::TYPE_BOOL => RuntimeTypeBox::Bool,
            Type::TYPE_DOUBLE => RuntimeTypeBox::F64,
            Type::TYPE_FLOAT => RuntimeTypeBox::F32,

            Type::TYPE_INT32 => RuntimeTypeBox::I32,
            Type::TYPE_SFIXED32 => RuntimeTypeBox::I32,
            Type::TYPE_SINT32 => RuntimeTypeBox::I32,

            Type::TYPE_INT64 => RuntimeTypeBox::I64,
            Type::TYPE_SFIXED64 => RuntimeTypeBox::I64,
            Type::TYPE_SINT64 => RuntimeTypeBox::I64,

            Type::TYPE_FIXED32 => RuntimeTypeBox::U32,
            Type::TYPE_UINT32 => RuntimeTypeBox::U32,

            Type::TYPE_UINT64 => RuntimeTypeBox::U64,
            Type::TYPE_FIXED64 => RuntimeTypeBox::U64,

            Type::TYPE_STRING => RuntimeTypeBox::String,
            Type::TYPE_BYTES => RuntimeTypeBox::VecU8,
            Type::TYPE_MESSAGE => unimplemented!(), // TODO
            Type::TYPE_ENUM => unimplemented!(),    // TODO
            Type::TYPE_GROUP => unimplemented!(),   // TODO
        }
    }

    fn try_map_type(&self) -> Option<RuntimeFieldType> {
        if self.get_proto().get_field_type() != field_descriptor_proto::Type::TYPE_MESSAGE {
            return None;
        }

        // TODO: unnecessary complicated: we know the message is declared in the same file
        let m = match self
            .file
            .find_message_or_enum_proto_in_all_files(self.get_proto().get_type_name())
        {
            Some((_, MessageOrEnum::Message(m))) => m,
            Some((_, MessageOrEnum::Enum(..))) | None => return None,
        };

        if !m.options.get_or_default().get_map_entry() {
            return None;
        }

        // TODO
        unimplemented!()
    }

    pub fn runtime_field_type(&self) -> RuntimeFieldType {
        let proto = self.get_proto();
        match proto.get_label() {
            field_descriptor_proto::Label::LABEL_OPTIONAL
            | field_descriptor_proto::Label::LABEL_REQUIRED => {
                RuntimeFieldType::Singular(self.element_type())
            }
            field_descriptor_proto::Label::LABEL_REPEATED => {
                if let Some(t) = self.try_map_type() {
                    return t;
                }

                RuntimeFieldType::Repeated(self.element_type())
            }
        }
    }
}

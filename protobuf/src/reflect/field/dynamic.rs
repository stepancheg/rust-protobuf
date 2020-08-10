use crate::descriptor::field_descriptor_proto;
use crate::reflect::message::dynamic::DynamicMessageDescriptor;
use crate::reflect::FileDescriptor;
use crate::reflect::{RuntimeFieldType, RuntimeTypeDynamic};

pub(crate) struct DynamicFieldDescriptorRef<'a> {
    pub(crate) file: &'a FileDescriptor,
    pub(crate) message: &'a DynamicMessageDescriptor,
    pub(crate) index: usize,
}

impl<'a> DynamicFieldDescriptorRef<'a> {
    fn element_type(&self) -> &'static dyn RuntimeTypeDynamic {
        unimplemented!()
    }

    pub fn runtime_field_type(&self) -> RuntimeFieldType {
        let field = &self.message.get_proto().field[self.index];
        match field.get_label() {
            field_descriptor_proto::Label::LABEL_OPTIONAL
            | field_descriptor_proto::Label::LABEL_REQUIRED => unimplemented!(), // TODO
            field_descriptor_proto::Label::LABEL_REPEATED => unimplemented!(), // TODO
        }
    }
}

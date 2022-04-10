use crate::descriptor::EnumDescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::message::path::MessagePath;

#[derive(Debug, Clone)]
pub(crate) struct EnumPath {
    pub(crate) message_path: MessagePath,
    pub(crate) enum_index: usize,
}

impl EnumPath {
    pub(crate) fn eval<'a>(&self, file: &'a FileDescriptorProto) -> &'a EnumDescriptorProto {
        match self.message_path.eval(file) {
            Some(message) => &message.enum_type[self.enum_index],
            None => &file.enum_type[self.enum_index],
        }
    }
}

use std::sync::Arc;

use crate::descriptor::EnumDescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::message::path::MessagePath;
use crate::reflect::name::append_path;

#[derive(Debug)]
pub(crate) struct DynamicEnumDescriptor {
    pub full_name: String,
    file_descriptor_proto: Arc<FileDescriptorProto>,
    path: MessagePath,
    enum_index: usize,
}

impl DynamicEnumDescriptor {
    pub fn new(
        proto: Arc<FileDescriptorProto>,
        path: &MessagePath,
        enum_index: usize,
    ) -> DynamicEnumDescriptor {
        let mut full_name = proto.package().to_owned();
        let e = if path.len() == 0 {
            &proto.enum_type[enum_index]
        } else {
            let messages = path.eval_path(&*proto);
            for m in &messages {
                append_path(&mut full_name, m.name());
            }
            &messages.last().cloned().unwrap().enum_type[enum_index]
        };
        append_path(&mut full_name, e.name());

        DynamicEnumDescriptor {
            full_name,
            file_descriptor_proto: proto,
            path: path.clone(),
            enum_index,
        }
    }

    pub fn proto(&self) -> &EnumDescriptorProto {
        // TODO: this `eval` is not free.
        match self.path.eval(&self.file_descriptor_proto) {
            None => &self.file_descriptor_proto.enum_type[self.enum_index],
            Some(m) => &m.enum_type[self.enum_index],
        }
    }
}

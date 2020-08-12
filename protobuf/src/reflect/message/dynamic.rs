use crate::descriptor::DescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::message::index::MessageIndex;
use crate::reflect::message::path::MessagePath;
use crate::reflect::name::append_path;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct DynamicMessageDescriptor {
    pub full_name: String,
    file_descriptor_proto: Arc<FileDescriptorProto>,
    path: MessagePath,
    pub indices: MessageIndex,
}

impl DynamicMessageDescriptor {
    pub fn new(proto: Arc<FileDescriptorProto>, path: &MessagePath) -> DynamicMessageDescriptor {
        let mut full_name = proto.get_package().to_owned();

        let messages = path.eval_path(&*proto);
        for m in &messages {
            append_path(&mut full_name, m.get_name());
        }

        let m = messages.last().unwrap();

        let indices = MessageIndex::index(m);

        DynamicMessageDescriptor {
            file_descriptor_proto: proto,
            full_name,
            path: path.clone(),
            indices,
        }
    }

    pub fn get_proto(&self) -> &DescriptorProto {
        self.path.eval(&self.file_descriptor_proto).unwrap()
    }
}

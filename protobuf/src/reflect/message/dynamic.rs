use crate::descriptor::DescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::message::common::MessageIndices;
use crate::reflect::name::append_path;
use std::sync::Arc;

pub(crate) struct DynamicMessageDescriptor {
    pub full_name: String,
    file_descriptor_proto: Arc<FileDescriptorProto>,
    path: Vec<usize>,
    pub indices: MessageIndices,
}

impl DynamicMessageDescriptor {
    pub fn new(proto: Arc<FileDescriptorProto>, path: &[usize]) -> DynamicMessageDescriptor {
        let mut full_name = proto.get_package().to_owned();
        let mut m = &proto.message_type[path[0]];
        append_path(&mut full_name, m.get_name());
        for &p in &path[1..] {
            m = &m.nested_type[p];
            append_path(&mut full_name, m.get_name());
        }

        let indices = MessageIndices::index(m);

        DynamicMessageDescriptor {
            file_descriptor_proto: proto,
            full_name,
            path: path.to_owned(),
            indices,
        }
    }

    pub fn get_proto(&self) -> &DescriptorProto {
        let mut m = &self.file_descriptor_proto.message_type[self.path[0]];
        for &p in &self.path[1..] {
            m = &m.nested_type[p];
        }
        m
    }
}

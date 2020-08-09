use crate::descriptor::EnumDescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::name::append_path;
use std::sync::Arc;

pub(crate) struct DynamicEnumDescriptor {
    full_name: String,
    file_descriptor_proto: Arc<FileDescriptorProto>,
    path: Vec<usize>,
    enum_index: usize,
}

impl DynamicEnumDescriptor {
    pub fn new(
        proto: Arc<FileDescriptorProto>,
        path: &[usize],
        enum_index: usize,
    ) -> DynamicEnumDescriptor {
        let mut full_name = proto.get_package().to_owned();
        let e = if path.len() == 0 {
            &proto.enum_type[enum_index]
        } else {
            let mut m = &proto.message_type[path[0]];
            append_path(&mut full_name, m.get_name());
            for &p in &path[1..] {
                m = &m.nested_type[p];
                append_path(&mut full_name, m.get_name());
            }
            &m.enum_type[enum_index]
        };
        append_path(&mut full_name, e.get_name());
        DynamicEnumDescriptor {
            full_name,
            file_descriptor_proto: proto,
            path: path.to_owned(),
            enum_index,
        }
    }

    pub fn get_proto(&self) -> &EnumDescriptorProto {
        if self.path.is_empty() {
            &self.file_descriptor_proto.enum_type[self.enum_index]
        } else {
            let mut m = &self.file_descriptor_proto.message_type[self.path[0]];
            for &p in &self.path[1..] {
                m = &m.nested_type[p];
            }
            &m.enum_type[self.enum_index]
        }
    }
}

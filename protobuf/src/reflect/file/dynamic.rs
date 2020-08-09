use crate::descriptor::DescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::enums::dynamic::DynamicEnumDescriptor;
use crate::reflect::message::dynamic::DynamicMessageDescriptor;
use crate::reflect::FileDescriptor;
use std::sync::Arc;

pub(crate) struct DynamicFileDescriptor {
    pub proto: Arc<FileDescriptorProto>,
    pub dependencies: Vec<FileDescriptor>,
    pub messages: Vec<DynamicMessageDescriptor>,
    pub enums: Vec<DynamicEnumDescriptor>,
}

impl DynamicFileDescriptor {
    pub fn new(
        proto: FileDescriptorProto,
        dependencies: Vec<FileDescriptor>,
    ) -> DynamicFileDescriptor {
        let proto = Arc::new(proto);
        DynamicFileDescriptor {
            messages: Self::messages(&proto),
            enums: Self::enums(&proto),
            proto,
            dependencies,
        }
    }

    fn messages(proto: &Arc<FileDescriptorProto>) -> Vec<DynamicMessageDescriptor> {
        let mut r = Vec::new();
        let mut path = Vec::new();
        for (i, m) in proto.message_type.iter().enumerate() {
            path.push(i);
            r.push(DynamicMessageDescriptor::new(proto.clone(), &path));
            Self::messages_from(proto, m, &mut r, &mut path);
            path.pop().unwrap();
        }
        assert!(path.is_empty());
        r
    }

    fn messages_from(
        proto: &Arc<FileDescriptorProto>,
        scope: &DescriptorProto,
        r: &mut Vec<DynamicMessageDescriptor>,
        path: &mut Vec<usize>,
    ) {
        for (i, m) in scope.nested_type.iter().enumerate() {
            path.push(i);
            r.push(DynamicMessageDescriptor::new(proto.clone(), &path));
            Self::messages_from(proto, m, r, path);
            path.pop().unwrap();
        }
    }

    fn enums(proto: &Arc<FileDescriptorProto>) -> Vec<DynamicEnumDescriptor> {
        let mut r = Vec::new();
        let mut path = Vec::new();
        for (i, _e) in proto.enum_type.iter().enumerate() {
            r.push(DynamicEnumDescriptor::new(proto.clone(), &path, i));
        }
        for (i, m) in proto.message_type.iter().enumerate() {
            path.push(i);
            Self::enums_from(proto, m, &mut r, &mut path);
            path.pop().unwrap();
        }
        assert!(path.is_empty());
        r
    }

    fn enums_from(
        proto: &Arc<FileDescriptorProto>,
        scope: &DescriptorProto,
        r: &mut Vec<DynamicEnumDescriptor>,
        path: &mut Vec<usize>,
    ) {
        for (i, _e) in scope.nested_type.iter().enumerate() {
            r.push(DynamicEnumDescriptor::new(proto.clone(), &path, i));
        }
        for (i, m) in scope.nested_type.iter().enumerate() {
            path.push(i);
            Self::enums_from(proto, m, r, path);
            path.pop().unwrap();
        }
    }
}

/// utilities to work with descriptor

use descriptor::FileDescriptorProto;
use descriptor::DescriptorProto;
use descriptor::EnumDescriptorProto;

#[deriving(Clone)]
struct MessagePath<'a> {
    path: Vec<&'a DescriptorProto>,
}

impl<'a> MessagePath<'a> {
    fn rust_prefix(&self) -> String {
        if self.path.is_empty() {
            "".to_string()
        } else {
            let v: Vec<&'a str> = self.path.iter().map(|m| m.get_name()).collect();
            let mut r = v.connect("_");
            r.push_str("_");
            r
        }
    }
}

#[deriving(Clone)]
struct MessageWithPath<'a> {
    path: Vec<&'a DescriptorProto>,
}

impl<'a> MessageWithPath<'a> {
    fn into_path(self) -> MessagePath<'a> {
        MessagePath {
            path: self.path,
        }
    }

    fn to_path(&self) -> MessagePath<'a> {
        self.clone().into_path()
    }
}

#[deriving(Clone)]
impl<'a> MessageWithPath<'a> {
    fn get_message(&self) -> &'a DescriptorProto {
        *self.path.last().unwrap()
    }

    fn rust_name(&self) -> String {
        let v: Vec<&'a str> = self.path.iter().map(|m| m.get_name()).collect();
        v.connect("_")
    }
}

struct EnumWithPath<'a> {
    path: MessagePath<'a>,
    en: &'a EnumDescriptorProto,
}

impl<'a> EnumWithPath<'a> {
    fn rust_name(&self) -> String {
        let mut r = self.path.rust_prefix();
        r.push_str(self.en.get_name());
        r
    }
}


fn find_messages<'a>(fd: &'a FileDescriptorProto) -> Vec<MessageWithPath<'a>> {
    fn collect<'a>(origin: &MessageWithPath<'a>) -> Vec<MessageWithPath<'a>> {
        let this_level: Vec<MessageWithPath<'a>> = origin.get_message().get_nested_type().iter()
                .map(|m| MessageWithPath { path: origin.path.clone().append(&[m]) })
                .collect();
        collect_and_walk(this_level.as_slice())
    }

    fn collect_and_walk<'a>(ms: &[MessageWithPath<'a>]) -> Vec<MessageWithPath<'a>> {
        let mut r = Vec::new();
        r.push_all(ms);
        r.extend(ms.iter().flat_map(|m| collect(m).into_iter()));
        r
    }

    let this_level: Vec<MessageWithPath<'a>> = fd.get_message_type().iter()
            .map(|m| MessageWithPath { path: vec!(m) })
            .collect();

    collect_and_walk(this_level.as_slice())
}

fn find_enums<'a>(fd: &'a FileDescriptorProto) -> Vec<EnumWithPath<'a>> {
    let mut r = Vec::new();

    r.extend(fd.get_enum_type().iter()
            .map(|e| EnumWithPath { path: MessagePath { path: Vec::new() }, en: e }));

    for m in find_messages(fd).into_iter() {
        r.extend(m.get_message().get_enum_type().iter()
                    .map(|e| EnumWithPath { path: m.to_path(), en: e }));
    }

    r
}

pub fn find_message_by_rust_name<'a>(fd: &'a FileDescriptorProto, rust_name: &str)
    -> &'a DescriptorProto
{
    find_messages(fd).iter()
            .find(|m| m.rust_name().as_slice() == rust_name)
            .unwrap()
            .get_message()
}

pub fn find_enum_by_rust_name<'a>(fd: &'a FileDescriptorProto, rust_name: &str)
    -> &'a EnumDescriptorProto
{
    find_enums(fd).iter()
            .find(|e| e.rust_name().as_slice() == rust_name)
            .unwrap()
            .en
}

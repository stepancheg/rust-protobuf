/// utilities to work with descriptor

use descriptor::FileDescriptorProto;
use descriptor::DescriptorProto;
use descriptor::EnumDescriptorProto;

//#[deriving(Clone)]
pub struct Scope<'a> {
    pub file_descriptor: &'a FileDescriptorProto,
    pub path: Vec<&'a DescriptorProto>,
}

// https://github.com/rust-lang/rust/issues/18405
impl<'a> Clone for Scope<'a> {
    fn clone(&self) -> Scope<'a> {
        Scope {
            file_descriptor: self.file_descriptor,
            path: self.path.clone(),
        }
    }
}

impl<'a> Scope<'a> {
    fn get_package(&self) -> &'a str {
        self.file_descriptor.get_package()
    }

    // get message descriptors in this scope
    fn get_message_descriptors(&self) -> &'a [DescriptorProto] {
        if self.path.is_empty() {
            self.file_descriptor.get_message_type()
        } else {
            self.path.last().unwrap().get_nested_type()
        }
    }

    // get enum descriptors in this scope
    fn get_enum_descriptors(&self) -> &'a [EnumDescriptorProto] {
        if self.path.is_empty() {
            self.file_descriptor.get_enum_type()
        } else {
            self.path.last().unwrap().get_enum_type()
        }
    }

    // get messages with attached scopes in this scope
    pub fn get_messages(&self) -> Vec<MessageWithScope<'a>> {
        self.get_message_descriptors().iter().map(|m| {
            MessageWithScope {
                scope: self.clone(),
                message: m,
            }
        }).collect()
    }

    // get enums with attached scopes in this scope
    pub fn get_enums(&self) -> Vec<EnumWithScope<'a>> {
        self.get_enum_descriptors().iter().map(|e| {
            EnumWithScope {
                scope: self.clone(),
                en: e,
            }
        }).collect()
    }

    // nested scopes, i. e. scopes of nested messages
    fn nested_scopes(&self) -> Vec<Scope<'a>> {
        self.get_message_descriptors().iter().map(|m| {
            let mut nested = self.clone();
            nested.path.push(m);
            nested
        }).collect()
    }

    // apply callback for this scope and all nested scopes
    fn walk_scopes(&self, callback: |&Scope<'a>|) {
        callback(self);

        for nested in self.nested_scopes().iter() {
            nested.walk_scopes(|scope| callback(scope));
        }
    }

    // rust type name prefix for this scope
    pub fn rust_prefix(&self) -> String {
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

pub trait WithScope<'a> {
    fn get_scope(&self) -> &Scope<'a>;

    // message or enum name
    fn get_name(&self) -> &'a str;

    // package name of this descriptor
    fn get_package(&self) -> &'a str {
        self.get_scope().get_package()
    }

    // rust type name of this descriptor
    fn rust_name(&self) -> String {
        let mut r = self.get_scope().rust_prefix();
        r.push_str(self.get_name());
        r
    }
}

//#[deriving(Clone)]
pub struct MessageWithScope<'a> {
    pub scope: Scope<'a>,
    pub message: &'a DescriptorProto,
}

// https://github.com/rust-lang/rust/issues/18405
impl<'a> Clone for MessageWithScope<'a> {
    fn clone(&self) -> MessageWithScope<'a> {
        MessageWithScope {
            scope: self.scope.clone(),
            message: self.message,
        }
    }
}

impl<'a> WithScope<'a> for MessageWithScope<'a> {
    fn get_scope(&self) -> &Scope<'a> {
        &self.scope
    }

    fn get_name(&self) -> &'a str {
        self.message.get_name()
    }
}

impl<'a> MessageWithScope<'a> {
    pub fn into_scope(mut self) -> Scope<'a> {
        self.scope.path.push(self.message);
        self.scope
    }

    pub fn to_scope(&self) -> Scope<'a> {
        self.clone().into_scope()
    }
}


//#[deriving(Clone)]
pub struct EnumWithScope<'a> {
    pub scope: Scope<'a>,
    pub en: &'a EnumDescriptorProto,
}

// https://github.com/rust-lang/rust/issues/18405
impl<'a> Clone for EnumWithScope<'a> {
    fn clone(&self) -> EnumWithScope<'a> {
        EnumWithScope {
            scope: self.scope.clone(),
            en: self.en,
        }
    }
}

impl<'a> WithScope<'a> for EnumWithScope<'a> {
    fn get_scope(&self) -> &Scope<'a> {
        &self.scope
    }

    fn get_name(&self) -> &'a str {
        self.en.get_name()
    }
}


// find all messages in given file descriptor
fn find_messages<'a>(fd: &'a FileDescriptorProto) -> Vec<MessageWithScope<'a>> {
    let root_scope = Scope {
        file_descriptor: fd,
        path: Vec::new(),
    };

    let mut r = Vec::new();

    root_scope.walk_scopes(|scope| {
        r.push_all(scope.get_messages().as_slice());
    });

    r
}

// find all enums in given file descriptor
fn find_enums<'a>(fd: &'a FileDescriptorProto) -> Vec<EnumWithScope<'a>> {
    let root_scope = Scope {
        file_descriptor: fd,
        path: Vec::new(),
    };

    let mut r = Vec::new();

    root_scope.walk_scopes(|scope| {
        r.push_all(scope.get_enums().as_slice());
    });

    r
}

// find message by rust type name
pub fn find_message_by_rust_name<'a>(fd: &'a FileDescriptorProto, rust_name: &str)
    -> MessageWithScope<'a>
{
    find_messages(fd).into_iter()
            .find(|m| m.rust_name().as_slice() == rust_name)
            .unwrap()
}

// find enum by rust type name
pub fn find_enum_by_rust_name<'a>(fd: &'a FileDescriptorProto, rust_name: &str)
    -> EnumWithScope<'a>
{
    find_enums(fd).into_iter()
            .find(|e| e.rust_name().as_slice() == rust_name)
            .unwrap()
}

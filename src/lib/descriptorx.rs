/// utilities to work with descriptor

use descriptor::FileDescriptorProto;
use descriptor::DescriptorProto;
use descriptor::EnumDescriptorProto;
use descriptor::EnumValueDescriptorProto;
use descriptor::FieldDescriptorProto;
use descriptor::OneofDescriptorProto;


pub struct RootScope<'a> {
    pub file_descriptors: &'a [FileDescriptorProto],
}

impl<'a> RootScope<'a> {
    fn packages(&'a self) -> Vec<FileScope<'a>> {
        self.file_descriptors.iter()
            .map(|fd| FileScope { file_descriptor: fd })
            .collect()
    }

    pub fn find_enum(&'a self, fqn: &str) -> EnumWithScope<'a> {
        assert!(fqn.starts_with("."));
        let fqn1 = &fqn[1..];
        self.packages().into_iter()
            .flat_map(|p| {
                (if p.get_package().is_empty() {
                    p.find_enum(fqn1)
                } else if fqn1.starts_with(&(p.get_package().to_string() + ".")) {
                    let remaining = &fqn1[(p.get_package().len() + 1)..];
                    p.find_enum(remaining)
                } else {
                    None
                }).into_iter()
            })
            .next()
            .expect(&format!("enum not found by name: {}", fqn))
    }
}


#[derive(Clone)]
pub struct FileScope<'a> {
    pub file_descriptor: &'a FileDescriptorProto,
}

impl<'a> FileScope<'a> {
    fn get_package(&self) -> &'a str {
        self.file_descriptor.get_package()
    }

    pub fn to_scope(&self) -> Scope<'a> {
        Scope {
            file_scope: self.clone(),
            path: Vec::new(),
        }
    }

    fn find_enum(&self, name: &str) -> Option<EnumWithScope<'a>> {
        assert!(!name.starts_with("."));
        self.find_enums().into_iter()
            .filter(|e| e.name_to_package() == name)
            .next()
    }

    // find all enums in given file descriptor
    fn find_enums(&self) -> Vec<EnumWithScope<'a>> {
        let mut r = Vec::new();

        self.to_scope().walk_scopes(|scope| {
            r.extend(scope.get_enums());
        });

        r
    }

    // find all messages in given file descriptor
    fn find_messages(&self) -> Vec<MessageWithScope<'a>> {
        let mut r = Vec::new();

        self.to_scope().walk_scopes(|scope| {
            r.extend(scope.get_messages());
        });

        r
    }

}


#[derive(Clone)]
pub struct Scope<'a> {
    pub file_scope: FileScope<'a>,
    pub path: Vec<&'a DescriptorProto>,
}


impl<'a> Scope<'a> {
    pub fn get_file_descriptor(&self) -> &'a FileDescriptorProto {
        self.file_scope.file_descriptor
    }

    fn get_package(&self) -> &'a str {
        self.file_scope.file_descriptor.get_package()
    }

    // get message descriptors in this scope
    fn get_message_descriptors(&self) -> &'a [DescriptorProto] {
        if self.path.is_empty() {
            self.file_scope.file_descriptor.get_message_type()
        } else {
            self.path.last().unwrap().get_nested_type()
        }
    }

    // get enum descriptors in this scope
    fn get_enum_descriptors(&self) -> &'a [EnumDescriptorProto] {
        if self.path.is_empty() {
            self.file_scope.file_descriptor.get_enum_type()
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

    fn walk_scopes_impl<F : FnMut(&Scope<'a>)>(&self, callback: &mut F) {
        (*callback)(self);

        for nested in self.nested_scopes().iter() {
            nested.walk_scopes_impl(callback);
        }
    }

    // apply callback for this scope and all nested scopes
    fn walk_scopes<F>(&self, mut callback: F)
        where F : FnMut(&Scope<'a>)
    {
        self.walk_scopes_impl(&mut callback);
    }

    pub fn prefix(&self) -> String {
        if self.path.is_empty() {
            String::new()
        } else {
            let v: Vec<&'a str> = self.path.iter().map(|m| m.get_name()).collect();
            let mut r = v.connect(".");
            r.push_str(".");
            r
        }
    }

    // rust type name prefix for this scope
    pub fn rust_prefix(&self) -> String {
        self.prefix().replace(".", "_")
    }
}

pub trait WithScope<'a> {
    fn get_scope(&self) -> &Scope<'a>;

    fn get_file_descriptor(&self) -> &'a FileDescriptorProto {
        self.get_scope().get_file_descriptor()
    }

    // message or enum name
    fn get_name(&self) -> &'a str;

    // package name of this descriptor
    fn get_package(&self) -> &'a str {
        self.get_scope().get_package()
    }

    fn name_to_package(&self) -> String {
        let mut r = self.get_scope().prefix();
        r.push_str(self.get_name());
        r
    }

    // rust type name of this descriptor
    fn rust_name(&self) -> String {
        let mut r = self.get_scope().rust_prefix();
        r.push_str(self.get_name());
        r
    }
}

#[derive(Clone)]
pub struct MessageWithScope<'a> {
    pub scope: Scope<'a>,
    pub message: &'a DescriptorProto,
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

    pub fn fields(&'a self) -> Vec<FieldWithContext<'a>> {
        self.message.get_field().iter()
            .map(|f| FieldWithContext { field: f, message: self })
            .collect()
    }

    pub fn oneofs(&'a self) -> Vec<OneofWithContext<'a>> {
        self.message.get_oneof_decl().iter()
            .enumerate()
            .map(|(index, oneof)| OneofWithContext {
                message: &self, oneof: &oneof, index: index as u32
            })
            .collect()
    }

    pub fn oneof_by_index(&'a self, index: u32) -> OneofWithContext<'a> {
        self.oneofs().swap_remove(index as usize)
    }
}


#[derive(Clone)]
pub struct EnumWithScope<'a> {
    pub scope: Scope<'a>,
    pub en: &'a EnumDescriptorProto,
}


impl<'a> EnumWithScope<'a> {
    // enum values
    pub fn values(&'a self) -> &'a [EnumValueDescriptorProto] {
        self.en.get_value()
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


pub struct FieldWithContext<'a> {
    pub field: &'a FieldDescriptorProto,
    pub message: &'a MessageWithScope<'a>,
}

impl<'a> FieldWithContext<'a> {
    fn is_oneof(&self) -> bool {
        self.field.has_oneof_index()
    }

    pub fn oneof(&'a self) -> Option<OneofWithContext<'a>> {
        if self.is_oneof() {
            Some(self.message.oneof_by_index(self.field.get_oneof_index() as u32))
        } else {
            None
        }
    }
}


#[derive(Clone)]
pub struct OneofVariantWithContext<'a> {
    pub oneof: &'a OneofWithContext<'a>,
    pub field: &'a FieldDescriptorProto,
}

impl<'a> OneofVariantWithContext<'a> {
    pub fn field_name(&self) -> &str {
        match self.field.get_name() {
            "type" => "field_type",
            x => x,
        }
    }
}


#[derive(Clone)]
pub struct OneofWithContext<'a> {
    pub oneof: &'a OneofDescriptorProto,
    pub index: u32,
    pub message: &'a MessageWithScope<'a>
}

impl<'a> OneofWithContext<'a> {
    pub fn name(&'a self) -> &'a str {
        match self.oneof.get_name() {
            "type" => "field_type",
            x => x,
        }
    }

    // rust type name of enum
    pub fn rust_name(&self) -> String {
        format!("{}_oneof_{}", self.message.rust_name(), self.oneof.get_name())
    }

    pub fn variants(&'a self) -> Vec<OneofVariantWithContext<'a>> {
        self.message.fields().iter()
            .filter(|f| f.field.has_oneof_index() && f.field.get_oneof_index() == self.index as i32)
            .map(|f| OneofVariantWithContext { oneof: self, field: &f.field })
            .collect()
    }
}


// find message by rust type name
pub fn find_message_by_rust_name<'a>(fd: &'a FileDescriptorProto, rust_name: &str)
    -> MessageWithScope<'a>
{
    FileScope { file_descriptor: fd }
        .find_messages()
        .into_iter()
        .find(|m| m.rust_name() == rust_name)
        .unwrap()
}

// find enum by rust type name
pub fn find_enum_by_rust_name<'a>(fd: &'a FileDescriptorProto, rust_name: &str)
    -> EnumWithScope<'a>
{
    FileScope { file_descriptor: fd }
        .find_enums()
        .into_iter()
        .find(|e| e.rust_name() == rust_name)
        .unwrap()
}

/// utilities to work with descriptor

use descriptor::FileDescriptorProto;
use descriptor::DescriptorProto;
use descriptor::EnumDescriptorProto;
use descriptor::EnumValueDescriptorProto;
use descriptor::ServiceDescriptorProto;
use descriptor::MethodDescriptorProto;
use descriptor::FieldDescriptorProto;
use descriptor::OneofDescriptorProto;

use strx;
use rust;


// Copy-pasted from libsyntax.
fn ident_start(c: char) -> bool {
    (c >= 'a' && c <= 'z')
        || (c >= 'A' && c <= 'Z')
        || c == '_'
}

// Copy-pasted from libsyntax.
fn ident_continue(c: char) -> bool {
    (c >= 'a' && c <= 'z')
        || (c >= 'A' && c <= 'Z')
        || (c >= '0' && c <= '9')
        || c == '_'
}

pub fn proto_path_to_rust_mod(path: &str) -> String {
    let without_dir = strx::remove_to(path, '/');
    let without_suffix = strx::remove_suffix(without_dir, ".proto");
    without_suffix.chars().enumerate().map(|(i, c)| {
        let valid = if i == 0 { ident_start(c) } else { ident_continue(c) };
        if valid { c } else { '_' }
    }).collect()
}


pub struct RootScope<'a> {
    pub file_descriptors: &'a [FileDescriptorProto],
}

impl<'a> RootScope<'a> {
    fn packages(&'a self) -> Vec<FileScope<'a>> {
        self.file_descriptors.iter()
            .map(|fd| FileScope { file_descriptor: fd })
            .collect()
    }

    pub fn find_message(&'a self, fqn: &str) -> MessageWithScope<'a> {
        match self.find_message_or_enum(fqn) {
            MessageOrEnumWithScope::Message(m) => m,
            _ => panic!("not an message: {}", fqn),
        }
    }

    pub fn find_enum(&'a self, fqn: &str) -> EnumWithScope<'a> {
        match self.find_message_or_enum(fqn) {
            MessageOrEnumWithScope::Enum(e) => e,
            _ => panic!("not an enum: {}", fqn),
        }
    }

    pub fn find_message_or_enum(&'a self, fqn: &str) -> MessageOrEnumWithScope<'a> {
        assert!(fqn.starts_with("."));
        let fqn1 = &fqn[1..];
        self.packages().into_iter()
            .flat_map(|p| {
                (if p.get_package().is_empty() {
                    p.find_message_or_enum(fqn1)
                } else if fqn1.starts_with(&(p.get_package().to_string() + ".")) {
                    let remaining = &fqn1[(p.get_package().len() + 1)..];
                    p.find_message_or_enum(remaining)
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

    fn find_message_or_enum(&self, name: &str) -> Option<MessageOrEnumWithScope<'a>> {
        assert!(!name.starts_with("."));
        self.find_messages_and_enums().into_iter()
            .filter(|e| e.name_to_package() == name)
            .next()
    }

    // find all enums in given file descriptor
    pub fn find_enums(&self) -> Vec<EnumWithScope<'a>> {
        let mut r = Vec::new();

        self.to_scope().walk_scopes(|scope| {
            r.extend(scope.get_enums());
        });

        r
    }

    // find all messages in given file descriptor
    pub fn find_messages(&self) -> Vec<MessageWithScope<'a>> {
        let mut r = Vec::new();

        self.to_scope().walk_scopes(|scope| {
            r.extend(scope.get_messages());
        });

        r
    }

    // find all messages and enums in given file descriptor
    pub fn find_messages_and_enums(&self) -> Vec<MessageOrEnumWithScope<'a>> {
        let mut r = Vec::new();

        self.to_scope().walk_scopes(|scope| {
            r.extend(scope.get_messages_and_enums());
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

    // get service descriptors in this scope
    fn get_service_descriptors(&self) -> &'a [ServiceDescriptorProto] {
        if self.path.is_empty() {
            self.file_scope.file_descriptor.get_service()
        } else {
            ::std::default::Default::default()
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

    // get messages and enums with attached scopes in this scope
    pub fn get_messages_and_enums(&self) -> Vec<MessageOrEnumWithScope<'a>> {
        self.get_messages().into_iter().map(|m| MessageOrEnumWithScope::Message(m))
            .chain(self.get_enums().into_iter().map(|m| MessageOrEnumWithScope::Enum(m)))
            .collect()
    }

    pub fn get_services(&self) -> Vec<ServiceWithScope<'a>> {
        self.get_service_descriptors()
            .iter()
            .map(|s| {
                ServiceWithScope {
                    scope: self.clone(),
                    service: s,
                }
            })
            .collect()
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

        for nested in self.nested_scopes() {
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
            "".to_string()
        } else {
            let v: Vec<&'a str> = self.path.iter().map(|m| m.get_name()).collect();
            let mut r = v.join(".");
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

    // find enum value by name
    pub fn value_by_name(&'a self, name: &str) -> &'a EnumValueDescriptorProto {
        self.en.get_value().into_iter().find(|v| v.get_name() == name).unwrap()
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


pub enum MessageOrEnumWithScope<'a> {
    Message(MessageWithScope<'a>),
    Enum(EnumWithScope<'a>),
}

impl<'a> WithScope<'a> for MessageOrEnumWithScope<'a> {
    fn get_scope(&self) -> &Scope<'a> {
        match self {
            &MessageOrEnumWithScope::Message(ref m) => m.get_scope(),
            &MessageOrEnumWithScope::Enum(ref e) => e.get_scope(),
        }
    }

    fn get_name(&self) -> &'a str {
        match self {
            &MessageOrEnumWithScope::Message(ref m) => m.get_name(),
            &MessageOrEnumWithScope::Enum(ref e) => e.get_name(),
        }
    }
}


#[derive(Clone)]
pub struct ServiceWithScope<'a> {
    pub scope: Scope<'a>,
    pub service: &'a ServiceDescriptorProto,
}

impl<'a> ServiceWithScope<'a> {
    // service methods
    pub fn methods(&'a self) -> Vec<MethodWithContext<'a>> {
        self.service
            .get_method()
            .iter()
            .map(|m| {
                MethodWithContext {
                    method: m,
                    service: self,
                }
            })
            .collect()
    }
}

impl<'a> WithScope<'a> for ServiceWithScope<'a> {
    fn get_scope(&self) -> &Scope<'a> {
        &self.scope
    }

    fn get_name(&self) -> &'a str {
        self.service.get_name()
    }
}


#[derive(Clone)]
pub struct MethodWithContext<'a> {
    pub method: &'a MethodDescriptorProto,
    pub service: &'a ServiceWithScope<'a>,
}

impl<'a> MethodWithContext<'a> {
    // method name in generated code
    pub fn rust_name(&self) -> String {
        if rust::is_rust_keyword(self.method.get_name()) {
            format!("method_{}", self.method.get_name())
        } else {
            self.method.get_name().to_string()
        }
    }

    pub fn request_type(&self) -> &'a str {
        self.method.get_input_type()
    }

    pub fn response_type(&self) -> &'a str {
        self.method.get_output_type()
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

    // field name in generated code
    pub fn rust_name(&self) -> String {
        if rust::is_rust_keyword(self.field.get_name()) {
            format!("field_{}", self.field.get_name())
        } else {
            self.field.get_name().to_string()
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
            "box" => "field_box",
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
            "box" => "field_box",
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

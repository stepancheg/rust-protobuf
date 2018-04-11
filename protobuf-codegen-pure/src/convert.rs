//! Convert protobuf_parser model to rust-protobuf model

use std::iter;

use protobuf_parser;
use protobuf;


enum MessageOrEnum {
    Message,
    Enum,
}

impl MessageOrEnum {
    fn descriptor_type(&self) -> protobuf::descriptor::FieldDescriptorProto_Type {
        match *self {
            MessageOrEnum::Message => protobuf::descriptor::FieldDescriptorProto_Type::TYPE_MESSAGE,
            MessageOrEnum::Enum => protobuf::descriptor::FieldDescriptorProto_Type::TYPE_ENUM,
        }
    }
}


#[derive(Debug, Eq, PartialEq)]
struct RelativePath {
    path: String,
}

impl RelativePath {
    fn empty() -> RelativePath {
        RelativePath::new(String::new())
    }

    fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    fn new(path: String) -> RelativePath {
        assert!(!path.starts_with("."));

        RelativePath {
            path
        }
    }

    fn append(&self, simple: &str) -> RelativePath {
        if self.path.is_empty() {
            RelativePath::new(simple.to_owned())
        } else {
            RelativePath::new(format!("{}.{}", self.path, simple))
        }
    }

    fn split(&self) -> Option<(String, RelativePath)> {
        if self.is_empty() {
            None
        } else {
            Some(match self.path.find('.') {
                Some(dot) => {
                    (
                        self.path[..dot].to_owned(),
                        RelativePath::new(self.path[dot+1..].to_owned())
                    )
                }
                None => {
                    (
                        self.path.clone(),
                        RelativePath::empty()
                    )
                }
            })
        }
    }
}


#[derive(Clone, Eq, PartialEq, Debug)]
struct AbsolutePath {
    path: String,
}

impl AbsolutePath {
    fn root() -> AbsolutePath {
        AbsolutePath::new(String::new())
    }

    fn new(path: String) -> AbsolutePath {
        assert!(path.is_empty() || path.starts_with("."));
        assert!(!path.ends_with("."));
        AbsolutePath { path }
    }

    fn from_path_without_dot(path: &str) -> AbsolutePath {
        if path.is_empty() {
            AbsolutePath::root()
        } else {
            assert!(!path.starts_with("."));
            assert!(!path.ends_with("."));
            AbsolutePath::new(format!(".{}", path))
        }
    }

    fn from_path_maybe_dot(path: &str) -> AbsolutePath {
        if path.starts_with(".") {
            AbsolutePath::new(path.to_owned())
        } else {
            AbsolutePath::from_path_without_dot(path)
        }
    }

    fn push_simple(&mut self, simple: &str) {
        assert!(!simple.is_empty());
        assert!(!simple.contains('.'));
        self.path.push('.');
        self.path.push_str(simple);
    }

    fn push_relative(&mut self, relative: &RelativePath) {
        if !relative.is_empty() {
            self.path.push('.');
            self.path.push_str(&relative.path);
        }
    }

    fn remove_prefix(&self, prefix: &AbsolutePath) -> Option<RelativePath> {
        if self.path.starts_with(&prefix.path) {
            let rem = &self.path[prefix.path.len()..];
            if rem.is_empty() {
                return Some(RelativePath::empty());
            }
            if rem.starts_with('.') {
                return Some(RelativePath::new(rem[1..].to_owned()));
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn absolute_path_push_simple() {
        let mut foo = AbsolutePath::new(".foo".to_owned());
        foo.push_simple("bar");
        assert_eq!(AbsolutePath::new(".foo.bar".to_owned()), foo);

        let mut foo = AbsolutePath::root();
        foo.push_simple("bar");
        assert_eq!(AbsolutePath::new(".bar".to_owned()), foo);
    }

    #[test]
    fn absolute_path_remove_prefix() {
        assert_eq!(
            Some(RelativePath::empty()),
            AbsolutePath::new(".foo".to_owned())
                .remove_prefix(&AbsolutePath::new(".foo".to_owned())));
        assert_eq!(
            Some(RelativePath::new("bar".to_owned())),
            AbsolutePath::new(".foo.bar".to_owned())
                .remove_prefix(&AbsolutePath::new(".foo".to_owned())));
        assert_eq!(
            Some(RelativePath::new("baz.qux".to_owned())),
            AbsolutePath::new(".foo.bar.baz.qux".to_owned())
                .remove_prefix(&AbsolutePath::new(".foo.bar".to_owned())));
        assert_eq!(
            None,
            AbsolutePath::new(".foo.barbaz".to_owned())
                .remove_prefix(&AbsolutePath::new(".foo.bar".to_owned())));
    }
}


enum LookupScope<'a> {
    File(&'a protobuf_parser::FileDescriptor),
    Message(&'a protobuf_parser::Message),
}

impl<'a> LookupScope<'a> {
    fn messages(&self) -> &[protobuf_parser::Message] {
        match self {
            &LookupScope::File(file) => &file.messages,
            &LookupScope::Message(messasge) => &messasge.messages,
        }
    }

    fn enums(&self) -> &[protobuf_parser::Enumeration] {
        match self {
            &LookupScope::File(file) => &file.enums,
            &LookupScope::Message(messasge) => &messasge.enums,
        }
    }

    fn members(&self) -> Vec<(&str, MessageOrEnum)> {
        let mut r = Vec::new();
        r.extend(self.enums().into_iter().map(|e| (&e.name[..], MessageOrEnum::Enum)));
        r.extend(self.messages().into_iter().map(|e| (&e.name[..], MessageOrEnum::Message)));
        r
    }

    fn resolve_message_or_enum(&self, current_path: &AbsolutePath, path: &RelativePath)
        -> Option<(AbsolutePath, MessageOrEnum)>
    {
        let (first, rem) = match path.split() {
            Some(x) => x,
            None => return None,
        };

        if rem.is_empty() {
            for member in self.members() {
                if member.0 == first {
                    let mut result_path = current_path.clone();
                    result_path.push_simple(member.0);
                    return Some((result_path, member.1));
                }
            }
            None
        } else {
            for message in self.messages() {
                if message.name == first {
                    let mut message_path = current_path.clone();
                    message_path.push_simple(&message.name);
                    let message_scope = LookupScope::Message(message);
                    return message_scope.resolve_message_or_enum(&message_path, &rem);
                }
            }
            None
        }
    }

}


struct Resolver<'a> {
    current_file: &'a protobuf_parser::FileDescriptor,
    deps: &'a [protobuf_parser::FileDescriptor],
}

impl<'a> Resolver<'a> {
    fn map_entry_name_for_field_name(field_name: &str) -> String {
        format!("{}_MapEntry", field_name)
    }

    fn map_entry_field(
        &self,
        name: &str,
        number: i32,
        field_type: &protobuf_parser::FieldType,
        path_in_file: &RelativePath)
        -> protobuf::descriptor::FieldDescriptorProto
    {
        let mut output = protobuf::descriptor::FieldDescriptorProto::new();
        output.set_name(name.to_owned());
        output.set_number(number);

        let (t, t_name) = self.field_type(name, field_type, path_in_file);
        output.set_field_type(t);
        if let Some(t_name) = t_name {
            output.set_type_name(t_name.path);
        }
        
        output
    }

    fn map_entry_message(
        &self,
        field_name: &str,
        key: &protobuf_parser::FieldType,
        value: &protobuf_parser::FieldType,
        path_in_file: &RelativePath)
        -> protobuf::descriptor::DescriptorProto
    {
        let mut output = protobuf::descriptor::DescriptorProto::new();

        output.mut_options().set_map_entry(true);
        output.set_name(Resolver::map_entry_name_for_field_name(field_name));
        output.mut_field().push(self.map_entry_field("key", 1, key, path_in_file));
        output.mut_field().push(self.map_entry_field("value", 2, value, path_in_file));

        output
    }

    fn message(&self, input: &protobuf_parser::Message, path_in_file: &RelativePath)
        -> protobuf::descriptor::DescriptorProto
    {
        let nested_path_in_file = path_in_file.append(&input.name);

        let mut output = protobuf::descriptor::DescriptorProto::new();
        output.set_name(input.name.clone());

        let nested_messages_regular = input.messages.iter()
            .map(|m| self.message(m, &nested_path_in_file));

        let nested_messages_map = input.fields.iter().filter_map(|f| {
            match f.typ {
                protobuf_parser::FieldType::Map(ref t) => {
                    Some(self.map_entry_message(&f.name, &t.0, &t.1, path_in_file))
                }
                _ => None,
            }
        });

        output.set_nested_type(nested_messages_regular.chain(nested_messages_map).collect());

        output.set_enum_type(input.enums.iter().map(|e| self.enumeration(e)).collect());

        {
            let regular_fields = input.fields.iter()
                .map(|f| self.field(f, None, &nested_path_in_file));

            let oneof_fields = input.oneofs.iter().enumerate()
                .flat_map(|(oneof_index, oneof)| {
                    let oneof_index = oneof_index as i32;
                    oneof.fields.iter().zip(iter::repeat(oneof_index))
                        .map(|(f, oneof_index)| self.field(f, Some(oneof_index), &nested_path_in_file))
                });

            output.set_field(regular_fields.chain(oneof_fields).collect());
        }

        let oneofs = input.oneofs.iter()
            .map(|o| self.oneof(o))
            .collect();
        output.set_oneof_decl(oneofs);

        output
    }

    fn field(&self, input: &protobuf_parser::Field, oneof_index: Option<i32>, path_in_file: &RelativePath)
        -> protobuf::descriptor::FieldDescriptorProto
    {
        let mut output = protobuf::descriptor::FieldDescriptorProto::new();
        output.set_name(input.name.clone());

        if let protobuf_parser::FieldType::Map(..) = input.typ {
            output.set_label(protobuf::descriptor::FieldDescriptorProto_Label::LABEL_REPEATED);
        } else {
            output.set_label(label(input.rule));
        }

        let (t, t_name) = self.field_type(&input.name, &input.typ, path_in_file);
        output.set_field_type(t);
        if let Some(t_name) = t_name {
            output.set_type_name(t_name.path);
        }

        output.set_number(input.number);
        if let Some(ref default) = input.default {
            let default = match output.get_field_type() {
                protobuf::descriptor::FieldDescriptorProto_Type::TYPE_STRING => {
                    if default.starts_with('"') && default.ends_with('"') {
                        default[1..default.len() - 1]
                            // TODO: properly decode
                            .replace("\\n", "\n")
                            .replace("\\t", "\t")
                    } else {
                        default.clone()
                    }
                }
                protobuf::descriptor::FieldDescriptorProto_Type::TYPE_BYTES => {
                    if default.starts_with('"') && default.ends_with('"') {
                        default[1..default.len() - 1].to_owned()
                    } else {
                        default.clone()
                    }
                }
                _ => {
                    default.clone()
                }
            };
            output.set_default_value(default);
        }
        if let Some(packed) = input.packed {
            output.mut_options().set_packed(packed);
        }

        if let Some(oneof_index) = oneof_index {
            output.set_oneof_index(oneof_index);
        }

        output.mut_options().set_deprecated(input.deprecated);
        output
    }

    fn all_files(&self) -> Vec<&protobuf_parser::FileDescriptor> {
        iter::once(self.current_file).chain(self.deps).collect()
    }

    fn current_file_package_files(&self) -> Vec<&protobuf_parser::FileDescriptor> {
        self.all_files().into_iter()
            .filter(|f| f.package == self.current_file.package)
            .collect()
    }

    fn resolve_message_or_enum(&self, name: &str, _path_in_file: &RelativePath)
        -> (AbsolutePath, MessageOrEnum)
    {
        // find message or enum in current package
        if !name.starts_with(".") {
            for file in self.current_file_package_files() {
                if let Some((n, t)) = LookupScope::File(file).resolve_message_or_enum(
                    &AbsolutePath::from_path_without_dot(&file.package),
                    &RelativePath::new(name.to_owned()))
                {
                    return (n, t)
                }
            }
        }

        // find message or enum in root package
        {
            let absolute_path = AbsolutePath::from_path_maybe_dot(name);
            for file in self.all_files() {
                let file_package = AbsolutePath::from_path_without_dot(&file.package);
                if let Some(relative) = absolute_path.remove_prefix(&file_package) {
                    if let Some((n, t)) = LookupScope::File(file).resolve_message_or_enum(
                        &file_package,
                        &relative)
                    {
                        return (n, t)
                    }
                }
            }
        }

        // TODO: find names in outer messages
        panic!("couldn't find message or enum {} when parsing {}", name, self.current_file.package);
    }

    fn field_type(&self, name: &str, input: &protobuf_parser::FieldType, path_in_file: &RelativePath)
        -> (protobuf::descriptor::FieldDescriptorProto_Type, Option<AbsolutePath>)
    {
        match *input {
            protobuf_parser::FieldType::Bool =>
                (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_BOOL, None),
            protobuf_parser::FieldType::Int32 =>
                (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_INT32, None),
            protobuf_parser::FieldType::Int64 =>
                (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_INT64, None),
            protobuf_parser::FieldType::Uint32 =>
                (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_UINT32, None),
            protobuf_parser::FieldType::Uint64 =>
                (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_UINT64, None),
            protobuf_parser::FieldType::Sint32 =>
                (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_SINT32, None),
            protobuf_parser::FieldType::Sint64 =>
                (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_SINT64, None),
            protobuf_parser::FieldType::Fixed32 =>
                (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_FIXED32, None),
            protobuf_parser::FieldType::Fixed64 =>
                (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_FIXED64, None),
            protobuf_parser::FieldType::Sfixed32 =>
                (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_SFIXED32, None),
            protobuf_parser::FieldType::Sfixed64 =>
                (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_SFIXED64, None),
            protobuf_parser::FieldType::Float =>
                (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_FLOAT, None),
            protobuf_parser::FieldType::Double =>
                (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_DOUBLE, None),
            protobuf_parser::FieldType::String =>
                (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_STRING, None),
            protobuf_parser::FieldType::Bytes =>
                (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_BYTES, None),
            protobuf_parser::FieldType::MessageOrEnum(ref name) => {
                let (name, me) = self.resolve_message_or_enum(&name, path_in_file);
                (me.descriptor_type(), Some(name))
            }
            protobuf_parser::FieldType::Map(..) => {
                let mut type_name = AbsolutePath::from_path_without_dot(&self.current_file.package);
                type_name.push_relative(path_in_file);
                type_name.push_simple(&Resolver::map_entry_name_for_field_name(name));
                (
                    protobuf::descriptor::FieldDescriptorProto_Type::TYPE_MESSAGE,
                    Some(type_name)
                )
            }
            protobuf_parser::FieldType::Group(..) => {
                (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_GROUP, None)
            }
        }
    }

    fn enum_value(&self, name: &str, number: i32) -> protobuf::descriptor::EnumValueDescriptorProto {
        let mut output = protobuf::descriptor::EnumValueDescriptorProto::new();
        output.set_name(name.to_owned());
        output.set_number(number);
        output
    }

    fn enumeration(&self, input: &protobuf_parser::Enumeration) -> protobuf::descriptor::EnumDescriptorProto {
        let mut output = protobuf::descriptor::EnumDescriptorProto::new();
        output.set_name(input.name.clone());
        output.set_value(input.values.iter().map(|v| self.enum_value(&v.name, v.number)).collect());
        output
    }

    fn oneof(&self, input: &protobuf_parser::OneOf) -> protobuf::descriptor::OneofDescriptorProto {
        let mut output = protobuf::descriptor::OneofDescriptorProto::new();
        output.set_name(input.name.clone());
        output
    }
}

fn syntax(input: protobuf_parser::Syntax) -> String {
    match input {
        protobuf_parser::Syntax::Proto2 => "proto2".to_owned(),
        protobuf_parser::Syntax::Proto3 => "proto3".to_owned(),
    }
}

fn label(input: protobuf_parser::Rule) -> protobuf::descriptor::FieldDescriptorProto_Label {
    match input {
        protobuf_parser::Rule::Optional =>
            protobuf::descriptor::FieldDescriptorProto_Label::LABEL_OPTIONAL,
        protobuf_parser::Rule::Required =>
            protobuf::descriptor::FieldDescriptorProto_Label::LABEL_REQUIRED,
        protobuf_parser::Rule::Repeated =>
            protobuf::descriptor::FieldDescriptorProto_Label::LABEL_REPEATED,
    }
}

pub fn file_descriptor(
    name: String,
    input: &protobuf_parser::FileDescriptor,
    deps: &[protobuf_parser::FileDescriptor])
    -> protobuf::descriptor::FileDescriptorProto
{
    let resolver = Resolver {
        current_file: &input,
        deps,
    };

    let mut output = protobuf::descriptor::FileDescriptorProto::new();
    output.set_name(name);
    output.set_package(input.package.clone());
    output.set_syntax(syntax(input.syntax));

    let messages = input.messages.iter()
        .map(|m| resolver.message(m, &RelativePath::empty()))
        .collect();
    output.set_message_type(messages);

    output.set_enum_type(input.enums.iter().map(|e| resolver.enumeration(e)).collect());
    output
}

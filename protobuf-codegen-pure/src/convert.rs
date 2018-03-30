//! Convert protobuf_parser model to rust-protobuf model

use std::iter;

use protobuf_parser;
use protobuf;


struct PackageName {
    /// Empty for root, starts with dot otherwise
    name: String,
}

impl PackageName {
    fn root() -> PackageName {
        PackageName::package(String::new())
    }

    fn package(name: String) -> PackageName {
        assert!(name.is_empty() || name.starts_with("."));
        PackageName {
            name
        }
    }

    fn package_without_dot(name: &str) -> PackageName {
        if name.is_empty() {
            PackageName::root()
        } else {
            PackageName::package(format!(".{}", name))
        }
    }
}


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


struct PathInFile {
    path: String,
}

impl PathInFile {
    fn root() -> PathInFile {
        PathInFile::new(String::new())
    }

    fn new(path: String) -> PathInFile {
        assert!(!path.starts_with("."));

        PathInFile {
            path
        }
    }

    fn append(&self, simple: &str) -> PathInFile {
        if self.path.is_empty() {
            PathInFile::new(simple.to_owned())
        } else {
            PathInFile::new(format!("{}.{}", self.path, simple))
        }
    }
}


struct Resolver<'a> {
    current_file: &'a protobuf_parser::FileDescriptor,
    deps: &'a [protobuf_parser::FileDescriptor],
}

impl<'a> Resolver<'a> {
    fn current_file_package(&self) -> PackageName {
        PackageName::package_without_dot(&self.current_file.package)
    }

    fn message(&self, input: &protobuf_parser::Message, path_in_file: &PathInFile)
        -> protobuf::descriptor::DescriptorProto
    {
        let nested_path_in_file = path_in_file.append(&input.name);

        let mut output = protobuf::descriptor::DescriptorProto::new();
        output.set_name(input.name.clone());

        let nested_messages = input.messages.iter()
            .map(|m| self.message(m, &nested_path_in_file))
            .collect();
        output.set_nested_type(nested_messages);

        output.set_enum_type(input.enums.iter().map(|e| self.enumeration(e)).collect());

        let fields = input.fields.iter()
            .map(|f| self.field(f, &nested_path_in_file))
            .collect();
        output.set_field(fields);

        let oneofs = input.oneofs.iter()
            .map(|o| self.oneof(o))
            .collect();
        output.set_oneof_decl(oneofs);

        output
    }

    fn field(&self, input: &protobuf_parser::Field, path_in_file: &PathInFile)
        -> protobuf::descriptor::FieldDescriptorProto
    {
        let mut output = protobuf::descriptor::FieldDescriptorProto::new();
        output.set_name(input.name.clone());
        output.set_label(label(input.rule));

        let (t, t_name) = self.field_type(&input.typ, path_in_file);
        output.set_field_type(t);
        if let Some(t_name) = t_name {
            output.set_type_name(t_name);
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
        output.mut_options().set_deprecated(input.deprecated);
        output
    }

    fn resolve_message_or_enum(&self, name: &str, path_in_file: &PathInFile)
        -> (String, MessageOrEnum)
    {
        if name.starts_with(".") {
            for _file in iter::once(self.current_file).chain(self.deps) {
                unimplemented!("absolute paths are to be implemented");
            }

            // TODO: error instead of panic
            panic!("type is not found: {}", name);
        }

        if name.contains(".") {
            unimplemented!("non-root names are not implemented either")
        }

        let message_or_enum;
        if self.current_file.messages.iter().any(|m| m.name == name) {
            message_or_enum = MessageOrEnum::Message;
        } else if self.current_file.enums.iter().any(|e| e.name == name) {
            message_or_enum = MessageOrEnum::Enum;
        } else {
            unimplemented!("name could be relative");
        }

        (format!("{}.{}", self.current_file_package().name, name), message_or_enum)
    }

    fn field_type(&self, input: &protobuf_parser::FieldType, path_in_file: &PathInFile)
        -> (protobuf::descriptor::FieldDescriptorProto_Type, Option<String>)
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
            protobuf_parser::FieldType::Map(..) => unimplemented!(),
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
        // TODO: fields
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
        .map(|m| resolver.message(m, &PathInFile::root()))
        .collect();
    output.set_message_type(messages);

    output.set_enum_type(input.enums.iter().map(|e| resolver.enumeration(e)).collect());
    output
}

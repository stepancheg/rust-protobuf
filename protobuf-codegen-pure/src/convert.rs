//! Convert protobuf_parser model to rust-protobuf model

use protobuf_parser;
use protobuf;


fn syntax(input: protobuf_parser::Syntax) -> String {
    match input {
        protobuf_parser::Syntax::Proto2 => "proto2".to_owned(),
        protobuf_parser::Syntax::Proto3 => "proto3".to_owned(),
    }
}

fn label(input: protobuf_parser::Frequency) -> protobuf::descriptor::FieldDescriptorProto_Label {
    match input {
        protobuf_parser::Frequency::Optional =>
            protobuf::descriptor::FieldDescriptorProto_Label::LABEL_OPTIONAL,
        protobuf_parser::Frequency::Required =>
            protobuf::descriptor::FieldDescriptorProto_Label::LABEL_REQUIRED,
        protobuf_parser::Frequency::Repeated =>
            protobuf::descriptor::FieldDescriptorProto_Label::LABEL_REPEATED,
    }
}

fn field_type(input: protobuf_parser::FieldType)
    -> (protobuf::descriptor::FieldDescriptorProto_Type, Option<String>)
{
    match input {
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
        protobuf_parser::FieldType::String_ =>
            (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_STRING, None),
        protobuf_parser::FieldType::Bytes =>
            (protobuf::descriptor::FieldDescriptorProto_Type::TYPE_BYTES, None),
        protobuf_parser::FieldType::Message(_name) => unimplemented!(),
        protobuf_parser::FieldType::Enum(_name) => unimplemented!(),
        protobuf_parser::FieldType::Map(..) => unimplemented!(),
    }
}

fn field(input: protobuf_parser::Field) -> protobuf::descriptor::FieldDescriptorProto {
    let mut output = protobuf::descriptor::FieldDescriptorProto::new();
    output.set_name(input.name);
    output.set_label(label(input.frequency));

    let (t, t_name) = field_type(input.typ);
    output.set_field_type(t);
    if let Some(t_name) = t_name {
        output.set_type_name(t_name);
    }

    output.set_number(input.number);
    if let Some(default) = input.default {
        output.set_default_value(default);
    }
    if let Some(packed) = input.packed {
        output.mut_options().set_packed(packed);
    }
    output.mut_options().set_deprecated(input.deprecated);
    output
}

fn enum_value(name: String, number: i32) -> protobuf::descriptor::EnumValueDescriptorProto {
    let mut output = protobuf::descriptor::EnumValueDescriptorProto::new();
    output.set_name(name);
    output.set_number(number);
    output
}

fn enumeration(input: protobuf_parser::Enumerator) -> protobuf::descriptor::EnumDescriptorProto {
    let mut output = protobuf::descriptor::EnumDescriptorProto::new();
    output.set_name(input.name);
    output.set_value(input.fields.into_iter().map(|(n, v)| enum_value(n, v)).collect());
    output
}

fn oneof(input: protobuf_parser::OneOf) -> protobuf::descriptor::OneofDescriptorProto {
    let mut output = protobuf::descriptor::OneofDescriptorProto::new();
    output.set_name(input.name);
    // TODO: fields
    output
}

fn message(input: protobuf_parser::Message)
    -> protobuf::descriptor::DescriptorProto
{
    let mut output = protobuf::descriptor::DescriptorProto::new();
    output.set_name(input.name);

    output.set_nested_type(input.messages.into_iter().map(message).collect());
    output.set_enum_type(input.enums.into_iter().map(enumeration).collect());

    output.set_field(input.fields.into_iter().map(field).collect());
    output.set_oneof_decl(input.oneofs.into_iter().map(oneof).collect());

    output
}

pub fn file_descriptor(name: String, input: protobuf_parser::FileDescriptor)
    -> protobuf::descriptor::FileDescriptorProto
{
    let mut output = protobuf::descriptor::FileDescriptorProto::new();
    output.set_name(name);
    output.set_package(input.package);
    output.set_syntax(syntax(input.syntax));
    output.set_message_type(input.messages.into_iter().map(message).collect());
    output.set_enum_type(input.enums.into_iter().map(enumeration).collect());
    output
}

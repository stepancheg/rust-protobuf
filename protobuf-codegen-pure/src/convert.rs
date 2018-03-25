//! Convert protobuf_parser model to rust-protobuf model

use protobuf_parser;
use protobuf;


fn syntax(input: protobuf_parser::Syntax) -> String {
    match input {
        protobuf_parser::Syntax::Proto2 => "proto2".to_owned(),
        protobuf_parser::Syntax::Proto3 => "proto3".to_owned(),
    }
}

fn message(input: protobuf_parser::Message)
    -> protobuf::descriptor::DescriptorProto
{
    let mut output = protobuf::descriptor::DescriptorProto::new();
    output.set_name(input.name);

    output.set_nested_type(input.messages.into_iter().map(message).collect());

    // TODO: fields
    // TODO: enums
    // TODO: oneofs

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
    output
}

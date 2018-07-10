use std::fs;
use std::process;
use std::io::Write;
use std::io::Read;

use tempfile;

use protobuf::Message;
use protobuf::reflect::MessageDescriptor;
use protobuf::descriptor::FileDescriptorSet;
use protobuf::text_format::merge_from_str;
use protobuf::text_format::print_to_string;


fn parse_using_rust_protobuf(text: &str, message_descriptor: &MessageDescriptor) -> Box<Message> {
    let mut message = message_descriptor.new_instance();

    merge_from_str(&mut *message, text).expect(&format!("merge_from_str: {:?}", text));

    message
}

fn parse_using_protoc(text: &str, message_descriptor: &MessageDescriptor) -> Box<Message> {
    let temp_dir = tempfile::Builder::new().prefix(message_descriptor.name()).tempdir()
        .expect("temp dir");

    let mut fds = FileDescriptorSet::new();
    fds.mut_file().push(message_descriptor.file_descriptor_proto().clone());

    let mut temp_file = temp_dir.path().to_owned();
    temp_file.push("fds");

    fs::write(&temp_file, fds.write_to_bytes().expect("seriailze")).expect("write");

    // TODO: use protoc crate
    let mut protoc = process::Command::new("protoc")
        .args(&[
            &format!("--descriptor_set_in={}", temp_file.to_str().expect("to_str")),
            &format!("--encode={}", message_descriptor.full_name()),
            message_descriptor.file_descriptor_proto().get_name(),
        ])
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::inherit())
        .spawn()
        .expect("protoc");

    let mut stdin = protoc.stdin.take().expect("stdin");
    stdin.write_all(text.as_bytes()).expect("write to stdin");
    drop(stdin);

    let mut encoded = Vec::new();
    protoc.stdout.take().expect("stdout").read_to_end(&mut encoded).expect("read_to_end");

    let exit_status = protoc.wait().expect("wait");
    assert!(exit_status.success(),
        "exit status: {:?} when parsing with protoc: {:?}",
        exit_status, text);

    let mut expected = message_descriptor.new_instance();
    expected.merge_from_bytes(&encoded).expect("merge_from_bytes");

    expected
}

fn print_using_protoc(message: &Message) -> String {
    let message_descriptor = message.descriptor();

    let temp_dir = tempfile::Builder::new().prefix(message_descriptor.name()).tempdir()
        .expect("temp dir");

    let mut fds = FileDescriptorSet::new();
    fds.mut_file().push(message_descriptor.file_descriptor_proto().clone());

    let mut temp_file = temp_dir.path().to_owned();
    temp_file.push("fds");

    fs::write(&temp_file, fds.write_to_bytes().expect("seriailze")).expect("write");

    // TODO: use protoc crate
    let mut protoc = process::Command::new("protoc")
        .args(&[
            &format!("--descriptor_set_in={}", temp_file.to_str().expect("to_str")),
            &format!("--decode={}", message_descriptor.full_name()),
            message_descriptor.file_descriptor_proto().get_name(),
        ])
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::inherit())
        .spawn()
        .expect("protoc");

    let mut stdin = protoc.stdin.take().expect("stdin");
    stdin.write_all(&message.write_to_bytes().expect("serialize")).expect("write to stdin");
    drop(stdin);

    let mut decoded = String::new();
    protoc.stdout.take().expect("stdout").read_to_string(&mut decoded).expect("read_to_end");

    let exit_status = protoc.wait().expect("wait");
    assert!(exit_status.success(),
        "exit status: {:?} while printing: {:?}", exit_status, message);

    decoded
}

pub fn test_text_format_str_descriptor(text: &str, message_descriptor: &MessageDescriptor) {
    let message = parse_using_rust_protobuf(text, message_descriptor);
    let expected = parse_using_protoc(text, message_descriptor);

    assert!(message_descriptor.eq(&*expected, &*message), "{:?} != {:?}", expected, message);

    // print using protoc and parse using rust-protobuf
    let printed_using_protoc = print_using_protoc(&*message);
    let pp = parse_using_rust_protobuf(&printed_using_protoc, message_descriptor);

    assert!(message_descriptor.eq(&*expected, &*pp), "{:?} != {:?}", expected, message);
}


pub fn test_text_format_str_message(expected: &str, message: &Message) {
    assert_eq!(expected, &*print_to_string(message));

    test_text_format_str_descriptor(expected, message.descriptor());
}

pub fn test_text_format_message(message: &Message) {
    let descriptor = message.descriptor();

    let printed_with_rust_protobuf = print_to_string(message);
    let printed_with_protoc = print_using_protoc(message);

    let from_protoc = parse_using_rust_protobuf(&printed_with_protoc, descriptor);
    let from_protobuf = parse_using_protoc(&printed_with_rust_protobuf, descriptor);

    assert!(descriptor.eq(&*message, &*from_protoc), "{:?} != {:?}", message, from_protoc);
    assert!(descriptor.eq(&*message, &*from_protobuf), "{:?} != {:?}", message, from_protobuf);
}

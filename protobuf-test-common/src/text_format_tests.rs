use std::error::Error;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::process;

use tempfile;

use protobuf::descriptor;
use protobuf::descriptor::FileDescriptorSet;
use protobuf::reflect::MessageDescriptor;
use protobuf::rustproto;
use protobuf::text_format::merge_from_str;
use protobuf::text_format::print_to_string;
use protobuf::Message;

pub fn parse_using_rust_protobuf(
    text: &str,
    message_descriptor: &MessageDescriptor,
) -> Result<Box<dyn Message>, Box<dyn Error>> {
    let mut message = message_descriptor.new_instance();

    merge_from_str(&mut *message, text)?;

    Ok(message)
}

fn parse_using_protoc(text: &str, message_descriptor: &MessageDescriptor) -> Box<dyn Message> {
    let temp_dir = tempfile::Builder::new()
        .prefix(message_descriptor.name())
        .tempdir()
        .expect("temp dir");

    let mut fds = FileDescriptorSet::new();
    fds.file = vec![
        descriptor::file_descriptor_proto().clone(),
        rustproto::file_descriptor_proto().clone(),
        message_descriptor.file_descriptor_proto().clone(),
    ]
    .into();

    let mut temp_file = temp_dir.path().to_owned();
    temp_file.push("fds");

    fs::write(&temp_file, fds.write_to_bytes().expect("seriailze")).expect("write");

    // TODO: use protoc crate
    let mut protoc = process::Command::new("protoc")
        .args(&[
            &format!(
                "--descriptor_set_in={}",
                temp_file.to_str().expect("to_str")
            ),
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
    protoc
        .stdout
        .take()
        .expect("stdout")
        .read_to_end(&mut encoded)
        .expect("read_to_end");

    let exit_status = protoc.wait().expect("wait");
    assert!(
        exit_status.success(),
        "exit status: {:?} when parsing with protoc: {:?}",
        exit_status,
        text
    );

    let mut expected = message_descriptor.new_instance();
    expected
        .merge_from_bytes(&encoded)
        .expect("merge_from_bytes");

    expected
}

fn print_using_protoc(message: &dyn Message) -> String {
    let message_descriptor = message.descriptor();

    // TODO: copy-paste of parse_using_protoc

    let temp_dir = tempfile::Builder::new()
        .prefix(message_descriptor.name())
        .tempdir()
        .expect("temp dir");

    let mut fds = FileDescriptorSet::new();
    fds.file = vec![
        descriptor::file_descriptor_proto().clone(),
        rustproto::file_descriptor_proto().clone(),
        message_descriptor.file_descriptor_proto().clone(),
    ]
    .into();

    let mut temp_file = temp_dir.path().to_owned();
    temp_file.push("fds");

    fs::write(&temp_file, fds.write_to_bytes().expect("seriailze")).expect("write");

    // TODO: use protoc crate
    let mut protoc = process::Command::new("protoc")
        .args(&[
            &format!(
                "--descriptor_set_in={}",
                temp_file.to_str().expect("to_str")
            ),
            &format!("--decode={}", message_descriptor.full_name()),
            message_descriptor.file_descriptor_proto().get_name(),
        ])
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::inherit())
        .spawn()
        .expect("protoc");

    let mut stdin = protoc.stdin.take().expect("stdin");
    stdin
        .write_all(&message.write_to_bytes().expect("serialize"))
        .expect("write to stdin");
    drop(stdin);

    let mut decoded = String::new();
    protoc
        .stdout
        .take()
        .expect("stdout")
        .read_to_string(&mut decoded)
        .expect("read_to_end");

    let exit_status = protoc.wait().expect("wait");
    assert!(
        exit_status.success(),
        "protoc exit status: {:?} while printing: {:?}",
        exit_status,
        message
    );

    decoded
}

pub fn test_text_format_str_descriptor(text: &str, message_descriptor: &MessageDescriptor) {
    let message = parse_using_rust_protobuf(text, message_descriptor)
        .expect(format!("parse_using_rust_protobuf: {:?}", &text).as_str());
    let expected = parse_using_protoc(text, message_descriptor);

    assert!(
        message_descriptor.eq(&*expected, &*message),
        "{:?} != {:?}",
        expected,
        message
    );

    // print using protoc and parse using rust-protobuf
    let printed_using_protoc = print_using_protoc(&*message);
    let pp = parse_using_rust_protobuf(&printed_using_protoc, message_descriptor)
        .expect(format!("parse_using_rust_protobuf: {:?}", &printed_using_protoc).as_str());

    assert!(
        message_descriptor.eq(&*expected, &*pp),
        "{:?} != {:?}",
        expected,
        message
    );
}

pub fn test_text_format_str_message(expected: &str, message: &dyn Message) {
    assert_eq!(expected, &*print_to_string(message));

    test_text_format_str_descriptor(expected, message.descriptor());
}

pub fn test_text_format_message(message: &dyn Message) {
    let descriptor = message.descriptor();

    let printed_with_rust_protobuf = print_to_string(message);
    let printed_with_protoc = print_using_protoc(message);

    let from_protoc = parse_using_rust_protobuf(&printed_with_protoc, descriptor)
        .expect(format!("parse_using_rust_protobuf: {:?}", &printed_with_protoc).as_str());
    let from_protobuf = parse_using_protoc(&printed_with_rust_protobuf, descriptor);

    assert!(
        message.reflect_eq(&*from_protoc),
        "{:?} != {:?}",
        message,
        from_protoc
    );
    assert!(
        message.reflect_eq(&*from_protobuf),
        "{:?} != {:?}",
        message,
        from_protobuf
    );
}

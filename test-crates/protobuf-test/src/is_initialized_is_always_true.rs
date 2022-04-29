use std::fs;

use protobuf::reflect::FileDescriptor;
use protobuf_parse::Parser;
use regex::Regex;

fn file_descriptor() -> FileDescriptor {
    let typechecked = Parser::new()
        .pure()
        .input("src/is_initialized_is_always_true.proto")
        .include("src")
        .parse_and_typecheck()
        .unwrap();
    let file_descriptors = FileDescriptor::new_dynamic_fds(typechecked.file_descriptors).unwrap();
    file_descriptors
        .into_iter()
        .find(|fd| fd.proto().name() == "is_initialized_is_always_true.proto")
        .unwrap()
}

fn parse_expected() -> Vec<(String, bool)> {
    let content = fs::read_to_string("src/is_initialized_is_always_true.proto").unwrap();
    let mut lines = content.lines();

    let mut r = Vec::new();
    while let Some(line) = lines.next() {
        let expected = if line.starts_with("// YES") {
            true
        } else if line.starts_with("// NO") {
            false
        } else {
            assert!(!line.starts_with("message"));
            continue;
        };
        let line = lines.next().unwrap();
        let message = Regex::new("message (.*) \\{")
            .unwrap()
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap();
        r.push((message.as_str().to_owned(), expected));
    }

    // Sanity check.
    assert!(r.len() > 2);

    r
}

#[test]
fn universal() {
    let file_descriptor = file_descriptor();
    let expected = parse_expected();
    for (name, expected) in expected {
        println!("checking message {name}, expected {expected}");
        let message = file_descriptor
            .message_by_package_relative_name(&name)
            .unwrap();
        assert_eq!(expected, message.is_initialized_is_always_true());
    }
}

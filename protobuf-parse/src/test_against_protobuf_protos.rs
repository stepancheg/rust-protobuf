#![cfg(test)]

use std::fs;
use std::io::Read;
use std::path::Path;

use anyhow::Context;

use crate::model;

fn parse_recursively(path: &Path) {
    assert!(path.exists());

    let file_name = path
        .file_name()
        .expect("file_name")
        .to_str()
        .expect("to_str");
    if path.is_dir() {
        for entry in fs::read_dir(path).expect("read_dir") {
            parse_recursively(&entry.expect("entry").path());
        }
    } else if file_name.ends_with(".proto") {
        println!("checking {}", path.display());
        let mut content = String::new();
        fs::File::open(path)
            .expect("open")
            .read_to_string(&mut content)
            .expect("read");
        model::FileDescriptor::parse(&content)
            .with_context(|| format!("testing `{}`", path.display()))
            .expect("parse");
    }
}

#[test]
fn test() {
    let path = &Path::new("../google-protobuf-all-protos/protobuf");
    parse_recursively(Path::new(path));
}

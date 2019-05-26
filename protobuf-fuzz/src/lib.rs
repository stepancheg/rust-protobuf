extern crate protobuf;

use protobuf::Message;
use std::io::BufReader;
use std::str;

pub mod all_types_pb;

fn test_bytes<M: Message>(bytes: &[u8]) {
    drop(protobuf::parse_from_bytes::<M>(bytes));
}

fn test_read<M: Message>(bytes: &[u8]) {
    let mut reader = BufReader::new(bytes);
    drop(protobuf::parse_from_reader::<M>(&mut reader));
}

fn test_parse_json<M: Message>(bytes: &[u8]) {
    let text = match str::from_utf8(bytes) {
        Ok(text) => text,
        Err(_) => return,
    };
    drop(protobuf::json::parse_from_str::<M>(text));
}

fn test_parse_text_format<M: Message>(bytes: &[u8]) {
    let text = match str::from_utf8(bytes) {
        Ok(text) => text,
        Err(_) => return,
    };
    drop(protobuf::text_format::parse_from_str::<M>(text));
}

fn test_write_to_bytes<M: Message>(bytes: &[u8]) {
    let message = match protobuf::parse_from_bytes::<M>(bytes) {
        Ok(message) => message,
        Err(_) => return,
    };
    drop(message.write_to_bytes());
}

pub fn fuzz_target_empty_message(bytes: &[u8]) {
    test_bytes::<all_types_pb::EmptyMessage>(bytes);
}

pub fn fuzz_target_empty_message_read(bytes: &[u8]) {
    test_read::<all_types_pb::EmptyMessage>(bytes);
}

pub fn fuzz_target_singular(bytes: &[u8]) {
    test_bytes::<all_types_pb::TestTypesSingular>(bytes);
}

pub fn fuzz_target_singular_read(bytes: &[u8]) {
    test_read::<all_types_pb::TestTypesSingular>(bytes);
}

pub fn fuzz_target_repeated(bytes: &[u8]) {
    test_bytes::<all_types_pb::TestTypesRepeated>(bytes);
}

pub fn fuzz_target_repeated_read(bytes: &[u8]) {
    test_read::<all_types_pb::TestTypesRepeated>(bytes);
}

pub fn fuzz_target_map(bytes: &[u8]) {
    test_bytes::<all_types_pb::TestTypesMap>(bytes);
}

pub fn fuzz_target_map_read(bytes: &[u8]) {
    test_read::<all_types_pb::TestTypesMap>(bytes);
}

fn test_message<M: Message>(bytes: &[u8]) {
    if bytes.len() < 1 {
        return;
    }
    match bytes[0] {
        0 => test_bytes::<M>(&bytes[1..]),
        1 => test_read::<M>(&bytes[1..]),
        2 => test_parse_json::<M>(&bytes[1..]),
        3 => test_parse_text_format::<M>(&bytes[1..]),
        4 => test_write_to_bytes::<M>(&bytes[1..]),
        _ => {}
    }
}

pub fn fuzz_target_all(bytes: &[u8]) {
    if bytes.len() < 1 {
        return;
    }
    match bytes[0] {
        0 => test_message::<all_types_pb::TestTypesSingular>(&bytes[1..]),
        1 => test_message::<all_types_pb::TestTypesRepeated>(&bytes[1..]),
        2 => test_message::<all_types_pb::TestTypesMap>(&bytes[1..]),
        _ => {}
    }
}

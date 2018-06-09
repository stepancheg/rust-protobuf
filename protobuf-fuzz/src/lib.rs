extern crate protobuf;

use std::io::BufReader;

pub mod all_types_pb;

pub fn fuzz_target_empty_message(bytes: &[u8]) {
    drop(protobuf::parse_from_bytes::<all_types_pb::EmptyMessage>(bytes));
}

pub fn fuzz_target_empty_message_read(bytes: &[u8]) {
    let mut reader = BufReader::new(bytes);
    drop(protobuf::parse_from_reader::<all_types_pb::EmptyMessage>(&mut reader));
}

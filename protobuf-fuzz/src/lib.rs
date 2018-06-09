extern crate protobuf;

use std::io::BufReader;
use protobuf::Message;

pub mod all_types_pb;


fn test_bytes<M : Message>(bytes: &[u8]) {
    drop(protobuf::parse_from_bytes::<M>(bytes));
}

fn test_read<M : Message>(bytes: &[u8]) {
    let mut reader = BufReader::new(bytes);
    drop(protobuf::parse_from_reader::<M>(&mut reader));
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

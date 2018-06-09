extern crate protobuf;

pub mod all_types_pb;

pub fn fuzz_target_empty_message(bytes: &[u8]) {
    drop(protobuf::parse_from_bytes::<all_types_pb::EmptyMessage>(bytes));
}

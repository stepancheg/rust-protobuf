// Disable on Windows because it's hard to compile interop tools on travis
#![cfg(not(windows))]

use protobuf::Message;
use protobuf_test_common::interop_json_decode;
use protobuf_test_common::interop_json_encode;

use crate::interop::interop_pb::InteropMessageList;

mod bin;
mod interop_pb;
mod json;

fn interop_json_encode_typed(m: &InteropMessageList) -> String {
    let m_bytes = m.write_to_bytes().expect("write_to_bytes");

    interop_json_encode(&m_bytes)
}

fn interop_json_decode_typed(m: &str) -> InteropMessageList {
    let bytes = interop_json_decode(m);

    InteropMessageList::parse_from_bytes(&bytes).expect("parse_from_bytes")
}

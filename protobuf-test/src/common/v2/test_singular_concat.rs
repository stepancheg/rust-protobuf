//! Test when two messages are concatenated, singular fields are taken from the last one.

use super::test_singular_concat_pb::*;

use protobuf_test_common::*;

#[test]
fn test_concat_bytes() {
    let mut m = TestSingularConcat::new();
    m.set_b(b"\xdd\xee".to_vec());

    test_deserialize("12 03 aa bb cc 12 02 dd ee", &m);
}

#[test]
fn test_concat_string() {
    let mut m = TestSingularConcat::new();
    m.set_s("\x61\x62".to_string());

    test_deserialize("0a 03 21 22 23 0a 02 61 62", &m);
}

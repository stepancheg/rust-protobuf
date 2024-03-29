use bytes::Bytes;
use protobuf::Chars;
use protobuf_test_common::*;

use super::test_tokio_bytes_pb::*;

#[test]
fn test() {
    let mut m = TestTokioBytes::new();
    m.set_b1(Bytes::from("aabb"));
    m.set_s1(Chars::from("ccdd"));

    let mut br = Vec::new();
    br.push(Bytes::from("bb1"));
    br.push(Bytes::from("bb2"));
    m.set_br(br);

    let mut sr = Vec::new();
    sr.push(Chars::from("ss1"));
    sr.push(Chars::from("ss2"));
    m.set_sr(sr);

    test_serialize_deserialize_no_hex_with_dynamic(&m);
}

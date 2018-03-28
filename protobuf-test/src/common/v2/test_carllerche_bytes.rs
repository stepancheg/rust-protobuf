use bytes::Bytes;
use protobuf::Chars;

use super::test_carllerche_bytes_pb::*;

use protobuf_test_common::*;

#[test]
fn test() {
    let mut m = TestCarllercheBytes::new();
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

    test_serialize_deserialize_no_hex(&m);
}

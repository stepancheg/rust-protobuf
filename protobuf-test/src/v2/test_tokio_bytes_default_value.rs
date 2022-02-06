use protobuf::*;

use super::test_tokio_bytes_default_value_pb::*;

#[test]
fn test_default_values() {
    assert_eq!(
        "sss",
        TestTokioBytesDefaultValues::default_instance().get_s()
    );
    assert_eq!(
        b"bbb",
        TestTokioBytesDefaultValues::default_instance().get_b()
    );
    assert_eq!(&""[..], &**TestTokioBytesDefaultValues::new().mut_s());
    assert_eq!(&b""[..], &**TestTokioBytesDefaultValues::new().mut_b());
}

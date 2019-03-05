use protobuf::*;

use super::test_carllerche_bytes_default_value_pb::*;

#[test]
fn test_default_values() {
    assert_eq!("sss", TestCarllercheBytesDefaultValues::default_instance().get_s());
    assert_eq!(b"bbb", TestCarllercheBytesDefaultValues::default_instance().get_b());
    assert_eq!(&""[..], &**TestCarllercheBytesDefaultValues::new().mut_s());
    assert_eq!(&b""[..], &**TestCarllercheBytesDefaultValues::new().mut_b());
}

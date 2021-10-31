use protobuf::Message;

use super::test_reflect_pb::M;

#[test]
fn test_get_sub_message_via_reflection() {
    let mut m = M::new();
    m.mut_sub_m().set_n(42);
    assert!(m.has_sub_m());

    let descriptor = m.descriptor().field_by_name("sub_m");
    assert_eq!("sub_m", descriptor.name());

    let sub_m = descriptor.get_message(&m);
    assert_eq!("test_reflect.SubM", sub_m.descriptor().full_name());
    assert_eq!(42, sub_m.descriptor().field_by_name("n").get_i32(sub_m));
}

#[test]
fn test_json_name() {
    let descriptor = M::descriptor_static().get_field_by_name("sub_m").unwrap();
    // Note that we intentionally do not call `descriptor.json_name()`, since
    // that will compute a JSON name if one is not already present in the proto.
    // We want to verify that the compiler has encoded the correct JSON name in
    // the descriptor itself.
    assert_eq!("subM", descriptor.proto().get_json_name());
}

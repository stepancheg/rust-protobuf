use protobuf::reflect::FileDescriptor;
use protobuf::reflect::ReflectValueRef;

use super::test_dynamic_pb;

fn do_test(file_descriptor: &FileDescriptor) {
    let m = file_descriptor
        .get_message_by_package_relative_name("ForDynamicTest")
        .unwrap();
    let f = m.get_field_by_name("ff").unwrap();

    let mut m = m.new_instance();
    let m = &mut *m;
    assert_eq!(None, f.get_singular(m));

    f.set_singular_field(m, 10u32.into());
    assert_eq!(Some(ReflectValueRef::from(10u32)), f.get_singular(m));
}

#[test]
fn test_generated() {
    do_test(&test_dynamic_pb::file_descriptor());
}

#[test]
fn test_dynamic() {
    do_test(&FileDescriptor::new_dynamic(
        test_dynamic_pb::file_descriptor().get_proto().clone(),
        Vec::new(),
    ));
}

use protobuf::reflect::FileDescriptor;
use protobuf::reflect::ReflectValueBox;
use protobuf::reflect::ReflectValueRef;

use super::test_dynamic_singular_get_set_pb;

fn dynamic_file_descriptor() -> FileDescriptor {
    FileDescriptor::new_dynamic(
        test_dynamic_singular_get_set_pb::file_descriptor()
            .proto()
            .clone(),
        Vec::new(),
    )
}

fn do_test_get_set(file_descriptor: &FileDescriptor) {
    let m = file_descriptor
        .message_by_package_relative_name("ForDynamicTest")
        .unwrap();
    let f = m.get_field_by_name("ff").unwrap();

    let mut m = m.new_instance();
    let m = &mut *m;
    assert_eq!(None, f.get_singular(m));

    f.set_singular_field(m, 10u32.into());
    assert_eq!(Some(ReflectValueRef::from(10u32)), f.get_singular(m));
}

#[test]
fn generated_get_set() {
    do_test_get_set(&test_dynamic_singular_get_set_pb::file_descriptor());
}

#[test]
fn dynamic_get_set() {
    do_test_get_set(&dynamic_file_descriptor());
}

fn do_test_set_panic_on_wrong_field_type(file_descriptor: &FileDescriptor) {
    let m = file_descriptor
        .message_by_package_relative_name("ForDynamicTest")
        .unwrap();
    let f = m.get_field_by_name("ff").unwrap();
    let mut m = m.new_instance();
    let m = &mut *m;
    f.set_singular_field(m, ReflectValueBox::from(10i64));
}

#[test]
#[should_panic]
fn generated_set_panic_on_wrong_field_type() {
    do_test_set_panic_on_wrong_field_type(&test_dynamic_singular_get_set_pb::file_descriptor());
}

#[test]
#[should_panic]
fn dynamic_set_panic_on_wrong_field_type() {
    do_test_set_panic_on_wrong_field_type(&dynamic_file_descriptor());
}

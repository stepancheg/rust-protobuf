use protobuf::reflect::FileDescriptor;
use protobuf::reflect::ReflectValueBox;

use super::test_dynamic_repeated_get_set_pb;

fn dynamic_file_descriptor() -> FileDescriptor {
    FileDescriptor::new_dynamic(
        test_dynamic_repeated_get_set_pb::file_descriptor()
            .proto()
            .clone(),
        Vec::new(),
    )
    .unwrap()
}

fn do_test_repeated(file_descriptor: &FileDescriptor) {
    let m = file_descriptor
        .message_by_package_relative_name("ForDynamicRepeatedTest")
        .unwrap();
    let f = m.field_by_name("ii").unwrap();
    let mut m = m.new_instance();
    assert!(f.get_repeated(&*m).is_empty());
    assert!(f.mut_repeated(&mut *m).is_empty());
    f.mut_repeated(&mut *m).push(ReflectValueBox::U32(19));
    f.mut_repeated(&mut *m).push(ReflectValueBox::U32(17));
    assert_eq!(2, f.get_repeated(&*m).len());
    assert_eq!(ReflectValueBox::U32(17), f.get_repeated(&*m).get(1));
    assert_eq!(
        &[ReflectValueBox::U32(19), ReflectValueBox::U32(17)][..],
        &f.get_repeated(&*m)
    );
}

#[test]
fn generated_repeated() {
    do_test_repeated(&dynamic_file_descriptor());
}

#[test]
fn dynamic_repeated() {
    do_test_repeated(&test_dynamic_repeated_get_set_pb::file_descriptor());
}

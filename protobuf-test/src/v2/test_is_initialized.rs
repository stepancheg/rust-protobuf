use protobuf::reflect::FileDescriptor;
use protobuf::reflect::ReflectValueBox;
use protobuf::MessageDyn;

use crate::v2::test_is_initialized_pb;
use crate::v2::test_is_initialized_pb::TestIsInitialized;

fn file_descriptor_dynamic() -> FileDescriptor {
    FileDescriptor::new_dynamic(
        test_is_initialized_pb::file_descriptor_proto().clone(),
        Vec::new(),
    )
}

fn test_is_initialized(message: &mut dyn MessageDyn) {
    assert!(!message.is_initialized_dyn());
    let field = message.descriptor_dyn().field_by_name("a").unwrap();
    field.set_singular_field(message, ReflectValueBox::I32(10));
    assert!(message.is_initialized_dyn());
}

#[test]
fn is_initialized_generated() {
    let mut message = TestIsInitialized::new();
    test_is_initialized(&mut message);
}

#[test]
fn is_initialized_dynamic() {
    let file_descriptor = file_descriptor_dynamic();
    let message_descriptor = file_descriptor
        .message_by_package_relative_name("TestIsInitialized")
        .unwrap();
    let mut message = message_descriptor.new_instance();
    test_is_initialized(&mut *message);
}

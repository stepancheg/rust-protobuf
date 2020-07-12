#[test]
fn test_no_package() {
    let file_descriptor = super::test_reflect_no_package_pb::file_descriptor_proto();
    assert!(!file_descriptor.has_package());
}

#[test]
fn test_no_package() {
    let file_descriptor = super::test_reflect_no_package_pb::file_descriptor();
    assert!(!file_descriptor.proto().has_package());
}

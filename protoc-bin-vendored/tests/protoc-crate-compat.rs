#[test]
fn protoc_crate_compat() {
    assert!(
        protoc::Protoc::from_path(protoc_bin_vendored::protoc_bin_path().unwrap())
            .version()
            .unwrap()
            .is_3()
    );
}

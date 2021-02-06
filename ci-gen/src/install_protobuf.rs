use crate::ghwf::Step;

pub fn install_protobuf() -> Vec<Step> {
    vec![
        Step::run("Install protobuf", "ci/install-protobuf.sh").env("PROTOBUF_VERSION", "3.6.1"),
        Step::run("Protoc check", "protoc --version"),
    ]
}

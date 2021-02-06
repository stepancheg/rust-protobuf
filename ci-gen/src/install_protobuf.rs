use crate::ghwf::Step;

pub fn install_protobuf() -> Vec<Step> {
    vec![
        Step::run("Install protobuf", "ci/install-protobuf.sh"),
        Step::run("Protoc check", "protoc --version"),
    ]
}

use crate::actions::cache;
use crate::ghwf::Step;
use crate::Os;
use crate::MACOS;

pub(crate) fn install_protobuf(os: Os) -> Vec<Step> {
    let protobuf_version = "3.20.0";

    let mut steps = Vec::new();
    steps.push(cache(
        "Cache protobuf",
        &format!("pb-{}-{}", os.name, protobuf_version),
        "~/pb",
    ));

    if os == MACOS {
        steps.push(Step::run("Install pkg-config", "brew install pkg-config"));
    }

    steps.push(
        Step::run("Install protobuf", "ci/install-protobuf.sh")
            .env("PROTOBUF_VERSION", protobuf_version),
    );
    steps.push(Step::run("Protoc check", "protoc --version"));
    steps
}

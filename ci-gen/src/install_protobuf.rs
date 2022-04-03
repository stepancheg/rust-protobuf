use crate::actions::cache;
use crate::ghwf::Step;
use crate::Os;
use crate::MACOS;
use crate::WINDOWS;

pub(crate) fn install_protobuf(os: Os) -> Vec<Step> {
    let mut steps = Vec::new();
    steps.push(cache(
        "Cache protobuf",
        &format!("pb-{}{}", os.name, if os == WINDOWS { "-1" } else { "" }),
        "~/pb",
    ));

    if os == MACOS {
        steps.push(Step::run("Install pkg-config", "brew install pkg-config"));
    }

    steps.push(
        Step::run("Install protobuf", "ci/install-protobuf.sh").env("PROTOBUF_VERSION", "3.20.0"),
    );
    steps.push(Step::run("Protoc check", "protoc --version"));
    steps
}

use crate::checkout_sources;
use crate::rust_install_toolchain;
use crate::Job;
use crate::RustToolchain;
use crate::Step;

pub(crate) fn cargo_sync_readme_job() -> Job {
    Job {
        id: "cargo-sync-readme".to_owned(),
        name: "Check sync-readme".to_owned(),
        steps: vec![
            checkout_sources(),
            rust_install_toolchain(RustToolchain::Stable),
            Step::run(
                "install cargo sync-readme",
                "cargo install cargo-sync-readme",
            ),
            Step::run(
                "sync-readme",
                "cd protoc-bin-vendored && cargo sync-readme --check",
            ),
        ],
        ..Job::default()
    }
}

use std::fs;
use std::io;

use crate::checkout_sources;
use crate::rust_install_toolchain;
use crate::Job;
use crate::RustToolchain;
use crate::Step;

fn find_sync_readme_crates() -> Vec<String> {
    let mut crates = Vec::new();
    for cr in fs::read_dir(".").unwrap() {
        let cr = cr.unwrap();
        if !fs::metadata(cr.path()).unwrap().is_dir() {
            continue;
        }
        let readme = cr.path().join("README.md");
        let readme = match fs::read_to_string(&readme) {
            Ok(readme) => readme,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                continue;
            }
            Err(e) => panic!("failed to read {}: {}", readme.display(), e),
        };
        if readme.contains("<!-- cargo-sync-readme") {
            crates.push(cr.file_name().to_str().unwrap().to_owned());
        }
    }
    crates.sort();
    crates
}

pub(crate) fn cargo_sync_readme_job() -> Job {
    let mut steps = vec![
        checkout_sources(),
        rust_install_toolchain(RustToolchain::Stable),
        Step::run(
            "install cargo sync-readme",
            "cargo install cargo-sync-readme",
        ),
    ];
    for cr in find_sync_readme_crates() {
        steps.push(Step::run(
            &format!("sync-readme {}", cr),
            &format!("cd {} && cargo sync-readme --check", cr),
        ));
    }
    Job {
        id: "cargo-sync-readme".to_owned(),
        name: "Check sync-readme".to_owned(),
        steps,
        ..Job::default()
    }
}

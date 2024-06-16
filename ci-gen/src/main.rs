use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use crate::actions::cargo_check;
use crate::actions::cargo_doc;
use crate::actions::cargo_miri_setup;
use crate::actions::cargo_miri_test;
use crate::actions::cargo_test;
use crate::actions::checkout_sources;
use crate::actions::checkout_sources_depth;
use crate::actions::rust_install_toolchain;
use crate::actions::rust_install_toolchain_with_components;
use crate::actions::RustToolchain;
use crate::cargo_sync_readme::cargo_sync_readme_job;
use crate::ghwf::Env;
use crate::ghwf::Job;
use crate::ghwf::Step;
use crate::yaml::Yaml;
use crate::yaml::YamlWriter;

mod actions;
mod cargo_sync_readme;
mod ghwf;
mod install_protobuf;
mod yaml;

#[derive(PartialEq, Eq, Copy, Clone)]
struct Os {
    name: &'static str,
    ghwf: Env,
}

const LINUX: Os = Os {
    name: "linux",
    ghwf: Env::UbuntuLatest,
};
const MACOS: Os = Os {
    name: "macos",
    ghwf: Env::MacosLatest,
};
const WINDOWS: Os = Os {
    name: "windows",
    ghwf: Env::WindowsLatest,
};

#[derive(Eq, PartialEq)]
enum Features {
    Default,
    Specific(&'static [&'static str]),
    All,
}

impl Features {
    fn flag(&self) -> String {
        match self {
            Features::Default => format!(""),
            Features::Specific(f) => format!("--features={}", f.join(",")),
            Features::All => format!("--all-features"),
        }
    }

    fn flag_suffix(&self) -> String {
        let flag = self.flag();
        if flag.is_empty() {
            "".to_owned()
        } else {
            format!(" {}", flag)
        }
    }

    fn id(&self) -> String {
        match self {
            Features::Default => format!("default-features"),
            Features::All => format!("all-features"),
            Features::Specific(s) => s.join("-"),
        }
    }

    fn name(&self) -> String {
        match self {
            Features::Default => format!("default features"),
            Features::All => format!("all features"),
            Features::Specific(s) => s.join(","),
        }
    }
}

fn self_check_job() -> Job {
    Job {
        id: format!("self-check"),
        name: format!("CI self-check"),
        runs_on: LINUX.ghwf,
        steps: vec![
            checkout_sources(),
            rust_install_toolchain(RustToolchain::Stable),
            Step::run("The check", "cargo run -p ci-gen -- --check"),
        ],
        ..Default::default()
    }
}

fn job(channel: RustToolchain, os: Os, features: Features) -> Job {
    let mut steps = vec![];
    steps.push(checkout_sources());
    steps.push(rust_install_toolchain(channel));

    steps.extend(install_protobuf::install_protobuf(os));

    if os != WINDOWS {
        steps.push(Step::run(
            "Compile interop",
            "test-crates/interop/cxx/compile.sh",
        ));
    }

    // * Nothing works on Windows
    // * Only regenerate with default features to avoid rebuilding twice
    if os != WINDOWS && features == Features::Default {
        steps.push(Step::run("Regenerate", "protobuf/regenerate.sh"));
    }

    match features {
        Features::Specific(..) => {
            steps.push(cargo_test(
                "protobuf-codegen-protoc-test",
                &format!(
                    "--manifest-path=test-crates/protobuf-codegen-protoc-test/Cargo.toml{}",
                    features.flag_suffix()
                ),
            ));
            steps.push(cargo_test(
                "protobuf-codegen-pure-test",
                &format!(
                    "--manifest-path=test-crates/protobuf-codegen-pure-test/Cargo.toml{}",
                    features.flag_suffix()
                ),
            ));
        }
        _ => {
            steps.push(Step::run(
                "Test all",
                &format!("cargo test --all --all-targets{}", features.flag_suffix()),
            ));
            if os == LINUX {
                // https://github.com/rust-lang/cargo/issues/6669
                steps.push(Step::run(
                    "Test all",
                    &format!("cargo test{}", features.flag_suffix()),
                ));

                steps.push(cargo_doc("cargo doc", &features.flag()));
            }
        }
    }

    let mut env = vec![("RUST_BACKTRACE".to_owned(), "1".to_owned())];
    if os == WINDOWS {
        env.push(("VCPKGRS_DYNAMIC".to_owned(), "1".to_owned()));
    };
    let id = format!("{}-{}-{}", os.name, channel, features.id());
    let name = format!("{} {} ({})", os.name, channel, features.name());
    Job {
        id,
        name,
        runs_on: os.ghwf.to_owned(),
        steps,
        env,
        ..Default::default()
    }
}

fn miri_test_job() -> Job {
    let mut steps = Vec::new();

    steps.push(checkout_sources());
    steps.push(rust_install_toolchain_with_components(
        RustToolchain::Nightly,
        &["miri"],
    ));
    steps.push(cargo_miri_setup("cargo-miri-setup"));
    steps.push(cargo_miri_test(
        "cargo-miri-test",
        "-p protobuf --lib --all-features",
    ));

    let id = "miri-test".to_owned();
    let name = "Miri test".to_owned();
    let env = vec![
        ("RUST_BACKTRACE".to_owned(), "1".to_owned()),
        ("RUST_TEST_THREADS".to_owned(), "1".to_owned()),
        // ("MIRIFLAGS".to_owned(), "-Zmiri-tag-raw-pointers".to_owned()),
    ];
    Job {
        id,
        name,
        runs_on: LINUX.ghwf.to_owned(),
        steps,
        env,
        timeout_minutes: Some(5),
        ..Job::default()
    }
}

// https://github.com/megalinter/megalinter
fn super_linter_job() -> Job {
    let mut steps = Vec::new();
    steps.push(checkout_sources_depth(Some(0)));
    steps.push(
        Step::uses("mega-linter", "megalinter/megalinter@v5")
            .env("VALIDATE_ALL_CODEBASE", "false")
            .env("DEFAULT_BRANCH", "master")
            .env("GITHUB_TOKEN", "${{ secrets.GITHUB_TOKEN }}")
            // Too many false positives
            .env("VALIDATE_JSCPD", "false")
            // Too many dull reports like how we should pluralise variable names
            .env("VALIDATE_PROTOBUF", "false")
            // Clippy reports too many false positives
            .env("VALIDATE_RUST_CLIPPY", "false")
            // We don't care about previous edition
            .env("VALIDATE_RUST_2015", "false")
            // Finds copy-paste in LICENSE files.
            .env("VALIDATE_COPYPASTE", "false")
            // Complains about protobuf" in yml files.
            .env("VALIDATE_SPELL", "false"),
    );
    Job {
        id: "mega-linter".to_owned(),
        name: "mega-linter".to_owned(),
        runs_on: LINUX.ghwf,
        steps,
        ..Default::default()
    }
}

fn rustfmt_job() -> Job {
    let os = LINUX;
    let mut steps = Vec::new();
    steps.push(checkout_sources());
    // force generate code
    steps.extend(install_protobuf::install_protobuf(os));
    steps.push(cargo_check("cargo check", ""));
    steps.push(Step::run("cargo fmt check", "cargo fmt -- --check"));
    Job {
        id: "rustfmt-check".to_owned(),
        name: "rustfmt check".to_owned(),
        runs_on: os.ghwf,
        steps,
        ..Default::default()
    }
}

fn jobs() -> Yaml {
    let mut r = Vec::new();

    r.push(job(RustToolchain::Stable, LINUX, Features::Default));
    r.push(job(RustToolchain::Beta, LINUX, Features::Default));
    r.push(job(
        RustToolchain::Stable,
        LINUX,
        Features::Specific(&["with-bytes"]),
    ));
    r.push(job(RustToolchain::Nightly, LINUX, Features::All));

    r.push(job(RustToolchain::Stable, WINDOWS, Features::Default));

    r.push(miri_test_job());

    r.push(super_linter_job());

    r.push(rustfmt_job());

    r.push(cargo_sync_readme_job());

    r.push(self_check_job());

    // TODO: enable macos
    //r.push(job(RustToolchain::Stable, MACOS, Features::Default));

    Yaml::map(r.into_iter().map(Job::into))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(|a| a.as_str()).collect();

    let yaml = Yaml::map(vec![
        ("on", Yaml::list(vec!["push", "pull_request"])),
        ("name", Yaml::string("CI")),
        ("jobs", jobs()),
    ]);

    let mut writer = YamlWriter::default();
    writer.write_line(&format!(
        "# @generated by {}, do not edit",
        env!("CARGO_PKG_NAME")
    ));
    writer.write_line("");
    writer.write_yaml(&yaml);
    let ci_yml = writer.buffer;

    let ci_yml_path = ".github/workflows/ci.yml";

    match args.as_slice() {
        &[] => {
            File::create(ci_yml_path)
                .unwrap()
                .write_all(ci_yml.as_bytes())
                .unwrap();
            eprintln!("written {}", ci_yml_path);
        }
        &["--check"] => {
            let mut actual = String::new();
            File::open(ci_yml_path)
                .unwrap()
                .read_to_string(&mut actual)
                .unwrap();
            assert!(ci_yml == actual);
            eprintln!("The file is correct")
        }
        args => panic!("unknown args: {:?}", args),
    }
}

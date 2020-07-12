use crate::actions::cargo_doc;
use crate::actions::cargo_test;
use crate::actions::checkout_sources;
use crate::actions::rust_install_toolchain;
use crate::actions::RustToolchain;
use crate::actions::cache;
use crate::ghwf::Env;
use crate::ghwf::Job;
use crate::ghwf::Step;
use crate::yaml::Yaml;
use crate::yaml::YamlWriter;
use std::fs::File;
use std::io::Write;

mod actions;
mod ghwf;
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

fn job(channel: RustToolchain, os: Os, features: Features) -> Job {
    let mut steps = vec![];
    steps.push(checkout_sources());
    steps.push(rust_install_toolchain(channel));

    steps.push(cache(
        "Cache protobuf",
        &format!("pb-{}{}", os.name, if os == WINDOWS { "-1" } else { "" }),
        "~/pb",
    ));

    if os == MACOS {
        steps.push(Step::run("Install pkg-config", "brew install pkg-config"));
    }

    steps.push(Step::run("Install protobuf", "ci/install-protobuf.sh"));
    steps.push(Step::run("Protoc check", "protoc --version"));

    if os != WINDOWS {
        steps.push(Step::run("Compile interop", "interop/cxx/compile.sh"));
    }

    // * Nothing works on Windows
    // * Only regenerate with default features to avoid rebuilding twice
    if os != WINDOWS && features == Features::Default {
        steps.push(Step::run("Regenerate", "protobuf/regenerate.sh"));
    }

    match features {
        Features::Specific(..) => {
            steps.push(cargo_test(
                "protobuf-test",
                &format!(
                    "--manifest-path=protobuf-test/Cargo.toml{}",
                    features.flag_suffix()
                ),
            ));
            steps.push(cargo_test(
                "protobuf-codegen-pure-test",
                &format!(
                    "--manifest-path=protobuf-codegen-pure-test/Cargo.toml{}",
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
                steps.push(cargo_doc("cargo doc", &features.flag()));
            }
        }
    }

    if os != WINDOWS {
        steps.push(Step::run("test protoc", "protoc/test.sh"));
    }

    let mut env = vec![
        ("PROTOBUF_VERSION".to_owned(), "3.6.1".to_owned()),
        ("RUST_BACKTRACE".to_owned(), "1".to_owned()),
    ];
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

fn jobs() -> Yaml {
    let mut r = Vec::new();

    r.push(job(RustToolchain::Stable, LINUX, Features::Default));
    r.push(job(RustToolchain::Beta, LINUX, Features::Default));
    r.push(job(
        RustToolchain::Stable,
        LINUX,
        Features::Specific(&["with-serde"]),
    ));
    r.push(job(
        RustToolchain::Stable,
        LINUX,
        Features::Specific(&["with-bytes"]),
    ));
    r.push(job(RustToolchain::Nightly, LINUX, Features::All));

    r.push(job(RustToolchain::Stable, WINDOWS, Features::Default));

    // TODO: enable macos
    //r.push(job(RustToolchain::Stable, MACOS, Features::Default));

    Yaml::map(r.into_iter().map(Job::into))
}

fn main() {
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
    File::create(".github/workflows/ci.yml")
        .unwrap()
        .write_all(writer.buffer.as_bytes())
        .unwrap();
}

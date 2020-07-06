use crate::ghwf::Step;
use crate::yaml::Yaml;
use std::fmt;

pub fn checkout_sources() -> Step {
    Step::uses("Checkout sources", "actions/checkout@v2")
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum RustToolchain {
    Stable,
    Beta,
    Nightly,
}

impl fmt::Display for RustToolchain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustToolchain::Stable => write!(f, "stable"),
            RustToolchain::Beta => write!(f, "beta"),
            RustToolchain::Nightly => write!(f, "nightly"),
        }
    }
}

pub fn rust_install_toolchain(channel: RustToolchain) -> Step {
    Step::uses_with(
        "Install toolchain",
        "actions-rs/toolchain@v1",
        Yaml::map(vec![
            ("profile", "minimal"),
            ("toolchain", &format!("{}", channel)),
            ("override", "true"),
        ]),
    )
}

pub fn cargo(name: &str, command: &str, args: &str) -> Step {
    let mut with = vec![("command", command)];
    if !args.is_empty() {
        with.push(("args", args));
    }
    Step::uses_with(name, "actions-rs/cargo@v1", Yaml::map(with))
}

pub fn cargo_test(name: &str, args: &str) -> Step {
    cargo(name, "test", args)
}

pub fn cargo_build(name: &str, args: &str) -> Step {
    cargo(name, "build", args)
}

pub fn cargo_doc(name: &str, args: &str) -> Step {
    cargo(name, "doc", args)
}

pub fn cache(name: &str, key: &str, path: &str) -> Step {
    Step::uses_env_with(
        name,
        "actions/cache@v2",
        &[("cache-name", "pb")],
        Yaml::map(vec![("key", key), ("path", path)]),
    )
}

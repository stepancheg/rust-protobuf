use std::env;
use std::io::Read;
use std::process;

// % rustc +stable --version
// rustc 1.26.0 (a77568041 2018-05-07)
// % rustc +beta --version
// rustc 1.27.0-beta.1 (03fb2f447 2018-05-09)
// % rustc +nightly --version
// rustc 1.27.0-nightly (acd3871ba 2018-05-10)
fn version_is_nightly(version: &str) -> bool {
    version.contains("nightly")
}

fn export_rustc_cfg() {
    let rustc = env::var("RUSTC").expect("RUSTC unset");

    let mut child = process::Command::new(rustc)
        .args(["--version"])
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("spawn rustc");

    let mut rustc_version = String::new();

    child
        .stdout
        .as_mut()
        .expect("stdout")
        .read_to_string(&mut rustc_version)
        .expect("read_to_string");
    assert!(child.wait().expect("wait").success());

    if version_is_nightly(&rustc_version) {
        println!("cargo:rustc-cfg=rustc_nightly");
    }
}

fn main() {
    export_rustc_cfg();
}

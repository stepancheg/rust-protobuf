//! Emit instructions to cargo.

use std::fs;
use std::path::Path;

fn print_rerun_if_changed<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    println!("cargo:rerun-if-changed={}", path.to_str().expect("to_str"));
}

pub fn print_rerun_if_changed_recursively<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    print_rerun_if_changed(path);
    if path.is_dir() {
        for child in fs::read_dir(path).expect("read_dir") {
            let child = child.expect("child").path();
            print_rerun_if_changed_recursively(child);
        }
    }
}

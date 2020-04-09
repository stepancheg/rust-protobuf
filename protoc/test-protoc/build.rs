extern crate protoc;

use std::ffi::OsString;
use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let exe_suffix = if cfg!(windows) {
        ".exe"
    } else if cfg!(unix) {
        ""
    } else {
        panic!("unknown OS")
    };

    // TODO: https://github.com/rust-lang/cargo/issues/4316

    let gen_rust_binary = format!("../../target/debug/protoc-gen-rust{}", exe_suffix);
    let gen_rust_binary = PathBuf::from(gen_rust_binary).canonicalize()?;

    assert!(gen_rust_binary.is_file(), "{:?}", gen_rust_binary);

    let mut gen_rust_plugin = OsString::from("protoc-gen-rust=");
    gen_rust_plugin.push(gen_rust_binary);

    protoc::ProtocLangOut::new()
        .lang("rust")
        .out_dir("src")
        .plugin(gen_rust_plugin)
        .input("src/data.proto")
        .run()?;

    Ok(())
}

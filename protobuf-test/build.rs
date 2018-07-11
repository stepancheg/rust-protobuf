extern crate glob;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate protoc;
extern crate protoc_rust;

extern crate protobuf_test_common;

use std::io::Write;
use std::fs;

use protobuf_test_common::build::*;


fn protoc_is_v3() -> bool {
    protoc::Protoc::from_env_path()
        .version()
        .expect("version")
        .is_3()
}

fn gen_in_dir(dir: &str, include_dir: &str) {
    let v3 = protoc_is_v3();
    gen_in_dir_impl(dir, include_dir, v3, |GenInDirArgs { out_dir, input, includes, customize }| {
        protoc_rust::run(protoc_rust::Args {
            out_dir, input, includes, customize
        })
    });
}

fn generate_in_common() {
    gen_in_dir("src/common/v2", "src/common/v2");

    if protoc_is_v3() {
        copy_tests_v2_v3("src/common/v2", "src/common/v3");
        gen_in_dir("src/common/v3", "src/common/v3");
    } else {
        let mut mod_rs = fs::File::create("src/common/v3/mod.rs").expect("create");
        writeln!(mod_rs, "// generated").expect("write");
        writeln!(mod_rs, "// no tests because protoc is not v3").expect("write");
        mod_rs.flush().expect("flush");
    }
}

fn generate_in_v2_v3() {

    gen_in_dir("src/v2", "src/v2");

    if protoc_is_v3() {
        gen_in_dir("src/v3", "src/v3");

        gen_in_dir("src/google/protobuf", "src");
    } else {
        info!("generating stubs in src/v3");

        // Because `#[cfg(nonexistent)]` still requires module files to exist
        // https://github.com/rust-lang/rust/pull/36482

        let g1 = glob_simple("src/v3/*.proto");
        let g2 = glob_simple("src/google/protobuf/*.proto");

        for mut f in g1.into_iter().chain(g2.into_iter()) {
            let suffix = ".proto";
            let len = f.len();
            f.truncate(len - suffix.len());

            let mut f = fs::File::create(f).expect("create");
            let content = b"// generated\n// empty file because protobuf 3 is not available\n";
            f.write_all(content).expect("write");
            f.flush().expect("flush");
        }
    }
}

fn generate_pb_rs() {
    generate_in_common();
    generate_in_v2_v3();
}

fn main() {
    env_logger::init();

    clean_old_files();

    generate_pb_rs();

    if protoc::Protoc::from_env_path()
        .version()
        .expect("version")
        .is_3()
    {
        println!("cargo:rustc-cfg=protoc3");
    }
}

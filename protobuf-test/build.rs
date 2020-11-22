extern crate glob;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate protoc;
extern crate protoc_rust;

extern crate protobuf_test_common;

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

use protobuf_test_common::build::*;

fn protoc_is_v3() -> bool {
    protoc::Protoc::from_env_path()
        .version()
        .expect("version")
        .is_3()
}

fn gen_in_dir(dir: &str, include_dir: &str) {
    let v3 = protoc_is_v3();
    gen_in_dir_impl(
        dir,
        v3,
        |GenInDirArgs {
             out_dir,
             input,
             customize,
         }| {
            protoc_rust::Codegen::new()
                .out_dir(out_dir)
                .inputs(input)
                .includes(&["../proto", include_dir])
                .customize(customize)
                .run()
        },
    );
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

fn generate_interop() {
    protoc_rust::Codegen::new()
        .out_dir("src/interop")
        .includes(&["../interop/cxx", "../proto"])
        .inputs(&["../interop/cxx/interop_pb.proto"])
        .run()
        .unwrap()
}

fn generate_include_generated() {
    let dir = format!("{}/include_generated", env::var("OUT_DIR").unwrap());
    if Path::new(&dir).exists() {
        fs::remove_dir_all(&dir).unwrap();
    }
    fs::create_dir(&dir).unwrap();
    protoc_rust::Codegen::new()
        .out_dir(dir)
        .input("src/include_generated/v2.proto")
        .input("src/include_generated/v3.proto")
        .customize(Customize {
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .include("src/include_generated")
        .run()
        .unwrap();
}

fn generate_pb_rs() {
    generate_in_common();
    generate_in_v2_v3();
    generate_interop();
    generate_include_generated();
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

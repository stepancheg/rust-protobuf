extern crate glob;
extern crate env_logger;

extern crate protoc;
extern crate protoc_rust;

use std::io::Write;
use std::fs;
use std::path;


fn glob_simple(pattern: &str) -> Vec<String> {
    glob::glob(pattern).expect("glob")
        .map(|g| {
            g.expect("item").as_path().to_str().expect("utf-8").to_owned()
        })
        .collect()
}


fn clean_old_files() {
    for f in glob_simple("src/*/pb_*.rs") {
        fs::remove_file(f).expect("rm");
    }
    for f in glob_simple("src/*/pb_*.rs") {
        fs::remove_file(f).expect("rm");
    }
}


fn generate_v_from_common() {
    for f in glob_simple("src/common/*.rs") {
        let f = path::PathBuf::from(f);
        let base_name = f.as_path().file_name().expect("file_name").to_str().expect("to_str");
        for v in &["v2", "v3"] {
            let mut child = fs::File::create(&format!("src/{}/{}", v, base_name)).expect("create");
            let content = format!(
                "// generated\n\
                include!(\"../common/{}\");\n",
                base_name);
            child.write_all(&content.as_bytes()).expect("write_all");
            child.flush().expect("flush");
        }
    }
}

fn generate_pb_rs() {

    fn gen_v2_v3(dir: &str) {

        let protos = glob_simple(&format!("{}/*.proto", dir));

        protoc_rust::run(protoc_rust::Args {
            out_dir: dir,
            input: &protos.iter().map(|a| a.as_ref()).collect::<Vec<&str>>(),
            includes: &["../proto", dir],
        }).expect("protoc");
    }

    let p = protoc::Protoc::from_env_path();
    let v3 = p.version().expect("version").is_3();
    gen_v2_v3("src/v2");
    if v3 {
        gen_v2_v3("src/v3");

        let protos = glob_simple("src/google/protobuf/*.proto");
        protoc_rust::run(protoc_rust::Args {
            out_dir: "src/google/protobuf",
            input: &protos.iter().map(|a| a.as_ref()).collect::<Vec<&str>>(),
            includes: &["../proto", "src"],
        }).expect("protoc");
    } else {
        // Because `#[cfg(nonexistent)]` still requires module files to exist
        // https://github.com/rust-lang/rust/pull/36482

        let g1 = glob::glob("src/v3/*.proto").expect("g1");
        let g2 = glob::glob("src/google/protobuf/*.proto").expect("g2");

        for f in g1.chain(g2) {
            let mut f: String = f.expect("f").as_path().to_str().expect("utf-8").to_owned();
            let suffix = ".proto";
            let len = f.len();
            f.truncate(len - suffix.len());

            let mut f = fs::File::create(f).expect("create");
            let content = b"// generated\n// empty file because protobuf 3 is not available\n";
            f.write_all(content).expect("write");
            f.flush().expect("flush");
        }
    }

    if p.version().expect("version").is_3() {
        println!("cargo:rustc-cfg=proto3");
    }
}

fn main() {
    env_logger::init().expect("env_logger");

    clean_old_files();
    generate_v_from_common();
    generate_pb_rs();
}

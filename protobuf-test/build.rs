extern crate glob;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate protoc;
extern crate protoc_rust;

use std::io::Read;
use std::io::Write;
use std::fs;
use std::path;


fn glob_simple(pattern: &str) -> Vec<String> {
    glob::glob(pattern)
        .expect("glob")
        .map(|g| {
            g.expect("item")
                .as_path()
                .to_str()
                .expect("utf-8")
                .to_owned()
        })
        .collect()
}


fn clean_old_files() {
    for f in glob_simple("src/**/*_pb.rs") {
        fs::remove_file(f).expect("rm");
    }
}


fn generate_v_from_common() {
    let v3 = protoc::Protoc::from_env_path()
        .version()
        .expect("version")
        .is_3();

    let mut mod_v2 = fs::File::create("src/common/v2/mod.rs").expect("mod.rs");
    let mut mod_v3 = fs::File::create("src/common/v3/mod.rs").expect("mod.rs");

    write!(mod_v2, "// generated\n").expect("write");
    write!(mod_v3, "// generated\n").expect("write");

    for f in glob_simple("src/common/v2/*.rs") {
        let f = path::PathBuf::from(f);
        let base_name = f.as_path()
            .file_name()
            .expect("file_name")
            .to_str()
            .expect("to_str");

        let without_suffix = &base_name[..base_name.len() - ".rs".len()];

        if without_suffix == "mod" {
            continue;
        }

        if without_suffix.ends_with("_pb") {
            continue;
        }

        let carllerche = without_suffix.contains("carllerche");

        for mod_v in &mut [&mut mod_v2, &mut mod_v3] {
            for suffix in &["", "_pb"] {
                if carllerche {
                    write!(mod_v, "#[cfg(feature = \"bytes\")]").expect("write");
                }
                write!(mod_v, "mod {}{};\n", without_suffix, suffix).expect("write");
            }
        }

        let mut p2f = fs::File::open(&format!("src/common/v2/{}_pb.proto", without_suffix))
            .expect("open v2 .proto");
        let mut proto = String::new();
        p2f.read_to_string(&mut proto).expect("read .proto");
        drop(p2f);

        let mut r2f = fs::File::open(&format!("src/common/v2/{}.rs", without_suffix))
            .expect("open v2 .rs");
        let mut rs = String::new();
        r2f.read_to_string(&mut rs).expect("read .rs");
        drop(r2f);

        let mut p3f = fs::File::create(&format!("src/common/v3/{}_pb.proto", without_suffix))
            .expect("create v3 .proto");
        let mut r3f = fs::File::create(&format!("src/common/v3/{}.rs", without_suffix))
            .expect("create v3 .rs");

        // convert proto2 to proto3
        let proto = proto.replace("optional ", "");
        let proto = proto.replace("required ", "");
        let proto = proto.replace("syntax = \"proto2\";", "syntax = \"proto3\";");
        write!(p3f, "// generated\n").expect("write");
        write!(p3f, "{}", proto).expect("write");
        p3f.flush().expect("flush");

        write!(r3f, "// generated\n").expect("write");
        write!(r3f, "{}", rs).expect("write");
        r3f.flush().expect("flush");

        for &v in &[2, 3] {
            if v == 3 && !v3 {
                continue;
            }

            protoc_rust::run(protoc_rust::Args {
                out_dir: &format!("src/common/v{}", v),
                includes: &[&format!("src/common/v{}", v), "../proto"],
                input: &[&format!("src/common/v{}/{}_pb.proto", v, without_suffix)],
            }).expect("protoc");
        }
    }

    mod_v2.flush().expect("flush");
    mod_v3.flush().expect("flush");
}

fn generate_pb_rs() {

    fn gen_v2_v3(dir: &str) {
        info!("generating protos in {}", dir);

        let protos = glob_simple(&format!("{}/*.proto", dir));

        assert!(!protos.is_empty());

        protoc_rust::run(protoc_rust::Args {
            out_dir: dir,
            input: &protos.iter().map(|a| a.as_ref()).collect::<Vec<&str>>(),
            includes: &["../proto", dir],
        }).expect("protoc");
    }

    gen_v2_v3("src/v2");

    if protoc::Protoc::from_env_path()
        .version()
        .expect("version")
        .is_3()
    {
        gen_v2_v3("src/v3");

        let protos = glob_simple("src/google/protobuf/*.proto");
        protoc_rust::run(protoc_rust::Args {
            out_dir: "src/google/protobuf",
            input: &protos.iter().map(|a| a.as_ref()).collect::<Vec<&str>>(),
            includes: &["../proto", "src"],
        }).expect("protoc");
    } else {
        info!("generating stubs in src/v3");

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
}

fn main() {
    env_logger::init();

    clean_old_files();
    generate_v_from_common();
    generate_pb_rs();

    if protoc::Protoc::from_env_path()
        .version()
        .expect("version")
        .is_3()
    {
        println!("cargo:rustc-cfg=protoc3");
    }
}

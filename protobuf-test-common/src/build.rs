//! Common code of `build.rs` of two tests

pub use protobuf_codegen::Customize;

use std::fs;
use std::fmt;

use glob;


pub fn glob_simple(pattern: &str) -> Vec<String> {
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


pub fn clean_old_files() {
    for f in glob_simple("src/**/*_pb.rs") {
        fs::remove_file(f).expect("rm");
    }
    for f in glob_simple("src/**/*_pb_proto3.rs") {
        fs::remove_file(f).expect("rm");
    }
}

#[derive(Default)]
pub struct GenInDirArgs<'a> {
    pub out_dir: &'a str,
    pub input: &'a [&'a str],
    pub includes: &'a [&'a str],
    pub customize: Customize,
}

pub fn gen_in_dir<F, E>(dir: &str, gen: F)
    where
        F : for<'a> Fn(GenInDirArgs<'a>) -> Result<(), E>,
        E : fmt::Debug,
{
    info!("generating protos in {}", dir);

    let mut protos = Vec::new();
    for suffix in &[".proto", ".proto3"] {
        protos.extend(glob_simple(&format!("{}/*{}", dir, suffix)));
    }

    assert!(!protos.is_empty());

    gen(GenInDirArgs {
        out_dir: dir,
        input: &protos.iter().map(|a| a.as_ref()).collect::<Vec<&str>>(),
        includes: &["../proto", dir],
        .. Default::default()
    }).expect("protoc");
}


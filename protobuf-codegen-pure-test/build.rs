extern crate glob;
extern crate log;
extern crate env_logger;

extern crate protobuf_codegen_pure;

extern crate protobuf_test_common;

use std::io::Read;
use std::io::Write;

use std::fs;
use std::path::Path;

use protobuf_test_common::build::*;


fn copy<P1 : AsRef<Path>, P2 : AsRef<Path>>(src: P1, dst: P2) {
    let mut content = Vec::new();
    fs::File::open(src.as_ref()).expect(&format!("open {}", src.as_ref().display()))
        .read_to_end(&mut content).expect("read_to_end");

    let mut write = fs::File::create(dst).expect("create");
    write.write_all(&content).expect("write_all");
    write.flush().expect("flush");
}


fn copy_tests(dir: &str) {
    let src_dir = format!("../protobuf-test/{}", dir);
    for entry in fs::read_dir(&src_dir).expect(&format!("read_dir {}", src_dir)) {
        let file_name = entry.expect("entry").file_name().into_string().unwrap();

        // skip temporary and generated files
        if file_name.starts_with(".") || file_name.ends_with("_pb.rs") {
            continue;
        }

        copy(
            &format!("../protobuf-test/{}/{}", dir, file_name),
            &format!("{}/{}", dir, file_name),
        )
    }
}


fn gen_in_dir_pure(dir: &str) {
    gen_in_dir(dir, |GenInDirArgs { out_dir, input, includes, customize }| {
        protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
            out_dir, input, includes, customize
        })
    });
}


fn generate_pb_rs() {

    copy_tests("src/v2");

    copy_tests("src/v3");
    // TODO
    fs::remove_file("src/v3/test_map_carllerche_pb.proto").expect("rm");
    fs::remove_file("src/v3/test_map_carllerche.rs").expect("rm");

    gen_in_dir_pure("src/v2");
    gen_in_dir_pure("src/v3");
}


fn main() {
    env_logger::init();

    clean_old_files();
    generate_pb_rs();
}

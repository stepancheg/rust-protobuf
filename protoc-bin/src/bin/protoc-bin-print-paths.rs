fn main() {
    println!(
        "PROTOC={}",
        protoc_bin_vendored::protoc_bin_path()
            .unwrap()
            .to_str()
            .unwrap()
    );
    println!(
        "PROTOBUF_INCLUDE={}",
        protoc_bin_vendored::include_path()
            .unwrap()
            .to_str()
            .unwrap()
    );
}

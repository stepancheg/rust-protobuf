fn main() {
    println!(
        "{}",
        protoc_bin_vendored::protoc_bin_path()
            .unwrap()
            .to_str()
            .unwrap()
    );
}

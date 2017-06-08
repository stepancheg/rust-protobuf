# Protoc command launcher

API to invoke `protoc` command from API (e. g. from `build.rs`), any

Note, `protoc` command must be in `$PATH` along with `protoc-gen-LANG` command.

Example of using `protoc` crate is in perftest's
[build.rs](https://github.com/stepancheg/rust-protobuf/blob/master/perftest/build.rs).

Note that to generate `rust` code from `.proto`,
[protoc-rust](https://github.com/stepancheg/rust-protobuf/tree/master/protoc-rust) crate can be used,
which does not require `protoc-gen-rust` present in `$PATH`.

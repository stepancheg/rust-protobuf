# API to generate .rs files

API to generate `.rs` files to be used e. g. [from build.rs](https://github.com/stepancheg/rust-protobuf/blob/master/protobuf-test/build.rs).

Example code:

```
extern crate protoc_rust;

protoc_rust::run(protoc_rust::Args {
    out_dir: "src/protos",
    input: &["protos/a.proto", "protos/b.proto"],
    includes: &["protos"],
    customize: Customize {
      ..Default::default()
    },
}).expect("protoc");
```

And in `Cargo.toml`:

```
[build-dependencies]
protoc-rust = "1.5"
```

Note this API requires `protoc` command present in `$PATH`.
Although `protoc-gen-rust` command is not needed.

The alternative is to use
[pure-rust .proto parser and code generator](https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-codegen-pure).

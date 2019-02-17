# API to generate .rs files

API to generate `.rs` files to be used e. g. [from build.rs](https://github.com/stepancheg/rust-protobuf/blob/master/protobuf-codegen-pure-test/build.rs).

Example code:

```
extern crate protobuf_codegen_pure;

protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
    out_dir: "src/protos",
    input: &["protos/a.proto", "protos/b.proto"],
    includes: &["protos"],
    customize: protobuf_codegen_pure::Customize {
      ..Default::default()
    },
}).expect("protoc");
```

And in `Cargo.toml`:

```
[build-dependencies]
protobuf-codegen-pure = "2.3"
```

The alternative is to use
[protoc-rust crate](https://github.com/stepancheg/rust-protobuf/tree/master/protoc-rust),
which relies on `protoc` command to parse descriptors. Both crates should produce the same result,
otherwise please file a bug report.

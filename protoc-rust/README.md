<!-- cargo-sync-readme start -->

# API to generate `.rs` files using `protoc` to parse files

This API requires `protoc` command present in `$PATH`
or explicitly passed to `Codegen` object
(but `protoc` *plugin* is not needed).

```rust
extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/protos")
        .inputs(&["protos/a.proto", "protos/b.proto"])
        .include("protos")
        .run()
        .expect("Running protoc failed.");
}
```

and in `build.rs`:

```toml
[build-dependencies]
protoc-rust = "2"
```

It is advisable that `protoc-rust` build-dependency version be the same as
`protobuf` dependency.

The alternative is to use
[`protobuf-codegen-pure` crate](https://docs.rs/protobuf-codegen-pure).

# Protoc binary

This crate searches for `protoc` binary in `$PATH` by default.

`protoc` binary can be obtained using
[`protoc-bin-vendored` crate](https://docs.rs/protoc-bin-vendored)
and supplied to `Codegen` object.

# This is version 2

In branch 3 of rust-protobuf this functionality is provided by
[`protobuf-codegen` crate](https://docs.rs/protobuf-codegen/%3E=3.0.0-alpha).

<!-- cargo-sync-readme end -->

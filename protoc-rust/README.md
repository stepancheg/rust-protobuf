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

<!-- cargo-sync-readme end -->

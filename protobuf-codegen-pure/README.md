<!-- cargo-sync-readme start -->

# API to generate `.rs` files

This API does not require `protoc` command present in `$PATH`.

```rust
extern crate protoc_rust;

fn main() {
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/protos")
        .inputs(&["protos/a.proto", "protos/b.proto"])
        .include("protos")
        .run()
        .expect("Codegen failed.");
}
```

And in `Cargo.toml`:

<!-- cargo-sync-readme end -->

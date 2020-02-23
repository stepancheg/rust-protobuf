# API to generate .rs files

API to generate `.rs` files to be used e. g. [from build.rs](https://github.com/stepancheg/rust-protobuf/blob/master/protobuf-test/build.rs).

Example code:

Using stable rust-protobuf:

```rust
extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
	protoc_rust::run(protoc_rust::Args {
	    out_dir: "src/protos",
	    input: &["protos/a.proto", "protos/b.proto"],
	    includes: &["protos"],
	    customize: Customize {
	      ..Default::default()
	    },
	}).expect("protoc");
}
```

Using rust-protobuf from master:

```rust
extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
    protoc_rust::Args::new()
        .out_dir("src/protos")
        .inputs(&["protos/a.proto", "protos/b.proto"])
        .include("protos")
        .run()
        .expect("protoc");
}
```

And in `Cargo.toml`:

```
[build-dependencies]
protoc-rust = "2.0"
```

Note 1: This API requires `protoc` command present in `$PATH`.
Although `protoc-gen-rust` command is not needed.

Note 2: Is advisable that `protoc-rust` build-dependecy version be the same as `protobuf` dependency. 

The alternative is to use
[pure-rust .proto parser and code generator](https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-codegen-pure).

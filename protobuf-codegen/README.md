<!-- cargo-sync-readme start -->

# Protobuf code generator for `protobuf` crate

This crate is useful mostly from `build.rs` scripts to generate `.rs` files during the build.

# How to generate code

There are three main ways to generate `.rs` files from `.proto` files:
* using `protoc` command line tool and `protoc-gen-rust` plugin
* using this crate `Codegen` with pure rust parser
* using this crate `Codegen` with `protoc` parser

Which one should you use depends on your needs.

If you are using non-cargo build system (like Bazel), you might prefer
using `protoc-gen-rust` plugin for `protoc`.

If you build with `cargo`, you probably want to use `Codegen` from this crate.

# Protoc parser vs pure rust parser

There are two protobuf parsers which can be plugged into this crate:
* `protoc`-based parser (`protoc` is a command like utility from Google protobuf)
* pure rust parser (`protobuf-parse` crate)

`protoc`-based parser is expected to parse `.proto` files very correctly:
all Google's protobuf implementations rely on it.

While there are no known bugs in `protobuf-parse`, it is not tested very well.
Also `protobuf-parse` does not implement certain rarely used features of `.proto` parser,
mostly complex message options specified in `.proto` files.
I never saw anyone using them, but you have been warned.

Note `protoc` command can be obtained from
[`protoc-bin-vendored`](https://docs.rs/protoc-bin-vendored) crate.

# Example

```rust
// Use this in build.rs
protobuf_codegen::Codegen::new()
    // Use `protoc` parser, optional.
    .protoc()
    // Use `protoc-bin-vendored` bundled protoc command, optional.
    .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
    // All inputs and imports from the inputs must reside in `includes` directories.
    .includes(&["src/protos"])
    // Inputs must reside in some of include paths.
    .input("src/protos/apple.proto")
    .input("src/protos/banana.proto")
    // Specify output directory relative to Cargo output directory.
    .cargo_out_dir("protos")
    .run_from_script();
```

## How to use `protoc-gen-rust`

If you have to.

(Note `protoc` can be invoked programmatically with
[protoc crate](https://docs.rs/protoc/%3E=3.0.0-alpha))

0) Install protobuf for `protoc` binary.

On OS X [Homebrew](https://github.com/Homebrew/brew) can be used:

```sh
brew install protobuf
```

On Ubuntu, `protobuf-compiler` package can be installed:

```sh
apt-get install protobuf-compiler
```

Protobuf is needed only for code generation, `rust-protobuf` runtime
does not use C++ protobuf library.

1) Install `protoc-gen-rust` program (which is `protoc` plugin)

It can be installed either from source or with `cargo install protobuf-codegen` command.

2) Add `protoc-gen-rust` to $PATH

If you installed it with cargo, it should be

```sh
PATH="$HOME/.cargo/bin:$PATH"
```

3) Generate .rs files:

```sh
protoc --rust_out . foo.proto
```

This will generate .rs files in current directory.

# Customize generate code

Sometimes generated code need to be adjusted, e. g. to have custom derives.

rust-protobuf provides two options to do that:
* generated `.rs` files contain `@@protoc_insertion_point(...)` markers
  (similar markers inserts Google's protobuf generator for C++ or Java).
  Simple script `sed` one-liners can be used to replace these markers with custom annotations.
* `Codegen::customize_callback` can be used to patch generated code
  when invoked from `build.rs` script.

# Serde

rust-protobuf since version 3 no longer directly supports serde.

Rust-protobuf 3 fully supports:
* runtime reflection
* JSON parsing and printing via
 [`protobuf-json-mapping`](https://docs.rs/protobuf-json-mapping)

Which covers the most of serde use cases.

If you still need serde, generic customization callback (see above) can be used
to insert `#[serde(...)]` annotations.

[Example project](https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-examples/customize-serde)
in the rust-protobuf repository demonstrates how to do it.

<!-- cargo-sync-readme end -->

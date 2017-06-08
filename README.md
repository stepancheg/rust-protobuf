rust-protobuf
=============

<!-- https://travis-ci.org/stepancheg/rust-protobuf.png -->
[![Build Status](https://img.shields.io/travis/stepancheg/rust-protobuf.svg)](https://travis-ci.org/stepancheg/rust-protobuf)
[![crates.io version](https://img.shields.io/crates/v/protobuf.svg)](https://crates.io/crates/protobuf)
[![License](https://img.shields.io/crates/l/protobuf.svg)](https://github.com/stepancheg/rust-protobuf/blob/master/LICENSE.txt)

[Protobuf](https://developers.google.com/protocol-buffers/docs/overview) implementation in [Rust](https://www.rust-lang.org/).

* Written in pure rust
* Generate rust code
* Has runtime library for generated code
  (Coded{Input|Output}Stream impl)

## How to generate rust code

There are several ways to generate rust code from `.proto` files

### With protoc command and protoc-gen-rust plugin

0) Install protobuf for `protoc` binary.

On OS X [Homebrew](https://github.com/Homebrew/homebrew) can be used:

```
brew install protobuf
```

On Ubuntu, `protobuf-compiler` package can be installed:

```
apt-get install protobuf-compiler
```

Protobuf is needed only for code generation, `rust-protobuf` runtime
does not use `protobuf` library.

1) Install `protoc-gen-rust` program (which is `protoc` plugin)

It can be installed either from source or with `cargo install protobuf` command.

2) Add `protoc-gen-rust` to $PATH

If you installed it with cargo, it should be

```
PATH="$HOME/.cargo/bin:$PATH"
```

3) Generate .rs files:

```
protoc --rust_out . foo.proto
```

This will generate .rs files in current directory.

## Invoke protoc programmatically with protoc crate

Have a look at readme in [protoc crate](https://github.com/stepancheg/rust-protobuf/tree/master/protoc).

## Invoke protoc programmatically with protoc-rust crate

Have a look at readme in [protoc-rust crate](https://github.com/stepancheg/rust-protobuf/tree/master/protoc-rust).

## Generated code

Have a look at generated files, used internally in rust-protobuf:

* [descriptor.rs](https://github.com/stepancheg/rust-protobuf/blob/master/src/lib/descriptor.rs)
  for [descriptor.proto](https://github.com/stepancheg/rust-protobuf/blob/master/src/proto/google/protobuf/descriptor.proto)
  (that is part of Google protobuf)

## Rustdoc

docs.rs hosts [rustdoc for protobuf](https://docs.rs/protobuf/*/protobuf/).

## TODO

* Implement some rust-specific options
* Deal better with namespaces
* Protobuf reflection
* Extensions

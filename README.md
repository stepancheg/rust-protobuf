rust-protobuf
=============

[Protobuf](https://developers.google.com/protocol-buffers/docs/overview) implementation in [Rust](http://www.rust-lang.org/).

* Written in pure rust
* Generate rust code
* Has runtime library for generated code
  (Coded{Input|Output}Stream impl)
* Incomplete and unstable

## How to use rust-protobuf

0) Install protobuf for `protoc` binary.

On OS X [Homebrew](https://github.com/mxcl/homebrew) can be used:

```
brew install protobuf
```

1) Compile the project:

```
cd src
./rebuild.sh
```

`protoc-gen-rust` binary is generated. `protoc-gen-rust` is a rust
plugin for protoc.

2) Add `protoc-gen-rust` to $PATH:

```
PATH="`pwd`:$PATH"
```

3) Generate .rs files:

```
protoc --rust_out . foo.proto
```

This will generate .rs files in current directory.

Same procedure is used to regenerate .rs files for rust-protobuf
itself, see `./regerate.sh`.

4) Include generated files into your project .rc file:

```
extern mod protobuf; // depend on rust-protobuf runtime
mod foo; // add generated file to the project
```


## Generated code

Have a look at generated files, used internally in rust-protobuf:

* [shrug.rs](https://github.com/stepancheg/rust-protobuf/blob/master/src/lib/shrug.rs)
  for [shrug.proto](https://github.com/stepancheg/rust-protobuf/blob/master/src/proto/shrug.proto)
  (used by rust-protobuf tests)
* [descriptor.rs](https://github.com/stepancheg/rust-protobuf/blob/master/src/lib/descriptor.rs)
  for [descriptor.proto](https://github.com/stepancheg/rust-protobuf/blob/master/src/proto/google/protobuf/descriptor.proto)
  (that is part of Google protobuf)

## TODO

* Implement some rust-specific options
* Deal better with namespaces
* Store unknown fields
* Protobuf reflection
* Extensions
* Accessors for types (like `has_foo` or `set_foo`)
* Generate stubs for services
* Comprehensive test suite

## Contact me

[@stepancheg](https://github.com/stepancheg/) on github if you have any questions, suggesions or patches.

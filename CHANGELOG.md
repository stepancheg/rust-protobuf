# Changelog

## [Unreleased]

- [Protobuf no longer exposes internal `hex`
  module](https://github.com/stepancheg/rust-protobuf/commit/8ad9687529a565c5ef2db93732cc20c8d8d22f00)
- Generated code can now be customized not only with `rustproto.proto`
  but also when invoked programmatically with
  [`protoc-rust`](https://github.com/stepancheg/rust-protobuf/blob/b8573bd53cf5a9611598abbf02b71c49e59a8891/protobuf-codegen/src/customize.rs#L9)
- [Drop `MessageStatic` trait](https://github.com/stepancheg/rust-protobuf/issues/214)
- [`protobuf-codegen` is a separate crate](https://github.com/stepancheg/rust-protobuf/pull/261)
- [Drop old reflection
  accessors](https://github.com/stepancheg/rust-protobuf/commit/7a03aee4e67bdd25ae6c403f37386707a0ab5eb9).
  Now code may need to be regenerated when protobuf version changed.
- [Option to specify recursion limit](https://github.com/stepancheg/rust-protobuf/pull/248)
- [Implement `std::io` traits by `CodedInputStream` and
  `CodedOutputStream`](https://github.com/stepancheg/rust-protobuf/pull/232)
- [Implement conversions for `Repeated*`](https://github.com/stepancheg/rust-protobuf/pull/236)
- [Generated code now uses closures instead of private functions
  for reflection](https://github.com/stepancheg/rust-protobuf/pull/267)
- [Proto files with suffixes others than `.proto`
  are now supported](https://github.com/stepancheg/rust-protobuf/pull/265)
- [Oneof are now public by
  default](https://github.com/stepancheg/rust-protobuf/commit/8bd911e2ea0d4461580105209ae11d9d3ec21fd0)
- `*descriptor_static()` function signatures no longer include `Option` param
  ([1](https://github.com/stepancheg/rust-protobuf/commit/8723fca5fb29e279b3ab7d2a28c8fab79189c9c2),
  [2](https://github.com/stepancheg/rust-protobuf/commit/c5446983be3b9d8d49ee39b443ed4fabd8f35440))

## [1.5] branch
- [Better error message when `protoc` command is not
  found](https://github.com/stepancheg/rust-protobuf/commit/d59eb368deea1d292a161c3f30ff1123a022046d)

## [1.5.1] - 2018-04-02
- [Fix serialization or large repeated packed fields](https://github.com/stepancheg/rust-protobuf/issues/281)

## [1.5.0] - 2018-03-25
- [Unknown enum values are now stored in unknown fields](https://github.com/stepancheg/rust-protobuf/pull/276)

## [1.4.5] - 2018-04-02
- [Fix serialization or large repeated packed fields](https://github.com/stepancheg/rust-protobuf/issues/281)

## [1.4.4] - 2018-03-05
- [Escape macro keyword](https://github.com/stepancheg/rust-protobuf/pull/269)

## [1.4.3] - 2017-12-03
- [Allow enum variants to be named `Self`](https://github.com/stepancheg/rust-protobuf/pull/259)

## [1.4.2] - 2017-10-14
- [Properly read messages from blocking streams](https://github.com/stepancheg/rust-protobuf/issues/157)

## [1.4.1] - 2017-06-24
- [Convert `String` to `Chars`](https://github.com/stepancheg/rust-protobuf/pull/225)

## [1.4] - 2017-06-24
- Start of changelog

# Changelog

## [Unreleased]

- [Drop `MessageStatic` trait](https://github.com/stepancheg/rust-protobuf/issues/214)
- [`protobuf-codegen` is a separate crate](https://github.com/stepancheg/rust-protobuf/pull/261)
- [Drop old reflection
  accessors](https://github.com/stepancheg/rust-protobuf/commit/7a03aee4e67bdd25ae6c403f37386707a0ab5eb9).
  Now code may need to be regenerated when protobuf version changed.
- [Better error message when `protoc` command is not
  found](https://github.com/stepancheg/rust-protobuf/commit/d59eb368deea1d292a161c3f30ff1123a022046d)
- [Option to specify recursion limit](https://github.com/stepancheg/rust-protobuf/pull/248)
- [Implement `std::io` traits by `CodedInputStream` and
  `CodedOutputStream`](https://github.com/stepancheg/rust-protobuf/pull/232)
- [Implement conversions for `Repeated*`](https://github.com/stepancheg/rust-protobuf/pull/236)
- [Generated code now uses closures instead of private functions
  for reflection](https://github.com/stepancheg/rust-protobuf/pull/267)
- [Proto files with suffixes others than .proto
  are now supported](https://github.com/stepancheg/rust-protobuf/pull/265)

## [1.4.3] - 2017-12-03
- [Allow enum variants to be named `Self`](https://github.com/stepancheg/rust-protobuf/pull/259)

## [1.4.2] - 2017-10-14
- [Properly read messages from blocking streams](https://github.com/stepancheg/rust-protobuf/issues/157)

## [1.4.1] - 2017-06-24
- [Convert `String` to `Chars`](https://github.com/stepancheg/rust-protobuf/pull/225)

## [1.4] - 2017-06-24
- Start of changelog

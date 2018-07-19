# Changelog

## [Unreleased]

- [`Default` is implemented for enums even in proto2](
  https://github.com/stepancheg/rust-protobuf/commit/166966627ebc1e5ce650acd1593489e52757178e)
- [Option to store repeated message fields in `Vec` instead of `RepeatedField`](
  https://github.com/stepancheg/rust-protobuf/issues/280). This option may be turned on by default later.
- Similarly, [option to store singular field on `Option` instead of `SingularPtrField`](
  https://github.com/stepancheg/rust-protobuf/issues/300), which also may be turned on by default later.
- `generate_accessors` and `generate_getter` options to disable generation of accessor functions.
- `PartialEq` with large number of fields
  [now panics](https://github.com/stepancheg/rust-protobuf/commit/4f1ca564a00e85b6e3821e91aace71ccb6592bf5).
  Previosly it could cause stack overflow in the Rust compiler.
- Text format parsing is implemented (previously it was only possible to print to text format)
- Reflection can now mutate data (previously reflection could only read data)
- Fix OOM on malformed input
- [Flush `CodedOutputStream` on `drop`](https://github.com/stepancheg/rust-protobuf/commit/0e9cc5964c2731a771725bcf70125d3eb1c273b3)

## [2.0.4] - 2018-07-19

- Minimum bytes version is 0.4 now (since protobuf doesn't work with 0.3 anyway)

## [2.0.3] - 2018-07-11

- [Fix panic on singular string field appeared more than
  once](https://github.com/stepancheg/rust-protobuf/commit/28adf07a0b0027ddc8ff57f04ffeb69f35f65620)
- [Properly handle map fields with key or value skipped in binary proto](
  https://github.com/stepancheg/rust-protobuf/issues/318)

## [2.0.2] - 2018-05-29

- Make rust-protobuf compatible with rust 1.23.0

## [2.0.1] - 2018-05-27

- Fix codegen with enum with
  [default value a reserved rust keyword](https://github.com/stepancheg/rust-protobuf/issues/295)

## [2.0.0] - 2018-05-17

- Rebublished branch 2.0 because of
  [backward compatibility issues in 1.6 branch](https://github.com/stepancheg/rust-protobuf/issues/289)

## [1.7.4] - 2018-07-11

- [Fix panic on singular string field appeared more than
  once](https://github.com/stepancheg/rust-protobuf/commit/28adf07a0b0027ddc8ff57f04ffeb69f35f65620)
- [Properly handle map fields with key or value skipped in binary proto](
  https://github.com/stepancheg/rust-protobuf/issues/318)

## [1.7.3] - 2018-05-29

- Make rust-protobuf compatible with rust 1.23.0

## [1.7.2] - 2018-05-27

- Fix codegen with enum with
  [default value a reserved rust keyword](https://github.com/stepancheg/rust-protobuf/issues/295)

## [1.7.1] - 2018-05-17

- Rebublished branch 1.5 because of
  [backward compatibility issues in 1.6 branch](https://github.com/stepancheg/rust-protobuf/issues/289)

## [1.6.0] - 2018-05-11

### New features

- [Pure rust codegen](https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-codegen-pure)
- Generated code can now be customized not only with `rustproto.proto`
  but also when invoked programmatically with
  [`protoc-rust`](https://github.com/stepancheg/rust-protobuf/blob/b8573bd53cf5a9611598abbf02b71c49e59a8891/protobuf-codegen/src/customize.rs#L9)
- [Oneof are now public by
  default](https://github.com/stepancheg/rust-protobuf/commit/8bd911e2ea0d4461580105209ae11d9d3ec21fd0)
- [Option to specify recursion limit](https://github.com/stepancheg/rust-protobuf/pull/248)
- [Implement conversions for `Repeated*`](https://github.com/stepancheg/rust-protobuf/pull/236)
- [Proto files with suffixes others than `.proto`
  are now supported](https://github.com/stepancheg/rust-protobuf/pull/265)
- [Generated code now uses closures instead of private functions
  for reflection](https://github.com/stepancheg/rust-protobuf/pull/267)

### Backward compatibility issues

- [Drop `MessageStatic` trait](https://github.com/stepancheg/rust-protobuf/issues/214)
- [Protobuf no longer exposes internal `hex`
  module](https://github.com/stepancheg/rust-protobuf/commit/8ad9687529a565c5ef2db93732cc20c8d8d22f00)
- [`protobuf-codegen` is a separate crate](https://github.com/stepancheg/rust-protobuf/pull/261)
- [Drop old reflection
  accessors](https://github.com/stepancheg/rust-protobuf/commit/7a03aee4e67bdd25ae6c403f37386707a0ab5eb9).
  Now code may need to be regenerated when protobuf version changed.
- [Implement `std::io` traits by `CodedInputStream` and
  `CodedOutputStream`](https://github.com/stepancheg/rust-protobuf/pull/232)
- `*descriptor_static()` function signatures no longer include `Option` param
  ([1](https://github.com/stepancheg/rust-protobuf/commit/8723fca5fb29e279b3ab7d2a28c8fab79189c9c2),
  [2](https://github.com/stepancheg/rust-protobuf/commit/c5446983be3b9d8d49ee39b443ed4fabd8f35440))

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

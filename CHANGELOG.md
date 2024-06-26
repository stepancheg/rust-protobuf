# Changelog

## [3.5] - Unreleased

- [Default to packed for repeated primitives in proto3](https://github.com/stepancheg/rust-protobuf/pull/707)
- [Put correct flag in comment of generated files](https://github.com/stepancheg/rust-protobuf/pull/655)
- [Option to disable non-exhausive annotation for oneof](https://github.com/stepancheg/rust-protobuf/pull/726)
- [Option to generate `BTreeMap` for map fields](https://github.com/stepancheg/rust-protobuf/pull/700)
- [Fix writing large messages](https://github.com/stepancheg/rust-protobuf/pull/725)

## [3.4] - 2024-02-24

* [Unnecessary copy in print_to_string_internal](https://github.com/stepancheg/rust-protobuf/pull/684)
* [Ignore error of `flush` in `Drop` of `CodedOutputStream`](https://github.com/stepancheg/rust-protobuf/issues/714)
* [Faster `encoded_varint64_len`](https://github.com/stepancheg/rust-protobuf/pull/709)
* [`reserved` keyword in enums](https://github.com/stepancheg/rust-protobuf/pull/712)
* [Set streaming options in pure parser](https://github.com/stepancheg/rust-protobuf/pull/646)

## [3.3.0] - 2023-09-30

* [protoc_extra_arg not passed through](https://github.com/stepancheg/rust-protobuf/issues/643)
* [move custom code before derive block in struct](https://github.com/stepancheg/rust-protobuf/issues/675)
* [Enum::from_str](https://github.com/stepancheg/rust-protobuf/pull/664)

## [3.2.0] - 2022-09-26

* [Correctly specify `log` version](https://github.com/stepancheg/rust-protobuf/pull/652)
* [Hash for SpecialFields](https://github.com/stepancheg/rust-protobuf/pull/648)

## [3.1.0] - 2022-06-22

* [Reflection API to clear fields](https://github.com/stepancheg/rust-protobuf/pull/635)

## [3.0.3] - 2022-05-31

* `optional` fields in proto3 are now
  [handled correctly in `protoc-gen-rust` plugin for `protoc`](https://github.com/stepancheg/rust-protobuf/issues/625)

## [3.0.2] - 2022-05-06

* [Sort modules in generated mod.rs](https://github.com/stepancheg/rust-protobuf/issues/621)

## [3.0.1] - 2022-05-01

* More up to date readme.

## [3.0.0] - 2022-05-01

* New stable version released.

## Before version 3

[Changelog before version 3](CHANGELOG-before-3.md)

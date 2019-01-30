# Contributing to rust-protobuf

## Code of conduct

You are welcome to contribute code of conduct.

Meanwhile, please be polite to everyone.

## I just want to ask a question

Feel free to open an issue to ask a question, the volume of questions is low,
so it's OK at the moment. But please don't expect prompt answer.

## I have found a bug

Please open an issue. When reporting a bug please include minimal example
providing as much information as possible. In particular, please specify:

* exact proto file
* generated file
* rust-protobuf version
* command which was used to generate code (ideally, temporary standalone repository)
* what is version of `protoc` command
* what is operating system

## Tests

Most of code changes should be accompanied by tests.

Most tests can be executed by invoking `cargo test` in `protobuf-test` directory.

## Codegen

If you change code generator, tests will check that code generator works correctly.

However, before submitting a PR, it's necessary to regenerate generated files
shipped with rust-protobuf, notably, `descriptor.rs`.

This can be done by invoking a script `protobuf/regenerate.sh`.

## Performance improvements

Are always welcome, especially if they are backward-compatible.

## API changes, generated code changes

There are tons 

## What if I just want to do something useful

Some issues in the tracker are marked with 

## Help wanted

Most of all documentation is needed, any changes to rustdoc or markdown pages on github are welcome.

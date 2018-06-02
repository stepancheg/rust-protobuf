# Tests for rust-protobuf

To execute tests simply execute `cargo test`.

If protobuf 3 is installed, command will include test for protobuf 3 generated code.

If `--features=with-bytes` flag is specified, tests will include test for `with-bytes` feature,
which is not enabled by default.

`./test.sh` is to be used from [travis-ci](https://travis-ci.org/stepancheg/rust-protobuf/),
and not needed for local development.

`cargo test` executes [`build.rs`](https://github.com/stepancheg/rust-protobuf/blob/master/protobuf-test/build.rs) script,
which generates `.rs` files from `.proto` and `mod.rs` files for certain folder.

## Test contents

* `v2` contains tests specific to protobuf 2
* `v3` contains tests specific to protobuf 2
* `google` contains tests `.proto` files taken from Google's protobuf implementation
* `common` contains tests which are identical for both versions of protobuf syntax.
  `common/v2` directory contains sources, and contents of `common/v3` is generated
  from `common/v2` by copy and replace.

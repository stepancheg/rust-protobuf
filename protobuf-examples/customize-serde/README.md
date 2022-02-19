<!-- cargo-sync-readme start -->

# How to use serde with rust-protobuf

rust-protobuf 3 no longer directly supports serde.

Practically, serde is needed mostly to be able to serialize and deserialize JSON,
and **rust-protobuf supports JSON directly**, and more correctly according to
official protobuf to JSON mapping. For that reason,
native serde support was removed from rust-protobuf.

This crate is an example how to inject serde annotations into generated code.

Annotations are configured from `build.rs`.

<!-- cargo-sync-readme end -->

<!-- cargo-sync-readme start -->

# Compare generated code and API between rust-protobuf and prost

This document tries to objectively compare rust-protobuf and prost.

Use

```rust
cargo expand -p protobuf-examples-vs-prost
```

to see the code.

Feel free to submit more items here.

## The differences

The comparison is
* rust-protobuf from master and
* prost version 0.9.0, which is the latest released version at the moment of writing.

The list is incomplete.

### Use of derives

Prost relies on `#[derive(::prost::Message)]` in generated code.
* seems to be unnecessary, if code is generated, not written by hand,
  derive does not help much.
* derives also make code harder to understand: something like `cargo expand`
  is needed, which is not as clear as looking at the generated code.

## Unknown fields

rust-protobuf preserves "unknown fields". This improves forward compatibility:
when new message is read using old version of schema and then written back,
rust-protobuf preserves the unknown fields, and prost discards them.

In 99% of the cases, unknown fields are not needed to be preserved.

## Cached size

Prost seems to not cache "cached size" of message before serialization.

In the worst case, with deeply nested messages, it results in exponential growth
serialization time. But deeply nested messages are rare, and API is clearer without it.

(Note, serialization can be linear if message sizes are stored in a queue/stack
during serialization. rust-protobuf did it
[before 2014](https://github.com/stepancheg/rust-protobuf/commit/86fe60cc67e3ea257fcad417bcb039973ace3bfc),
see `compute_sizes` function signature. But it was changed to storing cached size
because storing cached size is faster. If prost doesn't want to store cached size,
perhaps they can at least use similar approach.)

## Default instance

For each message, rust-protobuf generates `M::default_instance()` function
and `Default for <&M>` (similarly to what C++ and Java generators do).
So when fetching an optional field reference, rust-protobuf is able to
always provide a reference to a message instance: either a real message or a default instance
if a field is unset.

Prost doesn't do it.

## Enums

In prost, enum fields have type `i32`.

In rust-protobuf, `EnumOrUnknown<T>` is used to store enum values.

Note it is important to store full `i32` in enum value, not just the enum
to preserve unknown values when reading future message with old schema
and writing it back.

So rust-protobuf is a bit more type-safe, but generated code is somewhat harder to use.

## Reflection

Both prost and rust-protobuf can be configured to generate serde annotations.

But rust-protobuf fully supports runtime reflection
(e. g. find field by name, get field, set field).
This is implemented similarly to C++ implementation of protobuf.
The drawback is that generated code is quite large (which also affects binary size).

However, for prost there's [prost-reflect](https://github.com/andrewhickman/prost-reflect)
crate. I don't know what is it's status, seems like it is not mature enough yet:
the project started less than two months ago.

## Dynamic messages

rust-protobuf supports dynamic messages.

For prost, `prost-reflect` implements them.

## JSON

Proper [protobuf JSON mapping](https://developers.google.com/protocol-buffers/docs/proto3#json)
is supported natively by rust-protobuf.

For prost it can be done with `prost-reflect` package.

## Protobuf text format

rust-protobuf supports text format printing and parsing. Prost seems to be missing this feature.

## Dependency on `protoc` binary

prost depends on `protoc` binary to parse `.proto` files.

rust-protobuf can do both:
* parse using `protoc` binary
* has pure rust parser (and typechecker) of `.proto` files

## gRPC

[tonic](https://docs.rs/tonic/latest/tonic/) is a quality pure rust gRPC implementation
for prost.

[grpc-rs](https://github.com/tikv/grpc-rs) is an implementation for both
prost and rust-protobuf.

<!-- cargo-sync-readme end -->

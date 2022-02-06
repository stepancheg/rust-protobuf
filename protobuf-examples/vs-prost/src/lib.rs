//! # Compare generated code and API between rust-protobuf and prost
//!
//! This document tries to objectively compare rust-protobuf and prost.
//!
//! Use
//!
//! ```ignore
//! cargo expand -p protobuf-examples-vs-prost
//! ```
//!
//! to see the code.
//!
//! Feel free to submit more items here.
//!
//! ## The differences
//!
//! The comparison is rust-protobuf from master and
//! prost version 0.9.0, which is the latest released version at the moment of writing.
//!
//! ### Use of derives
//!
//! Prost relies on `#[derive(::prost::Message)]` in generated code.
//! * seems to be unnecessary, if code is generated, not written by hand,
//!   derive does not help much.
//! * derives also make code harder to understand: something like `cargo expand`
//!   is needed, which is not as clear as looking at the generated code.
//!
//! ## Unknown fields
//!
//! rust-protobuf preserves "unknown fields". This improves forward compatibility:
//! when new message is read using old version of schema and then written back,
//! rust-protobuf preserves the unknown fields, and prost discards them.
//!
//! In 99% of the cases, unknown fields are not needed to be preserved.
//!
//! ## Enums
//!
//! In prost, enum fields have type `i32`.
//!
//! In rust-protobuf, `EnumOrUnknown<T>` is used to store enum values.
//!
//! Note it is important to store full `i32` in enum value, not just the enum
//! to preserve unknown values when reading future message with old schema
//! and writing it back.
//!
//! So rust-protobuf is a bit more type-safe, but generated code is somewhat harder to use.
//!
//! ## Reflection
//!
//! Both prost and rust-protobuf can be configured to generate serde annotations.
//!
//! But rust-protobuf fully supports runtime reflection
//! (e. g. find field by name, get field, set field).
//! This is implemented similarly to C++ implementation of protobuf.
//! The drawback is that generated code is quite large (which also affects binary size).
//!
//! However, for prost there's [prost-reflect](https://github.com/andrewhickman/prost-reflect)
//! crate. I don't know what is it's status.
//!
//! ## Dynamic messages
//!
//! rust-protobuf supports dynamic messages.
//!
//! For prost, `prost-reflect` implements them.
//!
//! ## JSON
//!
//! Proper [protobuf JSON mapping](https://developers.google.com/protocol-buffers/docs/proto3#json)
//! is supported natively by rust-protobuf.
//!
//! For prost it can be done with `prost-reflect` package.
//!
//! ## Protobuf text format
//!
//! rust-protobuf supports text format printing and parsing. Prost seems to be missing this feature.
//!
//! ## Dependency on `protoc` binary
//!
//! prost depends on `protoc` binary to parse `.proto` files.
//!
//! rust-protobuf can do both:
//! * parse using `protoc` binary
//! * has pure rust parser (and typechecker) of `.proto` files
//!
//! ## gRPC
//!
//! [tonic](https://docs.rs/tonic/latest/tonic/) is a quality pure rust gRPC implementation
//! for prost.
//!
//! [grpc-rs](https://github.com/tikv/grpc-rs) is an implementation for both
//! prost and rust-protobuf.

mod rust_protobuf_protos {
    include!(concat!(env!("OUT_DIR"), "/rust_protobuf_protos/mod.rs"));
}

mod prost_protos {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}

#[cfg(test)]
mod test {
    use crate::prost_protos;
    use crate::rust_protobuf_protos;

    #[test]
    fn triangles() {
        let mut rp = rust_protobuf_protos::triangle::Triangle::new();
        let mut pr = prost_protos::Triangle::default();

        rp.description = "The triangle".to_owned();
        pr.description = "The triangle".to_owned();

        rp.color = rust_protobuf_protos::triangle::Color::RED.into();
        pr.color = prost_protos::Color::Red as i32;
    }
}

#![cfg(test)]

extern crate protobuf;

#[cfg(feature = "with-bytes")]
extern crate bytes;

mod v2;

// `cfg(protoc3)` is emitted by `build.rs`

#[cfg(protoc3)]
mod v3;

mod common;

#[cfg(protoc3)]
mod google;

mod test;

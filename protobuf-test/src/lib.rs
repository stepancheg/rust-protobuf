#![cfg(test)]

extern crate protobuf;

#[cfg(feature = "with-bytes")]
extern crate bytes;

mod v2;

// `cfg(proto3)` is emitted by `build.rs`

#[cfg(proto3)]
mod v3;

#[cfg(proto3)]
mod google;

mod test;

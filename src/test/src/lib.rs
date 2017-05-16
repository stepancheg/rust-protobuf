extern crate protobuf;

#[cfg(feature = "with-bytes")]
extern crate bytes;

mod v2;

#[cfg(feature = "proto3")]
mod v3;

#[cfg(feature = "proto3")]
mod google;

mod test;

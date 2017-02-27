extern crate protobuf;

mod v2;

#[cfg(proto3)]
mod v3;

#[cfg(proto3)]
mod google;

mod test;

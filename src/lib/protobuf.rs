#![crate_id(name = "protobuf#0.1.0")]
#![crate_type = "lib"]
#![feature(globs)]
#![feature(managed_boxes)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

#![desc = "protobuf implementation for rust"]
#![license = "BSD"]

extern crate collections;
extern crate sync;

pub use core::*;
pub use unknown::UnknownFields;
pub use unknown::UnknownValues;
pub use unknown::UnknownValue;
pub use unknown::UnknownValueRef;
pub use unknown::UnknownValuesIter;
pub use unknown::UnknownFieldIter;

mod core;
pub mod rt;
pub mod lazy;
pub mod descriptor;
pub mod codegen;
mod misc;
mod zigzag;
mod hex;
mod paginate;
mod unknown;
mod strx;

#[cfg(test)]
mod shrug;
#[cfg(test)]
mod test_root;
#[cfg(test)]
mod test;

// so `use protobuf::*` could work in descriptor mod
pub mod protobuf {
    pub use descriptor;
    pub use codegen;
    pub use core::*;
    pub use rt;
    pub use lazy;
    pub use unknown::UnknownFields;
    pub use unknown::UnknownValues;
    pub use unknown::UnknownValue;
    pub use unknown::UnknownValueRef;
    pub use unknown::UnknownValuesIter;
    pub use unknown::UnknownFieldIter;
}

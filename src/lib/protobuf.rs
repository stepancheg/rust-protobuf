#![crate_id(name = "protobuf#0.1.0")]
#![crate_type = "lib"]
#![feature(globs)]
#![allow(non_camel_case_types)]

#![desc = "protobuf implementation for rust"]
#![license = "BSD"]

extern crate collections;
extern crate sync;
extern crate collections;

pub use core::*;
pub use unknown::UnknownFields;
pub use unknown::UnknownValues;
pub use unknown::UnknownValue;
pub use unknown::UnknownValueRef;
pub use unknown::UnknownValuesIter;
pub use unknown::UnknownFieldIter;
pub use repeated::RepeatedField;
pub use singular::SingularField;
pub use clear::Clear;

mod core;
pub mod rt;
pub mod lazy;
pub mod descriptor;
pub mod codegen;
pub mod repeated;
pub mod singular;
pub mod clear;
pub mod reflect;
pub mod text_format;
mod misc;
mod zigzag;
mod hex;
mod paginate;
mod unknown;
mod strx;
mod descriptorx;

#[cfg(test)]
mod shrug;
#[cfg(test)]
mod test_root;
#[cfg(test)]
mod test;
#[cfg(test)]
mod text_format_test_data;
#[cfg(test)]
mod test_nonunique_enum;

// so `use protobuf::*` could work in descriptor mod
pub mod protobuf {
    pub use descriptor;
    pub use codegen;
    pub use reflect;
    pub use core::*;
    pub use rt;
    pub use lazy;
    pub use unknown::UnknownFields;
    pub use unknown::UnknownValues;
    pub use unknown::UnknownValue;
    pub use unknown::UnknownValueRef;
    pub use unknown::UnknownValuesIter;
    pub use unknown::UnknownFieldIter;
    pub use repeated::RepeatedField;
    pub use singular::SingularField;
    pub use clear::Clear;
}

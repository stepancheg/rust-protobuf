#![crate_type = "lib"]
#![feature(globs)]
#![allow(non_camel_case_types)]

extern crate collections;

pub use unknown::UnknownFields;
pub use unknown::UnknownValues;
pub use unknown::UnknownValue;
pub use unknown::UnknownValueRef;
pub use unknown::UnknownValuesIter;
pub use unknown::UnknownFieldIter;
pub use repeated::RepeatedField;
pub use singular::SingularField;
pub use singular::SingularPtrField;
pub use clear::Clear;
pub use core::Message;
pub use core::MessageStatic;
pub use core::ProtobufEnum;
pub use core::parse_from_bytes;
pub use core::parse_from_reader;
pub use core::parse_length_delimited_from;
pub use core::parse_length_delimited_from_bytes;
pub use stream::CodedInputStream;
pub use stream::CodedOutputStream;
pub use stream::wire_format;
pub use error::ProtobufResult;
pub use error::ProtobufError;

pub mod core;
pub mod rt;
pub mod lazy;
pub mod descriptor;
pub mod codegen;
pub mod repeated;
pub mod singular;
pub mod clear;
pub mod reflect;
pub mod text_format;
pub mod stream;
pub mod error;

// used by test
pub mod hex;

mod misc;
mod zigzag;
mod maybe_owned_slice;
mod paginate;
mod unknown;
mod strx;
mod descriptorx;

// so `use protobuf::*` could work in descriptor mod
mod protobuf {
    pub use descriptor;
    pub use codegen;
    pub use reflect;
    pub use core::*;
    pub use error::*;
    pub use stream::*;
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
    pub use singular::SingularPtrField;
    pub use clear::Clear;
}

//! Random code uses in protobuf test crates.

pub mod build;
pub mod hex;

mod serialize_deserialize_generated;
pub use serialize_deserialize_generated::*;

mod serialize_deserialize_dynamic;
pub use serialize_deserialize_dynamic::*;

mod serialize_deserialize_both;
pub use serialize_deserialize_both::*;

mod text_format_tests;
pub use text_format_tests::*;

mod json_tests;
pub use json_tests::*;

mod reflect_tests;
pub use reflect_tests::*;

mod cargo;
pub use cargo::*;

mod interop;
pub use interop::*;

mod dynamic;
pub use dynamic::*;

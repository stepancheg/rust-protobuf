//! # Protobuf code generator for `protobuf` crate
//!
//! This crate is useful mostly from `build.rs` scripts to generate `.rs` files during the build.
//!
//! # How to generate code
//!
//! There are three main ways to generate `.rs` files from `.proto` files:
//! * using `protoc` command line tool and `protoc-gen-rust` plugin
//! * using this crate [`Codegen`](crate::Codegen) with pure rust parser
//! * using this crate `Codegen` with `protoc` parser
//!
//! Which one should you use depends on your needs.
//!
//! If you are using non-cargo build system (like Bazel), you might prefer
//! using `protoc-gen-rust` plugin for `protoc`.
//!
//! If you build with `cargo`, you probably want to use `Codegen` from this crate.
//!
//! # Protoc parser vs pure rust parser
//!
//! There are two protobuf parsers which can be plugged into this crate:
//! * `protoc`-based parser (`protoc` is a command like utility from Google protobuf)
//! * pure rust parser (`protobuf-parse` crate)
//!
//! `protoc`-based parser is expected to parse `.proto` files very correctly:
//! all Google's protobuf implementations rely on it.
//!
//! Where there are no known bugs in `protobuf-parse`, it is not tested very well.
//! Also `protobuf-parse` does not implement certain rarely used features of `.proto` parser,
//! mostly complex message options specified in `.proto` files.
//! I never saw anyone using them, but you have been warned.
//!
//! Note `protoc` command can be obtained from `protoc-bin-vendored` crate.
//!
//! # Version 3
//!
//! Note this is documentation for protobuf-codegen version 3 (which is currently in development).
//!
//! In version 3 this crate encapsulates both `protoc`-based codegen and pure rust codegen.
//!
//! In version 2 `protobuf-codegen` contains `protoc`-based codegen,
//! and `protobuf-codegen-pure` is pure rust codegen.

#![deny(rustdoc::broken_intra_doc_links)]

mod codegen;
mod compiler_plugin;
mod customize;
mod gen;
pub mod gen_and_write;
pub mod protoc_gen_rust;

pub use codegen::Codegen;
pub use customize::Customize;
#[doc(hidden)]
pub use gen::paths::proto_name_to_rs;

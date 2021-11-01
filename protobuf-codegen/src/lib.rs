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

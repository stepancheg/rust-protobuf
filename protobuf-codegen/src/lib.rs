#![deny(rustdoc::broken_intra_doc_links)]

mod compiler_plugin;
mod customize;
mod gen;
mod gen_and_write;
pub mod protoc_gen_rust;

pub use customize::Customize;
#[doc(hidden)]
pub use gen::paths::proto_name_to_rs;
pub use gen_and_write::gen_and_write;

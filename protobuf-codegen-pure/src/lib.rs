//! API to generate `.rs` files.
//!
//! This API does not require `protoc` command present in `$PATH`.
//!
//! ```
//! extern crate protoc_rust;
//!
//! fn main() {
//!     protobuf_codegen_pure::Codegen::new()
//!         .out_dir("src/protos")
//!         .inputs(&["protos/a.proto", "protos/b.proto"])
//!         .include("protos")
//!         .run()
//!         .expect("Codegen failed.");
//! }
//! ```
//!
//! It is advisable that `protobuf-codegen-pure` build-dependecy version be the same as
//! `protobuf` dependency.
//!
//! The alternative is to use `protoc-rust` crate.

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

extern crate protobuf;
extern crate protobuf_codegen;

use std::path::Path;
use std::path::PathBuf;
use std::process;

pub use protobuf_codegen::Customize;
use protobuf_parse::pure;

/// Invoke pure rust codegen. See [crate docs](crate) for example.
// TODO: merge with protoc-rust def
#[derive(Debug, Default)]
pub struct Codegen {
    /// --lang_out= param
    out_dir: PathBuf,
    /// -I args
    includes: Vec<PathBuf>,
    /// List of .proto files to compile
    inputs: Vec<PathBuf>,
    /// Customize code generation
    customize: Customize,
}

impl Codegen {
    /// Fresh new codegen object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the output directory for codegen.
    pub fn out_dir(&mut self, out_dir: impl AsRef<Path>) -> &mut Self {
        self.out_dir = out_dir.as_ref().to_owned();
        self
    }

    /// Add an include directory.
    pub fn include(&mut self, include: impl AsRef<Path>) -> &mut Self {
        self.includes.push(include.as_ref().to_owned());
        self
    }

    /// Add include directories.
    pub fn includes(&mut self, includes: impl IntoIterator<Item = impl AsRef<Path>>) -> &mut Self {
        for include in includes {
            self.include(include);
        }
        self
    }

    /// Add an input (`.proto` file).
    pub fn input(&mut self, input: impl AsRef<Path>) -> &mut Self {
        self.inputs.push(input.as_ref().to_owned());
        self
    }

    /// Add inputs (`.proto` files).
    pub fn inputs(&mut self, inputs: impl IntoIterator<Item = impl AsRef<Path>>) -> &mut Self {
        for input in inputs {
            self.input(input);
        }
        self
    }

    /// Specify generated code [`Customize`] object.
    pub fn customize(&mut self, customize: Customize) -> &mut Self {
        self.customize = customize;
        self
    }

    /// Like `protoc --rust_out=...` but without requiring `protoc` or `protoc-gen-rust`
    /// commands in `$PATH`.
    pub fn run(&self) -> anyhow::Result<()> {
        let p = pure::parse_and_typecheck(&self.includes, &self.inputs)?;

        protobuf_codegen::gen_and_write(
            &p.file_descriptors,
            &format!("protobuf-codegen-pure={}", env!("CARGO_PKG_VERSION")),
            &p.relative_paths,
            &self.out_dir,
            &self.customize,
        )
    }

    /// Similar to `run`, but prints the message to stderr and exits the process on error.
    pub fn run_from_script(&self) {
        if let Err(e) = self.run() {
            eprintln!("pure rust codegen failed: {}", e);
            process::exit(1);
        }
    }
}

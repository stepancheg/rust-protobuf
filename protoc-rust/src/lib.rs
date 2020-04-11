//! API to generate `.rs` files.
//!
//! This API requires `protoc` command present in `$PATH`
//! or [passed explicitly to `Codegen` object](crate::Codegen::protoc_path).
//!
//! ```no_run
//! extern crate protoc_rust;
//!
//! fn main() {
//!     protoc_rust::Codegen::new()
//!         .out_dir("src/protos")
//!         .inputs(&["protos/a.proto", "protos/b.proto"])
//!         .include("protos")
//!         .run()
//!         .expect("Running protoc failed.");
//! }
//! ```
//!
//! It is advisable that `protoc-rust` build-dependecy version be the same as
//! `protobuf` dependency.
//!
//! The alternative is to use `protobuf-codegen-pure`.

#![deny(missing_docs)]
#![deny(intra_doc_link_resolution_failure)]

extern crate tempfile;

extern crate protobuf;
extern crate protobuf_codegen;
extern crate protoc;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub use protoc::Error;
pub use protoc::Result;

pub use protobuf_codegen::Customize;
use protoc::Protoc;

/// `Protoc --rust_out...` args
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
    /// Protoc command path
    protoc: Option<Protoc>,
}

impl Codegen {
    /// Arguments to the `protoc` found in `$PATH`
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `--LANG_out=...` param
    pub fn out_dir(&mut self, out_dir: impl AsRef<Path>) -> &mut Self {
        self.out_dir = out_dir.as_ref().to_owned();
        self
    }

    /// Append a path to `-I` args
    pub fn include(&mut self, include: impl AsRef<Path>) -> &mut Self {
        self.includes.push(include.as_ref().to_owned());
        self
    }

    /// Append multiple paths to `-I` args
    pub fn includes(&mut self, includes: impl IntoIterator<Item = impl AsRef<Path>>) -> &mut Self {
        for include in includes {
            self.include(include);
        }
        self
    }

    /// Append a `.proto` file path to compile
    pub fn input(&mut self, input: impl AsRef<Path>) -> &mut Self {
        self.inputs.push(input.as_ref().to_owned());
        self
    }

    /// Append multiple `.proto` file paths to compile
    pub fn inputs(&mut self, inputs: impl IntoIterator<Item = impl AsRef<Path>>) -> &mut Self {
        for input in inputs {
            self.input(input);
        }
        self
    }

    /// Specify `protoc` command path to be used when invoking code generation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # mod protoc_bin_vendored {
    /// #   pub fn protoc_bin_path() -> Result<std::path::PathBuf, std::io::Error> {
    /// #       unimplemented!()
    /// #   }
    /// # }
    ///
    /// use protoc_rust::Codegen;
    ///
    /// Codegen::new()
    ///     .protoc_path(protoc_bin_vendored::protoc_bin_path().unwrap())
    ///     // ...
    ///     .run()
    ///     .unwrap();
    /// ```
    pub fn protoc_path(&mut self, protoc: impl Into<PathBuf>) -> &mut Self {
        self.protoc = Some(Protoc::from_path(&protoc.into()));
        self
    }

    /// Set options to customize code generation
    pub fn customize(&mut self, customize: Customize) -> &mut Self {
        self.customize = customize;
        self
    }

    /// Like `protoc --rust_out=...` but without requiring `protoc-gen-rust` command in `$PATH`.
    pub fn run(&self) -> Result<()> {
        let protoc = match self.protoc.clone() {
            Some(protoc) => protoc,
            None => Protoc::from_env_path(),
        };
        protoc.check()?;

        let temp_dir = tempfile::Builder::new().prefix("protoc-rust").tempdir()?;
        let temp_file = temp_dir.path().join("descriptor.pbbin");

        protoc
            .descriptor_set_out_args()
            .out(&temp_file)
            .includes(&self.includes)
            .inputs(&self.inputs)
            .include_imports(true)
            .write_descriptor_set()?;

        let fds = fs::read(temp_file)?;
        drop(temp_dir);

        let fds: protobuf::descriptor::FileDescriptorSet = protobuf::parse_from_bytes(&fds)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let default_includes = vec![PathBuf::from(".")];
        let includes = if self.includes.is_empty() {
            &default_includes
        } else {
            &self.includes
        };

        let mut files_to_generate = Vec::new();
        'outer: for file in &self.inputs {
            for include in includes {
                if let Some(truncated) = remove_path_prefix(file, include) {
                    files_to_generate.push(truncated.to_owned());
                    continue 'outer;
                }
            }

            return Err(Error::new(
                io::ErrorKind::Other,
                format!("file {:?} is not found in includes {:?}", file, includes),
            ));
        }

        protobuf_codegen::gen_and_write(
            &fds.file,
            &format!("protoc {}", protoc.version()?),
            &files_to_generate,
            &self.out_dir,
            &self.customize,
        )
    }
}

fn remove_path_prefix<'a>(mut path: &'a Path, mut prefix: &Path) -> Option<&'a Path> {
    path = path.strip_prefix(".").unwrap_or(path);
    prefix = prefix.strip_prefix(".").unwrap_or(prefix);
    path.strip_prefix(prefix).ok()
}

#[test]
fn test_remove_path_prefix() {
    assert_eq!(
        Some(Path::new("abc.proto")),
        remove_path_prefix(Path::new("xxx/abc.proto"), Path::new("xxx"))
    );
    assert_eq!(
        Some(Path::new("abc.proto")),
        remove_path_prefix(Path::new("xxx/abc.proto"), Path::new("xxx/"))
    );
    assert_eq!(
        Some(Path::new("abc.proto")),
        remove_path_prefix(Path::new("../xxx/abc.proto"), Path::new("../xxx/"))
    );
    assert_eq!(
        Some(Path::new("abc.proto")),
        remove_path_prefix(Path::new("abc.proto"), Path::new("."))
    );
    assert_eq!(
        Some(Path::new("abc.proto")),
        remove_path_prefix(Path::new("abc.proto"), Path::new("./"))
    );
    assert_eq!(
        None,
        remove_path_prefix(Path::new("xxx/abc.proto"), Path::new("yyy"))
    );
    assert_eq!(
        None,
        remove_path_prefix(Path::new("xxx/abc.proto"), Path::new("yyy/"))
    );
}

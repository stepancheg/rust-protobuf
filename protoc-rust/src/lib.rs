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
#![deny(broken_intra_doc_links)]

extern crate tempfile;

extern crate protobuf;
extern crate protobuf_codegen;
extern crate protoc;

mod slashes;
use slashes::Slashes;

use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

pub use protoc::Error;
pub use protoc::Result;

use protobuf::descriptor::FileDescriptorSet;
use protobuf::Message;
pub use protobuf_codegen::Customize;
use protoc::Protoc;

/// `Protoc --rust_out...` args
#[derive(Debug, Default)]
#[deprecated(since = "2.14", note = "Use Codegen instead")]
pub struct Args<'a> {
    /// --lang_out= param
    pub out_dir: &'a str,
    /// -I args
    pub includes: &'a [&'a str],
    /// List of .proto files to compile
    pub input: &'a [&'a str],
    /// Customize code generation
    pub customize: Customize,
}

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
        self.protoc = Some(Protoc::from_path(&protoc.into().to_str().unwrap()));
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

        let includes: Vec<&str> = self.includes.iter().map(|p| p.to_str().unwrap()).collect();
        let inputs: Vec<&str> = self.inputs.iter().map(|p| p.to_str().unwrap()).collect();

        protoc.write_descriptor_set(protoc::DescriptorSetOutArgs {
            out: temp_file.as_os_str().to_str().unwrap(),
            includes: &includes,
            input: &inputs,
            include_imports: true,
        })?;

        let mut fds = Vec::new();
        let mut file = fs::File::open(temp_file)?;
        file.read_to_end(&mut fds)?;

        drop(file);
        drop(temp_dir);

        let fds = FileDescriptorSet::parse_from_bytes(&fds)
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
                if let Some(truncated) =
                    remove_path_prefix(file.to_str().unwrap(), include.to_str().unwrap())
                {
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
            fds.get_file(),
            &files_to_generate,
            &self.out_dir,
            &self.customize,
        )
    }
}

/// Like `protoc --rust_out=...` but without requiring `protoc-gen-rust` command in `$PATH`.
#[deprecated(since = "2.14", note = "Use Codegen instead")]
#[allow(deprecated)]
pub fn run(args: Args) -> Result<()> {
    Codegen::new()
        .out_dir(args.out_dir)
        .includes(args.includes)
        .inputs(args.input)
        .customize(args.customize)
        .run()
}

fn remove_path_prefix(mut path: &str, mut prefix: &str) -> Option<String> {
    let slashes = Slashes::here();
    path = slashes.remove_dot_slashes(path);
    prefix = slashes.remove_dot_slashes(prefix);

    if prefix == "" {
        return Some(path.to_owned());
    }

    let path = slashes.norm_path(path);
    let mut prefix = slashes.norm_path(prefix);

    if prefix.ends_with("/") {
        let l = prefix.len();
        prefix.truncate(l - 1);
    }

    if !path.starts_with(&prefix) {
        return None;
    }

    if path.len() <= prefix.len() {
        return None;
    }

    if path.as_bytes()[prefix.len()] == b'/' {
        return Some(path[prefix.len() + 1..].to_owned());
    } else {
        return None;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn remove_path_prefix() {
        assert_eq!(
            Some("abc.proto".to_owned()),
            super::remove_path_prefix("xxx/abc.proto", "xxx")
        );
        assert_eq!(
            Some("abc.proto".to_owned()),
            super::remove_path_prefix("xxx/abc.proto", "xxx/")
        );
        assert_eq!(
            Some("abc.proto".to_owned()),
            super::remove_path_prefix("../xxx/abc.proto", "../xxx/")
        );
        assert_eq!(
            Some("abc.proto".to_owned()),
            super::remove_path_prefix("abc.proto", ".")
        );
        assert_eq!(
            Some("abc.proto".to_owned()),
            super::remove_path_prefix("abc.proto", "./")
        );
        assert_eq!(None, super::remove_path_prefix("xxx/abc.proto", "yyy"));
        assert_eq!(None, super::remove_path_prefix("xxx/abc.proto", "yyy/"));
    }
}

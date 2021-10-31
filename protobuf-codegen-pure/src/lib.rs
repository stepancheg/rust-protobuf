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

mod convert;

use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::path::StripPrefixError;
use std::process;

mod linked_hash_map;
mod model;
mod parse_dependencies;
mod parser;
mod path;
mod rel_path;

use crate::parser::ParserErrorWithLocation;
use crate::rel_path::RelPath;
use crate::rel_path::RelPathBuf;
use linked_hash_map::LinkedHashMap;
use protobuf::descriptor::FileDescriptorProto;
use protobuf_codegen::amend_io_error;
pub use protobuf_codegen::Customize;

#[cfg(test)]
mod test_against_protobuf_protos;

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
    pub fn run(&self) -> io::Result<()> {
        let p = parse_and_typecheck(&self.includes, &self.inputs)?;

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

#[derive(Clone)]
pub(crate) struct FileDescriptorPair {
    parsed: model::FileDescriptor,
    descriptor: protobuf::descriptor::FileDescriptorProto,
}

#[derive(Debug)]
enum CodegenError {
    ParserErrorWithLocation(parser::ParserErrorWithLocation),
    ConvertError(convert::ConvertError),
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodegenError::ParserErrorWithLocation(e) => write!(f, "{}", e),
            CodegenError::ConvertError(e) => write!(f, "{}", e),
        }
    }
}

impl From<parser::ParserErrorWithLocation> for CodegenError {
    fn from(e: parser::ParserErrorWithLocation) -> Self {
        CodegenError::ParserErrorWithLocation(e)
    }
}

impl From<convert::ConvertError> for CodegenError {
    fn from(e: convert::ConvertError) -> Self {
        CodegenError::ConvertError(e)
    }
}

#[derive(Debug)]
struct WithFileError {
    file: String,
    error: CodegenError,
}

impl fmt::Display for WithFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error in {}: {}", self.file, self.error)
    }
}

impl Error for WithFileError {}

struct Run<'a> {
    parsed_files: LinkedHashMap<PathBuf, FileDescriptorPair>,
    includes: &'a [PathBuf],
}

impl<'a> Run<'a> {
    fn get_file_and_all_deps_already_parsed(
        &self,
        protobuf_path: &Path,
        result: &mut LinkedHashMap<PathBuf, FileDescriptorPair>,
    ) {
        if let Some(_) = result.get(protobuf_path) {
            return;
        }

        let pair = self
            .parsed_files
            .get(protobuf_path)
            .expect("must be already parsed");
        result.insert(protobuf_path.to_owned(), pair.clone());

        self.get_all_deps_already_parsed(&pair.parsed, result);
    }

    fn get_all_deps_already_parsed(
        &self,
        parsed: &model::FileDescriptor,
        result: &mut LinkedHashMap<PathBuf, FileDescriptorPair>,
    ) {
        for import in &parsed.imports {
            self.get_file_and_all_deps_already_parsed(Path::new(&import.path), result);
        }
    }

    fn add_file(&mut self, protobuf_path: &Path, fs_path: &Path) -> io::Result<()> {
        if let Some(_) = self.parsed_files.get(protobuf_path) {
            return Ok(());
        }

        let content = fs::read_to_string(fs_path)
            .map_err(|e| amend_io_error(e, format!("failed to read {:?}", fs_path)))?;

        self.add_file_content(protobuf_path, fs_path, &content)
    }

    fn add_file_content(
        &mut self,
        protobuf_path: &Path,
        fs_path: &Path,
        content: &str,
    ) -> io::Result<()> {
        let parsed = model::FileDescriptor::parse(content).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                WithFileError {
                    file: format!("{}", fs_path.display()),
                    error: e.into(),
                },
            )
        })?;

        for import in &parsed.imports {
            self.add_imported_file(Path::new(&import.path))?;
        }

        let mut this_file_deps = LinkedHashMap::new();
        self.get_all_deps_already_parsed(&parsed, &mut this_file_deps);

        let this_file_deps: Vec<_> = this_file_deps.into_iter().map(|(_, v)| v).collect();

        let descriptor = convert::file_descriptor(protobuf_path, &parsed, &this_file_deps)
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    WithFileError {
                        file: format!("{}", fs_path.display()),
                        error: e.into(),
                    },
                )
            })?;

        self.parsed_files.insert(
            protobuf_path.to_owned(),
            FileDescriptorPair { parsed, descriptor },
        );

        Ok(())
    }

    fn add_imported_file(&mut self, protobuf_path: &Path) -> io::Result<()> {
        for include_dir in self.includes {
            let fs_path = include_dir.join(protobuf_path);
            if fs_path.exists() {
                return self.add_file(protobuf_path, &fs_path);
            }
        }

        let embedded = match protobuf_path.to_str() {
            Some("rustproto.proto") => Some(RUSTPROTO_PROTO),
            Some("google/protobuf/any.proto") => Some(ANY_PROTO),
            Some("google/protobuf/api.proto") => Some(API_PROTO),
            Some("google/protobuf/descriptor.proto") => Some(DESCRIPTOR_PROTO),
            Some("google/protobuf/duration.proto") => Some(DURATION_PROTO),
            Some("google/protobuf/empty.proto") => Some(EMPTY_PROTO),
            Some("google/protobuf/field_mask.proto") => Some(FIELD_MASK_PROTO),
            Some("google/protobuf/source_context.proto") => Some(SOURCE_CONTEXT_PROTO),
            Some("google/protobuf/struct.proto") => Some(STRUCT_PROTO),
            Some("google/protobuf/timestamp.proto") => Some(TIMESTAMP_PROTO),
            Some("google/protobuf/type.proto") => Some(TYPE_PROTO),
            Some("google/protobuf/wrappers.proto") => Some(WRAPPERS_PROTO),
            _ => None,
        };

        match embedded {
            Some(content) => self.add_file_content(protobuf_path, protobuf_path, content),
            None => Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "protobuf path {:?} is not found in import path {:?}",
                    protobuf_path, self.includes
                ),
            )),
        }
    }

    fn strip_prefix<'b>(path: &'b Path, prefix: &Path) -> Result<&'b Path, StripPrefixError> {
        // special handling of `.` to allow successful `strip_prefix("foo.proto", ".")
        if prefix == Path::new(".") {
            Ok(path)
        } else {
            path.strip_prefix(prefix)
        }
    }

    fn add_fs_file(&mut self, fs_path: &Path) -> io::Result<RelPathBuf> {
        let relative_path = self
            .includes
            .iter()
            .filter_map(|include_dir| Self::strip_prefix(fs_path, include_dir).ok())
            .next();

        match relative_path {
            Some(relative_path) => {
                let relative_path = RelPath::new(relative_path);
                self.add_file(relative_path, fs_path)?;
                Ok(relative_path.to_owned())
            }
            None => Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "file {:?} must reside in include path {:?}",
                    fs_path, self.includes
                ),
            )),
        }
    }
}

/// Result of parsing `.proto` files.
#[doc(hidden)]
pub struct ParsedAndTypechecked {
    /// One entry for each input `.proto` file.
    pub relative_paths: Vec<PathBuf>,
    /// All parsed `.proto` files including dependencies of input files.
    pub file_descriptors: Vec<protobuf::descriptor::FileDescriptorProto>,
}

#[doc(hidden)]
pub fn parse_and_typecheck(
    includes: &[PathBuf],
    input: &[PathBuf],
) -> io::Result<ParsedAndTypechecked> {
    let mut run = Run {
        parsed_files: LinkedHashMap::new(),
        includes,
    };

    let mut relative_paths = Vec::new();

    for input in input {
        relative_paths.push(run.add_fs_file(input)?.into_path_buf());
    }

    let file_descriptors: Vec<_> = run
        .parsed_files
        .into_iter()
        .map(|(_, v)| v.descriptor)
        .collect();

    Ok(ParsedAndTypechecked {
        relative_paths,
        file_descriptors,
    })
}

/// Parse imports from a `.proto` file.
///
/// The result is [`FileDescriptorProto`] object with only `*dependency` fields filled.
#[doc(hidden)]
pub fn parse_dependencies(content: &str) -> Result<FileDescriptorProto, ParserErrorWithLocation> {
    parse_dependencies::parse_dependencies(content)
}

// TODO: these include don't work when publishing to crates
const RUSTPROTO_PROTO: &str = include_str!("proto/rustproto.proto");
const ANY_PROTO: &str = include_str!("proto/google/protobuf/any.proto");
const API_PROTO: &str = include_str!("proto/google/protobuf/api.proto");
const DESCRIPTOR_PROTO: &str = include_str!("proto/google/protobuf/descriptor.proto");
const DURATION_PROTO: &str = include_str!("proto/google/protobuf/duration.proto");
const EMPTY_PROTO: &str = include_str!("proto/google/protobuf/empty.proto");
const FIELD_MASK_PROTO: &str = include_str!("proto/google/protobuf/field_mask.proto");
const SOURCE_CONTEXT_PROTO: &str = include_str!("proto/google/protobuf/source_context.proto");
const STRUCT_PROTO: &str = include_str!("proto/google/protobuf/struct.proto");
const TIMESTAMP_PROTO: &str = include_str!("proto/google/protobuf/timestamp.proto");
const TYPE_PROTO: &str = include_str!("proto/google/protobuf/type.proto");
const WRAPPERS_PROTO: &str = include_str!("proto/google/protobuf/wrappers.proto");

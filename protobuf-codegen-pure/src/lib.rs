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

use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::str;

mod linked_hash_map;
mod model;
mod parse_dependencies;
mod parser;
mod path;
mod rel_path;

use linked_hash_map::LinkedHashMap;
use protobuf::descriptor::FileDescriptorProto;
pub use protobuf_codegen::Customize;
use protobuf_codegen::ProtoPath;
use protobuf_codegen::ProtoPathBuf;

use crate::parser::ParserErrorWithLocation;

mod convert;
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
    pub fn run(&self) -> anyhow::Result<()> {
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

#[derive(Debug, thiserror::Error)]
#[error("error in `{file}`: {error}")]
struct WithFileError {
    file: String,
    #[source]
    error: anyhow::Error,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("protobuf path `{0}` is not found in import path {1}")]
    FileNotFoundInImportPath(String, String),
    #[error("file `{0}` must reside in include path {1}")]
    FileMustResideInImportPath(String, String),
    #[error("could not read file `{0}`: {1}")]
    CouldNotReadFile(String, io::Error),
    #[error("file `{0}` content is not UTF-8")]
    FileContentIsNotUtf8(String),
}

/// Resolve `.proto` files. `Display` is used for error messages.
trait ProtoPathResolver: fmt::Display {
    fn resolve(&self, path: &ProtoPath) -> anyhow::Result<Option<ResolvedProtoFile>>;
}

struct Run<R>
where
    R: ProtoPathResolver,
{
    parsed_files: LinkedHashMap<ProtoPathBuf, FileDescriptorPair>,
    resolver: R,
}

impl<R> Run<R>
where
    R: ProtoPathResolver,
{
    fn get_file_and_all_deps_already_parsed(
        &self,
        protobuf_path: &ProtoPath,
        result: &mut LinkedHashMap<ProtoPathBuf, FileDescriptorPair>,
    ) {
        if let Some(_) = result.get(protobuf_path) {
            return;
        }

        let pair = self
            .parsed_files
            .get(protobuf_path)
            .expect("must be already parsed");
        result.insert(protobuf_path.to_proto_path_buf(), pair.clone());

        self.get_all_deps_already_parsed(&pair.parsed, result);
    }

    fn get_all_deps_already_parsed(
        &self,
        parsed: &model::FileDescriptor,
        result: &mut LinkedHashMap<ProtoPathBuf, FileDescriptorPair>,
    ) {
        for import in &parsed.imports {
            self.get_file_and_all_deps_already_parsed(&import.path, result);
        }
    }

    fn add_file_content(
        &mut self,
        protobuf_path: &ProtoPath,
        resolved: &ResolvedProtoFile,
    ) -> anyhow::Result<()> {
        let content = str::from_utf8(&resolved.content)
            .map_err(|_| Error::FileContentIsNotUtf8(protobuf_path.to_string()))?;

        let parsed = model::FileDescriptor::parse(&content).map_err(|e| WithFileError {
            file: resolved.path.clone(),
            error: e.into(),
        })?;

        for import in &parsed.imports {
            self.add_imported_file(&import.path)?;
        }

        let mut this_file_deps = LinkedHashMap::new();
        self.get_all_deps_already_parsed(&parsed, &mut this_file_deps);

        let this_file_deps: Vec<_> = this_file_deps.into_iter().map(|(_, v)| v).collect();

        let descriptor = convert::file_descriptor(protobuf_path, &parsed, &this_file_deps)
            .map_err(|e| WithFileError {
                file: resolved.path.clone(),
                error: e.into(),
            })?;

        self.parsed_files.insert(
            protobuf_path.to_proto_path_buf(),
            FileDescriptorPair { parsed, descriptor },
        );

        Ok(())
    }

    fn add_imported_file(&mut self, protobuf_path: &ProtoPath) -> anyhow::Result<()> {
        if let Some(_) = self.parsed_files.get(protobuf_path) {
            return Ok(());
        }

        let resolved = self.resolver.resolve(protobuf_path)?;
        if let Some(resolved) = resolved {
            return self.add_file_content(protobuf_path, &resolved);
        }

        let embedded = match protobuf_path.to_str() {
            "rustproto.proto" => Some(RUSTPROTO_PROTO),
            "google/protobuf/any.proto" => Some(ANY_PROTO),
            "google/protobuf/api.proto" => Some(API_PROTO),
            "google/protobuf/descriptor.proto" => Some(DESCRIPTOR_PROTO),
            "google/protobuf/duration.proto" => Some(DURATION_PROTO),
            "google/protobuf/empty.proto" => Some(EMPTY_PROTO),
            "google/protobuf/field_mask.proto" => Some(FIELD_MASK_PROTO),
            "google/protobuf/source_context.proto" => Some(SOURCE_CONTEXT_PROTO),
            "google/protobuf/struct.proto" => Some(STRUCT_PROTO),
            "google/protobuf/timestamp.proto" => Some(TIMESTAMP_PROTO),
            "google/protobuf/type.proto" => Some(TYPE_PROTO),
            "google/protobuf/wrappers.proto" => Some(WRAPPERS_PROTO),
            _ => None,
        };

        match embedded {
            Some(content) => self.add_file_content(
                protobuf_path,
                &ResolvedProtoFile {
                    path: protobuf_path.to_string(),
                    content: content.as_bytes().to_vec(),
                },
            ),
            None => Err(Error::FileNotFoundInImportPath(
                protobuf_path.to_string(),
                format!("{}", self.resolver),
            )
            .into()),
        }
    }
}

/// Result of parsing `.proto` files.
#[doc(hidden)]
pub struct ParsedAndTypechecked {
    /// One entry for each input `.proto` file.
    pub relative_paths: Vec<ProtoPathBuf>,
    /// All parsed `.proto` files including dependencies of input files.
    pub file_descriptors: Vec<protobuf::descriptor::FileDescriptorProto>,
}

fn path_to_proto_path(path: &Path, includes: &[PathBuf]) -> anyhow::Result<ProtoPathBuf> {
    for include in includes {
        if include == Path::new(".") && path.is_relative() {
            // Special handling of `.` to allow using `.` as an include path
            // and `foo.proto` as input.
            return ProtoPathBuf::from_path(path);
        }
        match path.strip_prefix(include) {
            Ok(stripped) => return ProtoPathBuf::from_path(stripped),
            Err(_) => continue,
        }
    }
    Err(
        Error::FileMustResideInImportPath(path.display().to_string(), format!("{:?}", includes))
            .into(),
    )
}

struct ResolvedProtoFile {
    /// For error reporting.
    pub path: String,
    /// File content.
    pub content: Vec<u8>,
}

fn fs_resolver(includes: &[PathBuf]) -> impl ProtoPathResolver {
    struct Impl {
        includes: Vec<PathBuf>,
    }

    impl fmt::Display for Impl {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.includes)
        }
    }

    impl ProtoPathResolver for Impl {
        fn resolve(&self, proto_path: &ProtoPath) -> anyhow::Result<Option<ResolvedProtoFile>> {
            for include_dir in &self.includes {
                let fs_path = include_dir.join(proto_path.to_path());
                match fs::read_to_string(&fs_path) {
                    Ok(content) => {
                        return Ok(Some(ResolvedProtoFile {
                            path: fs_path.display().to_string(),
                            content: content.into_bytes(),
                        }))
                    }
                    Err(e) if e.kind() == io::ErrorKind::NotFound => continue,
                    Err(e) => {
                        return Err(Error::CouldNotReadFile(fs_path.display().to_string(), e).into())
                    }
                }
            }
            Ok(None)
        }
    }

    Impl {
        includes: includes.to_vec(),
    }
}

#[doc(hidden)]
pub fn parse_and_typecheck(
    includes: &[PathBuf],
    input: &[PathBuf],
) -> anyhow::Result<ParsedAndTypechecked> {
    let mut run = Run {
        parsed_files: LinkedHashMap::new(),
        resolver: fs_resolver(includes),
    };

    let relative_paths = input
        .iter()
        .map(|input| path_to_proto_path(input, includes))
        .collect::<anyhow::Result<Vec<_>>>()?;

    for input in &relative_paths {
        run.add_imported_file(&input)?;
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

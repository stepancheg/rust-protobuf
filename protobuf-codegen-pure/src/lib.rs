extern crate protobuf;
extern crate protobuf_codegen;

mod convert;

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

mod model;
mod parser;

use protobuf_codegen::amend_io_error;
pub use protobuf_codegen::Customize;

#[cfg(test)]
mod test_against_protobuf_protos;

// TODO: merge with protoc-rust def
#[derive(Debug, Default)]
pub struct Args {
    /// --lang_out= param
    out_dir: PathBuf,
    /// -I args
    includes: Vec<PathBuf>,
    /// List of .proto files to compile
    inputs: Vec<PathBuf>,
    /// Customize code generation
    customize: Customize,
}

impl Args {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn out_dir(&mut self, out_dir: impl AsRef<Path>) -> &mut Self {
        self.out_dir = out_dir.as_ref().to_owned();
        self
    }

    pub fn include(&mut self, include: impl AsRef<Path>) -> &mut Self {
        self.includes.push(include.as_ref().to_owned());
        self
    }

    pub fn includes(&mut self, includes: impl IntoIterator<Item = impl AsRef<Path>>) -> &mut Self {
        for include in includes {
            self.include(include);
        }
        self
    }

    pub fn input(&mut self, input: impl AsRef<Path>) -> &mut Self {
        self.inputs.push(input.as_ref().to_owned());
        self
    }

    pub fn inputs(&mut self, inputs: impl IntoIterator<Item = impl AsRef<Path>>) -> &mut Self {
        for input in inputs {
            self.input(input);
        }
        self
    }

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
            &p.relative_paths,
            &self.out_dir,
            &self.customize,
        )
    }
}

#[derive(Clone)]
struct FileDescriptorPair {
    parsed: model::FileDescriptor,
    descriptor: protobuf::descriptor::FileDescriptorProto,
}

#[derive(Debug)]
enum CodegenError {
    ParserErrorWithLocation(parser::ParserErrorWithLocation),
    ConvertError(convert::ConvertError),
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
        write!(f, "WithFileError")
    }
}

impl Error for WithFileError {
    fn description(&self) -> &str {
        "WithFileError"
    }
}

struct Run<'a> {
    parsed_files: HashMap<PathBuf, FileDescriptorPair>,
    includes: &'a [PathBuf],
}

impl<'a> Run<'a> {
    fn get_file_and_all_deps_already_parsed(
        &self,
        protobuf_path: &Path,
        result: &mut HashMap<PathBuf, FileDescriptorPair>,
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
        result: &mut HashMap<PathBuf, FileDescriptorPair>,
    ) {
        for import in &parsed.import_paths {
            self.get_file_and_all_deps_already_parsed(Path::new(import), result);
        }
    }

    fn add_file(&mut self, protobuf_path: &Path, fs_path: &Path) -> io::Result<()> {
        if let Some(_) = self.parsed_files.get(protobuf_path) {
            return Ok(());
        }

        let content = fs::read_to_string(fs_path)
            .map_err(|e| amend_io_error(e, format!("failed to read {:?}", fs_path)))?;

        let parsed = model::FileDescriptor::parse(content).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                WithFileError {
                    file: format!("{}", fs_path.display()),
                    error: e.into(),
                },
            )
        })?;

        for import_path in &parsed.import_paths {
            self.add_imported_file(Path::new(import_path))?;
        }

        let mut this_file_deps = HashMap::new();
        self.get_all_deps_already_parsed(&parsed, &mut this_file_deps);

        let this_file_deps: Vec<_> = this_file_deps.into_iter().map(|(_, v)| v.parsed).collect();

        let descriptor =
            convert::file_descriptor(protobuf_path, &parsed, &this_file_deps).map_err(
                |e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        WithFileError {
                            file: format!("{}", fs_path.display()),
                            error: e.into(),
                        },
                    )
                },
            )?;

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

        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "protobuf path {:?} is not found in import path {:?}",
                protobuf_path, self.includes
            ),
        ))
    }

    fn add_fs_file(&mut self, fs_path: &Path) -> io::Result<PathBuf> {
        let relative_path = self
            .includes
            .iter()
            .filter_map(|include_dir| fs_path.strip_prefix(include_dir).ok())
            .next();

        match relative_path {
            Some(relative_path) => {
                assert!(relative_path.is_relative());
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

#[doc(hidden)]
pub struct ParsedAndTypechecked {
    pub relative_paths: Vec<PathBuf>,
    pub file_descriptors: Vec<protobuf::descriptor::FileDescriptorProto>,
}

#[doc(hidden)]
pub fn parse_and_typecheck(includes: &[PathBuf], input: &[PathBuf]) -> io::Result<ParsedAndTypechecked> {
    let mut run = Run {
        parsed_files: HashMap::new(),
        includes: includes,
    };

    let mut relative_paths = Vec::new();

    for input in input {
        relative_paths.push(run.add_fs_file(input)?);
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

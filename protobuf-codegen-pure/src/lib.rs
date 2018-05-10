extern crate protobuf;
extern crate protobuf_codegen;

mod convert;

use std::collections::HashMap;
use std::path::Path;
use std::io;
use std::io::Read;
use std::fs;
use std::error::Error;
use std::fmt;

mod model;
mod parser;
mod str_lit;

pub use protobuf_codegen::Customize;

#[cfg(test)]
mod test_against_protobuf_protos;


// TODO: merge with protoc-rust def
#[derive(Debug, Default)]
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

/// Convert OS path to protobuf path (with slashes)
/// Function is `pub(crate)` for test.
pub(crate) fn relative_path_to_protobuf_path(path: &Path) -> String {
    assert!(path.is_relative());
    let path = path.to_str().expect("not a valid UTF-8 name");
    if cfg!(windows) {
        path.replace('\\', "/")
    } else {
        path.to_owned()
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
    parsed_files: HashMap<String, FileDescriptorPair>,
    includes: &'a [&'a str],
}

impl<'a> Run<'a> {
    fn get_file_and_all_deps_already_parsed(
        &self, protobuf_path: &str, result: &mut HashMap<String, FileDescriptorPair>)
    {
        if let Some(_) = result.get(protobuf_path) {
            return;
        }

        let pair = self.parsed_files.get(protobuf_path).expect("must be already parsed");
        result.insert(protobuf_path.to_owned(), pair.clone());

        self.get_all_deps_already_parsed(&pair.parsed, result);
    }

    fn get_all_deps_already_parsed(
        &self,
        parsed: &model::FileDescriptor,
        result: &mut HashMap<String, FileDescriptorPair>)
    {
        for import in &parsed.import_paths {
            self.get_file_and_all_deps_already_parsed(import, result);
        }
    }

    fn add_file(&mut self, protobuf_path: &str, fs_path: &Path) -> io::Result<()> {
        if let Some(_) = self.parsed_files.get(protobuf_path) {
            return Ok(());
        }

        let mut content = String::new();
        fs::File::open(fs_path)?.read_to_string(&mut content)?;

        let parsed = model::FileDescriptor::parse(content)
            .map_err(|e| {
                io::Error::new(io::ErrorKind::Other,
                    WithFileError {
                        file: format!("{}", fs_path.display()),
                        error: e.into(),
                    })
            })?;

        for import_path in &parsed.import_paths {
            self.add_imported_file(import_path)?;
        }

        let mut this_file_deps = HashMap::new();
        self.get_all_deps_already_parsed(&parsed, &mut this_file_deps);

        let this_file_deps: Vec<_> = this_file_deps.into_iter().map(|(_, v)| v.parsed).collect();

        let descriptor = convert::file_descriptor(
            protobuf_path.to_owned(), &parsed, &this_file_deps)
            .map_err(|e| {
                io::Error::new(io::ErrorKind::Other,
                    WithFileError {
                        file: format!("{}", fs_path.display()),
                        error: e.into(),
                    })
            })?;

        self.parsed_files.insert(
            protobuf_path.to_owned(), FileDescriptorPair { parsed, descriptor });

        Ok(())
    }

    fn add_imported_file(&mut self, protobuf_path: &str) -> io::Result<()> {
        for include_dir in self.includes {
            let fs_path = Path::new(include_dir).join(protobuf_path);
            if fs_path.exists() {
                return self.add_file(protobuf_path, &fs_path)
            }
        }

        Err(io::Error::new(io::ErrorKind::Other,
             format!("protobuf path {:?} is not found in import path {:?}",
                 protobuf_path, self.includes)))
    }

    fn add_fs_file(&mut self, fs_path: &Path) -> io::Result<String> {
        let relative_path = self.includes.iter()
            .filter_map(|include_dir| fs_path.strip_prefix(include_dir).ok())
            .next();

        match relative_path {
            Some(relative_path) => {
                let protobuf_path = relative_path_to_protobuf_path(relative_path);
                self.add_file(&protobuf_path, fs_path)?;
                Ok(protobuf_path)
            }
            None => {
                Err(io::Error::new(io::ErrorKind::Other,
                    format!("file {:?} must reside in include path {:?}",
                        fs_path, self.includes)))
            }
        }
    }
}

#[doc(hidden)]
pub struct ParsedAndTypechecked {
    pub relative_paths: Vec<String>,
    pub file_descriptors: Vec<protobuf::descriptor::FileDescriptorProto>,
}

#[doc(hidden)]
pub fn parse_and_typecheck(includes: &[&str], input: &[&str]) -> io::Result<ParsedAndTypechecked> {
    let mut run = Run {
        parsed_files: HashMap::new(),
        includes: includes,
    };

    let mut relative_paths = Vec::new();

    for input in input {
        relative_paths.push(run.add_fs_file(&Path::new(input))?);
    }

    let file_descriptors: Vec<_> =
        run.parsed_files.into_iter().map(|(_, v)| v.descriptor).collect();

    Ok(ParsedAndTypechecked {
        relative_paths,
        file_descriptors,
    })
}

/// Like `protoc --rust_out=...` but without requiring `protoc` or `protoc-gen-rust`
/// commands in `$PATH`.
pub fn run(args: Args) -> io::Result<()> {
    let p = parse_and_typecheck(args.includes, args.input)?;

    protobuf_codegen::gen_and_write(
        &p.file_descriptors,
        &p.relative_paths,
        &Path::new(&args.out_dir),
        &args.customize)
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn test_relative_path_to_protobuf_path_windows() {
        assert_eq!("foo/bar.proto", relative_path_to_protobuf_path(&Path::new("foo\\bar.proto")));
    }

    #[test]
    fn test_relative_path_to_protobuf_path() {
        assert_eq!("foo/bar.proto", relative_path_to_protobuf_path(&Path::new("foo/bar.proto")));
    }
}

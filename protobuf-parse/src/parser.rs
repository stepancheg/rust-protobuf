use std::collections::HashSet;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

use protobuf::descriptor::FileDescriptorSet;

use crate::protoc;
use crate::pure;
use crate::which_parser::WhichParser;
use crate::ParsedAndTypechecked;

/// Configure and invoke `.proto` parser.
#[derive(Default, Debug)]
pub struct Parser {
    which_parser: WhichParser,
    pub(crate) includes: Vec<PathBuf>,
    pub(crate) inputs: Vec<PathBuf>,
    pub(crate) protoc: Option<PathBuf>,
    pub(crate) protoc_extra_args: Vec<OsString>,
}

impl Parser {
    /// Create new default configured parser.
    pub fn new() -> Parser {
        Parser::default()
    }

    /// Use pure rust parser.
    pub fn pure(&mut self) -> &mut Self {
        self.which_parser = WhichParser::Pure;
        self
    }

    /// Use `protoc` for parsing.
    pub fn protoc(&mut self) -> &mut Self {
        self.which_parser = WhichParser::Protoc;
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

    /// Specify `protoc` path used for parsing. Ignored if `protoc` parser is used.
    pub fn protoc_path(&mut self, protoc: &Path) -> &mut Self {
        self.protoc = Some(protoc.to_owned());
        self
    }

    /// Extra arguments to pass to `protoc` command (like experimental options).
    pub fn protoc_extra_args(
        &mut self,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> &mut Self {
        self.protoc_extra_args = args.into_iter().map(|s| s.as_ref().to_owned()).collect();
        self
    }

    /// Parse `.proto` files and typecheck them using pure Rust parser of `protoc` command.
    pub fn parse_and_typecheck(&self) -> anyhow::Result<ParsedAndTypechecked> {
        match &self.which_parser {
            WhichParser::Pure => pure::parse_and_typecheck::parse_and_typecheck(&self),
            WhichParser::Protoc => protoc::parse_and_typecheck::parse_and_typecheck(&self),
        }
    }

    /// Parse and convert result to `FileDescriptorSet`.
    pub fn file_descriptor_set(&self) -> anyhow::Result<FileDescriptorSet> {
        let mut generated = self.parse_and_typecheck()?;
        let relative_paths: HashSet<_> = generated
            .relative_paths
            .iter()
            .map(|path| path.to_string())
            .collect();
        generated
            .file_descriptors
            .retain(|fd| relative_paths.contains(fd.get_name()));
        let mut fds = FileDescriptorSet::new();
        fds.file = generated.file_descriptors;
        Ok(fds)
    }
}

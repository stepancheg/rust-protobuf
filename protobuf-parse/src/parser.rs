use std::path::Path;
use std::path::PathBuf;

use crate::parse_and_typecheck;
use crate::which_parser::WhichParser;
use crate::ParsedAndTypechecked;

/// Configure and invoke `.proto` parser.
#[derive(Default, Debug)]
pub struct Parser {
    which_parser: WhichParser,
    includes: Vec<PathBuf>,
    inputs: Vec<PathBuf>,
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

    /// Do the parsing.
    pub fn parse_and_typecheck(&self) -> anyhow::Result<ParsedAndTypechecked> {
        parse_and_typecheck(self.which_parser, &self.includes, &self.inputs)
    }
}

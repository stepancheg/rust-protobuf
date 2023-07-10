use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::process;

use anyhow::Context;
use protobuf_parse::Parser;

use crate::customize::CustomizeCallback;
use crate::customize::CustomizeCallbackHolder;
use crate::gen_and_write::gen_and_write;
use crate::Customize;

#[derive(Debug)]
enum WhichParser {
    Pure,
    Protoc,
}

impl Default for WhichParser {
    fn default() -> WhichParser {
        WhichParser::Pure
    }
}

#[derive(Debug, thiserror::Error)]
enum CodegenError {
    #[error("out_dir is not specified")]
    OutDirNotSpecified,
}

/// Entry point for `.proto` to `.rs` code generation.
///
/// This is similar to `protoc --rust_out...`.
#[derive(Debug, Default)]
pub struct Codegen {
    /// What parser to use to parse `.proto` files.
    which_parser: Option<WhichParser>,
    /// Create out directory.
    create_out_dir: bool,
    /// --lang_out= param
    out_dir: Option<PathBuf>,
    /// -I args
    includes: Vec<PathBuf>,
    /// List of .proto files to compile
    inputs: Vec<PathBuf>,
    /// Customize code generation
    customize: Customize,
    /// Customize code generation
    customize_callback: CustomizeCallbackHolder,
    /// Protoc command path
    protoc: Option<PathBuf>,
    /// Extra `protoc` args
    protoc_extra_args: Vec<OsString>,
    /// Capture stderr when running `protoc`.
    capture_stderr: bool,
}

impl Codegen {
    /// Create new codegen object.
    ///
    /// Uses `protoc` from `$PATH` by default.
    ///
    /// Can be switched to pure rust parser using [`pure`](Self::pure) function.
    pub fn new() -> Self {
        Self::default()
    }

    /// Switch to pure Rust parser of `.proto` files.
    pub fn pure(&mut self) -> &mut Self {
        self.which_parser = Some(WhichParser::Pure);
        self
    }

    /// Switch to `protoc` parser of `.proto` files.
    pub fn protoc(&mut self) -> &mut Self {
        self.which_parser = Some(WhichParser::Protoc);
        self
    }

    /// Output directory for generated code.
    ///
    /// When invoking from `build.rs`, consider using
    /// [`cargo_out_dir`](Self::cargo_out_dir) instead.
    pub fn out_dir(&mut self, out_dir: impl AsRef<Path>) -> &mut Self {
        self.out_dir = Some(out_dir.as_ref().to_owned());
        self
    }

    /// Set output directory relative to Cargo output dir.
    ///
    /// With this option, output directory is erased and recreated during invocation.
    pub fn cargo_out_dir(&mut self, rel: &str) -> &mut Self {
        let rel = Path::new(rel);
        let mut not_empty = false;
        for comp in rel.components() {
            match comp {
                Component::ParentDir => {
                    panic!("parent path in components of rel path: `{}`", rel.display());
                }
                Component::CurDir => {
                    continue;
                }
                Component::Normal(..) => {}
                Component::RootDir | Component::Prefix(..) => {
                    panic!("root dir in components of rel path: `{}`", rel.display());
                }
            }
            not_empty = true;
        }

        if !not_empty {
            panic!("empty rel path: `{}`", rel.display());
        }

        let cargo_out_dir = env::var("OUT_DIR").expect("OUT_DIR env var not set");
        let mut path = PathBuf::from(cargo_out_dir);
        path.push(rel);
        self.create_out_dir = true;
        self.out_dir(path)
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
    /// use protobuf_codegen::Codegen;
    ///
    /// Codegen::new()
    ///     .protoc()
    ///     .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
    ///     // ...
    ///     .run()
    ///     .unwrap();
    /// ```
    ///
    /// This option is ignored when pure Rust parser is used.
    pub fn protoc_path(&mut self, protoc: &Path) -> &mut Self {
        self.protoc = Some(protoc.to_owned());
        self
    }

    /// Capture stderr to error when running `protoc`.
    pub fn capture_stderr(&mut self) -> &mut Self {
        self.capture_stderr = true;
        self
    }

    /// Extra command line flags for `protoc` invocation.
    ///
    /// For example, `--experimental_allow_proto3_optional` option.
    ///
    /// This option is ignored when pure Rust parser is used.
    pub fn protoc_extra_arg(&mut self, arg: impl Into<OsString>) -> &mut Self {
        self.protoc_extra_args.push(arg.into());
        self
    }

    /// Set options to customize code generation
    pub fn customize(&mut self, customize: Customize) -> &mut Self {
        self.customize.update_with(&customize);
        self
    }

    /// Callback for dynamic per-element customization.
    pub fn customize_callback(&mut self, callback: impl CustomizeCallback) -> &mut Self {
        self.customize_callback = CustomizeCallbackHolder::new(callback);
        self
    }

    /// Invoke the code generation.
    ///
    /// This is roughly equivalent to `protoc --rust_out=...` but
    /// without requiring `protoc-gen-rust` command in `$PATH`.
    ///
    /// This function uses pure Rust parser or `protoc` parser depending on
    /// how this object was configured.
    pub fn run(&self) -> anyhow::Result<()> {
        let out_dir = match &self.out_dir {
            Some(out_dir) => out_dir,
            None => return Err(CodegenError::OutDirNotSpecified.into()),
        };

        if self.create_out_dir {
            if out_dir.exists() {
                fs::remove_dir_all(&out_dir)?;
            }
            fs::create_dir(&out_dir)?;
        }

        let mut parser = Parser::new();
        parser.protoc();
        if let Some(protoc) = &self.protoc {
            parser.protoc_path(protoc);
        }
        match &self.which_parser {
            Some(WhichParser::Protoc) => {
                parser.protoc();
            }
            Some(WhichParser::Pure) => {
                parser.pure();
            }
            None => {}
        }

        parser.inputs(&self.inputs);
        parser.includes(&self.includes);
        parser.protoc_extra_args(&self.protoc_extra_args);

        if self.capture_stderr {
            parser.capture_stderr();
        }

        let parsed_and_typechecked = parser
            .parse_and_typecheck()
            .context("parse and typecheck")?;

        gen_and_write(
            &parsed_and_typechecked.file_descriptors,
            &parsed_and_typechecked.parser,
            &parsed_and_typechecked.relative_paths,
            &out_dir,
            &self.customize,
            &*self.customize_callback,
        )
    }

    /// Similar to `run`, but prints the message to stderr and exits the process on error.
    pub fn run_from_script(&self) {
        if let Err(e) = self.run() {
            eprintln!("codegen failed: {:?}", e);
            process::exit(1);
        }
    }
}

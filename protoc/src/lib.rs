//! # API to invoke `protoc` command programmatically
//!
//! API to invoke `protoc` command using API (e. g. from `build.rs`).
//!
//! Note that to generate `rust` code from `.proto`,
//! [protoc-rust](https://docs.rs/protoc-rust) crate can be used,
//! which does not require `protoc-gen-rust` present in `$PATH`.

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

use std::ffi::OsStr;
use std::ffi::OsString;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process;

#[macro_use]
extern crate log;

/// Alias for io::Error
pub type Error = io::Error;
/// Alias for io::Error
pub type Result<T> = io::Result<T>;

fn err_other(s: impl AsRef<str>) -> Error {
    Error::new(io::ErrorKind::Other, s.as_ref().to_owned())
}

/// `protoc --lang_out=... ...` command builder and spawner.
///
/// # Examples
///
/// ```no_run
/// use protoc::ProtocLangOut;
/// ProtocLangOut::new()
///     .lang("go")
///     .include("protos")
///     .include("more-protos")
///     .out_dir("generated-protos")
///     .run()
///     .unwrap();
/// ```
#[derive(Default)]
pub struct ProtocLangOut {
    protoc: Option<Protoc>,
    /// `LANG` part in `--LANG_out=...`
    lang: Option<String>,
    /// `--LANG_out=...` param
    out_dir: Option<PathBuf>,
    /// `--plugin` param. Not needed if plugin is in `$PATH`
    plugin: Option<OsString>,
    /// `-I` args
    includes: Vec<PathBuf>,
    /// List of `.proto` files to compile
    inputs: Vec<PathBuf>,
}

impl ProtocLangOut {
    /// Arguments to the `protoc` found in `$PATH`
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `LANG` part in `--LANG_out=...`
    pub fn lang(&mut self, lang: &str) -> &mut Self {
        self.lang = Some(lang.to_owned());
        self
    }

    /// Set `--LANG_out=...` param
    pub fn out_dir(&mut self, out_dir: impl AsRef<Path>) -> &mut Self {
        self.out_dir = Some(out_dir.as_ref().to_owned());
        self
    }

    /// Set `--plugin` param. Not needed if plugin is in `$PATH`
    pub fn plugin(&mut self, plugin: impl AsRef<OsStr>) -> &mut Self {
        self.plugin = Some(plugin.as_ref().to_owned());
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

    /// Execute `protoc` with given args
    pub fn run(&self) -> Result<()> {
        let protoc = match &self.protoc {
            Some(protoc) => protoc.clone(),
            None => {
                let protoc = Protoc::from_env_path();
                // Check with have good `protoc`
                protoc.check()?;
                protoc
            }
        };

        if self.inputs.is_empty() {
            return Err(err_other("input is empty"));
        }

        let out_dir = self
            .out_dir
            .as_ref()
            .ok_or_else(|| err_other("out_dir is empty"))?;
        let lang = self
            .lang
            .as_ref()
            .ok_or_else(|| err_other("lang is empty"))?;

        // --{lang}_out={out_dir}
        let mut lang_out_flag = OsString::from("--");
        lang_out_flag.push(lang);
        lang_out_flag.push("_out=");
        lang_out_flag.push(out_dir);

        // --plugin={plugin}
        let plugin_flag = self.plugin.as_ref().map(|plugin| {
            let mut flag = OsString::from("--plugin=");
            flag.push(plugin);
            flag
        });

        // -I{include}
        let include_flags = self.includes.iter().map(|include| {
            let mut flag = OsString::from("-I");
            flag.push(include);
            flag
        });

        let mut cmd_args = Vec::new();
        cmd_args.push(lang_out_flag);
        cmd_args.extend(self.inputs.iter().map(|path| path.as_os_str().to_owned()));
        cmd_args.extend(plugin_flag);
        cmd_args.extend(include_flags);
        protoc.run_with_args(cmd_args)
    }
}

/// `Protoc --lang_out...` args
#[derive(Default)]
#[deprecated(since = "2.13", note = "Use ProtocLangOut instead")]
pub struct Args<'a> {
    /// `LANG` part in `--LANG_out=...`
    pub lang: &'a str,
    /// `--LANG_out=...` param
    pub out_dir: &'a str,
    /// `--plugin` param. Not needed if plugin is in `$PATH`
    pub plugin: Option<&'a str>,
    /// `-I` args
    pub includes: &'a [&'a str],
    /// List of `.proto` files to compile
    pub input: &'a [&'a str],
}

/// `Protoc --descriptor_set_out...` args
#[derive(Debug)]
pub struct DescriptorSetOutArgs<'a> {
    /// `--file_descriptor_out=...` param
    pub out: &'a str,
    /// `-I` args
    pub includes: &'a [&'a str],
    /// List of `.proto` files to compile
    pub input: &'a [&'a str],
    /// `--include_imports`
    pub include_imports: bool,
}

/// Protoc command.
#[derive(Clone, Debug)]
pub struct Protoc {
    exec: OsString,
}

impl Protoc {
    /// New `protoc` command from `$PATH`
    pub fn from_env_path() -> Protoc {
        match which::which("protoc") {
            Ok(path) => Protoc {
                exec: path.into_os_string(),
            },
            Err(e) => {
                panic!("protoc binary not found: {}", e);
            }
        }
    }

    /// New `protoc` command from specified path
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
    /// // Use a binary from `protoc-bin-vendored` crate
    /// let protoc = protoc::Protoc::from_path(
    ///     protoc_bin_vendored::protoc_bin_path().unwrap());
    /// ```
    pub fn from_path(path: impl AsRef<OsStr>) -> Protoc {
        Protoc {
            exec: path.as_ref().to_owned(),
        }
    }

    /// Check `protoc` command found and valid
    pub fn check(&self) -> Result<()> {
        self.version().map(|_| ())
    }

    fn spawn(&self, cmd: &mut process::Command) -> io::Result<process::Child> {
        info!("spawning command {:?}", cmd);

        cmd.spawn()
            .map_err(|e| Error::new(e.kind(), format!("failed to spawn `{:?}`: {}", cmd, e)))
    }

    /// Obtain `protoc` version
    pub fn version(&self) -> Result<Version> {
        let child = self.spawn(
            process::Command::new(&self.exec)
                .stdin(process::Stdio::null())
                .stdout(process::Stdio::piped())
                .stderr(process::Stdio::piped())
                .args(&["--version"]),
        )?;

        let output = child.wait_with_output()?;
        if !output.status.success() {
            return Err(err_other("protoc failed with error"));
        }
        let output =
            String::from_utf8(output.stdout).map_err(|e| Error::new(io::ErrorKind::Other, e))?;
        let output = match output.lines().next() {
            None => return Err(err_other("output is empty")),
            Some(line) => line,
        };
        let prefix = "libprotoc ";
        if !output.starts_with(prefix) {
            return Err(err_other("output does not start with prefix"));
        }
        let output = &output[prefix.len()..];
        if output.is_empty() {
            return Err(err_other("version is empty"));
        }
        let first = output.chars().next().unwrap();
        if !first.is_digit(10) {
            return Err(err_other("version does not start with digit"));
        }
        Ok(Version {
            version: output.to_owned(),
        })
    }

    /// Execute `protoc` command with given args, check it completed correctly.
    fn run_with_args(&self, args: Vec<OsString>) -> Result<()> {
        let mut cmd = process::Command::new(&self.exec);
        cmd.stdin(process::Stdio::null());
        cmd.args(args);

        let mut child = self.spawn(&mut cmd)?;

        if !child.wait()?.success() {
            return Err(err_other(&format!(
                "protoc ({:?}) exited with non-zero exit code",
                cmd
            )));
        }

        Ok(())
    }

    /// Execute configured `protoc` with given args
    #[deprecated(since = "2.13", note = "Use ProtocLangOut instead")]
    #[allow(deprecated)]
    pub fn run(&self, args: Args) -> Result<()> {
        let mut cmd_args: Vec<OsString> = Vec::new();

        if args.out_dir.is_empty() {
            return Err(err_other("out_dir is empty"));
        }

        if args.lang.is_empty() {
            return Err(err_other("lang is empty"));
        }

        cmd_args.push(format!("--{}_out={}", args.lang, args.out_dir).into());

        if args.input.is_empty() {
            return Err(err_other("input is empty"));
        }

        cmd_args.extend(args.input.into_iter().map(|a| OsString::from(*a)));

        if let Some(plugin) = args.plugin {
            cmd_args.push(format!("--plugin={}", plugin).into());
        }

        for include in args.includes {
            cmd_args.push(format!("-I{}", include).into());
        }

        self.run_with_args(cmd_args)
    }

    /// Execute `protoc --descriptor_set_out=`
    pub fn write_descriptor_set(&self, args: DescriptorSetOutArgs) -> Result<()> {
        let mut cmd_args: Vec<OsString> = Vec::new();

        for include in args.includes {
            cmd_args.push(format!("-I{}", include).into());
        }

        if args.out.is_empty() {
            return Err(err_other("out is empty"));
        }

        cmd_args.push(format!("--descriptor_set_out={}", args.out).into());

        if args.include_imports {
            cmd_args.push("--include_imports".into());
        }

        if args.input.is_empty() {
            return Err(err_other("input is empty"));
        }

        cmd_args.extend(args.input.into_iter().map(|a| OsString::from(*a)));

        self.run_with_args(cmd_args)
    }
}

/// Deprecated version of [`ProtocLangOut`].
#[deprecated(since = "2.13", note = "Use ProtocLangOut instead")]
#[allow(deprecated)]
pub fn run(args: Args) -> Result<()> {
    let mut protoc_lang_out = ProtocLangOut::new();
    if !args.lang.is_empty() {
        protoc_lang_out.lang(args.lang);
    }
    if !args.out_dir.is_empty() {
        protoc_lang_out.out_dir(args.out_dir);
    }
    if let Some(plugin) = args.plugin {
        protoc_lang_out.plugin(plugin);
    }
    if !args.includes.is_empty() {
        protoc_lang_out.includes(args.includes);
    }
    if !args.input.is_empty() {
        protoc_lang_out.inputs(args.input);
    }
    protoc_lang_out.run()
}

/// Protobuf (protoc) version.
pub struct Version {
    version: String,
}

impl Version {
    /// `true` if the protoc major version is 3.
    pub fn is_3(&self) -> bool {
        self.version.starts_with("3")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn version() {
        Protoc::from_env_path().version().expect("version");
    }
}

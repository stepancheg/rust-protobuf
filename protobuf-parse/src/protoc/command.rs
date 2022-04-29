//! API to invoke `protoc` command.
//!
//! `protoc` command must be in `$PATH`, along with `protoc-gen-LANG` command.
//!
//! Note that to generate `rust` code from `.proto` files, `protoc-rust` crate
//! can be used, which does not require `protoc-gen-rust` present in `$PATH`.

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

use std::ffi::OsStr;
use std::ffi::OsString;
use std::fmt;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::process::Stdio;

use log::info;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("protoc command exited with non-zero code")]
    ProtocNonZero,
    #[error("protoc command {0} exited with non-zero code")]
    ProtocNamedNonZero(String),
    #[error("protoc command {0} exited with non-zero code; stderr: {1:?}")]
    ProtocNamedNonZeroStderr(String, String),
    #[error("input is empty")]
    InputIsEmpty,
    #[error("output is empty")]
    OutputIsEmpty,
    #[error("output does not start with prefix")]
    OutputDoesNotStartWithPrefix,
    #[error("version is empty")]
    VersionIsEmpty,
    #[error("version does not start with digit")]
    VersionDoesNotStartWithDigit,
    #[error("failed to spawn command `{0}`")]
    FailedToSpawnCommand(String, #[source] io::Error),
    #[error("protoc output is not UTF-8")]
    ProtocOutputIsNotUtf8,
}

/// `Protoc --descriptor_set_out...` args
#[derive(Debug)]
pub(crate) struct DescriptorSetOutArgs {
    protoc: Protoc,
    /// `--file_descriptor_out=...` param
    out: Option<PathBuf>,
    /// `-I` args
    includes: Vec<PathBuf>,
    /// List of `.proto` files to compile
    inputs: Vec<PathBuf>,
    /// `--include_imports`
    include_imports: bool,
    /// Extra command line flags (like `--experimental_allow_proto3_optional`)
    extra_args: Vec<OsString>,
    /// Capture stderr instead of inheriting it.
    capture_stderr: bool,
}

impl DescriptorSetOutArgs {
    /// Set `--file_descriptor_out=...` param
    pub fn out(&mut self, out: impl AsRef<Path>) -> &mut Self {
        self.out = Some(out.as_ref().to_owned());
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

    /// Set `--include_imports`
    pub fn include_imports(&mut self, include_imports: bool) -> &mut Self {
        self.include_imports = include_imports;
        self
    }

    /// Add command line flags like `--experimental_allow_proto3_optional`.
    pub fn extra_arg(&mut self, arg: impl Into<OsString>) -> &mut Self {
        self.extra_args.push(arg.into());
        self
    }

    /// Add command line flags like `--experimental_allow_proto3_optional`.
    pub fn extra_args(&mut self, args: impl IntoIterator<Item = impl Into<OsString>>) -> &mut Self {
        for arg in args {
            self.extra_arg(arg);
        }
        self
    }

    /// Capture stderr instead of inheriting it.
    pub(crate) fn capture_stderr(&mut self, capture_stderr: bool) -> &mut Self {
        self.capture_stderr = capture_stderr;
        self
    }

    /// Execute `protoc --descriptor_set_out=`
    pub fn write_descriptor_set(&self) -> anyhow::Result<()> {
        if self.inputs.is_empty() {
            return Err(Error::InputIsEmpty.into());
        }

        let out = self.out.as_ref().ok_or_else(|| Error::OutputIsEmpty)?;

        // -I{include}
        let include_flags = self.includes.iter().map(|include| {
            let mut flag = OsString::from("-I");
            flag.push(include);
            flag
        });

        // --descriptor_set_out={out}
        let mut descriptor_set_out_flag = OsString::from("--descriptor_set_out=");
        descriptor_set_out_flag.push(out);

        // --include_imports
        let include_imports_flag = match self.include_imports {
            false => None,
            true => Some("--include_imports".into()),
        };

        let mut cmd_args = Vec::new();
        cmd_args.extend(include_flags);
        cmd_args.push(descriptor_set_out_flag);
        cmd_args.extend(include_imports_flag);
        cmd_args.extend(self.inputs.iter().map(|path| path.as_os_str().to_owned()));
        cmd_args.extend(self.extra_args.iter().cloned());
        self.protoc.run_with_args(cmd_args, self.capture_stderr)
    }
}

/// Protoc command.
#[derive(Clone, Debug)]
pub(crate) struct Protoc {
    exec: OsString,
}

impl Protoc {
    /// New `protoc` command from `$PATH`
    pub(crate) fn from_env_path() -> Protoc {
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
    pub(crate) fn from_path(path: impl AsRef<OsStr>) -> Protoc {
        Protoc {
            exec: path.as_ref().to_owned(),
        }
    }

    /// Check `protoc` command found and valid
    pub(crate) fn _check(&self) -> anyhow::Result<()> {
        self.version()?;
        Ok(())
    }

    fn spawn(&self, cmd: &mut process::Command) -> anyhow::Result<process::Child> {
        info!("spawning command {:?}", cmd);

        cmd.spawn()
            .map_err(|e| Error::FailedToSpawnCommand(format!("{:?}", cmd), e).into())
    }

    /// Obtain `protoc` version
    pub(crate) fn version(&self) -> anyhow::Result<Version> {
        let child = self.spawn(
            process::Command::new(&self.exec)
                .stdin(process::Stdio::null())
                .stdout(process::Stdio::piped())
                .stderr(process::Stdio::piped())
                .args(&["--version"]),
        )?;

        let output = child.wait_with_output()?;
        if !output.status.success() {
            return Err(Error::ProtocNonZero.into());
        }
        let output = String::from_utf8(output.stdout).map_err(|_| Error::ProtocOutputIsNotUtf8)?;
        let output = match output.lines().next() {
            None => return Err(Error::OutputIsEmpty.into()),
            Some(line) => line,
        };
        let prefix = "libprotoc ";
        if !output.starts_with(prefix) {
            return Err(Error::OutputDoesNotStartWithPrefix.into());
        }
        let output = &output[prefix.len()..];
        if output.is_empty() {
            return Err(Error::VersionIsEmpty.into());
        }
        let first = output.chars().next().unwrap();
        if !first.is_digit(10) {
            return Err(Error::VersionDoesNotStartWithDigit.into());
        }
        Ok(Version {
            version: output.to_owned(),
        })
    }

    /// Execute `protoc` command with given args, check it completed correctly.
    fn run_with_args(&self, args: Vec<OsString>, capture_stderr: bool) -> anyhow::Result<()> {
        let mut cmd = process::Command::new(&self.exec);
        cmd.stdin(process::Stdio::null());
        cmd.args(args);

        if capture_stderr {
            cmd.stderr(Stdio::piped());
        }

        let mut child = self.spawn(&mut cmd)?;

        if capture_stderr {
            let output = child.wait_with_output()?;
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stderr = stderr.trim_end().to_owned();
                return Err(Error::ProtocNamedNonZeroStderr(format!("{:?}", cmd), stderr).into());
            }
        } else {
            if !child.wait()?.success() {
                return Err(Error::ProtocNamedNonZero(format!("{:?}", cmd)).into());
            }
        }

        Ok(())
    }

    /// Get default DescriptorSetOutArgs for this command.
    pub(crate) fn descriptor_set_out_args(&self) -> DescriptorSetOutArgs {
        DescriptorSetOutArgs {
            protoc: self.clone(),
            out: None,
            includes: Vec::new(),
            inputs: Vec::new(),
            include_imports: false,
            extra_args: Vec::new(),
            capture_stderr: false,
        }
    }
}

/// Protobuf (protoc) version.
pub(crate) struct Version {
    version: String,
}

impl Version {
    /// `true` if the protoc major version is 3.
    pub fn _is_3(&self) -> bool {
        self.version.starts_with("3")
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.version, f)
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

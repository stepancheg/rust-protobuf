//! API to invoke `protoc` command. `protoc` command must be in `$PATH`.

#![deny(missing_docs)]

use std::io;
use std::process;

#[macro_use]
extern crate log;


/// Alias for io::Error
pub type Error = io::Error;
/// Alias for io::Error
pub type Result<T> = io::Result<T>;


fn err_other<T>(s: &str) -> Result<T> {
    Err(Error::new(io::ErrorKind::Other, s.to_owned()))
}


/// `Protoc --lang_out...` args
#[derive(Default)]
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
pub struct Protoc {
    exec: String,
}

impl Protoc {
    /// New `protoc` command from `$PATH`
    pub fn from_env_path() -> Protoc {
        Protoc { exec: "protoc".to_owned() }
    }

    /// New `protoc` command from specified path
    pub fn from_path(path: &str) -> Protoc {
        Protoc { exec: path.to_owned() }
    }

    /// Check `protoc` command found and valid
    pub fn check(&self) -> Result<()> {
        self.version().map(|_| ())
    }

    fn spawn(&self, cmd: &mut process::Command) -> io::Result<process::Child> {
        info!("spawning command {:?}", cmd);

        cmd.spawn()
            .map_err(|e| {
                Error::new(e.kind(), format!("failed to spawn `{:?}`: {}", cmd, e))
            })
    }

    /// Obtain `protoc` version
    pub fn version(&self) -> Result<Version> {
        let child = self.spawn(process::Command::new(&self.exec)
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .args(&["--version"]))?;

        let output = child.wait_with_output()?;
        if !output.status.success() {
            return err_other("protoc failed with error");
        }
        let output = String::from_utf8(output.stdout)
            .map_err(|e| Error::new(io::ErrorKind::Other, e))?;
        let output = match output.lines().next() {
            None => return err_other("output is empty"),
            Some(line) => line,
        };
        let prefix = "libprotoc ";
        if !output.starts_with(prefix) {
            return err_other("output does not start with prefix");
        }
        let output = &output[prefix.len()..];
        if output.is_empty() {
            return err_other("version is empty");
        }
        let first = output.chars().next().unwrap();
        if !first.is_digit(10) {
            return err_other("version does not start with digit");
        }
        Ok(Version { version: output.to_owned() })
    }

    /// Execute `protoc` command with given args, check it completed correctly.
    fn run_with_args(&self, args: Vec<String>) -> Result<()> {
        let mut cmd = process::Command::new(&self.exec);
        cmd.stdin(process::Stdio::null());
        cmd.args(args);

        let mut child = self.spawn(&mut cmd)?;

        if !child.wait()?.success() {
            return err_other(&format!("protoc ({:?}) exited with non-zero exit code", cmd));
        }

        Ok(())
    }

    /// Execute configured `protoc` with given args
    pub fn run(&self, args: Args) -> Result<()> {
        let mut cmd_args: Vec<String> = Vec::new();

        if args.out_dir.is_empty() {
            return err_other("out_dir is empty");
        }

        if args.lang.is_empty() {
            return err_other("lang is empty");
        }

        cmd_args.push(format!("--{}_out={}", args.lang, args.out_dir));

        if args.input.is_empty() {
            return err_other("input is empty");
        }

        cmd_args.extend(args.input.into_iter().map(|a| String::from(*a)));

        if let Some(plugin) = args.plugin {
            cmd_args.push(format!("--plugin={}", plugin));
        }

        for include in args.includes {
            cmd_args.push(format!("-I{}", include));
        }

        self.run_with_args(cmd_args)
    }

    /// Execute `protoc --descriptor_set_out=`
    pub fn write_descriptor_set(&self, args: DescriptorSetOutArgs) -> Result<()> {
        let mut cmd_args: Vec<String> = Vec::new();

        for include in args.includes {
            cmd_args.push(format!("-I{}", include));
        }

        if args.out.is_empty() {
            return err_other("out is empty");
        }

        cmd_args.push(format!("--descriptor_set_out={}", args.out));

        if args.include_imports {
            cmd_args.push("--include_imports".to_owned());
        }

        if args.input.is_empty() {
            return err_other("input is empty");
        }

        cmd_args.extend(args.input.into_iter().map(|a| String::from(*a)));

        self.run_with_args(cmd_args)
    }
}

/// Execute `protoc` found in `$PATH` with given args
pub fn run(args: Args) -> Result<()> {
    let protoc = Protoc::from_env_path();

    // First check with have good `protoc`
    protoc.check()?;

    protoc.run(args)
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

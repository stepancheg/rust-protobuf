use std::io;
use std::process;

pub type Error = io::Error;
pub type Result<T> = io::Result<T>;


fn err_other<T>(s: &str) -> Result<T> {
    Err(Error::new(io::ErrorKind::Other, s.to_owned()))
}


#[derive(Default)]
pub struct Args {
    /// lang part in --lang_out=...
    pub lang: String,
    /// --lang_out= param
    pub out_dir: String,
    /// --plugin param. Not needed if plugin is in $PATH
    pub plugin: Option<String>,
    /// -I args
    pub includes: Vec<String>,
    /// List of .proto files to compile
    pub input: Vec<String>,
}


/// Protoc command.
pub struct Protoc {
    exec: String,
}

impl Protoc {
    /// New `protoc` command from `$PATH`
    pub fn from_env_path() -> Protoc {
        Protoc {
            exec: "protoc".to_owned(),
        }
    }

    /// New `protoc` command from specified path
    pub fn from_path(path: &str) -> Protoc {
        Protoc {
            exec: path.to_owned(),
        }
    }

    /// Check `protoc` command found and valid
    pub fn check(&self) -> Result<()> {
        self.version().map(|_| ())
    }

    /// Obtain `protoc` version
    pub fn version(&self) -> Result<Version> {
        let child = process::Command::new(&self.exec)
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .args(&["--version"])
            .spawn()?;
        
        let output = child.wait_with_output()?;
        if !output.status.success() {
            return err_other("protoc failed with error");
        }
        let output = String::from_utf8(output.stdout)
            .map_err(|e| { Error::new(io::ErrorKind::Other, e) })?;
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
        Ok(Version {
            version: output.to_owned(),
        })
    }

    /// Execute configured `protoc` with given args
    pub fn run(&self, args: Args) -> Result<()> {
        let mut cmd_args = Vec::new();

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

        cmd_args.extend(args.input);

        if let Some(plugin) = args.plugin {
            cmd_args.push(format!("--plugin={}", plugin));
        }

        for include in args.includes {
            cmd_args.push(format!("-I{}", include));
        }

        let mut child = process::Command::new(&self.exec)
            .stdin(process::Stdio::null())
            .args(cmd_args)
            .spawn()?;

        if !child.wait()?.success() {
            return err_other("protoc exited with non-zero exit code");
        }

        Ok(())
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

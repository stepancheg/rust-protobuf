//! `protoc` binary downloaded and stored inside the crate.
//!
//! Can be used to avoid downloading and installing `protoc` binary.

#![deny(missing_docs)]
#![deny(intra_doc_link_resolution_failure)]

use std::env;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;

/// Error returned when a binary is not available.
#[derive(Debug)]
pub struct Error {
    os: &'static str,
    arch: &'static str,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "protoc binary cannot be found for platform {}-{}",
            self.os, self.arch
        )
    }
}

impl std::error::Error for Error {}

/// Return a path to `protoc` binary.
///
/// This function returns an error when binary is not available for
/// the current operating system and architecture.
pub fn protoc_bin_path() -> Result<PathBuf, Error> {
    let protoc_bin_name = match (env::consts::OS, env::consts::ARCH) {
        ("linux", "x86") => "protoc-linux-x86_32",
        ("linux", "x86_64") => "protoc-linux-x86_64",
        ("linux", "aarch64") => "protoc-linux-aarch_64",
        ("linux", "powerpc64") => "protoc-linux-ppcle_64",
        ("macos", "x86_64") => "protoc-osx-x86_64",
        ("windows", _) => "protoc-win32.exe",
        (os, arch) => return Err(Error { os, arch }),
    };
    let protoc_bin_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("bin")
        .join(protoc_bin_name);
    assert!(
        protoc_bin_path.exists(),
        "internal: protoc not found {}",
        protoc_bin_name
    );
    Ok(protoc_bin_path)
}

#[cfg(test)]
mod test {
    use std::io::Read;
    use std::process;

    #[test]
    fn smoke() {
        let process = process::Command::new(super::protoc_bin_path().unwrap())
            .arg("--version")
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::piped())
            .spawn()
            .unwrap();
        let mut stdout = String::new();
        process.stdout.unwrap().read_to_string(&mut stdout).unwrap();
        assert!(stdout.contains("libprotoc"), "stdout is: {:?}", stdout)
    }
}

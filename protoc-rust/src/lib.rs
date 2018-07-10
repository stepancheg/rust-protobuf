extern crate tempfile;

extern crate protoc;
extern crate protobuf;
extern crate protobuf_codegen;

mod slashes;
use slashes::Slashes;

use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;

pub use protoc::Error;
pub use protoc::Result;

pub use protobuf_codegen::Customize;


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

/// Like `protoc --rust_out=...` but without requiring `protoc-gen-rust` command in `$PATH`.
pub fn run(args: Args) -> Result<()> {
    let protoc = protoc::Protoc::from_env_path();
    protoc.check()?;

    let temp_dir = tempfile::Builder::new().prefix("protoc-rust").tempdir()?;
    let temp_file = temp_dir.path().join("descriptor.pbbin");
    let temp_file = temp_file.to_str().expect("utf-8 file name");

    protoc.write_descriptor_set(protoc::DescriptorSetOutArgs {
        out: temp_file,
        includes: args.includes,
        input: args.input,
        include_imports: true,
    })?;

    let mut fds = Vec::new();
    let mut file = fs::File::open(temp_file)?;
    file.read_to_end(&mut fds)?;

    drop(file);
    drop(temp_dir);

    let fds: protobuf::descriptor::FileDescriptorSet = protobuf::parse_from_bytes(&fds)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let mut includes = args.includes;
    if includes.is_empty() {
        static DOT_SLICE: &'static [&'static str] = &["."];
        includes = DOT_SLICE;
    }

    let mut files_to_generate = Vec::new();
    'outer: for file in args.input {
        for include in includes {
            if let Some(truncated) = remove_path_prefix(file, include) {
                files_to_generate.push(truncated.to_owned());
                continue 'outer;
            }
        }

        return Err(Error::new(
            io::ErrorKind::Other,
            format!(
                "file {:?} is not found in includes {:?}",
                file,
                args.includes
            ),
        ));
    }

    protobuf_codegen::gen_and_write(
        fds.get_file(),
        &files_to_generate,
        &Path::new(&args.out_dir),
        &args.customize)
}

fn remove_path_prefix(mut path: &str, mut prefix: &str) -> Option<String> {
    let slashes = Slashes::here();
    path = slashes.remove_dot_slashes(path);
    prefix = slashes.remove_dot_slashes(prefix);

    if prefix == "" {
        return Some(path.to_owned());
    }

    let path = slashes.norm_path(path);
    let mut prefix = slashes.norm_path(prefix);

    if prefix.ends_with("/") {
        let l = prefix.len();
        prefix.truncate(l - 1);
    }

    if !path.starts_with(&prefix) {
        return None;
    }

    if path.len() <= prefix.len() {
        return None;
    }

    if path.as_bytes()[prefix.len()] == b'/' {
        return Some(path[prefix.len() + 1..].to_owned());
    } else {
        return None;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn remove_path_prefix() {
        assert_eq!(
            Some("abc.proto".to_owned()),
            super::remove_path_prefix("xxx/abc.proto", "xxx")
        );
        assert_eq!(
            Some("abc.proto".to_owned()),
            super::remove_path_prefix("xxx/abc.proto", "xxx/")
        );
        assert_eq!(
            Some("abc.proto".to_owned()),
            super::remove_path_prefix("../xxx/abc.proto", "../xxx/")
        );
        assert_eq!(
            Some("abc.proto".to_owned()),
            super::remove_path_prefix("abc.proto", ".")
        );
        assert_eq!(
            Some("abc.proto".to_owned()),
            super::remove_path_prefix("abc.proto", "./")
        );
        assert_eq!(None, super::remove_path_prefix("xxx/abc.proto", "yyy"));
        assert_eq!(None, super::remove_path_prefix("xxx/abc.proto", "yyy/"));
    }
}

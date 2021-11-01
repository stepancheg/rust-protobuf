use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::str;

use protobuf::descriptor::FileDescriptorProto;
use protobuf_codegen::ProtoPath;
use protobuf_codegen::ProtoPathBuf;

use crate::convert;
use crate::linked_hash_map::LinkedHashMap;
use crate::model;
use crate::FileDescriptorPair;
use crate::WithFileError;
use crate::ANY_PROTO;
use crate::API_PROTO;
use crate::DESCRIPTOR_PROTO;
use crate::DURATION_PROTO;
use crate::EMPTY_PROTO;
use crate::FIELD_MASK_PROTO;
use crate::RUSTPROTO_PROTO;
use crate::SOURCE_CONTEXT_PROTO;
use crate::STRUCT_PROTO;
use crate::TIMESTAMP_PROTO;
use crate::TYPE_PROTO;
use crate::WRAPPERS_PROTO;

#[derive(Debug, thiserror::Error)]
enum ParseAndTypeckError {
    #[error("file `{0}` content is not UTF-8")]
    FileContentIsNotUtf8(String),
    #[error("protobuf path `{0}` is not found in import path {1}")]
    FileNotFoundInImportPath(String, String),
    #[error("file `{0}` must reside in include path {1}")]
    FileMustResideInImportPath(String, String),
    #[error("could not read file `{0}`: {1}")]
    CouldNotReadFile(String, io::Error),
}

/// Resolve `.proto` files. `Display` is used for error messages.
pub trait ProtoPathResolver: fmt::Display {
    /// Resolve a `.proto` file.
    ///
    /// Return `None` if a path is unknown, and if a path is a built-in protobuf file,
    /// like `google/protobuf/descriptor.proto`, it will be handled by the library.
    fn resolve(&self, path: &ProtoPath) -> anyhow::Result<Option<ResolvedProtoFile>>;
}

struct Run<R>
where
    R: ProtoPathResolver,
{
    parsed_files: LinkedHashMap<ProtoPathBuf, FileDescriptorPair>,
    resolver: R,
}

impl<R> Run<R>
where
    R: ProtoPathResolver,
{
    fn get_file_and_all_deps_already_parsed(
        &self,
        protobuf_path: &ProtoPath,
        result: &mut LinkedHashMap<ProtoPathBuf, FileDescriptorPair>,
    ) {
        if let Some(_) = result.get(protobuf_path) {
            return;
        }

        let pair = self
            .parsed_files
            .get(protobuf_path)
            .expect("must be already parsed");
        result.insert(protobuf_path.to_proto_path_buf(), pair.clone());

        self.get_all_deps_already_parsed(&pair.parsed, result);
    }

    fn get_all_deps_already_parsed(
        &self,
        parsed: &model::FileDescriptor,
        result: &mut LinkedHashMap<ProtoPathBuf, FileDescriptorPair>,
    ) {
        for import in &parsed.imports {
            self.get_file_and_all_deps_already_parsed(&import.path, result);
        }
    }

    fn add_file_content(
        &mut self,
        protobuf_path: &ProtoPath,
        resolved: &ResolvedProtoFile,
    ) -> anyhow::Result<()> {
        let content = str::from_utf8(&resolved.content)
            .map_err(|_| ParseAndTypeckError::FileContentIsNotUtf8(protobuf_path.to_string()))?;

        let parsed = model::FileDescriptor::parse(&content).map_err(|e| WithFileError {
            file: resolved.path.clone(),
            error: e.into(),
        })?;

        for import in &parsed.imports {
            self.add_imported_file(&import.path)?;
        }

        let mut this_file_deps = LinkedHashMap::new();
        self.get_all_deps_already_parsed(&parsed, &mut this_file_deps);

        let this_file_deps: Vec<_> = this_file_deps.into_iter().map(|(_, v)| v).collect();

        let descriptor = convert::file_descriptor(protobuf_path, &parsed, &this_file_deps)
            .map_err(|e| WithFileError {
                file: resolved.path.clone(),
                error: e.into(),
            })?;

        self.parsed_files.insert(
            protobuf_path.to_proto_path_buf(),
            FileDescriptorPair { parsed, descriptor },
        );

        Ok(())
    }

    fn add_imported_file(&mut self, protobuf_path: &ProtoPath) -> anyhow::Result<()> {
        if let Some(_) = self.parsed_files.get(protobuf_path) {
            return Ok(());
        }

        let resolved = self.resolver.resolve(protobuf_path)?;
        if let Some(resolved) = resolved {
            return self.add_file_content(protobuf_path, &resolved);
        }

        let embedded = match protobuf_path.to_str() {
            "rustproto.proto" => Some(RUSTPROTO_PROTO),
            "google/protobuf/any.proto" => Some(ANY_PROTO),
            "google/protobuf/api.proto" => Some(API_PROTO),
            "google/protobuf/descriptor.proto" => Some(DESCRIPTOR_PROTO),
            "google/protobuf/duration.proto" => Some(DURATION_PROTO),
            "google/protobuf/empty.proto" => Some(EMPTY_PROTO),
            "google/protobuf/field_mask.proto" => Some(FIELD_MASK_PROTO),
            "google/protobuf/source_context.proto" => Some(SOURCE_CONTEXT_PROTO),
            "google/protobuf/struct.proto" => Some(STRUCT_PROTO),
            "google/protobuf/timestamp.proto" => Some(TIMESTAMP_PROTO),
            "google/protobuf/type.proto" => Some(TYPE_PROTO),
            "google/protobuf/wrappers.proto" => Some(WRAPPERS_PROTO),
            _ => None,
        };

        match embedded {
            Some(content) => self.add_file_content(
                protobuf_path,
                &ResolvedProtoFile {
                    path: protobuf_path.to_string(),
                    content: content.as_bytes().to_vec(),
                },
            ),
            None => Err(ParseAndTypeckError::FileNotFoundInImportPath(
                protobuf_path.to_string(),
                format!("{}", self.resolver),
            )
            .into()),
        }
    }
}

/// Result of parsing `.proto` files.
#[doc(hidden)]
pub struct ParsedAndTypechecked {
    /// One entry for each input `.proto` file.
    pub relative_paths: Vec<ProtoPathBuf>,
    /// All parsed `.proto` files including dependencies of input files.
    pub file_descriptors: Vec<protobuf::descriptor::FileDescriptorProto>,
}

fn path_to_proto_path(path: &Path, includes: &[PathBuf]) -> anyhow::Result<ProtoPathBuf> {
    for include in includes {
        if include == Path::new(".") && path.is_relative() {
            // Special handling of `.` to allow using `.` as an include path
            // and `foo.proto` as input.
            return ProtoPathBuf::from_path(path);
        }
        match path.strip_prefix(include) {
            Ok(stripped) => return ProtoPathBuf::from_path(stripped),
            Err(_) => continue,
        }
    }
    Err(ParseAndTypeckError::FileMustResideInImportPath(
        path.display().to_string(),
        format!("{:?}", includes),
    )
    .into())
}

/// `.proto` file result provided from the [`ProtoPathResolver`].
pub struct ResolvedProtoFile {
    /// For error reporting.
    pub path: String,
    /// File content.
    pub content: Vec<u8>,
}

fn fs_resolver(includes: &[PathBuf]) -> impl ProtoPathResolver {
    struct Impl {
        includes: Vec<PathBuf>,
    }

    impl fmt::Display for Impl {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.includes)
        }
    }

    impl ProtoPathResolver for Impl {
        fn resolve(&self, proto_path: &ProtoPath) -> anyhow::Result<Option<ResolvedProtoFile>> {
            for include_dir in &self.includes {
                let fs_path = include_dir.join(proto_path.to_path());
                match fs::read_to_string(&fs_path) {
                    Ok(content) => {
                        return Ok(Some(ResolvedProtoFile {
                            path: fs_path.display().to_string(),
                            content: content.into_bytes(),
                        }))
                    }
                    Err(e) if e.kind() == io::ErrorKind::NotFound => continue,
                    Err(e) => {
                        return Err(ParseAndTypeckError::CouldNotReadFile(
                            fs_path.display().to_string(),
                            e,
                        )
                        .into())
                    }
                }
            }
            Ok(None)
        }
    }

    Impl {
        includes: includes.to_vec(),
    }
}

#[doc(hidden)]
pub fn parse_and_typecheck(
    includes: &[PathBuf],
    input: &[PathBuf],
) -> anyhow::Result<ParsedAndTypechecked> {
    let mut run = Run {
        parsed_files: LinkedHashMap::new(),
        resolver: fs_resolver(includes),
    };

    let relative_paths = input
        .iter()
        .map(|input| Ok((path_to_proto_path(input, includes)?, input)))
        .collect::<anyhow::Result<Vec<_>>>()?;

    for (proto_path, path) in &relative_paths {
        let content = fs::read_to_string(path)
            .map_err(|e| ParseAndTypeckError::CouldNotReadFile(path.display().to_string(), e))?;
        run.add_file_content(
            proto_path,
            &ResolvedProtoFile {
                path: path.display().to_string(),
                content: content.into_bytes(),
            },
        )?;
    }

    let file_descriptors: Vec<_> = run
        .parsed_files
        .into_iter()
        .map(|(_, v)| v.descriptor)
        .collect();

    Ok(ParsedAndTypechecked {
        relative_paths: relative_paths.into_iter().map(|(p, _)| p).collect(),
        file_descriptors,
    })
}

#[doc(hidden)]
pub fn parse_and_typecheck_custom(
    input: &[ProtoPathBuf],
    resolver: impl ProtoPathResolver,
) -> anyhow::Result<Vec<FileDescriptorProto>> {
    let mut run = Run {
        parsed_files: LinkedHashMap::new(),
        resolver,
    };

    for proto_path in input {
        run.add_imported_file(proto_path)?;
    }

    Ok(run
        .parsed_files
        .into_iter()
        .map(|(_, v)| v.descriptor)
        .collect())
}

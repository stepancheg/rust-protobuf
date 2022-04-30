use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::str;

use indexmap::IndexMap;
use protobuf::descriptor::FileDescriptorProto;
use protobuf::reflect::FileDescriptor;

use crate::parse_and_typecheck::ParsedAndTypechecked;
use crate::proto;
use crate::proto_path::ProtoPath;
use crate::proto_path::ProtoPathBuf;
use crate::pure::convert;
use crate::pure::model;
use crate::FileDescriptorPair;
use crate::Parser;

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

#[derive(Debug, thiserror::Error)]
#[error("error in `{file}`: {error}")]
struct WithFileError {
    file: String,
    #[source]
    error: anyhow::Error,
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
    parsed_files: IndexMap<ProtoPathBuf, FileDescriptorPair>,
    resolver: R,
}

impl<R> Run<R>
where
    R: ProtoPathResolver,
{
    fn file_and_all_deps_already_parsed(
        &self,
        protobuf_path: &ProtoPath,
        result: &mut IndexMap<ProtoPathBuf, FileDescriptorPair>,
    ) {
        if let Some(_) = result.get(protobuf_path) {
            return;
        }

        let pair = self
            .parsed_files
            .get(protobuf_path)
            .expect("must be already parsed");
        result.insert(protobuf_path.to_proto_path_buf(), pair.clone());

        self.all_deps_already_parsed(&pair.parsed, result);
    }

    fn all_deps_already_parsed(
        &self,
        parsed: &model::FileDescriptor,
        result: &mut IndexMap<ProtoPathBuf, FileDescriptorPair>,
    ) {
        for import in &parsed.imports {
            self.file_and_all_deps_already_parsed(&import.path, result);
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

        let mut this_file_deps = IndexMap::new();
        self.all_deps_already_parsed(&parsed, &mut this_file_deps);

        let this_file_deps: Vec<_> = this_file_deps.into_iter().map(|(_, v)| v).collect();

        let descriptor_proto = convert::file_descriptor(protobuf_path, &parsed, &this_file_deps)
            .map_err(|e| WithFileError {
                file: resolved.path.clone(),
                error: e.into(),
            })?;

        let deps: Vec<FileDescriptor> = self
            .parsed_files
            .values()
            .map(|v| v.descriptor.clone())
            .collect();
        let descriptor = FileDescriptor::new_dynamic(descriptor_proto.clone(), &deps)?;

        self.parsed_files.insert(
            protobuf_path.to_proto_path_buf(),
            FileDescriptorPair {
                parsed,
                descriptor_proto,
                descriptor,
            },
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
            "rustproto.proto" => Some(proto::RUSTPROTO_PROTO),
            "google/protobuf/any.proto" => Some(proto::ANY_PROTO),
            "google/protobuf/api.proto" => Some(proto::API_PROTO),
            "google/protobuf/descriptor.proto" => Some(proto::DESCRIPTOR_PROTO),
            "google/protobuf/duration.proto" => Some(proto::DURATION_PROTO),
            "google/protobuf/empty.proto" => Some(proto::EMPTY_PROTO),
            "google/protobuf/field_mask.proto" => Some(proto::FIELD_MASK_PROTO),
            "google/protobuf/source_context.proto" => Some(proto::SOURCE_CONTEXT_PROTO),
            "google/protobuf/struct.proto" => Some(proto::STRUCT_PROTO),
            "google/protobuf/timestamp.proto" => Some(proto::TIMESTAMP_PROTO),
            "google/protobuf/type.proto" => Some(proto::TYPE_PROTO),
            "google/protobuf/wrappers.proto" => Some(proto::WRAPPERS_PROTO),
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

pub(crate) fn path_to_proto_path(
    path: &Path,
    includes: &[PathBuf],
) -> anyhow::Result<ProtoPathBuf> {
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

/// Parse `.proto` files using pure Rust implementation.
pub fn parse_and_typecheck(parser: &Parser) -> anyhow::Result<ParsedAndTypechecked> {
    let mut run = Run {
        parsed_files: IndexMap::new(),
        resolver: fs_resolver(&parser.includes),
    };

    let relative_paths = parser
        .inputs
        .iter()
        .map(|input| Ok((path_to_proto_path(input, &parser.includes)?, input)))
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
        .map(|(_, v)| v.descriptor_proto)
        .collect();

    Ok(ParsedAndTypechecked {
        relative_paths: relative_paths.into_iter().map(|(p, _)| p).collect(),
        file_descriptors,
        parser: "pure".to_owned(),
    })
}

/// TODO: this API is to be refactored.
pub fn parse_and_typecheck_custom(
    input: &[ProtoPathBuf],
    resolver: impl ProtoPathResolver,
) -> anyhow::Result<Vec<FileDescriptorProto>> {
    let mut run = Run {
        parsed_files: IndexMap::new(),
        resolver,
    };

    for proto_path in input {
        run.add_imported_file(proto_path)?;
    }

    Ok(run
        .parsed_files
        .into_iter()
        .map(|(_, v)| v.descriptor_proto)
        .collect())
}

#[cfg(test)]
mod test {
    use std::fmt;

    use crate::proto_path::ProtoPath;
    use crate::pure::parse_and_typecheck::ProtoPathResolver;
    use crate::pure::parse_and_typecheck::ResolvedProtoFile;
    use crate::ProtoPathBuf;

    #[test]
    fn parse_and_typecheck_custom() {
        struct ResolverImpl;

        impl fmt::Display for ResolverImpl {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "ResolverImpl")
            }
        }

        impl ProtoPathResolver for ResolverImpl {
            fn resolve(&self, proto_path: &ProtoPath) -> anyhow::Result<Option<ResolvedProtoFile>> {
                if proto_path == "xx.proto" {
                    Ok(Some(ResolvedProtoFile {
                        path: "xx.proto".to_string(),
                        content: "syntax = 'proto3'; message Foo {}".as_bytes().to_vec(),
                    }))
                } else {
                    Ok(None)
                }
            }
        }

        let resolved = super::parse_and_typecheck_custom(
            &[ProtoPathBuf::new("xx.proto".to_owned()).unwrap()],
            ResolverImpl,
        )
        .unwrap();
        assert_eq!(1, resolved.len());
        assert_eq!("Foo", resolved[0].message_type[0].name());
    }
}

use std::fmt;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("path is empty")]
    Empty,
    #[error("backslashes in path: {0:?}")]
    Backslashes(String),
    #[error("path contains empty components: {0:?}")]
    EmptyComponent(String),
    #[error("dot in path: {0:?}")]
    Dot(String),
    #[error("dot-dot in path: {0:?}")]
    DotDot(String),
    #[error("path is absolute: `{}`", _0.display())]
    Absolute(PathBuf),
    #[error("non-UTF-8 component in path: `{}`", _0.display())]
    NotUtf8(PathBuf),
}

/// Protobuf file relative normalized file path.
#[repr(transparent)]
#[derive(Eq, PartialEq, Hash, Debug)]
pub struct ProtoPath {
    path: str,
}

/// Protobuf file relative normalized file path.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProtoPathBuf {
    path: String,
}

impl fmt::Display for ProtoPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.path)
    }
}

impl fmt::Display for ProtoPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.path)
    }
}

impl ProtoPath {
    fn unchecked_new(path: &str) -> &ProtoPath {
        unsafe { &*(path as *const str as *const ProtoPath) }
    }

    pub fn new(path: &str) -> anyhow::Result<&ProtoPath> {
        if path.is_empty() {
            return Err(Error::Empty.into());
        }
        if path.contains('\\') {
            return Err(Error::Backslashes(path.to_owned()).into());
        }
        for component in path.split('/') {
            if component.is_empty() {
                return Err(Error::EmptyComponent(path.to_owned()).into());
            }
            if component == "." {
                return Err(Error::Dot(path.to_owned()).into());
            }
            if component == ".." {
                return Err(Error::DotDot(path.to_owned()).into());
            }
        }
        Ok(Self::unchecked_new(path))
    }
}

impl ProtoPathBuf {
    pub fn as_path(&self) -> &ProtoPath {
        ProtoPath::unchecked_new(&self.path)
    }

    pub fn new(path: String) -> anyhow::Result<ProtoPathBuf> {
        ProtoPath::new(&path)?;
        Ok(ProtoPathBuf { path })
    }

    pub fn from_path(path: &Path) -> anyhow::Result<ProtoPathBuf> {
        let mut path_str = String::new();
        for component in path.components() {
            match component {
                Component::Prefix(..) => return Err(Error::Absolute(path.to_owned()).into()),
                Component::RootDir => return Err(Error::Absolute(path.to_owned()).into()),
                Component::CurDir => return Err(Error::Dot(path.display().to_string()).into()),
                Component::ParentDir => {
                    return Err(Error::DotDot(path.display().to_string()).into())
                }
                Component::Normal(c) => {
                    if !path_str.is_empty() {
                        path_str.push('/');
                    }
                    path_str.push_str(c.to_str().ok_or_else(|| Error::NotUtf8(path.to_owned()))?);
                }
            }
        }
        Ok(ProtoPathBuf { path: path_str })
    }
}

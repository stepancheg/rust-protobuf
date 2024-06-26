use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

/// Wrapper for `Path` that asserts that the path is relative.
#[repr(transparent)]
pub(crate) struct RelPath {
    path: Path,
}

/// Wrapper for `PathBuf` that asserts that the path is relative.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct _RelPathBuf {
    path: PathBuf,
}

impl RelPath {
    pub(crate) fn _new(path: &Path) -> &RelPath {
        assert!(
            !path.is_absolute(),
            "path must be relative: {}",
            path.display()
        );
        unsafe { &*(path as *const Path as *const RelPath) }
    }

    pub(crate) fn _to_owned(&self) -> _RelPathBuf {
        _RelPathBuf {
            path: self.path.to_owned(),
        }
    }
}

impl _RelPathBuf {
    pub(crate) fn _new(path: PathBuf) -> _RelPathBuf {
        assert!(
            !path.is_absolute(),
            "path must be relative: {}",
            path.display()
        );
        _RelPathBuf { path }
    }
}

impl Deref for RelPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl Deref for _RelPathBuf {
    type Target = RelPath;

    fn deref(&self) -> &Self::Target {
        RelPath::_new(&self.path)
    }
}

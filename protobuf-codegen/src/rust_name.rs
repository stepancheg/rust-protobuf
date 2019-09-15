use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RustRelativePath(pub String);

impl fmt::Display for RustRelativePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

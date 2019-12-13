use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JsonNumberLit(pub(crate) String);

impl fmt::Display for JsonNumberLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

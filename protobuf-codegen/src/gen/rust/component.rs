use std::fmt;
use std::fmt::Formatter;

use crate::gen::rust::ident::RustIdent;
use crate::gen::rust::keywords::parse_rust_keyword;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum RustPathComponent {
    Ident(RustIdent),
    Keyword(&'static str),
}

impl fmt::Display for RustPathComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RustPathComponent::Ident(ident) => write!(f, "{}", ident),
            RustPathComponent::Keyword(keyword) => write!(f, "{}", keyword),
        }
    }
}

impl RustPathComponent {
    pub(crate) const SUPER: RustPathComponent = RustPathComponent::Keyword("super");

    pub(crate) fn parse(s: &str) -> RustPathComponent {
        if s.starts_with("r#") {
            RustPathComponent::Ident(RustIdent::new(&s[2..]))
        } else if let Some(kw) = parse_rust_keyword(s) {
            RustPathComponent::Keyword(kw)
        } else {
            RustPathComponent::Ident(RustIdent::new(s))
        }
    }
}

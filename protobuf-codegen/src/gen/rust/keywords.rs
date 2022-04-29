#[cfg_attr(rustfmt, rustfmt_skip)]
static RUST_KEYWORDS: &'static [&'static str] = &[
    "_",
    "as",
    "async",
    "await",
    "break",
    "crate",
    "dyn",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "static",
    "self",
    "Self",
    "struct",
    "super",
    "true",
    "trait",
    "type",
    "unsafe",
    "use",
    "while",
    "continue",
    "box",
    "const",
    "where",
    "virtual",
    "proc",
    "alignof",
    "become",
    "offsetof",
    "priv",
    "pure",
    "sizeof",
    "typeof",
    "unsized",
    "yield",
    "do",
    "abstract",
    "final",
    "override",
    "macro",
];

// https://internals.rust-lang.org/t/raw-identifiers-dont-work-for-all-identifiers/9094/3
#[cfg_attr(rustfmt, rustfmt_skip)]
static RUST_KEYWORDS_WHICH_CANNOT_BE_RAW: &'static [&'static str] = &[
    "super",
    "self",
    "Self",
    "extern",
    "crate",
];

pub(crate) fn parse_rust_keyword(word: &str) -> Option<&'static str> {
    RUST_KEYWORDS.iter().cloned().find(|&kw| kw == word)
}

pub(crate) fn is_rust_keyword(ident: &str) -> bool {
    parse_rust_keyword(ident).is_some()
}

#[allow(dead_code)]
pub(crate) fn is_rust_keyword_which_cannot_be_raw(ident: &str) -> bool {
    RUST_KEYWORDS_WHICH_CANNOT_BE_RAW
        .iter()
        .cloned()
        .find(|&kw| kw == ident)
        .is_some()
}

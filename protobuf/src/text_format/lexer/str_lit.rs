use super::lexer_impl::Lexer;
use super::lexer_impl::LexerError;
use crate::text_format::lexer::ParserLanguage;
use core::fmt;

#[derive(Debug)]
pub enum StrLitDecodeError {
    // TODO: be more specific
    Error,
}

impl fmt::Display for StrLitDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StrLitDecodeError::Error => write!(f, "String literal decode error"),
        }
    }
}

impl std::error::Error for StrLitDecodeError {}

impl From<LexerError> for StrLitDecodeError {
    fn from(_: LexerError) -> Self {
        StrLitDecodeError::Error
    }
}

pub type StrLitDecodeResult<T> = Result<T, StrLitDecodeError>;

/// String literal, both `string` and `bytes`.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct StrLit {
    pub escaped: String,
}

impl StrLit {
    /// May fail if not valid UTF8
    pub fn decode_utf8(&self) -> StrLitDecodeResult<String> {
        let mut lexer = Lexer::new(&self.escaped, ParserLanguage::Json);
        let mut r = String::new();
        while !lexer.eof() {
            r.push(lexer.next_char_value()?);
        }
        Ok(r)
    }

    pub fn decode_bytes(&self) -> StrLitDecodeResult<Vec<u8>> {
        let mut lexer = Lexer::new(&self.escaped, ParserLanguage::Json);
        let mut r = Vec::new();
        while !lexer.eof() {
            r.push(lexer.next_byte_value()?);
        }
        Ok(r)
    }

    pub fn quoted(&self) -> String {
        format!("\"{}\"", self.escaped)
    }
}

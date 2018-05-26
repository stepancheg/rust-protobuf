use super::lexer_impl::Lexer;
use super::lexer_impl::LexerError;
use text_format::lexer::LexerCommentStyle;


#[derive(Debug)]
pub enum StrLitDecodeError {
    Error,
}

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
        // comment style does not matter here
        let comment_style = Lexer::new(&self.escaped, LexerCommentStyle::Cpp);
        let mut lexer = comment_style;
        let mut r = String::new();
        while !lexer.eof() {
            r.push(lexer.next_char_value()?);
        }
        Ok(r)
    }

    pub fn decode_bytes(&self) -> StrLitDecodeResult<Vec<u8>> {
        // comment style does not matter here
        let comment_style = Lexer::new(&self.escaped, LexerCommentStyle::Cpp);
        let mut lexer = comment_style;
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

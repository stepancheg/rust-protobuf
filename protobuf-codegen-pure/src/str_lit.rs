use parser::Loc;
use parser::Lexer;
use parser::ParserError;

#[derive(Debug)]
pub enum StrLitDecodeError {
    Error,
}

impl From<ParserError> for StrLitDecodeError {
    fn from(_: ParserError) -> Self {
        StrLitDecodeError::Error
    }
}

pub type StrLitDecodeResult<T> = Result<T, StrLitDecodeError>;


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct StrLit {
    pub quoted: String,
}

impl StrLit {
    /// May fail if not valid UTF8
    pub fn decode_utf8(&self) -> StrLitDecodeResult<String> {
        assert!(self.quoted.len() >= 2);
        assert!(self.quoted.as_bytes()[0] == self.quoted.as_bytes()[self.quoted.len() - 1]);
        let mut lexer = Lexer {
            input: &self.quoted[1 .. self.quoted.len() - 1],
            pos: 0,
            loc: Loc::start(),
        };
        let mut r = String::new();
        while !lexer.eof() {
            r.push(lexer.next_char_value()?);
        }
        Ok(r)
    }
}

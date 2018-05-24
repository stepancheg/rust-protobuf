use super::str_lit::StrLit;
use super::num_lit::NumLit;
use super::loc::Loc;
use super::lexer_impl::LexerError;
use super::lexer_impl::LexerResult;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Symbol(char),
    IntLit(u64),
    // including quotes
    StrLit(StrLit),
    FloatLit(f64),
}

impl Token {
    /// Back to original
    pub fn format(&self) -> String {
        match self {
            &Token::Ident(ref s) => s.clone(),
            &Token::Symbol(c) => c.to_string(),
            &Token::IntLit(ref i) => i.to_string(),
            &Token::StrLit(ref s) => s.quoted(),
            &Token::FloatLit(ref f) => f.to_string(),
        }
    }

    pub fn to_num_lit(&self) -> LexerResult<NumLit> {
        match self {
            &Token::IntLit(i) => Ok(NumLit::U64(i)),
            &Token::FloatLit(f) => Ok(NumLit::F64(f)),
            _ => Err(LexerError::IncorrectInput),
        }
    }
}

#[derive(Clone)]
pub struct TokenWithLocation {
    pub token: Token,
    pub loc: Loc,
}

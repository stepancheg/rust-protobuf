use crate::text_format::lexer::Lexer;
use crate::text_format::lexer::LexerError;
use crate::text_format::lexer::Loc;
use crate::text_format::lexer::ParserLanguage;
use crate::text_format::lexer::StrLit;
use crate::text_format::lexer::StrLitDecodeError;
use crate::text_format::lexer::Token;
use crate::text_format::lexer::TokenWithLocation;

#[derive(Debug)]
pub enum TokenizerError {
    LexerError(LexerError),
    StrLitDecodeError(StrLitDecodeError),
    InternalError,
    IncorrectInput, // TODO: too broad
    UnexpectedEof,
    ExpectStrLit,
    ExpectIntLit,
    ExpectFloatLit,
    ExpectIdent,
    ExpectNamedIdent(String),
    ExpectChar(char),
    ExpectAnyChar(Vec<char>),
}

pub type TokenizerResult<R> = Result<R, TokenizerError>;

impl From<LexerError> for TokenizerError {
    fn from(e: LexerError) -> Self {
        TokenizerError::LexerError(e)
    }
}

impl From<StrLitDecodeError> for TokenizerError {
    fn from(e: StrLitDecodeError) -> Self {
        TokenizerError::StrLitDecodeError(e)
    }
}

#[derive(Clone)]
pub struct Tokenizer<'a> {
    lexer: Lexer<'a>,
    next_token: Option<TokenWithLocation>,
    last_token_loc: Option<Loc>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str, comment_style: ParserLanguage) -> Tokenizer<'a> {
        Tokenizer {
            lexer: Lexer::new(input, comment_style),
            next_token: None,
            last_token_loc: None,
        }
    }

    pub fn loc(&self) -> Loc {
        // After lookahead return the location of the next token
        self.next_token
            .as_ref()
            .map(|t| t.loc.clone())
            // After token consumed return the location of that token
            .or(self.last_token_loc.clone())
            // Otherwise return the position of lexer
            .unwrap_or(self.lexer.loc)
    }

    fn lookahead(&mut self) -> TokenizerResult<Option<&Token>> {
        Ok(match self.next_token {
            Some(ref token) => Some(&token.token),
            None => {
                self.next_token = self.lexer.next_token()?;
                self.last_token_loc = self.next_token.as_ref().map(|t| t.loc.clone());
                match self.next_token {
                    Some(ref token) => Some(&token.token),
                    None => None,
                }
            }
        })
    }

    pub fn lookahead_some(&mut self) -> TokenizerResult<&Token> {
        match self.lookahead()? {
            Some(token) => Ok(token),
            None => Err(TokenizerError::UnexpectedEof),
        }
    }

    fn next(&mut self) -> TokenizerResult<Option<Token>> {
        self.lookahead()?;
        Ok(self
            .next_token
            .take()
            .map(|TokenWithLocation { token, .. }| token))
    }

    pub fn next_some(&mut self) -> TokenizerResult<Token> {
        match self.next()? {
            Some(token) => Ok(token),
            None => Err(TokenizerError::UnexpectedEof),
        }
    }

    /// Can be called only after lookahead, otherwise it's error
    pub fn advance(&mut self) -> TokenizerResult<Token> {
        self.next_token
            .take()
            .map(|TokenWithLocation { token, .. }| token)
            .ok_or(TokenizerError::InternalError)
    }

    /// No more tokens
    pub fn syntax_eof(&mut self) -> TokenizerResult<bool> {
        Ok(self.lookahead()?.is_none())
    }

    pub fn next_token_if_map<P, R>(&mut self, p: P) -> TokenizerResult<Option<R>>
    where
        P: FnOnce(&Token) -> Option<R>,
    {
        self.lookahead()?;
        let v = match self.next_token {
            Some(ref token) => match p(&token.token) {
                Some(v) => v,
                None => return Ok(None),
            },
            _ => return Ok(None),
        };
        self.next_token = None;
        Ok(Some(v))
    }

    pub fn next_token_check_map<P, R, E>(&mut self, p: P) -> Result<R, E>
    where
        P: FnOnce(&Token) -> Result<R, E>,
        E: From<TokenizerError>,
    {
        self.lookahead()?;
        let r = match self.next_token {
            Some(ref token) => p(&token.token)?,
            None => return Err(TokenizerError::UnexpectedEof.into()),
        };
        self.next_token = None;
        Ok(r)
    }

    fn next_token_if<P>(&mut self, p: P) -> TokenizerResult<Option<Token>>
    where
        P: FnOnce(&Token) -> bool,
    {
        self.next_token_if_map(|token| if p(token) { Some(token.clone()) } else { None })
    }

    pub fn next_ident_if_in(&mut self, idents: &[&str]) -> TokenizerResult<Option<String>> {
        let v = match self.lookahead()? {
            Some(&Token::Ident(ref next)) => {
                if idents.into_iter().find(|&i| i == next).is_some() {
                    next.clone()
                } else {
                    return Ok(None);
                }
            }
            _ => return Ok(None),
        };
        self.advance()?;
        Ok(Some(v))
    }

    pub fn next_ident_if_eq(&mut self, word: &str) -> TokenizerResult<bool> {
        Ok(self.next_ident_if_in(&[word])? != None)
    }

    pub fn next_ident_expect_eq(&mut self, word: &str) -> TokenizerResult<()> {
        if self.next_ident_if_eq(word)? {
            Ok(())
        } else {
            Err(TokenizerError::ExpectNamedIdent(word.to_owned()))
        }
    }

    pub fn next_ident_if_eq_error(&mut self, word: &str) -> TokenizerResult<()> {
        if self.clone().next_ident_if_eq(word)? {
            return Err(TokenizerError::IncorrectInput);
        }
        Ok(())
    }

    pub fn next_symbol_if_eq(&mut self, symbol: char) -> TokenizerResult<bool> {
        Ok(self.next_token_if(|token| match token {
            &Token::Symbol(c) if c == symbol => true,
            _ => false,
        })? != None)
    }

    pub fn next_symbol_expect_eq(&mut self, symbol: char) -> TokenizerResult<()> {
        if self.lookahead_is_symbol(symbol)? {
            self.advance()?;
            Ok(())
        } else {
            Err(TokenizerError::ExpectChar(symbol))
        }
    }

    pub fn next_symbol_expect_eq_oneof(&mut self, symbols: &[char]) -> TokenizerResult<char> {
        for symbol in symbols {
            if let Ok(()) = self.next_symbol_expect_eq(*symbol) {
                return Ok(*symbol);
            }
        }
        Err(TokenizerError::ExpectAnyChar(symbols.to_owned()))
    }

    pub fn lookahead_is_str_lit(&mut self) -> TokenizerResult<bool> {
        Ok(match self.lookahead()? {
            Some(&Token::StrLit(..)) => true,
            _ => false,
        })
    }

    pub fn lookahead_is_int_lit(&mut self) -> TokenizerResult<bool> {
        Ok(match self.lookahead()? {
            Some(&Token::IntLit(..)) => true,
            _ => false,
        })
    }

    pub fn lookahead_is_json_number(&mut self) -> TokenizerResult<bool> {
        Ok(match self.lookahead()? {
            Some(&Token::JsonNumber(..)) => true,
            _ => false,
        })
    }

    pub fn lookahead_if_symbol(&mut self) -> TokenizerResult<Option<char>> {
        Ok(match self.lookahead()? {
            Some(&Token::Symbol(c)) => Some(c),
            _ => None,
        })
    }

    pub fn lookahead_is_symbol(&mut self, symbol: char) -> TokenizerResult<bool> {
        Ok(self.lookahead_if_symbol()? == Some(symbol))
    }

    pub fn lookahead_is_ident(&mut self, ident: &str) -> TokenizerResult<bool> {
        Ok(match self.lookahead()? {
            Some(Token::Ident(i)) => i == ident,
            _ => false,
        })
    }

    pub fn next_ident(&mut self) -> TokenizerResult<String> {
        self.next_token_check_map(|token| match token {
            &Token::Ident(ref ident) => Ok(ident.clone()),
            _ => Err(TokenizerError::ExpectIdent),
        })
    }

    pub fn next_str_lit(&mut self) -> TokenizerResult<StrLit> {
        self.next_token_check_map(|token| match token {
            &Token::StrLit(ref str_lit) => Ok(str_lit.clone()),
            _ => Err(TokenizerError::ExpectStrLit),
        })
    }

    pub fn next_int_lit(&mut self) -> TokenizerResult<u64> {
        self.next_token_check_map(|token| match token {
            &Token::IntLit(v) => Ok(v),
            _ => Err(TokenizerError::ExpectIntLit),
        })
    }

    pub fn next_float_lit(&mut self) -> TokenizerResult<f64> {
        self.next_token_check_map(|token| match token {
            &Token::FloatLit(v) => Ok(v),
            _ => Err(TokenizerError::ExpectFloatLit),
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn tokenize<P, R>(input: &str, what: P) -> R
    where
        P: FnOnce(&mut Tokenizer) -> TokenizerResult<R>,
    {
        let mut tokenizer = Tokenizer::new(input, ParserLanguage::Proto);
        let r = what(&mut tokenizer).expect(&format!("parse failed at {}", tokenizer.loc()));
        let eof = tokenizer
            .syntax_eof()
            .expect(&format!("check eof failed at {}", tokenizer.loc()));
        assert!(eof, "{}", tokenizer.loc());
        r
    }

    #[test]
    fn test_ident() {
        let msg = r#"  aabb_c  "#;
        let mess = tokenize(msg, |p| p.next_ident().map(|s| s.to_owned()));
        assert_eq!("aabb_c", mess);
    }

    #[test]
    fn test_str_lit() {
        let msg = r#"  "a\nb"  "#;
        let mess = tokenize(msg, |p| p.next_str_lit());
        assert_eq!(
            StrLit {
                escaped: r#"a\nb"#.to_owned()
            },
            mess
        );
    }
}

use std::str;

use Message;

use text_format::lexer::Lexer;
use text_format::lexer::Loc;
use text_format::lexer::LexerCommentStyle;
use text_format::lexer::LexerError;
use text_format::lexer::TokenWithLocation;
use text_format::lexer::Token;
use reflect::MessageDescriptor;
use reflect::RuntimeFieldType;
use reflect::ReflectValueBox;
use reflect::RuntimeTypeDynamic;
use reflect::RuntimeTypeBox;
use reflect::EnumDescriptor;
use reflect::EnumValueDescriptor;
use text_format::lexer::StrLit;
use text_format::lexer::StrLitDecodeError;

#[derive(Debug)]
pub enum ParseError {
    LexerError(LexerError),
    StrLitDecodeError(StrLitDecodeError),
    UnexpectedEof,
    ExpectChar(char),
    ExpectIdent,
    ExpectStrLit,
    InternalError,
    IncorrectInput, // TODO: something better
    UnknownField(String),
    UnknownEnumValue(String),
}

impl From<LexerError> for ParseError {
    fn from(e: LexerError) -> Self {
        ParseError::LexerError(e)
    }
}

impl From<StrLitDecodeError> for ParseError {
    fn from(e: StrLitDecodeError) -> Self {
        ParseError::StrLitDecodeError(e)
    }
}

#[derive(Debug)]
pub struct ParseErrorWithLoc {
    error: ParseError,
    loc: Loc,
}

pub type ParseResult<A> = Result<A, ParseError>;
pub type ParseWithLocResult<A> = Result<A, ParseErrorWithLoc>;

#[derive(Clone)]
struct Parser<'a> {
    lexer: Lexer<'a>,
    next_token: Option<TokenWithLocation>,
}

impl<'a> Parser<'a> {
    pub fn loc(&self) -> Loc {
        self.next_token.clone().map_or(self.lexer.loc, |n| n.loc)
    }

    fn lookahead(&mut self) -> ParseResult<Option<&Token>> {
        Ok(match self.next_token {
            Some(ref token) => Some(&token.token),
            None => {
                self.next_token = self.lexer.next_token()?;
                match self.next_token {
                    Some(ref token) => Some(&token.token),
                    None => None,
                }
            }
        })
    }

    fn lookahead_some(&mut self) -> ParseResult<&Token> {
        match self.lookahead()? {
            Some(token) => Ok(token),
            None => Err(ParseError::UnexpectedEof),
        }
    }

    fn next(&mut self) -> ParseResult<Option<Token>> {
        self.lookahead()?;
        Ok(self.next_token.take().map(|TokenWithLocation { token, .. }| token))
    }

    fn next_some(&mut self) -> ParseResult<Token> {
        match self.next()? {
            Some(token) => Ok(token),
            None => Err(ParseError::UnexpectedEof),
        }
    }

    /// Can be called only after lookahead, otherwise it's error
    fn advance(&mut self) -> ParseResult<Token> {
        self.next_token.take()
            .map(|TokenWithLocation { token, .. }| token)
            .ok_or(ParseError::InternalError)
    }

    /// No more tokens
    fn syntax_eof(&mut self) -> ParseResult<bool> {
        Ok(self.lookahead()?.is_none())
    }

    fn next_token_if_map<P, R>(&mut self, p: P) -> ParseResult<Option<R>>
        where P : FnOnce(&Token) -> Option<R>
    {
        self.lookahead()?;
        let v = match self.next_token {
            Some(ref token) => {
                match p(&token.token) {
                    Some(v) => v,
                    None => return Ok(None),
                }
            }
            _ => return Ok(None),
        };
        self.next_token = None;
        Ok(Some(v))
    }

    fn next_token_check_map<P, R>(&mut self, p: P) -> ParseResult<R>
        where P : FnOnce(&Token) -> ParseResult<R>
    {
        self.lookahead()?;
        let r = match self.next_token {
            Some(ref token) => p(&token.token)?,
            None => return Err(ParseError::UnexpectedEof),
        };
        self.next_token = None;
        Ok(r)
    }

    fn next_token_if<P>(&mut self, p: P) -> ParseResult<Option<Token>>
        where P : FnOnce(&Token) -> bool
    {
        self.next_token_if_map(|token| if p(token) { Some(token.clone()) } else { None })
    }

    fn next_ident_if_in(&mut self, idents: &[&str]) -> ParseResult<Option<String>> {
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

    fn next_ident_if_eq(&mut self, word: &str) -> ParseResult<bool> {
        Ok(self.next_ident_if_in(&[word])? != None)
    }

    fn next_ident_if_eq_error(&mut self, word: &str) -> ParseResult<()> {
        if self.clone().next_ident_if_eq(word)? {
            return Err(ParseError::IncorrectInput);
        }
        Ok(())
    }

    fn next_symbol_if_eq(&mut self, symbol: char) -> ParseResult<bool> {
        Ok(self.next_token_if(|token| match token {
            &Token::Symbol(c) if c == symbol => true,
            _ => false,
        })? != None)
    }

    fn next_symbol_expect_eq(&mut self, symbol: char) -> ParseResult<()> {
        if self.lookahead_is_symbol(symbol)? {
            self.advance()?;
            Ok(())
        } else {
            Err(ParseError::ExpectChar(symbol))
        }
    }

    fn lookahead_if_symbol(&mut self) -> ParseResult<Option<char>> {
        Ok(match self.lookahead()? {
            Some(&Token::Symbol(c)) => Some(c),
            _ => None,
        })
    }

    fn lookahead_is_symbol(&mut self, symbol: char) -> ParseResult<bool> {
        Ok(self.lookahead_if_symbol()? == Some(symbol))
    }

    // Text format

    fn next_ident(&mut self) -> ParseResult<String> {
        self.next_token_check_map(|token| {
            match token {
                &Token::Ident(ref ident) => Ok(ident.clone()),
                _ => Err(ParseError::ExpectIdent),
            }
        })
    }

    fn next_str_lit(&mut self) -> ParseResult<StrLit> {
        self.next_token_check_map(|token| {
            match token {
                &Token::StrLit(ref str_lit) => Ok(str_lit.clone()),
                _ => Err(ParseError::ExpectStrLit),
            }
        })
    }

    fn next_field_name(&mut self) -> ParseResult<String> {
        self.next_ident()
    }

    fn read_enum<'e>(&mut self, e: &'e EnumDescriptor) -> ParseResult<&'e EnumValueDescriptor> {
        let ident = self.next_ident()?;
        let value = match e.value_by_name(&ident) {
            Some(value) => value,
            None => return Err(ParseError::UnknownEnumValue(ident)),
        };
        Ok(value)
    }

    fn read_string(&mut self) -> ParseResult<String> {
        self.next_str_lit().and_then(|s| s.decode_utf8().map_err(From::from))
    }

    fn read_value_of_type(&mut self, t: &RuntimeTypeDynamic) -> ParseResult<ReflectValueBox> {
        match t.to_box() {
            RuntimeTypeBox::Enum(e) => Ok(ReflectValueBox::Enum(self.read_enum(e)?)),
            RuntimeTypeBox::String => Ok(ReflectValueBox::String(self.read_string()?)),
            _ => unimplemented!(),
        }
    }

    fn merge_field(&mut self, message: &mut Message, descriptor: &MessageDescriptor)
        -> ParseResult<()>
    {
        let field_name = self.next_field_name()?;

        self.next_symbol_expect_eq(':')?;

        let field = match descriptor.field_by_name(&field_name) {
            Some(field) => field,
            None => {
                // TODO: shouldn't unknown fields be quietly skipped?
                return Err(ParseError::UnknownField(field_name))
            }
        };

        match field.runtime_field_type() {
            RuntimeFieldType::Singular(t) => {
                let value = self.read_value_of_type(t)?;
                field.set_singular_field(message, value);
            }
            RuntimeFieldType::Repeated(t) => {
                let value = self.read_value_of_type(t)?;
                field.mut_repeated(message).push(value);
            }
            _ => unimplemented!(),
        };

        Ok(())
    }

    fn merge_inner(&mut self, message: &mut Message) -> ParseResult<()> {
        loop {
            self.lexer.skip_ws()?;
            if self.lexer.eof() {
                break;
            }
            let descriptor = message.descriptor();
            self.merge_field(message, descriptor)?;
        }
        Ok(())
    }

    fn merge(&mut self, message: &mut Message) -> ParseWithLocResult<()> {
        match self.merge_inner(message) {
            Ok(()) => Ok(()),
            Err(error) => Err(ParseErrorWithLoc {
                error,
                loc: self.lexer.loc,
            })
        }
    }
}

pub fn merge_from_str(message: &mut Message, input: &str) -> ParseWithLocResult<()> {
    let lexer = Lexer::new(input, LexerCommentStyle::Sh);
    let mut parser = Parser {
        lexer,
        next_token: None,
    };
    parser.merge(message)
}

#[doc(hidden)]
pub fn unescape_string(string: &str) -> Vec<u8> {
    fn parse_if_digit(chars: &mut str::Chars) -> u8 {
        let mut copy = chars.clone();
        let f = match copy.next() {
            None => return 0,
            Some(f) => f,
        };
        let d = match f {
            '0'...'9' => (f as u8 - b'0'),
            _ => return 0,
        };
        *chars = copy;
        d
    }

    fn parse_hex_digit(chars: &mut str::Chars) -> u8 {
        match chars.next().unwrap() {
            c @ '0'...'9' => (c as u8) - b'0',
            c @ 'a'...'f' => (c as u8) - b'a' + 10,
            c @ 'A'...'F' => (c as u8) - b'A' + 10,
            _ => panic!("incorrect hex escape"),
        }
    }

    fn parse_escape_rem(chars: &mut str::Chars) -> u8 {
        let n = chars.next().unwrap();
        match n {
            'a' => return b'\x07',
            'b' => return b'\x08',
            'f' => return b'\x0c',
            'n' => return b'\n',
            'r' => return b'\r',
            't' => return b'\t',
            'v' => return b'\x0b',
            '"' => return b'"',
            '\'' => return b'\'',
            '0'...'9' => {
                let d1 = n as u8 - b'0';
                let d2 = parse_if_digit(chars);
                let d3 = parse_if_digit(chars);
                return (d1 * 64 + d2 * 8 + d3) as u8;
            },
            'x' => {
                let d1 = parse_hex_digit(chars);
                let d2 = parse_hex_digit(chars);
                return d1 * 16 + d2;
            }
            c => return c as u8, // TODO: validate ASCII
        };
    }

    let mut chars = string.chars();
    let mut r = Vec::new();

    loop {
        let f = match chars.next() {
            None => return r,
            Some(f) => f,
        };

        if f == '\\' {
            r.push(parse_escape_rem(&mut chars));
        } else {
            r.push(f as u8); // TODO: escape UTF-8
        }
    }
}

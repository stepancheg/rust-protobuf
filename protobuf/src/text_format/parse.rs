use std::str;

use Message;

use text_format::lexer::Loc;
use text_format::lexer::LexerCommentStyle;
use reflect::MessageDescriptor;
use reflect::RuntimeFieldType;
use reflect::ReflectValueBox;
use reflect::RuntimeTypeDynamic;
use reflect::RuntimeTypeBox;
use reflect::EnumDescriptor;
use reflect::EnumValueDescriptor;
use text_format::lexer::StrLitDecodeError;
use text_format::lexer::Tokenizer;
use text_format::lexer::TokenizerError;
use text_format::lexer::int;

#[derive(Debug)]
pub enum ParseError {
    TokenizerError(TokenizerError),
    StrLitDecodeError(StrLitDecodeError),
    UnknownField(String),
    UnknownEnumValue(String),
    IntegerOverflow,
    ExpectingBool,
}

impl From<TokenizerError> for ParseError {
    fn from(e: TokenizerError) -> Self {
        ParseError::TokenizerError(e)
    }
}

impl From<StrLitDecodeError> for ParseError {
    fn from(e: StrLitDecodeError) -> Self {
        ParseError::StrLitDecodeError(e)
    }
}

impl From<int::Overflow> for ParseError {
    fn from(_: int::Overflow) -> Self {
        ParseError::IntegerOverflow
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
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    // Text format

    fn next_field_name(&mut self) -> ParseResult<String> {
        Ok(self.tokenizer.next_ident()?)
    }

    fn read_enum<'e>(&mut self, e: &'e EnumDescriptor) -> ParseResult<&'e EnumValueDescriptor> {
        let ident = self.tokenizer.next_ident()?;
        let value = match e.value_by_name(&ident) {
            Some(value) => value,
            None => return Err(ParseError::UnknownEnumValue(ident)),
        };
        Ok(value)
    }

    fn read_u32(&mut self) -> ParseResult<u32> {
        let int_lit = self.tokenizer.next_int_lit()?;
        let value_u32 = int_lit as u32;
        if value_u32 as u64 != int_lit {
            return Err(ParseError::IntegerOverflow);
        }
        Ok(value_u32)
    }

    fn read_u64(&mut self) -> ParseResult<u64> {
        Ok(self.tokenizer.next_int_lit()?)
    }

    fn read_i32(&mut self) -> ParseResult<i32> {
        let value = self.read_i64()?;
        if value < i32::min_value() as i64 || value > i32::max_value() as i64 {
            return Err(ParseError::IntegerOverflow);
        }
        Ok(value as i32)
    }

    fn read_i64(&mut self) -> ParseResult<i64> {
        if self.tokenizer.next_symbol_if_eq('-')? {
            let int_lit = self.tokenizer.next_int_lit()?;
            Ok(int::neg(int_lit)?)
        } else {
            let int_lit = self.tokenizer.next_int_lit()?;
            if int_lit > i64::max_value() as u64 {
                return Err(ParseError::IntegerOverflow);
            }
            Ok(int_lit as i64)
        }
    }

    fn read_f32(&mut self) -> ParseResult<f32> {
        unimplemented!();
    }

    fn read_f64(&mut self) -> ParseResult<f64> {
        unimplemented!();
    }

    fn read_bool(&mut self) -> ParseResult<bool> {
        if self.tokenizer.next_ident_if_eq("true")? {
            Ok(true)
        } else if self.tokenizer.next_ident_if_eq("false")? {
            Ok(false)
        } else {
            Err(ParseError::ExpectingBool)
        }
    }

    fn read_string(&mut self) -> ParseResult<String> {
        Ok(self.tokenizer.next_str_lit().and_then(|s| s.decode_utf8().map_err(From::from))?)
    }

    fn read_bytes(&mut self) -> ParseResult<Vec<u8>> {
        unimplemented!()
    }

    fn read_message(&mut self, _message_descriptor: &'static MessageDescriptor) -> ParseResult<Box<Message>> {
        unimplemented!()
    }

    fn read_value_of_type(&mut self, t: &RuntimeTypeDynamic) -> ParseResult<ReflectValueBox> {
        Ok(match t.to_box() {
            RuntimeTypeBox::Enum(e) => ReflectValueBox::Enum(self.read_enum(e)?),
            RuntimeTypeBox::U32 => ReflectValueBox::U32(self.read_u32()?),
            RuntimeTypeBox::U64 => ReflectValueBox::U64(self.read_u64()?),
            RuntimeTypeBox::I32 => ReflectValueBox::I32(self.read_i32()?),
            RuntimeTypeBox::I64 => ReflectValueBox::I64(self.read_i64()?),
            RuntimeTypeBox::F32 => ReflectValueBox::F32(self.read_f32()?),
            RuntimeTypeBox::F64 => ReflectValueBox::F64(self.read_f64()?),
            RuntimeTypeBox::Bool => ReflectValueBox::Bool(self.read_bool()?),
            RuntimeTypeBox::String |
            RuntimeTypeBox::Chars => ReflectValueBox::String(self.read_string()?),
            RuntimeTypeBox::VecU8 |
            RuntimeTypeBox::CarllercheBytes => ReflectValueBox::Bytes(self.read_bytes()?),
            RuntimeTypeBox::Message(m) => ReflectValueBox::Message(self.read_message(m)?),
        })
    }

    fn merge_field(&mut self, message: &mut Message, descriptor: &MessageDescriptor)
        -> ParseResult<()>
    {
        let field_name = self.next_field_name()?;

        self.tokenizer.next_symbol_expect_eq(':')?;

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
            if self.tokenizer.syntax_eof()? {
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
                loc: self.tokenizer.loc(),
            })
        }
    }
}

pub fn merge_from_str(message: &mut Message, input: &str) -> ParseWithLocResult<()> {
    let mut parser = Parser {
        tokenizer: Tokenizer::new(input, LexerCommentStyle::Sh)
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

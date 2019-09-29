use std::fmt;
use std::str;

use crate::core::Message;

use crate::reflect::EnumDescriptor;
use crate::reflect::EnumValueDescriptor;
use crate::reflect::MessageDescriptor;
use crate::reflect::ReflectValueBox;
use crate::reflect::RuntimeFieldType;
use crate::reflect::RuntimeTypeBox;
use crate::reflect::RuntimeTypeDynamic;
use crate::text_format::lexer::int;
use crate::text_format::lexer::Loc;
use crate::text_format::lexer::ParserLanguage;
use crate::text_format::lexer::StrLitDecodeError;
use crate::text_format::lexer::Tokenizer;
use crate::text_format::lexer::TokenizerError;

#[derive(Debug)]
pub enum ParseErrorWithoutLoc {
    TokenizerError(TokenizerError),
    StrLitDecodeError(StrLitDecodeError),
    UnknownField(String),
    UnknownEnumValue(String),
    MapFieldIsSpecifiedMoreThanOnce(String),
    IntegerOverflow,
    ExpectingBool,
    MessageNotInitialized,
}

impl From<TokenizerError> for ParseErrorWithoutLoc {
    fn from(e: TokenizerError) -> Self {
        ParseErrorWithoutLoc::TokenizerError(e)
    }
}

impl From<StrLitDecodeError> for ParseErrorWithoutLoc {
    fn from(e: StrLitDecodeError) -> Self {
        ParseErrorWithoutLoc::StrLitDecodeError(e)
    }
}

impl From<int::Overflow> for ParseErrorWithoutLoc {
    fn from(_: int::Overflow) -> Self {
        ParseErrorWithoutLoc::IntegerOverflow
    }
}

/// Text format parse error.
#[derive(Debug)]
pub struct ParseError {
    error: ParseErrorWithoutLoc,
    loc: Loc,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.loc, self.error)
    }
}

impl std::error::Error for ParseError {}

pub type ParseResult<A> = Result<A, ParseErrorWithoutLoc>;
pub type ParseWithLocResult<A> = Result<A, ParseError>;

#[derive(Clone)]
struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    // Text format

    fn next_field_name(&mut self) -> ParseResult<String> {
        Ok(self.tokenizer.next_ident()?)
    }

    fn read_colon(&mut self) -> ParseResult<()> {
        Ok(self.tokenizer.next_symbol_expect_eq(':')?)
    }

    fn read_enum<'e>(&mut self, e: &'e EnumDescriptor) -> ParseResult<&'e EnumValueDescriptor> {
        self.read_colon()?;

        // TODO: read integer?
        let ident = self.tokenizer.next_ident()?;
        let value = match e.get_value_by_name(&ident) {
            Some(value) => value,
            None => return Err(ParseErrorWithoutLoc::UnknownEnumValue(ident)),
        };
        Ok(value)
    }

    fn read_u64(&mut self) -> ParseResult<u64> {
        self.read_colon()?;

        Ok(self.tokenizer.next_int_lit()?)
    }

    fn read_u32(&mut self) -> ParseResult<u32> {
        self.read_colon()?;

        let int_lit = self.tokenizer.next_int_lit()?;
        let value_u32 = int_lit as u32;
        if value_u32 as u64 != int_lit {
            return Err(ParseErrorWithoutLoc::IntegerOverflow);
        }
        Ok(value_u32)
    }

    fn read_i64(&mut self) -> ParseResult<i64> {
        self.read_colon()?;

        if self.tokenizer.next_symbol_if_eq('-')? {
            let int_lit = self.tokenizer.next_int_lit()?;
            Ok(int::neg(int_lit)?)
        } else {
            let int_lit = self.tokenizer.next_int_lit()?;
            if int_lit > i64::max_value() as u64 {
                return Err(ParseErrorWithoutLoc::IntegerOverflow);
            }
            Ok(int_lit as i64)
        }
    }

    fn read_i32(&mut self) -> ParseResult<i32> {
        let value = self.read_i64()?;
        if value < i32::min_value() as i64 || value > i32::max_value() as i64 {
            return Err(ParseErrorWithoutLoc::IntegerOverflow);
        }
        Ok(value as i32)
    }

    fn read_f64(&mut self) -> ParseResult<f64> {
        self.read_colon()?;

        let minus = self.tokenizer.next_symbol_if_eq('-')?;

        let value = if let Ok(value) = self.tokenizer.next_int_lit() {
            value as f64
        } else {
            self.tokenizer.next_float_lit()?
        };

        Ok(if minus { -value } else { value })
    }

    fn read_f32(&mut self) -> ParseResult<f32> {
        Ok(self.read_f64()? as f32)
    }

    fn read_bool(&mut self) -> ParseResult<bool> {
        self.read_colon()?;

        if self.tokenizer.next_ident_if_eq("true")? {
            Ok(true)
        } else if self.tokenizer.next_ident_if_eq("false")? {
            Ok(false)
        } else {
            Err(ParseErrorWithoutLoc::ExpectingBool)
        }
    }

    fn read_string(&mut self) -> ParseResult<String> {
        self.read_colon()?;

        Ok(self
            .tokenizer
            .next_str_lit()
            .and_then(|s| s.decode_utf8().map_err(From::from))?)
    }

    fn read_bytes(&mut self) -> ParseResult<Vec<u8>> {
        self.read_colon()?;

        Ok(self
            .tokenizer
            .next_str_lit()
            .and_then(|s| s.decode_bytes().map_err(From::from))?)
    }

    fn read_message(
        &mut self,
        descriptor: &'static MessageDescriptor,
    ) -> ParseResult<Box<dyn Message>> {
        let mut message = descriptor.new_instance();

        let symbol = self.tokenizer.next_symbol_expect_eq_oneof(&['{', '<'])?;
        let terminator = if symbol == '{' { '}' } else { '>' };
        while !self.tokenizer.lookahead_is_symbol(terminator)? {
            self.merge_field(&mut *message, descriptor)?;
        }
        self.tokenizer.next_symbol_expect_eq(terminator)?;
        Ok(message)
    }

    fn read_map_entry(
        &mut self,
        k: &dyn RuntimeTypeDynamic,
        v: &dyn RuntimeTypeDynamic,
    ) -> ParseResult<(ReflectValueBox, ReflectValueBox)> {
        let key_field_name: &str = "key";
        let value_field_name: &str = "value";

        let mut key = None;
        let mut value = None;
        self.tokenizer.next_symbol_expect_eq('{')?;
        while !self.tokenizer.lookahead_is_symbol('}')? {
            let ident = self.next_field_name()?;
            let (field, field_type) = if ident == key_field_name {
                (&mut key, k)
            } else if ident == value_field_name {
                (&mut value, v)
            } else {
                return Err(ParseErrorWithoutLoc::UnknownField(ident));
            };

            if let Some(..) = *field {
                return Err(ParseErrorWithoutLoc::MapFieldIsSpecifiedMoreThanOnce(ident));
            }

            let field_value = self.read_value_of_type(field_type)?;

            *field = Some(field_value);
        }
        self.tokenizer.next_symbol_expect_eq('}')?;
        let key = match key {
            Some(key) => key,
            None => k.default_value_ref().to_box(),
        };
        let value = match value {
            Some(value) => value,
            None => v.default_value_ref().to_box(),
        };
        Ok((key, value))
    }

    fn read_value_of_type(&mut self, t: &dyn RuntimeTypeDynamic) -> ParseResult<ReflectValueBox> {
        Ok(match t.to_box() {
            RuntimeTypeBox::Enum(d) => ReflectValueBox::Enum(d, self.read_enum(d)?.value()),
            RuntimeTypeBox::U32 => ReflectValueBox::U32(self.read_u32()?),
            RuntimeTypeBox::U64 => ReflectValueBox::U64(self.read_u64()?),
            RuntimeTypeBox::I32 => ReflectValueBox::I32(self.read_i32()?),
            RuntimeTypeBox::I64 => ReflectValueBox::I64(self.read_i64()?),
            RuntimeTypeBox::F32 => ReflectValueBox::F32(self.read_f32()?),
            RuntimeTypeBox::F64 => ReflectValueBox::F64(self.read_f64()?),
            RuntimeTypeBox::Bool => ReflectValueBox::Bool(self.read_bool()?),
            RuntimeTypeBox::String => ReflectValueBox::String(self.read_string()?),
            RuntimeTypeBox::VecU8 => ReflectValueBox::Bytes(self.read_bytes()?),
            RuntimeTypeBox::Message(m) => ReflectValueBox::Message(self.read_message(m)?),
        })
    }

    fn merge_field(
        &mut self,
        message: &mut dyn Message,
        descriptor: &MessageDescriptor,
    ) -> ParseResult<()> {
        let field_name = self.next_field_name()?;

        let field = match descriptor.get_field_by_name(&field_name) {
            Some(field) => field,
            None => {
                // TODO: shouldn't unknown fields be quietly skipped?
                return Err(ParseErrorWithoutLoc::UnknownField(field_name));
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
            RuntimeFieldType::Map(k, v) => {
                let (k, v) = self.read_map_entry(k, v)?;
                field.mut_map(message).insert(k, v);
            }
        };

        Ok(())
    }

    fn merge_inner(&mut self, message: &mut dyn Message) -> ParseResult<()> {
        loop {
            if self.tokenizer.syntax_eof()? {
                break;
            }
            let descriptor = message.descriptor();
            self.merge_field(message, descriptor)?;
        }
        Ok(())
    }

    fn merge(&mut self, message: &mut dyn Message) -> ParseWithLocResult<()> {
        match self.merge_inner(message) {
            Ok(()) => Ok(()),
            Err(error) => Err(ParseError {
                error,
                loc: self.tokenizer.loc(),
            }),
        }
    }
}

/// Parse text format message.
///
/// This function does not check if message required fields are set.
pub fn merge_from_str(message: &mut dyn Message, input: &str) -> ParseWithLocResult<()> {
    let mut parser = Parser {
        tokenizer: Tokenizer::new(input, ParserLanguage::TextFormat),
    };
    parser.merge(message)
}

/// Parse text format message.
pub fn parse_from_str<M: Message>(input: &str) -> ParseWithLocResult<M> {
    let mut m = M::new();
    merge_from_str(&mut m, input)?;
    if let Err(_) = m.check_initialized() {
        return Err(ParseError {
            error: ParseErrorWithoutLoc::MessageNotInitialized,
            loc: Loc::start(),
        });
    }
    Ok(m)
}

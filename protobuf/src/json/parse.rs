use std::num::ParseIntError;
use std::num::ParseFloatError;

use std::f32;
use std::f64;

use super::base64;

use Message;
use enums::ProtobufEnum;
use text_format::lexer::TokenizerError;
use text_format::lexer::Loc;
use text_format::lexer::Tokenizer;
use text_format::lexer::ParserLanguage;
use reflect::FieldDescriptor;
use reflect::RuntimeFieldType;
use reflect::RuntimeTypeDynamic;
use reflect::ReflectValueBox;
use reflect::RuntimeTypeBox;
use reflect::EnumDescriptor;
use reflect::EnumValueDescriptor;
use reflect::MessageDescriptor;
use json::base64::FromBase64Error;
use text_format::lexer::Lexer;
use text_format::lexer::LexerError;
use text_format::lexer::Token;

use super::float;
use text_format::lexer::JsonNumberLit;

use well_known_types::Duration;
use well_known_types::NullValue;
use well_known_types::Value;
use well_known_types::Value_oneof_kind;
use well_known_types::ListValue;
use well_known_types::DoubleValue;
use well_known_types::FloatValue;
use well_known_types::Int64Value;
use well_known_types::UInt64Value;
use well_known_types::Int32Value;
use well_known_types::UInt32Value;
use well_known_types::BoolValue;
use well_known_types::StringValue;
use well_known_types::BytesValue;
use well_known_types::Struct;
use json::well_known_wrapper::WellKnownWrapper;

#[derive(Debug)]
pub enum ParseError {
    TokenizerError(TokenizerError),
    UnknownFieldName(String),
    UnknownEnumVariantName(String),
    UnknownEnumVariantNumber(i32),
    FromBase64Error(FromBase64Error),
    IncorrectStrLit(LexerError),
    IncorrectDuration,
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    ExpectingBool,
    ExpectingStrOrInt,
    ExpectingNumber,
    UnexpectedToken,
}

impl From<TokenizerError> for ParseError {
    fn from(e: TokenizerError) -> Self {
        ParseError::TokenizerError(e)
    }
}

impl From<FromBase64Error> for ParseError {
    fn from(e: FromBase64Error) -> Self {
        ParseError::FromBase64Error(e)
    }
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::ParseIntError(e)
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(e: ParseFloatError) -> Self {
        ParseError::ParseFloatError(e)
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

trait FromJsonNumber : PartialEq + Sized {
    fn from_f64(v: f64) -> Self;
    fn to_f64(&self) -> f64;
    fn from_string(v: &str) -> ParseResult<Self>;
}

impl FromJsonNumber for u32 {
    fn from_f64(v: f64) -> Self {
        v as u32
    }

    fn to_f64(&self) -> f64 {
        *self as f64
    }

    fn from_string(v: &str) -> Result<Self, ParseError> {
        Ok(v.parse()?)
    }
}

impl FromJsonNumber for u64 {
    fn from_f64(v: f64) -> Self {
        v as u64
    }

    fn to_f64(&self) -> f64 {
        *self as f64
    }

    fn from_string(v: &str) -> Result<Self, ParseError> {
        Ok(v.parse()?)
    }
}

impl FromJsonNumber for i32 {
    fn from_f64(v: f64) -> Self {
        v as i32
    }

    fn to_f64(&self) -> f64 {
        *self as f64
    }

    fn from_string(v: &str) -> Result<Self, ParseError> {
        Ok(v.parse()?)
    }
}

impl FromJsonNumber for i64 {
    fn from_f64(v: f64) -> Self {
        v as i64
    }

    fn to_f64(&self) -> f64 {
        *self as f64
    }

    fn from_string(v: &str) -> Result<Self, ParseError> {
        Ok(v.parse()?)
    }
}

impl FromJsonNumber for f32 {
    fn from_f64(v: f64) -> Self {
        v as f32
    }

    fn to_f64(&self) -> f64 {
        *self as f64
    }

    fn from_string(v: &str) -> Result<Self, ParseError> {
        if v == float::PROTOBUF_JSON_INF {
            Ok(f32::INFINITY)
        } else if v == float::PROTOBUF_JSON_MINUS_INF {
            Ok(f32::NEG_INFINITY)
        } else if v == float::PROTOBUF_JSON_NAN {
            Ok(f32::NAN)
        } else {
            Ok(v.parse()?)
        }
    }
}

impl FromJsonNumber for f64 {
    fn from_f64(v: f64) -> Self {
        v
    }

    fn to_f64(&self) -> f64 {
        *self
    }

    fn from_string(v: &str) -> Result<Self, ParseError> {
        if v == float::PROTOBUF_JSON_INF {
            Ok(f64::INFINITY)
        } else if v == float::PROTOBUF_JSON_MINUS_INF {
            Ok(f64::NEG_INFINITY)
        } else if v == float::PROTOBUF_JSON_NAN {
            Ok(f64::NAN)
        } else {
            Ok(v.parse()?)
        }
    }
}


impl<'a> Parser<'a> {
    fn read_bool(&mut self) -> ParseResult<bool> {
        if self.tokenizer.next_ident_if_eq("true")? {
            Ok(true)
        } else if self.tokenizer.next_ident_if_eq("false")? {
            Ok(false)
        } else {
            Err(ParseError::ExpectingBool)
        }
    }

    fn read_json_number_opt(&mut self) -> ParseResult<Option<JsonNumberLit>> {
        Ok(self.tokenizer.next_token_if_map(|t| {
            match t {
                Token::JsonNumber(v) => Some(v.clone()),
                _ => None,
            }
        })?)
    }

    fn read_number<V : FromJsonNumber>(&mut self) -> ParseResult<V> {
        if let Some(v) = self.read_json_number_opt()? {
            V::from_string(&v.0)
        } else if self.tokenizer.lookahead_is_str_lit()? {
            let v = self.read_string()?;
            V::from_string(&v)
        } else {
            Err(ParseError::ExpectingNumber)
        }
    }

    fn merge_wrapper<W>(&mut self, w: &mut W) -> ParseResult<()>
        where
            W : WellKnownWrapper,
            W::Underlying : FromJsonNumber,
    {
        *w.get_mut() = self.read_number()?;
        Ok(())
    }

    fn merge_bool_value(&mut self, w: &mut BoolValue) -> ParseResult<()> {
        w.value = self.read_bool()?;
        Ok(())
    }

    fn merge_string_value(&mut self, w: &mut StringValue) -> ParseResult<()> {
        w.value = self.read_string()?;
        Ok(())
    }

    fn merge_bytes_value(&mut self, w: &mut BytesValue) -> ParseResult<()> {
        w.value = self.read_bytes()?;
        Ok(())
    }

    fn read_u32(&mut self) -> ParseResult<u32> {
        self.read_number()
    }

    fn read_u64(&mut self) -> ParseResult<u64> {
        self.read_number()
    }

    fn read_i32(&mut self) -> ParseResult<i32> {
        self.read_number()
    }

    fn read_i64(&mut self) -> ParseResult<i64> {
        self.read_number()
    }

    fn read_f32(&mut self) -> ParseResult<f32> {
        self.read_number()
    }

    fn read_f64(&mut self) -> ParseResult<f64> {
        self.read_number()
    }

    fn read_string(&mut self) -> ParseResult<String> {
        let str_lit = self.tokenizer.next_str_lit()?;

        let mut lexer = Lexer::new(&str_lit.escaped, ParserLanguage::Json);
        let mut r = String::new();
        while !lexer.eof() {
            r.push(lexer.next_json_char_value().map_err(ParseError::IncorrectStrLit)?);
        }
        Ok(r)
    }

    fn read_bytes(&mut self) -> ParseResult<Vec<u8>> {
        let s = self.read_string()?;
        Ok(base64::decode(&s)?)
    }

    fn read_enum<'e>(&mut self, descriptor: &'e EnumDescriptor)
        -> ParseResult<&'e EnumValueDescriptor>
    {
        if descriptor.is::<NullValue>() {
            return Ok(self.read_wk_null_value()?.descriptor());
        }

        if self.tokenizer.lookahead_is_str_lit()? {
            let name = self.read_string()?;
            match descriptor.value_by_name(&name) {
                Some(v) => Ok(v),
                None => Err(ParseError::UnknownEnumVariantName(name)),
            }
        } else if self.tokenizer.lookahead_is_json_number()? {
            let number = self.read_i32()?;
            match descriptor.value_by_number(number) {
                Some(v) => Ok(v),
                // TODO: EnumValueOrUnknown
                None => Err(ParseError::UnknownEnumVariantNumber(number)),
            }
        } else {
            Err(ParseError::ExpectingStrOrInt)
        }
    }

    fn read_wk_null_value(&mut self) -> ParseResult<NullValue> {
        self.tokenizer.next_ident_expect_eq("null")?;
        Ok(NullValue::NULL_VALUE)
    }

    fn read_message(&mut self, descriptor: &MessageDescriptor) -> ParseResult<Box<Message>> {
        let mut m = descriptor.new_instance();
        self.merge_inner(&mut *m)?;
        Ok(m)
    }

    fn read_value(&mut self, t: &RuntimeTypeDynamic) -> ParseResult<ReflectValueBox> {
        match t.to_box() {
            RuntimeTypeBox::I32 => self.read_i32().map(ReflectValueBox::from),
            RuntimeTypeBox::I64 => self.read_i64().map(ReflectValueBox::from),
            RuntimeTypeBox::U32 => self.read_u32().map(ReflectValueBox::from),
            RuntimeTypeBox::U64 => self.read_u64().map(ReflectValueBox::from),
            RuntimeTypeBox::F32 => self.read_f32().map(ReflectValueBox::from),
            RuntimeTypeBox::F64 => self.read_f64().map(ReflectValueBox::from),
            RuntimeTypeBox::Bool => self.read_bool().map(ReflectValueBox::from),
            RuntimeTypeBox::String | RuntimeTypeBox::Chars => {
                self.read_string().map(ReflectValueBox::from)
            }
            RuntimeTypeBox::VecU8 | RuntimeTypeBox::CarllercheBytes => {
                self.read_bytes().map(ReflectValueBox::from)
            }
            RuntimeTypeBox::Enum(e) => self.read_enum(e).map(ReflectValueBox::from),
            RuntimeTypeBox::Message(m) => self.read_message(m).map(ReflectValueBox::from),
        }
    }

    fn merge_singular_field(
        &mut self,
        message: &mut Message,
        field: &FieldDescriptor,
        t: &RuntimeTypeDynamic)
        -> ParseResult<()>
    {
        field.set_singular_field(message, self.read_value(t)?);
        Ok(())
    }

    fn merge_repeated_field(
        &mut self,
        message: &mut Message,
        field: &FieldDescriptor,
        t: &RuntimeTypeDynamic)
        -> ParseResult<()>
    {
        let mut repeated = field.mut_repeated(message);
        repeated.clear();

        if self.tokenizer.next_ident_if_eq("null")? {
            return Ok(());
        }

        // TODO: better error reporting on wrong field type
        self.tokenizer.next_symbol_expect_eq('[')?;
        let mut first = true;
        while !self.tokenizer.next_symbol_if_eq(']')? {
            if !first {
                self.tokenizer.next_symbol_expect_eq(',')?;
            }
            first = false;

            repeated.push(self.read_value(t)?);
        }

        Ok(())
    }

    fn merge_map_field(
        &mut self,
        message: &mut Message,
        field: &FieldDescriptor,
        kt: &RuntimeTypeDynamic,
        vt: &RuntimeTypeDynamic)
        -> ParseResult<()>
    {
        let mut map = field.mut_map(message);
        map.clear();

        if self.tokenizer.next_ident_if_eq("null")? {
            return Ok(())
        }

        self.tokenizer.next_symbol_expect_eq('{')?;
        let mut first = true;
        while !self.tokenizer.next_symbol_if_eq('}')? {
            if !first {
                self.tokenizer.next_symbol_expect_eq(',')?;
            }
            first = false;

            let k = self.read_value(kt)?;
            self.tokenizer.next_symbol_expect_eq(':')?;
            let v = self.read_value(vt)?;
            map.insert(k, v);
        }

        Ok(())
    }

    fn merge_field(&mut self, message: &mut Message, field: &FieldDescriptor) -> ParseResult<()> {
        match field.runtime_field_type() {
            RuntimeFieldType::Singular(t) => self.merge_singular_field(message, field, t),
            RuntimeFieldType::Repeated(t) => self.merge_repeated_field(message, field, t),
            RuntimeFieldType::Map(kt, vt) => self.merge_map_field(message, field, kt, vt),
        }
    }

    fn merge_inner(&mut self, message: &mut Message) -> ParseResult<()> {
        if let Some(duration) = message.as_any_mut().downcast_mut() {
            return self.merge_wk_duration(duration);
        }

        if let Some(value) = message.as_any_mut().downcast_mut() {
            return self.merge_wk_value(value);
        }

        if let Some(value) = message.as_any_mut().downcast_mut::<DoubleValue>() {
            return self.merge_wrapper(value);
        }

        if let Some(value) = message.as_any_mut().downcast_mut::<FloatValue>() {
            return self.merge_wrapper(value);
        }

        if let Some(value) = message.as_any_mut().downcast_mut::<Int64Value>() {
            return self.merge_wrapper(value);
        }

        if let Some(value) = message.as_any_mut().downcast_mut::<UInt64Value>() {
            return self.merge_wrapper(value);
        }

        if let Some(value) = message.as_any_mut().downcast_mut::<Int32Value>() {
            return self.merge_wrapper(value);
        }

        if let Some(value) = message.as_any_mut().downcast_mut::<UInt32Value>() {
            return self.merge_wrapper(value);
        }

        if let Some(value) = message.as_any_mut().downcast_mut::<BoolValue>() {
            return self.merge_bool_value(value);
        }

        if let Some(value) = message.as_any_mut().downcast_mut::<StringValue>() {
            return self.merge_string_value(value);
        }

        if let Some(value) = message.as_any_mut().downcast_mut::<BytesValue>() {
            return self.merge_bytes_value(value);
        }

        let descriptor = message.descriptor();

        self.tokenizer.next_symbol_expect_eq('{')?;
        let mut first = true;
        while !self.tokenizer.next_symbol_if_eq('}')? {
            if !first {
                self.tokenizer.next_symbol_expect_eq(',')?;
            }
            first = false;

            let field_name = self.tokenizer.next_ident()?;
            // Proto3 JSON parsers are required to accept both
            // the converted `lowerCamelCase` name and the proto field name.
            let field = match descriptor.field_by_name_or_json_name(&field_name) {
                Some(field) => field,
                // TODO: option to skip unknown types
                None => return Err(ParseError::UnknownFieldName(field_name)),
            };
            self.tokenizer.next_symbol_expect_eq(':')?;
            self.merge_field(message, field)?;
        }
        Ok(())
    }

    fn merge_wk_duration(&mut self, duration: &mut Duration) -> ParseResult<()> {
        let s = self.read_string()?;
        let mut lexer = Lexer::new(&s, ParserLanguage::Json);

        fn next_dec(lexer: &mut Lexer) -> ParseResult<(u64, u32)> {
            let s = lexer.take_while(|c| c >= '0' && c <= '9');

            if s.len() == 0 {
                Ok((0, 0))
            } else {
                match s.parse() {
                    Ok(n) => Ok((n, s.len() as u32)),
                    Err(_) => Err(ParseError::IncorrectDuration),
                }
            }
        }

        let minus = lexer.next_char_if_eq('-');
        let seconds = match next_dec(&mut lexer)? {
            (_, 0) => return Err(ParseError::IncorrectDuration),
            (s, _) => s,
        };
        let nanos =
            if lexer.next_char_if_eq('.') {
                let (mut a, mut b) = next_dec(&mut lexer)?;
                if b > 9 {
                    return Err(ParseError::IncorrectDuration);
                }
                while b != 9 {
                    b += 1;
                    a *= 10;
                }

                if a > 999_999_999 {
                    return Err(ParseError::IncorrectDuration);
                }

                a
            } else {
                0
            };

        // The suffix "s" is required
        if !lexer.next_char_if_eq('s') {
            return Err(ParseError::IncorrectDuration);
        }

        if !lexer.eof() {
            return Err(ParseError::IncorrectDuration);
        }

        if minus {
            duration.seconds = -(seconds as i64);
            duration.nanos = -(nanos as i32);
        } else {
            duration.seconds = seconds as i64;
            duration.nanos = nanos as i32;
        }
        Ok(())
    }

    fn read_wk_list_value(&mut self) -> ParseResult<ListValue> {
        unimplemented!()
    }

    fn read_wk_struct(&mut self) -> ParseResult<Struct> {
        unimplemented!()
    }

    fn merge_wk_value(&mut self, value: &mut Value) -> ParseResult<()> {
        if self.tokenizer.lookahead_is_ident("null")? {
            value.kind = Some(Value_oneof_kind::null_value(self.read_wk_null_value()?));
        } else if self.tokenizer.lookahead_is_ident("true")?
            || self.tokenizer.lookahead_is_ident("false")?
        {
            value.kind = Some(Value_oneof_kind::bool_value(self.read_bool()?));
        } else if self.tokenizer.lookahead_is_json_number()? {
            value.kind = Some(Value_oneof_kind::number_value(self.read_f64()?));
        } else if self.tokenizer.lookahead_is_str_lit()? {
            value.kind = Some(Value_oneof_kind::string_value(self.read_string()?));
        } else if self.tokenizer.lookahead_is_symbol('[')? {
            value.kind = Some(Value_oneof_kind::list_value(self.read_wk_list_value()?));
        } else if self.tokenizer.lookahead_is_symbol('{')? {
            value.kind = Some(Value_oneof_kind::struct_value(self.read_wk_struct()?));
        } else {
            return Err(ParseError::UnexpectedToken);
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

pub fn merge_from_str(message: &mut Message, json: &str) -> ParseWithLocResult<()> {
    let mut parser = Parser {
        tokenizer: Tokenizer::new(json, ParserLanguage::Json),
    };
    parser.merge(message)
}

use std::str;

use protobuf::text_format::lexer::LexerError;
use protobuf::text_format::lexer::NumLit;
use protobuf::text_format::lexer::ParserLanguage;
use protobuf::text_format::lexer::StrLitDecodeError;
use protobuf::text_format::lexer::Token;

use crate::model::*;
use protobuf::text_format::lexer::int;
use protobuf::text_format::lexer::Tokenizer;
use protobuf::text_format::lexer::TokenizerError;

/// Basic information about parsing error.
#[derive(Debug)]
pub enum ParserError {
    TokenizerError(TokenizerError),
    IncorrectInput,
    NotUtf8,
    ExpectConstant,
    UnknownSyntax,
    IntegerOverflow,
    LabelNotAllowed,
    LabelRequired,
    GroupNameShouldStartWithUpperCase,
    MapFieldNotAllowed,
    StrLitDecodeError(StrLitDecodeError),
    LexerError(LexerError),
}

impl From<TokenizerError> for ParserError {
    fn from(e: TokenizerError) -> Self {
        ParserError::TokenizerError(e)
    }
}

impl From<StrLitDecodeError> for ParserError {
    fn from(e: StrLitDecodeError) -> Self {
        ParserError::StrLitDecodeError(e)
    }
}

impl From<LexerError> for ParserError {
    fn from(e: LexerError) -> Self {
        ParserError::LexerError(e)
    }
}

impl From<int::Overflow> for ParserError {
    fn from(_: int::Overflow) -> Self {
        ParserError::IntegerOverflow
    }
}

#[derive(Debug)]
pub struct ParserErrorWithLocation {
    pub error: ParserError,
    /// 1-based
    pub line: u32,
    /// 1-based
    pub col: u32,
}

pub type ParserResult<T> = Result<T, ParserError>;

trait ToU8 {
    fn to_u8(&self) -> ParserResult<u8>;
}

trait ToI32 {
    fn to_i32(&self) -> ParserResult<i32>;
}

trait ToI64 {
    fn to_i64(&self) -> ParserResult<i64>;
}

trait ToChar {
    fn to_char(&self) -> ParserResult<char>;
}

impl ToI32 for u64 {
    fn to_i32(&self) -> ParserResult<i32> {
        if *self <= i32::max_value() as u64 {
            Ok(*self as i32)
        } else {
            Err(ParserError::IntegerOverflow)
        }
    }
}

impl ToI32 for i64 {
    fn to_i32(&self) -> ParserResult<i32> {
        if *self <= i32::max_value() as i64 && *self >= i32::min_value() as i64 {
            Ok(*self as i32)
        } else {
            Err(ParserError::IntegerOverflow)
        }
    }
}

impl ToI64 for u64 {
    fn to_i64(&self) -> Result<i64, ParserError> {
        if *self <= i64::max_value() as u64 {
            Ok(*self as i64)
        } else {
            Err(ParserError::IntegerOverflow)
        }
    }
}

impl ToChar for u8 {
    fn to_char(&self) -> Result<char, ParserError> {
        if *self <= 0x7f {
            Ok(*self as char)
        } else {
            Err(ParserError::NotUtf8)
        }
    }
}

impl ToU8 for u32 {
    fn to_u8(&self) -> Result<u8, ParserError> {
        if *self as u8 as u32 == *self {
            Ok(*self as u8)
        } else {
            Err(ParserError::IntegerOverflow)
        }
    }
}

#[derive(Clone)]
pub(crate) struct Parser<'a> {
    pub tokenizer: Tokenizer<'a>,
    syntax: Syntax,
}

#[derive(Copy, Clone)]
enum MessageBodyParseMode {
    MessageProto2,
    MessageProto3,
    Oneof,
    ExtendProto2,
    ExtendProto3,
}

impl MessageBodyParseMode {
    fn label_allowed(&self, label: Rule) -> bool {
        match label {
            Rule::Repeated => match *self {
                MessageBodyParseMode::MessageProto2
                | MessageBodyParseMode::MessageProto3
                | MessageBodyParseMode::ExtendProto2
                | MessageBodyParseMode::ExtendProto3 => true,
                MessageBodyParseMode::Oneof => false,
            },
            Rule::Optional | Rule::Required => match *self {
                MessageBodyParseMode::MessageProto2 | MessageBodyParseMode::ExtendProto2 => true,
                MessageBodyParseMode::MessageProto3
                | MessageBodyParseMode::ExtendProto3
                | MessageBodyParseMode::Oneof => false,
            },
        }
    }

    fn some_label_required(&self) -> bool {
        match *self {
            MessageBodyParseMode::MessageProto2 | MessageBodyParseMode::ExtendProto2 => true,
            MessageBodyParseMode::MessageProto3
            | MessageBodyParseMode::ExtendProto3
            | MessageBodyParseMode::Oneof => false,
        }
    }

    fn map_allowed(&self) -> bool {
        match *self {
            MessageBodyParseMode::MessageProto2
            | MessageBodyParseMode::MessageProto3
            | MessageBodyParseMode::ExtendProto2
            | MessageBodyParseMode::ExtendProto3 => true,
            MessageBodyParseMode::Oneof => false,
        }
    }

    fn is_most_non_fields_allowed(&self) -> bool {
        match *self {
            MessageBodyParseMode::MessageProto2 | MessageBodyParseMode::MessageProto3 => true,
            MessageBodyParseMode::ExtendProto2
            | MessageBodyParseMode::ExtendProto3
            | MessageBodyParseMode::Oneof => false,
        }
    }

    fn is_option_allowed(&self) -> bool {
        match *self {
            MessageBodyParseMode::MessageProto2
            | MessageBodyParseMode::MessageProto3
            | MessageBodyParseMode::Oneof => true,
            MessageBodyParseMode::ExtendProto2 | MessageBodyParseMode::ExtendProto3 => false,
        }
    }
}

#[derive(Default)]
pub struct MessageBody {
    pub fields: Vec<Field>,
    pub oneofs: Vec<OneOf>,
    pub reserved_nums: Vec<FieldNumberRange>,
    pub reserved_names: Vec<String>,
    pub messages: Vec<Message>,
    pub enums: Vec<Enumeration>,
    pub options: Vec<ProtobufOption>,
}

trait NumLitEx {
    fn to_option_value(&self, sign_is_plus: bool) -> ParserResult<ProtobufConstant>;
}

impl NumLitEx for NumLit {
    fn to_option_value(&self, sign_is_plus: bool) -> ParserResult<ProtobufConstant> {
        Ok(match (*self, sign_is_plus) {
            (NumLit::U64(u), true) => ProtobufConstant::U64(u),
            (NumLit::F64(f), true) => ProtobufConstant::F64(f),
            (NumLit::U64(u), false) => ProtobufConstant::I64(int::neg(u)?),
            (NumLit::F64(f), false) => ProtobufConstant::F64(-f),
        })
    }
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            tokenizer: Tokenizer::new(input, ParserLanguage::Proto),
            syntax: Syntax::Proto2,
        }
    }

    // Protobuf grammar

    // fullIdent = ident { "." ident }
    fn next_full_ident(&mut self) -> ParserResult<String> {
        let mut full_ident = String::new();
        // https://github.com/google/protobuf/issues/4563
        if self.tokenizer.next_symbol_if_eq('.')? {
            full_ident.push('.');
        }
        full_ident.push_str(&self.tokenizer.next_ident()?);
        while self.tokenizer.next_symbol_if_eq('.')? {
            full_ident.push('.');
            full_ident.push_str(&self.tokenizer.next_ident()?);
        }
        Ok(full_ident)
    }

    // emptyStatement = ";"
    fn next_empty_statement_opt(&mut self) -> ParserResult<Option<()>> {
        if self.tokenizer.next_symbol_if_eq(';')? {
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }

    // messageName = ident
    // enumName = ident
    // messageType = [ "." ] { ident "." } messageName
    // enumType = [ "." ] { ident "." } enumName
    fn next_message_or_enum_type(&mut self) -> ParserResult<String> {
        let mut full_name = String::new();
        if self.tokenizer.next_symbol_if_eq('.')? {
            full_name.push('.');
        }
        full_name.push_str(&self.tokenizer.next_ident()?);
        while self.tokenizer.next_symbol_if_eq('.')? {
            full_name.push('.');
            full_name.push_str(&self.tokenizer.next_ident()?);
        }
        Ok(full_name)
    }

    // groupName = capitalLetter { letter | decimalDigit | "_" }
    fn next_group_name(&mut self) -> ParserResult<String> {
        // lexer cannot distinguish between group name and other ident
        let mut clone = self.clone();
        let ident = clone.tokenizer.next_ident()?;
        if !ident.chars().next().unwrap().is_ascii_uppercase() {
            return Err(ParserError::GroupNameShouldStartWithUpperCase);
        }
        *self = clone;
        Ok(ident)
    }

    // Boolean

    // boolLit = "true" | "false"
    fn next_bool_lit_opt(&mut self) -> ParserResult<Option<bool>> {
        Ok(if self.tokenizer.next_ident_if_eq("true")? {
            Some(true)
        } else if self.tokenizer.next_ident_if_eq("false")? {
            Some(false)
        } else {
            None
        })
    }

    // Constant

    fn next_num_lit(&mut self) -> ParserResult<NumLit> {
        self.tokenizer
            .next_token_check_map(|token| Ok(token.to_num_lit()?))
    }

    // constant = fullIdent | ( [ "-" | "+" ] intLit ) | ( [ "-" | "+" ] floatLit ) |
    //            strLit | boolLit
    fn next_constant(&mut self) -> ParserResult<ProtobufConstant> {
        // https://github.com/google/protobuf/blob/a21f225824e994ebd35e8447382ea4e0cd165b3c/src/google/protobuf/unittest_custom_options.proto#L350
        if self.tokenizer.lookahead_is_symbol('{')? {
            return Ok(ProtobufConstant::BracedExpr(self.next_braces()?));
        }

        if let Some(b) = self.next_bool_lit_opt()? {
            return Ok(ProtobufConstant::Bool(b));
        }

        if let &Token::Symbol(c) = self.tokenizer.lookahead_some()? {
            if c == '+' || c == '-' {
                self.tokenizer.advance()?;
                let sign = c == '+';
                return Ok(self.next_num_lit()?.to_option_value(sign)?);
            }
        }

        if let Some(r) = self.tokenizer.next_token_if_map(|token| match token {
            &Token::StrLit(ref s) => Some(ProtobufConstant::String(s.clone())),
            _ => None,
        })? {
            return Ok(r);
        }

        match self.tokenizer.lookahead_some()? {
            &Token::IntLit(..) | &Token::FloatLit(..) => {
                return self.next_num_lit()?.to_option_value(true);
            }
            &Token::Ident(..) => {
                return Ok(ProtobufConstant::Ident(self.next_full_ident()?));
            }
            _ => {}
        }

        Err(ParserError::ExpectConstant)
    }

    fn next_int_lit(&mut self) -> ParserResult<u64> {
        self.tokenizer.next_token_check_map(|token| match token {
            &Token::IntLit(i) => Ok(i),
            _ => Err(ParserError::IncorrectInput),
        })
    }

    // Syntax

    // syntax = "syntax" "=" quote "proto2" quote ";"
    // syntax = "syntax" "=" quote "proto3" quote ";"
    fn next_syntax(&mut self) -> ParserResult<Option<Syntax>> {
        if self.tokenizer.next_ident_if_eq("syntax")? {
            self.tokenizer.next_symbol_expect_eq('=')?;
            let syntax_str = self.tokenizer.next_str_lit()?.decode_utf8()?;
            let syntax = if syntax_str == "proto2" {
                Syntax::Proto2
            } else if syntax_str == "proto3" {
                Syntax::Proto3
            } else {
                return Err(ParserError::UnknownSyntax);
            };
            self.tokenizer.next_symbol_expect_eq(';')?;
            Ok(Some(syntax))
        } else {
            Ok(None)
        }
    }

    // Import Statement

    // import = "import" [ "weak" | "public" ] strLit ";"
    fn next_import_opt(&mut self) -> ParserResult<Option<String>> {
        if self.tokenizer.next_ident_if_eq("import")? {
            self.tokenizer.next_ident_if_in(&["weak", "public"])?;
            let import_path = self.tokenizer.next_str_lit()?.decode_utf8()?;
            self.tokenizer.next_symbol_expect_eq(';')?;
            Ok(Some(import_path))
        } else {
            Ok(None)
        }
    }

    // Package

    // package = "package" fullIdent ";"
    fn next_package_opt(&mut self) -> ParserResult<Option<String>> {
        if self.tokenizer.next_ident_if_eq("package")? {
            let package = self.next_full_ident()?;
            self.tokenizer.next_symbol_expect_eq(';')?;
            Ok(Some(package))
        } else {
            Ok(None)
        }
    }

    // Option

    fn next_ident_or_braced(&mut self) -> ParserResult<String> {
        let mut ident_or_braced = String::new();
        if self.tokenizer.next_symbol_if_eq('(')? {
            ident_or_braced.push('(');
            ident_or_braced.push_str(&self.next_full_ident()?);
            self.tokenizer.next_symbol_expect_eq(')')?;
            ident_or_braced.push(')');
        } else {
            ident_or_braced.push_str(&self.tokenizer.next_ident()?);
        }
        Ok(ident_or_braced)
    }

    // https://github.com/google/protobuf/issues/4563
    // optionName = ( ident | "(" fullIdent ")" ) { "." ident }
    fn next_option_name(&mut self) -> ParserResult<String> {
        let mut option_name = String::new();
        option_name.push_str(&self.next_ident_or_braced()?);
        while self.tokenizer.next_symbol_if_eq('.')? {
            option_name.push('.');
            option_name.push_str(&self.next_ident_or_braced()?);
        }
        Ok(option_name)
    }

    // option = "option" optionName  "=" constant ";"
    fn next_option_opt(&mut self) -> ParserResult<Option<ProtobufOption>> {
        if self.tokenizer.next_ident_if_eq("option")? {
            let name = self.next_option_name()?;
            self.tokenizer.next_symbol_expect_eq('=')?;
            let value = self.next_constant()?;
            self.tokenizer.next_symbol_expect_eq(';')?;
            Ok(Some(ProtobufOption { name, value }))
        } else {
            Ok(None)
        }
    }

    // Fields

    // label = "required" | "optional" | "repeated"
    fn next_label(&mut self, mode: MessageBodyParseMode) -> ParserResult<Rule> {
        let map = &[
            ("optional", Rule::Optional),
            ("required", Rule::Required),
            ("repeated", Rule::Repeated),
        ];
        for &(name, value) in map {
            let mut clone = self.clone();
            if clone.tokenizer.next_ident_if_eq(name)? {
                if !mode.label_allowed(value) {
                    return Err(ParserError::LabelNotAllowed);
                }

                *self = clone;
                return Ok(value);
            }
        }

        if mode.some_label_required() {
            Err(ParserError::LabelRequired)
        } else {
            Ok(Rule::Optional)
        }
    }

    fn next_field_type(&mut self) -> ParserResult<FieldType> {
        let simple = &[
            ("int32", FieldType::Int32),
            ("int64", FieldType::Int64),
            ("uint32", FieldType::Uint32),
            ("uint64", FieldType::Uint64),
            ("sint32", FieldType::Sint32),
            ("sint64", FieldType::Sint64),
            ("fixed32", FieldType::Fixed32),
            ("sfixed32", FieldType::Sfixed32),
            ("fixed64", FieldType::Fixed64),
            ("sfixed64", FieldType::Sfixed64),
            ("bool", FieldType::Bool),
            ("string", FieldType::String),
            ("bytes", FieldType::Bytes),
            ("float", FieldType::Float),
            ("double", FieldType::Double),
        ];
        for &(ref n, ref t) in simple {
            if self.tokenizer.next_ident_if_eq(n)? {
                return Ok(t.clone());
            }
        }

        if let Some(t) = self.next_map_field_type_opt()? {
            return Ok(t);
        }

        let message_or_enum = self.next_message_or_enum_type()?;
        Ok(FieldType::MessageOrEnum(message_or_enum))
    }

    fn next_field_number(&mut self) -> ParserResult<i32> {
        self.tokenizer.next_token_check_map(|token| match token {
            &Token::IntLit(i) => i.to_i32(),
            _ => Err(ParserError::IncorrectInput),
        })
    }

    // fieldOption = optionName "=" constant
    fn next_field_option(&mut self) -> ParserResult<ProtobufOption> {
        let name = self.next_option_name()?;
        self.tokenizer.next_symbol_expect_eq('=')?;
        let value = self.next_constant()?;
        Ok(ProtobufOption { name, value })
    }

    // fieldOptions = fieldOption { ","  fieldOption }
    fn next_field_options(&mut self) -> ParserResult<Vec<ProtobufOption>> {
        let mut options = Vec::new();

        options.push(self.next_field_option()?);

        while self.tokenizer.next_symbol_if_eq(',')? {
            options.push(self.next_field_option()?);
        }

        Ok(options)
    }

    // field = label type fieldName "=" fieldNumber [ "[" fieldOptions "]" ] ";"
    // group = label "group" groupName "=" fieldNumber messageBody
    fn next_field(&mut self, mode: MessageBodyParseMode) -> ParserResult<Field> {
        let rule = if self.clone().tokenizer.next_ident_if_eq("map")? {
            if !mode.map_allowed() {
                return Err(ParserError::MapFieldNotAllowed);
            }
            Rule::Optional
        } else {
            self.next_label(mode)?
        };
        if self.tokenizer.next_ident_if_eq("group")? {
            let name = self.next_group_name()?.to_owned();
            self.tokenizer.next_symbol_expect_eq('=')?;
            let number = self.next_field_number()?;

            let mode = match self.syntax {
                Syntax::Proto2 => MessageBodyParseMode::MessageProto2,
                Syntax::Proto3 => MessageBodyParseMode::MessageProto3,
            };

            let MessageBody { fields, .. } = self.next_message_body(mode)?;

            Ok(Field {
                name,
                rule,
                typ: FieldType::Group(fields),
                number,
                options: Vec::new(),
            })
        } else {
            let typ = self.next_field_type()?;
            let name = self.tokenizer.next_ident()?.to_owned();
            self.tokenizer.next_symbol_expect_eq('=')?;
            let number = self.next_field_number()?;

            let mut options = Vec::new();

            if self.tokenizer.next_symbol_if_eq('[')? {
                for o in self.next_field_options()? {
                    options.push(o);
                }
                self.tokenizer.next_symbol_expect_eq(']')?;
            }
            self.tokenizer.next_symbol_expect_eq(';')?;
            Ok(Field {
                name,
                rule,
                typ,
                number,
                options,
            })
        }
    }

    // oneof = "oneof" oneofName "{" { oneofField | emptyStatement } "}"
    // oneofField = type fieldName "=" fieldNumber [ "[" fieldOptions "]" ] ";"
    fn next_oneof_opt(&mut self) -> ParserResult<Option<OneOf>> {
        if self.tokenizer.next_ident_if_eq("oneof")? {
            let name = self.tokenizer.next_ident()?.to_owned();
            let MessageBody { fields, .. } = self.next_message_body(MessageBodyParseMode::Oneof)?;
            Ok(Some(OneOf { name, fields }))
        } else {
            Ok(None)
        }
    }

    // mapField = "map" "<" keyType "," type ">" mapName "=" fieldNumber [ "[" fieldOptions "]" ] ";"
    // keyType = "int32" | "int64" | "uint32" | "uint64" | "sint32" | "sint64" |
    //           "fixed32" | "fixed64" | "sfixed32" | "sfixed64" | "bool" | "string"
    fn next_map_field_type_opt(&mut self) -> ParserResult<Option<FieldType>> {
        if self.tokenizer.next_ident_if_eq("map")? {
            self.tokenizer.next_symbol_expect_eq('<')?;
            // TODO: restrict key types
            let key = self.next_field_type()?;
            self.tokenizer.next_symbol_expect_eq(',')?;
            let value = self.next_field_type()?;
            self.tokenizer.next_symbol_expect_eq('>')?;
            Ok(Some(FieldType::Map(Box::new((key, value)))))
        } else {
            Ok(None)
        }
    }

    // Extensions and Reserved

    // Extensions

    // range =  intLit [ "to" ( intLit | "max" ) ]
    fn next_range(&mut self) -> ParserResult<FieldNumberRange> {
        let from = self.next_field_number()?;
        let to = if self.tokenizer.next_ident_if_eq("to")? {
            if self.tokenizer.next_ident_if_eq("max")? {
                i32::max_value()
            } else {
                self.next_field_number()?
            }
        } else {
            from
        };
        Ok(FieldNumberRange { from, to })
    }

    // ranges = range { "," range }
    fn next_ranges(&mut self) -> ParserResult<Vec<FieldNumberRange>> {
        let mut ranges = Vec::new();
        ranges.push(self.next_range()?);
        while self.tokenizer.next_symbol_if_eq(',')? {
            ranges.push(self.next_range()?);
        }
        Ok(ranges)
    }

    // extensions = "extensions" ranges ";"
    fn next_extensions_opt(&mut self) -> ParserResult<Option<Vec<FieldNumberRange>>> {
        if self.tokenizer.next_ident_if_eq("extensions")? {
            Ok(Some(self.next_ranges()?))
        } else {
            Ok(None)
        }
    }

    // Reserved

    // Grammar is incorrect: https://github.com/google/protobuf/issues/4558
    // reserved = "reserved" ( ranges | fieldNames ) ";"
    // fieldNames = fieldName { "," fieldName }
    fn next_reserved_opt(&mut self) -> ParserResult<Option<(Vec<FieldNumberRange>, Vec<String>)>> {
        if self.tokenizer.next_ident_if_eq("reserved")? {
            let (ranges, names) = if let &Token::StrLit(..) = self.tokenizer.lookahead_some()? {
                let mut names = Vec::new();
                names.push(self.tokenizer.next_str_lit()?.decode_utf8()?);
                while self.tokenizer.next_symbol_if_eq(',')? {
                    names.push(self.tokenizer.next_str_lit()?.decode_utf8()?);
                }
                (Vec::new(), names)
            } else {
                (self.next_ranges()?, Vec::new())
            };

            self.tokenizer.next_symbol_expect_eq(';')?;

            Ok(Some((ranges, names)))
        } else {
            Ok(None)
        }
    }

    // Top Level definitions

    // Enum definition

    // enumValueOption = optionName "=" constant
    fn next_enum_value_option(&mut self) -> ParserResult<()> {
        self.next_option_name()?;
        self.tokenizer.next_symbol_expect_eq('=')?;
        self.next_constant()?;
        Ok(())
    }

    // https://github.com/google/protobuf/issues/4561
    fn next_enum_value(&mut self) -> ParserResult<i32> {
        let minus = self.tokenizer.next_symbol_if_eq('-')?;
        let lit = self.next_int_lit()?;
        Ok(if minus {
            let unsigned = lit.to_i64()?;
            match unsigned.checked_neg() {
                Some(neg) => neg.to_i32()?,
                None => return Err(ParserError::IntegerOverflow),
            }
        } else {
            lit.to_i32()?
        })
    }

    // enumField = ident "=" intLit [ "[" enumValueOption { ","  enumValueOption } "]" ]";"
    fn next_enum_field(&mut self) -> ParserResult<EnumValue> {
        let name = self.tokenizer.next_ident()?.to_owned();
        self.tokenizer.next_symbol_expect_eq('=')?;
        let number = self.next_enum_value()?;
        if self.tokenizer.next_symbol_if_eq('[')? {
            self.next_enum_value_option()?;
            while self.tokenizer.next_symbol_if_eq(',')? {
                self.next_enum_value_option()?;
            }
            self.tokenizer.next_symbol_expect_eq(']')?;
        }

        Ok(EnumValue { name, number })
    }

    // enum = "enum" enumName enumBody
    // enumBody = "{" { option | enumField | emptyStatement } "}"
    fn next_enum_opt(&mut self) -> ParserResult<Option<Enumeration>> {
        if self.tokenizer.next_ident_if_eq("enum")? {
            let name = self.tokenizer.next_ident()?.to_owned();

            let mut values = Vec::new();
            let mut options = Vec::new();

            self.tokenizer.next_symbol_expect_eq('{')?;
            while self.tokenizer.lookahead_if_symbol()? != Some('}') {
                // emptyStatement
                if self.tokenizer.next_symbol_if_eq(';')? {
                    continue;
                }

                if let Some(o) = self.next_option_opt()? {
                    options.push(o);
                    continue;
                }

                values.push(self.next_enum_field()?);
            }
            self.tokenizer.next_symbol_expect_eq('}')?;
            Ok(Some(Enumeration {
                name,
                values,
                options,
            }))
        } else {
            Ok(None)
        }
    }

    // Message definition

    // messageBody = "{" { field | enum | message | extend | extensions | group |
    //               option | oneof | mapField | reserved | emptyStatement } "}"
    fn next_message_body(&mut self, mode: MessageBodyParseMode) -> ParserResult<MessageBody> {
        self.tokenizer.next_symbol_expect_eq('{')?;

        let mut r = MessageBody::default();

        while self.tokenizer.lookahead_if_symbol()? != Some('}') {
            // emptyStatement
            if self.tokenizer.next_symbol_if_eq(';')? {
                continue;
            }

            if mode.is_most_non_fields_allowed() {
                if let Some((field_nums, field_names)) = self.next_reserved_opt()? {
                    r.reserved_nums.extend(field_nums);
                    r.reserved_names.extend(field_names);
                    continue;
                }

                if let Some(oneof) = self.next_oneof_opt()? {
                    r.oneofs.push(oneof);
                    continue;
                }

                if let Some(_extensions) = self.next_extensions_opt()? {
                    continue;
                }

                if let Some(_extend) = self.next_extend_opt()? {
                    continue;
                }

                if let Some(nested_message) = self.next_message_opt()? {
                    r.messages.push(nested_message);
                    continue;
                }

                if let Some(nested_enum) = self.next_enum_opt()? {
                    r.enums.push(nested_enum);
                    continue;
                }
            } else {
                self.tokenizer.next_ident_if_eq_error("reserved")?;
                self.tokenizer.next_ident_if_eq_error("oneof")?;
                self.tokenizer.next_ident_if_eq_error("extensions")?;
                self.tokenizer.next_ident_if_eq_error("extend")?;
                self.tokenizer.next_ident_if_eq_error("message")?;
                self.tokenizer.next_ident_if_eq_error("enum")?;
            }

            if mode.is_option_allowed() {
                if let Some(option) = self.next_option_opt()? {
                    r.options.push(option);
                    continue;
                }
            } else {
                self.tokenizer.next_ident_if_eq_error("option")?;
            }

            r.fields.push(self.next_field(mode)?);
        }

        self.tokenizer.next_symbol_expect_eq('}')?;

        Ok(r)
    }

    // message = "message" messageName messageBody
    fn next_message_opt(&mut self) -> ParserResult<Option<Message>> {
        if self.tokenizer.next_ident_if_eq("message")? {
            let name = self.tokenizer.next_ident()?.to_owned();

            let mode = match self.syntax {
                Syntax::Proto2 => MessageBodyParseMode::MessageProto2,
                Syntax::Proto3 => MessageBodyParseMode::MessageProto3,
            };

            let MessageBody {
                fields,
                oneofs,
                reserved_nums,
                reserved_names,
                messages,
                enums,
                options,
            } = self.next_message_body(mode)?;

            Ok(Some(Message {
                name,
                fields,
                oneofs,
                reserved_nums,
                reserved_names,
                messages,
                enums,
                options,
            }))
        } else {
            Ok(None)
        }
    }

    // Extend

    // extend = "extend" messageType "{" {field | group | emptyStatement} "}"
    fn next_extend_opt(&mut self) -> ParserResult<Option<Vec<Extension>>> {
        let mut clone = self.clone();
        if clone.tokenizer.next_ident_if_eq("extend")? {
            // According to spec `extend` is only for `proto2`, but it is used in `proto3`
            // https://github.com/google/protobuf/issues/4610

            *self = clone;

            let extendee = self.next_message_or_enum_type()?;

            let mode = match self.syntax {
                Syntax::Proto2 => MessageBodyParseMode::ExtendProto2,
                Syntax::Proto3 => MessageBodyParseMode::ExtendProto3,
            };

            let MessageBody { fields, .. } = self.next_message_body(mode)?;

            let extensions = fields
                .into_iter()
                .map(|field| {
                    let extendee = extendee.clone();
                    Extension { extendee, field }
                })
                .collect();

            Ok(Some(extensions))
        } else {
            Ok(None)
        }
    }

    // Service definition

    fn next_braces(&mut self) -> ParserResult<String> {
        let mut r = String::new();
        self.tokenizer.next_symbol_expect_eq('{')?;
        r.push('{');
        loop {
            if self.tokenizer.lookahead_if_symbol()? == Some('{') {
                r.push_str(&self.next_braces()?);
                continue;
            }
            let next = self.tokenizer.next_some()?;
            r.push_str(&next.format());
            if let Token::Symbol('}') = next {
                break;
            }
        }
        Ok(r)
    }

    fn next_options_or_colon(&mut self) -> ParserResult<Vec<ProtobufOption>> {
        let mut options = Vec::new();
        if self.tokenizer.next_symbol_if_eq('{')? {
            while self.tokenizer.lookahead_if_symbol()? != Some('}') {
                if let Some(option) = self.next_option_opt()? {
                    options.push(option);
                    continue;
                }

                if let Some(()) = self.next_empty_statement_opt()? {
                    continue;
                }

                return Err(ParserError::IncorrectInput);
            }
            self.tokenizer.next_symbol_expect_eq('}')?;
        } else {
            self.tokenizer.next_symbol_expect_eq(';')?;
        }

        Ok(options)
    }

    // stream = "stream" streamName "(" messageType "," messageType ")"
    //        (( "{" { option | emptyStatement } "}") | ";" )
    fn next_stream_opt(&mut self) -> ParserResult<Option<Method>> {
        assert_eq!(Syntax::Proto2, self.syntax);
        if self.tokenizer.next_ident_if_eq("stream")? {
            let name = self.tokenizer.next_ident()?;
            self.tokenizer.next_symbol_expect_eq('(')?;
            let input_type = self.tokenizer.next_ident()?;
            self.tokenizer.next_symbol_expect_eq(',')?;
            let output_type = self.tokenizer.next_ident()?;
            self.tokenizer.next_symbol_expect_eq(')')?;
            let options = self.next_options_or_colon()?;
            Ok(Some(Method {
                name,
                input_type,
                output_type,
                client_streaming: true,
                server_streaming: true,
                options,
            }))
        } else {
            Ok(None)
        }
    }

    // rpc = "rpc" rpcName "(" [ "stream" ] messageType ")"
    //     "returns" "(" [ "stream" ] messageType ")"
    //     (( "{" { option | emptyStatement } "}" ) | ";" )
    fn next_rpc_opt(&mut self) -> ParserResult<Option<Method>> {
        if self.tokenizer.next_ident_if_eq("rpc")? {
            let name = self.tokenizer.next_ident()?;
            self.tokenizer.next_symbol_expect_eq('(')?;
            let client_streaming = self.tokenizer.next_ident_if_eq("stream")?;
            let input_type = self.next_message_or_enum_type()?;
            self.tokenizer.next_symbol_expect_eq(')')?;
            self.tokenizer.next_ident_expect_eq("returns")?;
            self.tokenizer.next_symbol_expect_eq('(')?;
            let server_streaming = self.tokenizer.next_ident_if_eq("stream")?;
            let output_type = self.next_message_or_enum_type()?;
            self.tokenizer.next_symbol_expect_eq(')')?;
            let options = self.next_options_or_colon()?;
            Ok(Some(Method {
                name,
                input_type,
                output_type,
                client_streaming,
                server_streaming,
                options,
            }))
        } else {
            Ok(None)
        }
    }

    // proto2:
    // service = "service" serviceName "{" { option | rpc | stream | emptyStatement } "}"
    //
    // proto3:
    // service = "service" serviceName "{" { option | rpc | emptyStatement } "}"
    fn next_service_opt(&mut self) -> ParserResult<Option<Service>> {
        if self.tokenizer.next_ident_if_eq("service")? {
            let name = self.tokenizer.next_ident()?;
            let mut methods = Vec::new();
            let mut options = Vec::new();
            self.tokenizer.next_symbol_expect_eq('{')?;
            while self.tokenizer.lookahead_if_symbol()? != Some('}') {
                if let Some(method) = self.next_rpc_opt()? {
                    methods.push(method);
                    continue;
                }

                if self.syntax == Syntax::Proto2 {
                    if let Some(method) = self.next_stream_opt()? {
                        methods.push(method);
                        continue;
                    }
                }

                if let Some(o) = self.next_option_opt()? {
                    options.push(o);
                    continue;
                }

                if let Some(()) = self.next_empty_statement_opt()? {
                    continue;
                }

                return Err(ParserError::IncorrectInput);
            }
            self.tokenizer.next_symbol_expect_eq('}')?;
            Ok(Some(Service {
                name,
                methods,
                options,
            }))
        } else {
            Ok(None)
        }
    }

    // Proto file

    // proto = syntax { import | package | option | topLevelDef | emptyStatement }
    // topLevelDef = message | enum | extend | service
    pub fn next_proto(&mut self) -> ParserResult<FileDescriptor> {
        let syntax = self.next_syntax()?.unwrap_or(Syntax::Proto2);
        self.syntax = syntax;

        let mut import_paths = Vec::new();
        let mut package = String::new();
        let mut messages = Vec::new();
        let mut enums = Vec::new();
        let mut extensions = Vec::new();
        let mut options = Vec::new();
        let mut services = Vec::new();

        while !self.tokenizer.syntax_eof()? {
            if let Some(import_path) = self.next_import_opt()? {
                import_paths.push(import_path);
                continue;
            }

            if let Some(next_package) = self.next_package_opt()? {
                package = next_package.to_owned();
                continue;
            }

            if let Some(option) = self.next_option_opt()? {
                options.push(option);
                continue;
            }

            if let Some(message) = self.next_message_opt()? {
                messages.push(message);
                continue;
            }

            if let Some(enumeration) = self.next_enum_opt()? {
                enums.push(enumeration);
                continue;
            }

            if let Some(more_extensions) = self.next_extend_opt()? {
                extensions.extend(more_extensions);
                continue;
            }

            if let Some(service) = self.next_service_opt()? {
                services.push(service);
                continue;
            }

            if self.tokenizer.next_symbol_if_eq(';')? {
                continue;
            }

            return Err(ParserError::IncorrectInput);
        }

        Ok(FileDescriptor {
            import_paths,
            package,
            syntax,
            messages,
            enums,
            extensions,
            services,
            options,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse<P, R>(input: &str, parse_what: P) -> R
    where
        P: FnOnce(&mut Parser) -> ParserResult<R>,
    {
        let mut parser = Parser::new(input);
        let r =
            parse_what(&mut parser).expect(&format!("parse failed at {}", parser.tokenizer.loc()));
        let eof = parser
            .tokenizer
            .syntax_eof()
            .expect(&format!("check eof failed at {}", parser.tokenizer.loc()));
        assert!(eof, "{}", parser.tokenizer.loc());
        r
    }

    fn parse_opt<P, R>(input: &str, parse_what: P) -> R
    where
        P: FnOnce(&mut Parser) -> ParserResult<Option<R>>,
    {
        let mut parser = Parser::new(input);
        let o =
            parse_what(&mut parser).expect(&format!("parse failed at {}", parser.tokenizer.loc()));
        let r = o.expect(&format!(
            "parser returned none at {}",
            parser.tokenizer.loc()
        ));
        assert!(parser.tokenizer.syntax_eof().unwrap());
        r
    }

    #[test]
    fn test_syntax() {
        let msg = r#"  syntax = "proto3";  "#;
        let mess = parse_opt(msg, |p| p.next_syntax());
        assert_eq!(Syntax::Proto3, mess);
    }

    #[test]
    fn test_field_default_value_int() {
        let msg = r#"  optional int64 f = 4 [default = 12];  "#;
        let mess = parse(msg, |p| p.next_field(MessageBodyParseMode::MessageProto2));
        assert_eq!("f", mess.name);
        assert_eq!("default", mess.options[0].name);
        assert_eq!("12", mess.options[0].value.format());
    }

    #[test]
    fn test_field_default_value_float() {
        let msg = r#"  optional float f = 2 [default = 10.0];  "#;
        let mess = parse(msg, |p| p.next_field(MessageBodyParseMode::MessageProto2));
        assert_eq!("f", mess.name);
        assert_eq!("default", mess.options[0].name);
        assert_eq!("10.0", mess.options[0].value.format());
    }

    #[test]
    fn test_message() {
        let msg = r#"message ReferenceData
    {
        repeated ScenarioInfo  scenarioSet = 1;
        repeated CalculatedObjectInfo calculatedObjectSet = 2;
        repeated RiskFactorList riskFactorListSet = 3;
        repeated RiskMaturityInfo riskMaturitySet = 4;
        repeated IndicatorInfo indicatorSet = 5;
        repeated RiskStrikeInfo riskStrikeSet = 6;
        repeated FreeProjectionList freeProjectionListSet = 7;
        repeated ValidationProperty ValidationSet = 8;
        repeated CalcProperties calcPropertiesSet = 9;
        repeated MaturityInfo maturitySet = 10;
    }"#;

        let mess = parse_opt(msg, |p| p.next_message_opt());
        assert_eq!(10, mess.fields.len());
    }

    #[test]
    fn test_enum() {
        let msg = r#"enum PairingStatus {
                DEALPAIRED        = 0;
                INVENTORYORPHAN   = 1;
                CALCULATEDORPHAN  = 2;
                CANCELED          = 3;
    }"#;

        let enumeration = parse_opt(msg, |p| p.next_enum_opt());
        assert_eq!(4, enumeration.values.len());
    }

    #[test]
    fn test_ignore() {
        let msg = r#"option optimize_for = SPEED;"#;

        parse_opt(msg, |p| p.next_option_opt());
    }

    #[test]
    fn test_import() {
        let msg = r#"syntax = "proto3";

    import "test_import_nested_imported_pb.proto";

    message ContainsImportedNested {
        ContainerForNested.NestedMessage m = 1;
        ContainerForNested.NestedEnum e = 2;
    }
    "#;
        let desc = parse(msg, |p| p.next_proto());

        assert_eq!(
            vec!["test_import_nested_imported_pb.proto"],
            desc.import_paths
        );
    }

    #[test]
    fn test_package() {
        let msg = r#"
        package foo.bar;

    message ContainsImportedNested {
        optional ContainerForNested.NestedMessage m = 1;
        optional ContainerForNested.NestedEnum e = 2;
    }
    "#;
        let desc = parse(msg, |p| p.next_proto());
        assert_eq!("foo.bar".to_string(), desc.package);
    }

    #[test]
    fn test_nested_message() {
        let msg = r#"message A
    {
        message B {
            repeated int32 a = 1;
            optional string b = 2;
        }
        optional string b = 1;
    }"#;

        let mess = parse_opt(msg, |p| p.next_message_opt());
        assert_eq!(1, mess.messages.len());
    }

    #[test]
    fn test_map() {
        let msg = r#"message A
    {
        optional map<string, int32> b = 1;
    }"#;

        let mess = parse_opt(msg, |p| p.next_message_opt());
        assert_eq!(1, mess.fields.len());
        match mess.fields[0].typ {
            FieldType::Map(ref f) => match &**f {
                &(FieldType::String, FieldType::Int32) => (),
                ref f => panic!("Expecting Map<String, Int32> found {:?}", f),
            },
            ref f => panic!("Expecting map, got {:?}", f),
        }
    }

    #[test]
    fn test_oneof() {
        let msg = r#"message A
    {
        optional int32 a1 = 1;
        oneof a_oneof {
            string a2 = 2;
            int32 a3 = 3;
            bytes a4 = 4;
        }
        repeated bool a5 = 5;
    }"#;

        let mess = parse_opt(msg, |p| p.next_message_opt());
        assert_eq!(1, mess.oneofs.len());
        assert_eq!(3, mess.oneofs[0].fields.len());
    }

    #[test]
    fn test_reserved() {
        let msg = r#"message Sample {
       reserved 4, 15, 17 to 20, 30;
       reserved "foo", "bar";
       optional uint64 age =1;
       required bytes name =2;
    }"#;

        let mess = parse_opt(msg, |p| p.next_message_opt());
        assert_eq!(
            vec![
                FieldNumberRange { from: 4, to: 4 },
                FieldNumberRange { from: 15, to: 15 },
                FieldNumberRange { from: 17, to: 20 },
                FieldNumberRange { from: 30, to: 30 }
            ],
            mess.reserved_nums
        );
        assert_eq!(
            vec!["foo".to_string(), "bar".to_string()],
            mess.reserved_names
        );
        assert_eq!(2, mess.fields.len());
    }

    #[test]
    fn test_default_value_int() {
        let msg = r#"message Sample {
            optional int32 x = 1 [default = 17];
        }"#;

        let mess = parse_opt(msg, |p| p.next_message_opt());
        assert_eq!("default", mess.fields[0].options[0].name);
        assert_eq!("17", mess.fields[0].options[0].value.format());
    }

    #[test]
    fn test_default_value_string() {
        let msg = r#"message Sample {
            optional string x = 1 [default = "ab\nc d\"g\'h\0\"z"];
        }"#;

        let mess = parse_opt(msg, |p| p.next_message_opt());
        assert_eq!(
            r#""ab\nc d\"g\'h\0\"z""#,
            mess.fields[0].options[0].value.format()
        );
    }

    #[test]
    fn test_default_value_bytes() {
        let msg = r#"message Sample {
            optional bytes x = 1 [default = "ab\nc d\xfeE\"g\'h\0\"z"];
        }"#;

        let mess = parse_opt(msg, |p| p.next_message_opt());
        assert_eq!(
            r#""ab\nc d\xfeE\"g\'h\0\"z""#,
            mess.fields[0].options[0].value.format()
        );
    }

    #[test]
    fn test_group() {
        let msg = r#"message MessageWithGroup {
            optional string aaa = 1;

            repeated group Identifier = 18 {
                optional int32 iii = 19;
                optional string sss = 20;
            }

            required int bbb = 3;
        }"#;
        let mess = parse_opt(msg, |p| p.next_message_opt());

        assert_eq!("Identifier", mess.fields[1].name);
        if let FieldType::Group(ref group_fields) = mess.fields[1].typ {
            assert_eq!(2, group_fields.len());
        } else {
            panic!("expecting group");
        }

        assert_eq!("bbb", mess.fields[2].name);
    }

    #[test]
    fn test_service() {
        let msg = r#"service SearchService {
            rpc Search (SearchRequest) returns (SearchResponse);
        }"#;
        let service = parse_opt(msg, |p| p.next_service_opt());

        assert_eq!("SearchService", service.name);
        assert_eq!(1, service.methods.len());
        let method = &service.methods[0];
        assert_eq!("Search", method.name);
        assert_eq!("SearchRequest", method.input_type);
        assert_eq!("SearchResponse", method.output_type);
        assert!(!method.client_streaming);
        assert!(!method.server_streaming);
    }

    #[test]
    fn test_incorrect_file_descriptor() {
        let msg = r#"
            message Foo {}

            dfgdg
        "#;

        let err = FileDescriptor::parse(msg).err().expect("err");
        assert_eq!(4, err.line);
    }

    #[test]
    fn test_extend() {
        let proto = r#"
            syntax = "proto2";

            extend google.protobuf.FileOptions {
                optional bool foo = 17001;
                optional string bar = 17002;
            }

            extend google.protobuf.MessageOptions {
                optional bool baz = 17003;
            }
        "#;

        let fd = FileDescriptor::parse(proto).expect("fd");
        assert_eq!(3, fd.extensions.len());
        assert_eq!("google.protobuf.FileOptions", fd.extensions[0].extendee);
        assert_eq!("google.protobuf.FileOptions", fd.extensions[1].extendee);
        assert_eq!("google.protobuf.MessageOptions", fd.extensions[2].extendee);
        assert_eq!(17003, fd.extensions[2].field.number);
    }
}

use std::str;

use protobuf::text_format::lexer::int;
use protobuf::text_format::lexer::LexerError;
use protobuf::text_format::lexer::NumLit;
use protobuf::text_format::lexer::ParserLanguage;
use protobuf::text_format::lexer::StrLitDecodeError;
use protobuf::text_format::lexer::Token;
use protobuf::text_format::lexer::Tokenizer;
use protobuf::text_format::lexer::TokenizerError;
use protobuf_codegen::ProtobufAbsolutePath;
use protobuf_codegen::ProtobufIdent;
use protobuf_codegen::ProtobufPath;
use protobuf_codegen::ProtobufRelativePath;

use crate::fmt;
use crate::model::*;

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
    OneOfInGroup,
    OneOfInOneOf,
    OneOfInExtend,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::TokenizerError(e) => write!(f, "{}", e),
            // TODO
            ParserError::IncorrectInput => write!(f, "incorrect input"),
            ParserError::NotUtf8 => write!(f, "not UTF-8"),
            ParserError::ExpectConstant => write!(f, "expecting a constant"),
            ParserError::UnknownSyntax => write!(f, "unknown syntax"),
            ParserError::IntegerOverflow => write!(f, "integer overflow"),
            ParserError::LabelNotAllowed => write!(f, "label not allowed"),
            ParserError::LabelRequired => write!(f, "label required"),
            ParserError::GroupNameShouldStartWithUpperCase => {
                write!(f, "group name should start with upper case")
            }
            ParserError::MapFieldNotAllowed => write!(f, "map field not allowed"),
            ParserError::StrLitDecodeError(e) => write!(f, "string literal decode error: {}", e),
            ParserError::LexerError(e) => write!(f, "lexer error: {}", e),
            ParserError::OneOfInGroup => write!(f, "oneof in group"),
            ParserError::OneOfInOneOf => write!(f, "oneof in oneof"),
            ParserError::OneOfInExtend => write!(f, "oneof in extend"),
        }
    }
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

#[derive(Debug, thiserror::Error)]
#[error("at {line}:{col}: {error}")]
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
    pub fields: Vec<WithLoc<FieldOrOneOf>>,
    pub reserved_nums: Vec<FieldNumberRange>,
    pub reserved_names: Vec<String>,
    pub messages: Vec<WithLoc<Message>>,
    pub enums: Vec<Enumeration>,
    pub options: Vec<ProtobufOption>,
    pub extension_ranges: Vec<FieldNumberRange>,
    pub extensions: Vec<WithLoc<Extension>>,
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
    fn next_full_ident(&mut self) -> ParserResult<ProtobufPath> {
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
        Ok(ProtobufPath::new(full_ident))
    }

    // fullIdent = ident { "." ident }
    fn next_full_ident_rel(&mut self) -> ParserResult<ProtobufRelativePath> {
        let mut full_ident = String::new();
        full_ident.push_str(&self.tokenizer.next_ident()?);
        while self.tokenizer.next_symbol_if_eq('.')? {
            full_ident.push('.');
            full_ident.push_str(&self.tokenizer.next_ident()?);
        }
        Ok(ProtobufRelativePath::new(full_ident))
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
    fn next_message_or_enum_type(&mut self) -> ParserResult<ProtobufPath> {
        self.next_full_ident()
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

    fn next_message_constant(&mut self) -> ParserResult<ProtobufConstantMessage> {
        let mut r = ProtobufConstantMessage::default();
        self.tokenizer.next_symbol_expect_eq('{')?;
        while !self.tokenizer.lookahead_is_symbol('}')? {
            if self.tokenizer.next_symbol_if_eq('[')? {
                let n = self.next_full_ident()?;
                self.tokenizer.next_symbol_expect_eq(']')?;
                let v = self.next_message_constant()?;
                r.extensions.insert(format!("{}", n), v);
            } else {
                let n = self.tokenizer.next_ident()?;
                let v = if self.tokenizer.next_symbol_if_eq(':')? {
                    self.next_constant()?
                } else {
                    ProtobufConstant::Message(self.next_message_constant()?)
                };
                r.fields.insert(n, v);
            }
        }
        self.tokenizer.next_symbol_expect_eq('}')?;
        Ok(r)
    }

    // constant = fullIdent | ( [ "-" | "+" ] intLit ) | ( [ "-" | "+" ] floatLit ) |
    //            strLit | boolLit
    fn next_constant(&mut self) -> ParserResult<ProtobufConstant> {
        // https://github.com/google/protobuf/blob/a21f225824e994ebd35e8447382ea4e0cd165b3c/src/google/protobuf/unittest_custom_options.proto#L350
        if self.tokenizer.lookahead_is_symbol('{')? {
            return Ok(ProtobufConstant::Message(self.next_message_constant()?));
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
    fn next_import_opt(&mut self) -> ParserResult<Option<Import>> {
        if self.tokenizer.next_ident_if_eq("import")? {
            let vis = if self.tokenizer.next_ident_if_eq("weak")? {
                ImportVis::Weak
            } else if self.tokenizer.next_ident_if_eq("public")? {
                ImportVis::Public
            } else {
                ImportVis::Default
            };
            let path = self.tokenizer.next_str_lit()?.decode_utf8()?;
            self.tokenizer.next_symbol_expect_eq(';')?;
            Ok(Some(Import { path, vis }))
        } else {
            Ok(None)
        }
    }

    // Package

    // package = "package" fullIdent ";"
    fn next_package_opt(&mut self) -> ParserResult<Option<ProtobufAbsolutePath>> {
        if self.tokenizer.next_ident_if_eq("package")? {
            let package = self.next_full_ident_rel()?;
            self.tokenizer.next_symbol_expect_eq(';')?;
            Ok(Some(package.into_absolute()))
        } else {
            Ok(None)
        }
    }

    // Option

    fn next_ident(&mut self) -> ParserResult<ProtobufIdent> {
        Ok(ProtobufIdent::from(self.tokenizer.next_ident()?))
    }

    fn next_option_name_component(&mut self) -> ParserResult<ProtobufOptionNameComponent> {
        if self.tokenizer.next_symbol_if_eq('(')? {
            let comp = self.next_full_ident()?;
            self.tokenizer.next_symbol_expect_eq(')')?;
            Ok(ProtobufOptionNameComponent::Ext(comp))
        } else {
            Ok(ProtobufOptionNameComponent::Direct(self.next_ident()?))
        }
    }

    // https://github.com/google/protobuf/issues/4563
    // optionName = ( ident | "(" fullIdent ")" ) { "." ident }
    fn next_option_name(&mut self) -> ParserResult<ProtobufOptionName> {
        let mut components = Vec::new();
        components.push(self.next_option_name_component()?);
        while self.tokenizer.next_symbol_if_eq('.')? {
            components.push(self.next_option_name_component()?);
        }
        if components.len() == 1 {
            if let ProtobufOptionNameComponent::Direct(n) = &components[0] {
                return Ok(ProtobufOptionName::Builtin(n.clone()));
            }
        }
        Ok(ProtobufOptionName::Ext(ProtobufOptionNameExt(components)))
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
        // TODO: not all integers are valid field numbers
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
    fn next_field(&mut self, mode: MessageBodyParseMode) -> ParserResult<WithLoc<Field>> {
        let loc = self.tokenizer.lookahead_loc();
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

            let fields = fields
                .into_iter()
                .map(|fo| match fo.t {
                    FieldOrOneOf::Field(f) => Ok(f),
                    FieldOrOneOf::OneOf(_) => Err(ParserError::OneOfInGroup),
                })
                .collect::<Result<_, ParserError>>()?;

            let field = Field {
                // The field name is a lowercased version of the type name
                // (which has been verified to start with an uppercase letter).
                // https://git.io/JvxAP
                name: name.to_ascii_lowercase(),
                rule,
                typ: FieldType::Group(Group { name, fields }),
                number,
                options: Vec::new(),
            };
            Ok(WithLoc { t: field, loc })
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
            let field = Field {
                name,
                rule,
                typ,
                number,
                options,
            };
            Ok(WithLoc { t: field, loc })
        }
    }

    // oneof = "oneof" oneofName "{" { oneofField | emptyStatement } "}"
    // oneofField = type fieldName "=" fieldNumber [ "[" fieldOptions "]" ] ";"
    fn next_oneof_opt(&mut self) -> ParserResult<Option<OneOf>> {
        if self.tokenizer.next_ident_if_eq("oneof")? {
            let name = self.tokenizer.next_ident()?.to_owned();
            let MessageBody {
                fields, options, ..
            } = self.next_message_body(MessageBodyParseMode::Oneof)?;
            let fields = fields
                .into_iter()
                .map(|fo| match fo.t {
                    FieldOrOneOf::Field(f) => Ok(f),
                    FieldOrOneOf::OneOf(_) => Err(ParserError::OneOfInOneOf),
                })
                .collect::<Result<_, ParserError>>()?;
            Ok(Some(OneOf {
                name,
                fields,
                options,
            }))
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
                0x20000000 - 1
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
    fn next_enum_value_option(&mut self) -> ParserResult<ProtobufOption> {
        let name = self.next_option_name()?;
        self.tokenizer.next_symbol_expect_eq('=')?;
        let value = self.next_constant()?;
        Ok(ProtobufOption { name, value })
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
        let mut options = Vec::new();
        if self.tokenizer.next_symbol_if_eq('[')? {
            options.push(self.next_enum_value_option()?);
            while self.tokenizer.next_symbol_if_eq(',')? {
                options.push(self.next_enum_value_option()?);
            }
            self.tokenizer.next_symbol_expect_eq(']')?;
        }

        Ok(EnumValue {
            name,
            number,
            options,
        })
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
            let loc = self.tokenizer.lookahead_loc();

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
                    let one_of = FieldOrOneOf::OneOf(oneof);
                    r.fields.push(WithLoc { t: one_of, loc });
                    continue;
                }

                if let Some(extension_ranges) = self.next_extensions_opt()? {
                    r.extension_ranges.extend(extension_ranges);
                    continue;
                }

                if let Some(extensions) = self.next_extend_opt()? {
                    r.extensions.extend(extensions);
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

            let field = FieldOrOneOf::Field(self.next_field(mode)?);
            r.fields.push(WithLoc { t: field, loc });
        }

        self.tokenizer.next_symbol_expect_eq('}')?;

        Ok(r)
    }

    // message = "message" messageName messageBody
    fn next_message_opt(&mut self) -> ParserResult<Option<WithLoc<Message>>> {
        let loc = self.tokenizer.lookahead_loc();

        if self.tokenizer.next_ident_if_eq("message")? {
            let name = self.tokenizer.next_ident()?.to_owned();

            let mode = match self.syntax {
                Syntax::Proto2 => MessageBodyParseMode::MessageProto2,
                Syntax::Proto3 => MessageBodyParseMode::MessageProto3,
            };

            let MessageBody {
                fields,
                reserved_nums,
                reserved_names,
                messages,
                enums,
                options,
                extensions,
                extension_ranges,
            } = self.next_message_body(mode)?;

            let message = Message {
                name,
                fields,
                reserved_nums,
                reserved_names,
                messages,
                enums,
                options,
                extensions,
                extension_ranges,
            };
            Ok(Some(WithLoc { t: message, loc }))
        } else {
            Ok(None)
        }
    }

    // Extend

    // extend = "extend" messageType "{" {field | group | emptyStatement} "}"
    fn next_extend_opt(&mut self) -> ParserResult<Option<Vec<WithLoc<Extension>>>> {
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

            // TODO: is oneof allowed in extend?
            let fields: Vec<WithLoc<Field>> = fields
                .into_iter()
                .map(|fo| match fo.t {
                    FieldOrOneOf::Field(f) => Ok(f),
                    FieldOrOneOf::OneOf(_) => Err(ParserError::OneOfInExtend),
                })
                .collect::<Result<_, ParserError>>()?;

            let extensions = fields
                .into_iter()
                .map(|field| {
                    let extendee = extendee.clone();
                    let loc = field.loc;
                    let extension = Extension { extendee, field };
                    WithLoc { t: extension, loc }
                })
                .collect();

            Ok(Some(extensions))
        } else {
            Ok(None)
        }
    }

    // Service definition

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
            let input_type = self.next_message_or_enum_type()?;
            self.tokenizer.next_symbol_expect_eq(',')?;
            let output_type = self.next_message_or_enum_type()?;
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
    fn next_service_opt(&mut self) -> ParserResult<Option<WithLoc<Service>>> {
        let loc = self.tokenizer.lookahead_loc();

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
            Ok(Some(WithLoc {
                loc,
                t: Service {
                    name,
                    methods,
                    options,
                },
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

        let mut imports = Vec::new();
        let mut package = ProtobufAbsolutePath::root();
        let mut messages = Vec::new();
        let mut enums = Vec::new();
        let mut extensions = Vec::new();
        let mut options = Vec::new();
        let mut services = Vec::new();

        while !self.tokenizer.syntax_eof()? {
            if let Some(import) = self.next_import_opt()? {
                imports.push(import);
                continue;
            }

            if let Some(next_package) = self.next_package_opt()? {
                package = next_package;
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
            imports,
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
        assert_eq!("f", mess.t.name);
        assert_eq!(
            ProtobufOptionName::simple("default"),
            mess.t.options[0].name
        );
        assert_eq!("12", mess.t.options[0].value.format());
    }

    #[test]
    fn test_field_default_value_float() {
        let msg = r#"  optional float f = 2 [default = 10.0];  "#;
        let mess = parse(msg, |p| p.next_field(MessageBodyParseMode::MessageProto2));
        assert_eq!("f", mess.t.name);
        assert_eq!(
            ProtobufOptionName::simple("default"),
            mess.t.options[0].name
        );
        assert_eq!("10", mess.t.options[0].value.format());
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
        assert_eq!(10, mess.t.fields.len());
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
            desc.imports.into_iter().map(|i| i.path).collect::<Vec<_>>()
        );
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
        assert_eq!(1, mess.t.messages.len());
    }

    #[test]
    fn test_map() {
        let msg = r#"message A
    {
        optional map<string, int32> b = 1;
    }"#;

        let mess = parse_opt(msg, |p| p.next_message_opt());
        assert_eq!(1, mess.t.fields.len());
        match mess.t.regular_fields_for_test()[0].typ {
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
        assert_eq!(1, mess.t.oneofs_for_test().len());
        assert_eq!(3, mess.t.oneofs_for_test()[0].fields.len());
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
            mess.t.reserved_nums
        );
        assert_eq!(
            vec!["foo".to_string(), "bar".to_string()],
            mess.t.reserved_names
        );
        assert_eq!(2, mess.t.fields.len());
    }

    #[test]
    fn test_default_value_int() {
        let msg = r#"message Sample {
            optional int32 x = 1 [default = 17];
        }"#;

        let mess = parse_opt(msg, |p| p.next_message_opt());
        assert_eq!(
            ProtobufOptionName::simple("default"),
            mess.t.regular_fields_for_test()[0].options[0].name
        );
        assert_eq!(
            "17",
            mess.t.regular_fields_for_test()[0].options[0]
                .value
                .format()
        );
    }

    #[test]
    fn test_default_value_string() {
        let msg = r#"message Sample {
            optional string x = 1 [default = "ab\nc d\"g\'h\0\"z"];
        }"#;

        let mess = parse_opt(msg, |p| p.next_message_opt());
        assert_eq!(
            r#""ab\nc d\"g\'h\0\"z""#,
            mess.t.regular_fields_for_test()[0].options[0]
                .value
                .format()
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
            mess.t.regular_fields_for_test()[0].options[0]
                .value
                .format()
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

        assert_eq!("identifier", mess.t.regular_fields_for_test()[1].name);
        if let FieldType::Group(Group { fields, .. }) = &mess.t.regular_fields_for_test()[1].typ {
            assert_eq!(2, fields.len());
        } else {
            panic!("expecting group");
        }

        assert_eq!("bbb", mess.t.regular_fields_for_test()[2].name);
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
}

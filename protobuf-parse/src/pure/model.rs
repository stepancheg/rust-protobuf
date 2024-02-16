//! A nom-based protobuf file parser
//!
//! This crate can be seen as a rust transcription of the
//! [descriptor.proto](https://github.com/google/protobuf/blob/master/src/google/protobuf/descriptor.proto) file

use std::fmt;
use std::fmt::Write;
use std::ops::Deref;
use std::ops::RangeInclusive;

use indexmap::IndexMap;
use protobuf::reflect::ReflectValueBox;
use protobuf::reflect::RuntimeType;
use protobuf_support::lexer::float::format_protobuf_float;
use protobuf_support::lexer::loc::Loc;
use protobuf_support::lexer::str_lit::StrLit;

use crate::model;
use crate::proto_path::ProtoPathBuf;
use crate::protobuf_abs_path::ProtobufAbsPath;
use crate::protobuf_ident::ProtobufIdent;
use crate::protobuf_path::ProtobufPath;
use crate::pure::parser::Parser;
pub use crate::pure::parser::ParserErrorWithLocation;

#[derive(thiserror::Error, Debug)]
enum ModelError {
    #[error("cannot convert value `{1}` to type `{0}`")]
    InconvertibleValue(RuntimeType, model::ProtobufConstant),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct WithLoc<T> {
    pub loc: Loc,
    pub t: T,
}

impl<T> Deref for WithLoc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

impl<T> WithLoc<T> {
    pub fn with_loc(loc: Loc) -> impl FnOnce(T) -> WithLoc<T> {
        move |t| WithLoc {
            t,
            loc: loc.clone(),
        }
    }
}

/// Protobuf syntax.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum Syntax {
    /// Protobuf syntax [2](https://developers.google.com/protocol-buffers/docs/proto) (default)
    Proto2,
    /// Protobuf syntax [3](https://developers.google.com/protocol-buffers/docs/proto3)
    Proto3,
}

impl Default for Syntax {
    fn default() -> Syntax {
        Syntax::Proto2
    }
}

/// A field rule
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum Rule {
    /// A well-formed message can have zero or one of this field (but not more than one).
    Optional,
    /// This field can be repeated any number of times (including zero) in a well-formed message.
    /// The order of the repeated values will be preserved.
    Repeated,
    /// A well-formed message must have exactly one of this field.
    Required,
}

impl Rule {
    pub(crate) const ALL: [Rule; 3] = [Rule::Optional, Rule::Repeated, Rule::Required];

    pub(crate) const fn as_str(&self) -> &'static str {
        match self {
            Rule::Optional => "optional",
            Rule::Repeated => "repeated",
            Rule::Required => "required",
        }
    }
}

/// Protobuf group
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Group {
    /// Group name
    pub name: String,
    pub fields: Vec<WithLoc<Field>>,
}

/// Protobuf supported field types
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FieldType {
    /// Protobuf int32
    ///
    /// # Remarks
    ///
    /// Uses variable-length encoding. Inefficient for encoding negative numbers – if
    /// your field is likely to have negative values, use sint32 instead.
    Int32,
    /// Protobuf int64
    ///
    /// # Remarks
    ///
    /// Uses variable-length encoding. Inefficient for encoding negative numbers – if
    /// your field is likely to have negative values, use sint64 instead.
    Int64,
    /// Protobuf uint32
    ///
    /// # Remarks
    ///
    /// Uses variable-length encoding.
    Uint32,
    /// Protobuf uint64
    ///
    /// # Remarks
    ///
    /// Uses variable-length encoding.
    Uint64,
    /// Protobuf sint32
    ///
    /// # Remarks
    ///
    /// Uses ZigZag variable-length encoding. Signed int value. These more efficiently
    /// encode negative numbers than regular int32s.
    Sint32,
    /// Protobuf sint64
    ///
    /// # Remarks
    ///
    /// Uses ZigZag variable-length encoding. Signed int value. These more efficiently
    /// encode negative numbers than regular int32s.
    Sint64,
    /// Protobuf bool
    Bool,
    /// Protobuf fixed64
    ///
    /// # Remarks
    ///
    /// Always eight bytes. More efficient than uint64 if values are often greater than 2^56.
    Fixed64,
    /// Protobuf sfixed64
    ///
    /// # Remarks
    ///
    /// Always eight bytes.
    Sfixed64,
    /// Protobuf double
    Double,
    /// Protobuf string
    ///
    /// # Remarks
    ///
    /// A string must always contain UTF-8 encoded or 7-bit ASCII text.
    String,
    /// Protobuf bytes
    ///
    /// # Remarks
    ///
    /// May contain any arbitrary sequence of bytes.
    Bytes,
    /// Protobut fixed32
    ///
    /// # Remarks
    ///
    /// Always four bytes. More efficient than uint32 if values are often greater than 2^28.
    Fixed32,
    /// Protobut sfixed32
    ///
    /// # Remarks
    ///
    /// Always four bytes.
    Sfixed32,
    /// Protobut float
    Float,
    /// Protobuf message or enum (holds the name)
    MessageOrEnum(ProtobufPath),
    /// Protobut map
    Map(Box<(FieldType, FieldType)>),
    /// Protobuf group (deprecated)
    Group(Group),
}

/// A Protobuf Field
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Field {
    /// Field name
    pub name: String,
    /// Field `Rule`
    pub rule: Option<Rule>,
    /// Field type
    pub typ: FieldType,
    /// Tag number
    pub number: i32,
    /// Non-builtin options
    pub options: Vec<ProtobufOption>,
}

/// A Protobuf field of oneof group
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FieldOrOneOf {
    Field(WithLoc<Field>),
    OneOf(OneOf),
}

/// A protobuf message
#[derive(Debug, Clone, Default)]
pub(crate) struct Message {
    /// Message name
    pub name: String,
    /// Message fields and oneofs
    pub fields: Vec<WithLoc<FieldOrOneOf>>,
    /// Message reserved numbers
    pub reserved_nums: Vec<RangeInclusive<i32>>,
    /// Message reserved names
    pub reserved_names: Vec<String>,
    /// Nested messages
    pub messages: Vec<WithLoc<Message>>,
    /// Nested enums
    pub enums: Vec<WithLoc<Enumeration>>,
    /// Non-builtin options
    pub options: Vec<ProtobufOption>,
    /// Extension field numbers
    pub extension_ranges: Vec<RangeInclusive<i32>>,
    /// Extensions
    pub extensions: Vec<WithLoc<Extension>>,
}

impl Message {
    pub fn regular_fields_including_in_oneofs(&self) -> Vec<&WithLoc<Field>> {
        self.fields
            .iter()
            .flat_map(|fo| match &fo.t {
                FieldOrOneOf::Field(f) => vec![f],
                FieldOrOneOf::OneOf(o) => o.fields.iter().collect(),
            })
            .collect()
    }

    /** Find a field by name. */
    pub fn field_by_name(&self, name: &str) -> Option<&Field> {
        self.regular_fields_including_in_oneofs()
            .iter()
            .find(|f| f.t.name == name)
            .map(|f| &f.t)
    }

    pub fn _nested_extensions(&self) -> Vec<&Group> {
        self.regular_fields_including_in_oneofs()
            .into_iter()
            .flat_map(|f| match &f.t.typ {
                FieldType::Group(g) => Some(g),
                _ => None,
            })
            .collect()
    }

    #[cfg(test)]
    pub fn regular_fields_for_test(&self) -> Vec<&Field> {
        self.fields
            .iter()
            .flat_map(|fo| match &fo.t {
                FieldOrOneOf::Field(f) => Some(&f.t),
                FieldOrOneOf::OneOf(_) => None,
            })
            .collect()
    }

    pub(crate) fn oneofs(&self) -> Vec<&OneOf> {
        self.fields
            .iter()
            .flat_map(|fo| match &fo.t {
                FieldOrOneOf::Field(_) => None,
                FieldOrOneOf::OneOf(o) => Some(o),
            })
            .collect()
    }
}

/// A protobuf enumeration field
#[derive(Debug, Clone)]
pub(crate) struct EnumValue {
    /// enum value name
    pub name: String,
    /// enum value number
    pub number: i32,
    /// enum value options
    pub options: Vec<ProtobufOption>,
}

/// A protobuf enumerator
#[derive(Debug, Clone)]
pub(crate) struct Enumeration {
    /// enum name
    pub name: String,
    /// enum values
    pub values: Vec<EnumValue>,
    /// enum options
    pub options: Vec<ProtobufOption>,
    /// enum reserved numbers
    pub reserved_nums: Vec<RangeInclusive<i32>>,
    /// enum reserved names
    pub reserved_names: Vec<String>,
}

/// A OneOf
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct OneOf {
    /// OneOf name
    pub name: String,
    /// OneOf fields
    pub fields: Vec<WithLoc<Field>>,
    /// oneof options
    pub options: Vec<ProtobufOption>,
}

#[derive(Debug, Clone)]
pub(crate) struct Extension {
    /// Extend this type with field
    pub extendee: ProtobufPath,
    /// Extension field
    pub field: WithLoc<Field>,
}

/// Service method
#[derive(Debug, Clone)]
pub(crate) struct Method {
    /// Method name
    pub name: String,
    /// Input type
    pub input_type: ProtobufPath,
    /// Output type
    pub output_type: ProtobufPath,
    /// If this method is client streaming
    #[allow(dead_code)] // TODO
    pub client_streaming: bool,
    /// If this method is server streaming
    #[allow(dead_code)] // TODO
    pub server_streaming: bool,
    /// Method options
    pub options: Vec<ProtobufOption>,
}

/// Service definition
#[derive(Debug, Clone)]
pub(crate) struct Service {
    /// Service name
    pub name: String,
    pub methods: Vec<Method>,
    pub options: Vec<ProtobufOption>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct AnyTypeUrl {
    pub(crate) prefix: String,
    pub(crate) full_type_name: ProtobufPath,
}

impl fmt::Display for AnyTypeUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.prefix, self.full_type_name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ProtobufConstantMessageFieldName {
    Regular(String),
    Extension(ProtobufPath),
    AnyTypeUrl(AnyTypeUrl),
}

impl fmt::Display for ProtobufConstantMessageFieldName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtobufConstantMessageFieldName::Regular(s) => write!(f, "{}", s),
            ProtobufConstantMessageFieldName::Extension(p) => write!(f, "[{}]", p),
            ProtobufConstantMessageFieldName::AnyTypeUrl(a) => write!(f, "[{}]", a),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct ProtobufConstantMessage {
    pub(crate) fields: IndexMap<ProtobufConstantMessageFieldName, ProtobufConstant>,
}

/// constant = fullIdent | ( [ "-" | "+" ] intLit ) | ( [ "-" | "+" ] floatLit ) |
//                 strLit | boolLit
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ProtobufConstant {
    U64(u64),
    I64(i64),
    F64(f64), // TODO: eq
    Bool(bool),
    Ident(ProtobufPath),
    String(StrLit),
    Message(ProtobufConstantMessage),
}

impl fmt::Display for ProtobufConstant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtobufConstant::U64(v) => write!(f, "{}", v),
            ProtobufConstant::I64(v) => write!(f, "{}", v),
            ProtobufConstant::F64(v) => write!(f, "{}", format_protobuf_float(*v)),
            ProtobufConstant::Bool(v) => write!(f, "{}", v),
            ProtobufConstant::Ident(v) => write!(f, "{}", v),
            ProtobufConstant::String(v) => write!(f, "{}", v),
            // TODO: text format explicitly
            ProtobufConstant::Message(v) => write!(f, "{:?}", v),
        }
    }
}

impl ProtobufConstantMessage {
    pub fn format(&self) -> String {
        let mut s = String::new();
        write!(s, "{{").unwrap();
        for (n, v) in &self.fields {
            match v {
                ProtobufConstant::Message(m) => write!(s, "{} {}", n, m.format()).unwrap(),
                v => write!(s, "{}: {}", n, v.format()).unwrap(),
            }
        }
        write!(s, "}}").unwrap();
        s
    }
}

impl ProtobufConstant {
    pub fn format(&self) -> String {
        match *self {
            ProtobufConstant::U64(u) => u.to_string(),
            ProtobufConstant::I64(i) => i.to_string(),
            ProtobufConstant::F64(f) => format_protobuf_float(f),
            ProtobufConstant::Bool(b) => b.to_string(),
            ProtobufConstant::Ident(ref i) => format!("{}", i),
            ProtobufConstant::String(ref s) => s.quoted(),
            ProtobufConstant::Message(ref s) => s.format(),
        }
    }

    /** Interpret .proto constant as an reflection value. */
    pub fn as_type(&self, ty: RuntimeType) -> anyhow::Result<ReflectValueBox> {
        match (self, &ty) {
            (ProtobufConstant::Ident(ident), RuntimeType::Enum(e)) => {
                if let Some(v) = e.value_by_name(&ident.to_string()) {
                    return Ok(ReflectValueBox::Enum(e.clone(), v.value()));
                }
            }
            (ProtobufConstant::Bool(b), RuntimeType::Bool) => return Ok(ReflectValueBox::Bool(*b)),
            (ProtobufConstant::String(lit), RuntimeType::String) => {
                return Ok(ReflectValueBox::String(lit.decode_utf8()?))
            }
            _ => {}
        }
        Err(ModelError::InconvertibleValue(ty.clone(), self.clone()).into())
    }
}

/// Equivalent of `UninterpretedOption.NamePart`.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ProtobufOptionNamePart {
    Direct(ProtobufIdent),
    Ext(ProtobufPath),
}

impl fmt::Display for ProtobufOptionNamePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtobufOptionNamePart::Direct(n) => write!(f, "{}", n),
            ProtobufOptionNamePart::Ext(n) => write!(f, "({})", n),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ProtobufOptionNameExt(pub Vec<ProtobufOptionNamePart>);

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ProtobufOptionName {
    Builtin(ProtobufIdent),
    Ext(ProtobufOptionNameExt),
}

impl ProtobufOptionName {
    pub fn simple(name: &str) -> ProtobufOptionName {
        ProtobufOptionName::Builtin(ProtobufIdent::new(name))
    }
}

impl fmt::Display for ProtobufOptionNameExt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, comp) in self.0.iter().enumerate() {
            if index != 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", comp)?;
        }
        Ok(())
    }
}

impl fmt::Display for ProtobufOptionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtobufOptionName::Builtin(n) => write!(f, "{}", n),
            ProtobufOptionName::Ext(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ProtobufOption {
    pub name: ProtobufOptionName,
    pub value: ProtobufConstant,
}

/// Visibility of import statement
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum ImportVis {
    Default,
    Public,
    Weak,
}

impl Default for ImportVis {
    fn default() -> Self {
        ImportVis::Default
    }
}

/// Import statement
#[derive(Debug, Default, Clone)]
pub(crate) struct Import {
    pub path: ProtoPathBuf,
    pub vis: ImportVis,
}

/// A File descriptor representing a whole .proto file
#[derive(Debug, Default, Clone)]
pub(crate) struct FileDescriptor {
    /// Imports
    pub imports: Vec<Import>,
    /// Package
    pub package: ProtobufAbsPath,
    /// Protobuf Syntax
    pub syntax: Syntax,
    /// Top level messages
    pub messages: Vec<WithLoc<Message>>,
    /// Enums
    pub enums: Vec<WithLoc<Enumeration>>,
    /// Extensions
    pub extensions: Vec<WithLoc<Extension>>,
    /// Services
    pub services: Vec<WithLoc<Service>>,
    /// Non-builtin options
    pub options: Vec<ProtobufOption>,
}

impl FileDescriptor {
    /// Parses a .proto file content into a `FileDescriptor`
    pub fn parse<S: AsRef<str>>(file: S) -> Result<Self, ParserErrorWithLocation> {
        let mut parser = Parser::new(file.as_ref());
        match parser.next_proto() {
            Ok(r) => Ok(r),
            Err(error) => {
                let Loc { line, col } = parser.tokenizer.loc();
                Err(ParserErrorWithLocation { error, line, col })
            }
        }
    }
}

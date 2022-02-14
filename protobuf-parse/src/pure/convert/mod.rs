//! Convert parser model to rust-protobuf model

mod type_resolver;

use protobuf;
use protobuf::descriptor::descriptor_proto::ReservedRange;
use protobuf::descriptor::field_descriptor_proto;
use protobuf::descriptor::field_descriptor_proto::Type;
use protobuf::descriptor::DescriptorProto;
use protobuf::descriptor::FieldDescriptorProto;
use protobuf::json::json_name;
use protobuf::reflect::RuntimeTypeBox;
use protobuf::text_format::lexer::StrLitDecodeError;
use protobuf::text_format::quote_bytes_to;
use protobuf::Message;
use protobuf::UnknownFields;
use protobuf::UnknownValue;

use crate::case_convert::camel_case;
use crate::model::ProtobufConstantMessageFieldName;
use crate::path::fs_path_to_proto_path;
use crate::proto_path::ProtoPath;
use crate::protobuf_abs_path::ProtobufAbsPath;
use crate::protobuf_ident::ProtobufIdent;
use crate::protobuf_path::ProtobufPath;
use crate::pure::convert::type_resolver::LookupScopeUnion;
use crate::pure::convert::type_resolver::MessageOrEnum;
use crate::pure::convert::type_resolver::TypeResolver;
use crate::pure::model;
use crate::pure::model::ProtobufConstant;
use crate::pure::model::ProtobufOptionName;
use crate::pure::model::ProtobufOptionNameExt;
use crate::pure::model::ProtobufOptionNamePart;
use crate::FileDescriptorPair;
use crate::ProtobufAbsPathRef;
use crate::ProtobufIdentRef;
use crate::ProtobufRelPathRef;

#[derive(Debug, thiserror::Error)]
pub(crate) enum ConvertError {
    #[error(transparent)]
    OtherError(anyhow::Error),
    #[error("builtin option {0} not found for options {1}")]
    BuiltinOptionNotFound(String, String),
    #[error("builtin option {0} points to a non-singular field of {1}")]
    BuiltinOptionPointsToNonSingularField(String, String),
    #[error("extension not found: {0}")]
    ExtensionNotFound(String),
    #[error("extension is not a message: {0}")]
    ExtensionIsNotMessage(String),
    // TODO: what are a, b?
    #[error("wrong extension type: option {0} extendee {1} expected extendee {2}")]
    WrongExtensionType(String, String, String),
    #[error("unsupported extension type: {0} {1} {2}")]
    UnsupportedExtensionType(String, String, model::ProtobufConstant),
    #[error("incorrect string literal: {0}")]
    StrLitDecodeError(#[source] StrLitDecodeError),
    #[error("default value is not a string literal")]
    DefaultValueIsNotStringLiteral,
    #[error("wrong option type, expecting {0}, got `{1}`")]
    WrongOptionType(&'static str, String),
    #[error("cannot convert value {1} to type {0}")]
    InconvertibleValue(RuntimeTypeBox, model::ProtobufConstant),
    #[error("constants of this type are not implemented")]
    ConstantsOfTypeMessageEnumGroupNotImplemented,
    #[error("expecting a message for name {0}")]
    ExpectingMessage(ProtobufAbsPath),
    #[error("expecting an enum for name {0}")]
    ExpectingEnum(ProtobufAbsPath),
    #[error("unknown enum value: {0}")]
    UnknownEnumValue(String),
    #[error("unknown field name: {0}")]
    UnknownFieldName(String),
}

impl From<StrLitDecodeError> for ConvertError {
    fn from(e: StrLitDecodeError) -> Self {
        ConvertError::StrLitDecodeError(e)
    }
}

trait ProtobufOptions {
    fn by_name(&self, name: &str) -> Option<&model::ProtobufConstant>;

    fn by_name_bool(&self, name: &str) -> anyhow::Result<Option<bool>> {
        match self.by_name(name) {
            Some(model::ProtobufConstant::Bool(b)) => Ok(Some(*b)),
            Some(c) => Err(ConvertError::WrongOptionType("bool", c.to_string()).into()),
            None => Ok(None),
        }
    }

    fn by_name_string(&self, name: &str) -> anyhow::Result<Option<String>> {
        match self.by_name(name) {
            Some(model::ProtobufConstant::String(s)) => s
                .decode_utf8()
                .map(Some)
                .map_err(|e| ConvertError::StrLitDecodeError(e).into()),
            Some(c) => Err(ConvertError::WrongOptionType("string", c.to_string()).into()),
            None => Ok(None),
        }
    }
}

impl<'a> ProtobufOptions for &'a [model::ProtobufOption] {
    fn by_name(&self, name: &str) -> Option<&model::ProtobufConstant> {
        let option_name = ProtobufOptionName::simple(name);
        for model::ProtobufOption { name, value } in *self {
            if name == &option_name {
                return Some(&value);
            }
        }
        None
    }
}

pub struct WithFullName<T> {
    full_name: ProtobufAbsPath,
    t: T,
}

#[derive(Debug, PartialEq)]
enum TypeResolved {
    Int32,
    Int64,
    Uint32,
    Uint64,
    Sint32,
    Sint64,
    Bool,
    Fixed64,
    Sfixed64,
    Double,
    String,
    Bytes,
    Fixed32,
    Sfixed32,
    Float,
    Message(ProtobufAbsPath),
    Enum(ProtobufAbsPath),
    Group(ProtobufAbsPath),
}

impl TypeResolved {
    fn from_field(field: &FieldDescriptorProto) -> TypeResolved {
        match field.get_field_type() {
            Type::TYPE_DOUBLE => TypeResolved::Double,
            Type::TYPE_FLOAT => TypeResolved::Float,
            Type::TYPE_INT64 => TypeResolved::Int64,
            Type::TYPE_UINT64 => TypeResolved::Uint64,
            Type::TYPE_INT32 => TypeResolved::Int32,
            Type::TYPE_FIXED64 => TypeResolved::Fixed64,
            Type::TYPE_FIXED32 => TypeResolved::Fixed32,
            Type::TYPE_UINT32 => TypeResolved::Uint32,
            Type::TYPE_SFIXED32 => TypeResolved::Sfixed32,
            Type::TYPE_SFIXED64 => TypeResolved::Sfixed64,
            Type::TYPE_SINT32 => TypeResolved::Sint32,
            Type::TYPE_SINT64 => TypeResolved::Sint64,
            Type::TYPE_BOOL => TypeResolved::Bool,
            Type::TYPE_STRING => TypeResolved::String,
            Type::TYPE_BYTES => TypeResolved::Bytes,
            Type::TYPE_GROUP => {
                assert!(!field.get_type_name().is_empty());
                TypeResolved::Group(ProtobufAbsPath::new(field.get_type_name()))
            }
            Type::TYPE_ENUM => {
                assert!(!field.get_type_name().is_empty());
                TypeResolved::Enum(ProtobufAbsPath::new(field.get_type_name()))
            }
            Type::TYPE_MESSAGE => {
                assert!(!field.get_type_name().is_empty());
                TypeResolved::Message(ProtobufAbsPath::new(field.get_type_name()))
            }
        }
    }

    fn type_enum(&self) -> Type {
        match self {
            TypeResolved::Bool => Type::TYPE_BOOL,
            TypeResolved::Int32 => Type::TYPE_INT32,
            TypeResolved::Int64 => Type::TYPE_INT64,
            TypeResolved::Uint32 => Type::TYPE_UINT32,
            TypeResolved::Uint64 => Type::TYPE_UINT64,
            TypeResolved::Sint32 => Type::TYPE_SINT32,
            TypeResolved::Sint64 => Type::TYPE_SINT64,
            TypeResolved::Fixed32 => Type::TYPE_FIXED32,
            TypeResolved::Fixed64 => Type::TYPE_FIXED64,
            TypeResolved::Sfixed32 => Type::TYPE_SFIXED32,
            TypeResolved::Sfixed64 => Type::TYPE_SFIXED64,
            TypeResolved::Float => Type::TYPE_FLOAT,
            TypeResolved::Double => Type::TYPE_DOUBLE,
            TypeResolved::String => Type::TYPE_STRING,
            TypeResolved::Bytes => Type::TYPE_BYTES,
            TypeResolved::Message(_) => Type::TYPE_MESSAGE,
            TypeResolved::Enum(_) => Type::TYPE_ENUM,
            TypeResolved::Group(_) => Type::TYPE_GROUP,
        }
    }

    fn type_name(&self) -> Option<&ProtobufAbsPath> {
        match self {
            TypeResolved::Message(t) | TypeResolved::Enum(t) | TypeResolved::Group(t) => Some(t),
            _ => None,
        }
    }
}

struct Resolver<'a> {
    type_resolver: TypeResolver<'a>,
    current_file: &'a model::FileDescriptor,
}

impl<'a> Resolver<'a> {
    fn map_entry_name_for_field_name(field_name: &str) -> ProtobufIdent {
        // Field name and message name must match, otherwise
        // Google's validation fails.
        // https://git.io/JeOvF
        ProtobufIdent::from(format!("{}Entry", camel_case(field_name)))
    }

    fn map_entry_field(
        &self,
        scope: &ProtobufAbsPath,
        name: &str,
        number: i32,
        field_type: &model::FieldType,
    ) -> anyhow::Result<protobuf::descriptor::FieldDescriptorProto> {
        // should be consisent with DescriptorBuilder::ValidateMapEntry

        let mut output = protobuf::descriptor::FieldDescriptorProto::new();

        output.set_name(name.to_owned());
        output.set_number(number);

        let t = self.field_type(&scope, name, field_type)?;
        output.set_field_type(t.type_enum());
        if let Some(t_name) = t.type_name() {
            output.set_type_name(t_name.path.clone());
        }

        output.set_label(field_descriptor_proto::Label::LABEL_OPTIONAL);

        output.set_json_name(json_name(&name));

        Ok(output)
    }

    fn map_entry_message(
        &self,
        scope: &ProtobufAbsPath,
        field_name: &str,
        key: &model::FieldType,
        value: &model::FieldType,
    ) -> anyhow::Result<protobuf::descriptor::DescriptorProto> {
        let mut output = protobuf::descriptor::DescriptorProto::new();

        output.options.mut_or_default().set_map_entry(true);
        output.set_name(Resolver::map_entry_name_for_field_name(field_name).into_string());
        output
            .field
            .push(self.map_entry_field(&scope, "key", 1, key)?);
        output
            .field
            .push(self.map_entry_field(&scope, "value", 2, value)?);

        Ok(output)
    }

    fn group_message(
        &self,
        scope: &ProtobufAbsPath,
        name: &str,
        fields: &[model::WithLoc<model::Field>],
    ) -> anyhow::Result<protobuf::descriptor::DescriptorProto> {
        let mut output = protobuf::descriptor::DescriptorProto::new();

        output.set_name(name.to_owned());

        for f in fields {
            output.field.push(self.field(scope, f, None)?);
        }

        Ok(output)
    }

    fn message_options(
        &self,
        scope: &ProtobufAbsPathRef,
        input: &[model::ProtobufOption],
    ) -> anyhow::Result<Option<protobuf::descriptor::MessageOptions>> {
        self.custom_options(scope, input)
    }

    fn message(
        &self,
        scope: &ProtobufAbsPathRef,
        input: &model::Message,
    ) -> anyhow::Result<protobuf::descriptor::DescriptorProto> {
        let mut nested_scope = scope.to_owned();
        nested_scope.push_simple(ProtobufIdentRef::new(&input.name));

        let mut output = protobuf::descriptor::DescriptorProto::new();
        output.set_name(input.name.clone());

        let mut nested_messages = Vec::new();

        for m in &input.messages {
            let message = self.message(&nested_scope, &m.t)?;
            nested_messages.push(model::WithLoc {
                t: message,
                loc: m.loc,
            });
        }

        for f in input.regular_fields_including_in_oneofs() {
            match &f.t.typ {
                model::FieldType::Map(t) => {
                    let message = self.map_entry_message(&nested_scope, &f.t.name, &t.0, &t.1)?;
                    nested_messages.push(model::WithLoc {
                        t: message,
                        loc: f.loc,
                    });
                }
                model::FieldType::Group(model::Group {
                    name: group_name,
                    fields,
                    ..
                }) => {
                    let message = self.group_message(&nested_scope, group_name, fields)?;
                    nested_messages.push(model::WithLoc {
                        t: message,
                        loc: f.loc,
                    });
                }
                _ => (),
            }
        }

        // Preserve declaration order
        nested_messages.sort_by_key(|m| m.loc);
        output.nested_type = nested_messages
            .into_iter()
            .map(|model::WithLoc { t, .. }| t)
            .collect();

        output.enum_type = input
            .enums
            .iter()
            .map(|e| self.enumeration(scope, e))
            .collect::<Result<_, _>>()?;

        {
            let mut fields = Vec::new();

            for fo in &input.fields {
                match &fo.t {
                    model::FieldOrOneOf::Field(f) => {
                        fields.push(self.field(&nested_scope, f, None)?);
                    }
                    model::FieldOrOneOf::OneOf(o) => {
                        let oneof_index = output.oneof_decl.len();
                        for f in &o.fields {
                            fields.push(self.field(&nested_scope, f, Some(oneof_index as i32))?);
                        }
                        output.oneof_decl.push(self.oneof(scope, o)?);
                    }
                }
            }

            output.field = fields;
        }

        output.options = self.message_options(scope, &input.options)?.into();

        for ext in &input.extension_ranges {
            let mut extension_range = protobuf::descriptor::descriptor_proto::ExtensionRange::new();
            extension_range.set_start(ext.from);
            extension_range.set_end(ext.to + 1);
            output.extension_range.push(extension_range);
        }
        for ext in &input.extensions {
            let mut extension = self.field(scope, &ext.t.field, None)?;
            extension.set_extendee(
                self.type_resolver
                    .resolve_message_or_enum(scope, &ext.t.extendee)?
                    .full_name
                    .path,
            );
            output.extension.push(extension);
        }

        for reserved in &input.reserved_nums {
            let mut reserved_range = ReservedRange::new();
            reserved_range.set_start(reserved.from);
            reserved_range.set_end(reserved.to + 1);
            output.reserved_range.push(reserved_range);
        }
        output.reserved_name = input.reserved_names.clone().into();

        Ok(output)
    }

    fn service_options(
        &self,
        input: &[model::ProtobufOption],
    ) -> anyhow::Result<Option<protobuf::descriptor::ServiceOptions>> {
        self.custom_options(&self.current_file.package, input)
    }

    fn service_method_options(
        &self,
        input: &[model::ProtobufOption],
    ) -> anyhow::Result<Option<protobuf::descriptor::MethodOptions>> {
        self.custom_options(&self.current_file.package, input)
    }

    fn service_method(
        &self,
        input: &model::Method,
    ) -> anyhow::Result<protobuf::descriptor::MethodDescriptorProto> {
        let scope = &self.current_file.package;
        let mut output = protobuf::descriptor::MethodDescriptorProto::new();
        output.set_name(input.name.clone());
        output.options = self.service_method_options(&input.options)?.into();
        output.set_input_type(
            self.type_resolver
                .resolve_message_or_enum(scope, &input.input_type)?
                .full_name
                .to_string(),
        );
        output.set_output_type(
            self.type_resolver
                .resolve_message_or_enum(scope, &input.output_type)?
                .full_name
                .to_string(),
        );
        Ok(output)
    }

    fn service(
        &self,
        input: &model::Service,
    ) -> anyhow::Result<protobuf::descriptor::ServiceDescriptorProto> {
        let mut output = protobuf::descriptor::ServiceDescriptorProto::new();
        output.set_name(input.name.clone());
        output.options = self.service_options(&input.options)?.into();

        output.method = input
            .methods
            .iter()
            .map(|m| self.service_method(m))
            .collect::<Result<_, _>>()?;

        Ok(output)
    }

    fn custom_option_builtin<M>(
        &self,
        _scope: &ProtobufAbsPathRef,
        options: &mut M,
        option: &ProtobufIdent,
        option_value: &ProtobufConstant,
    ) -> anyhow::Result<()>
    where
        M: Message,
    {
        if M::descriptor_static().full_name() == "google.protobuf.FieldOptions" {
            if option.get() == "default" || option.get() == "json_name" {
                // some options are written to non-options message and handled outside
                return Ok(());
            }
        }
        match M::descriptor_static().get_field_by_name(option.get()) {
            Some(field) => {
                if field.is_repeated_or_map() {
                    return Err(ConvertError::BuiltinOptionPointsToNonSingularField(
                        M::descriptor_static().full_name().to_owned(),
                        option.get().to_owned(),
                    )
                    .into());
                }

                field.set_singular_field(
                    options,
                    option_value.as_type(field.singular_runtime_type())?,
                );
                return Ok(());
            }
            None => {
                return Err(ConvertError::BuiltinOptionNotFound(
                    M::descriptor_static().full_name().to_owned(),
                    option.get().to_owned(),
                )
                .into())
            }
        }
    }

    fn ext_resolve_field_ext(
        &self,
        scope: &ProtobufAbsPathRef,
        message: &WithFullName<&DescriptorProto>,
        field_name: &ProtobufPath,
    ) -> anyhow::Result<FieldDescriptorProto> {
        let expected_extendee = &message.full_name;
        let (_extension, field) = self.find_extension_by_path(scope, field_name)?;
        if &ProtobufAbsPath::new(field.get_extendee()) != expected_extendee {
            return Err(ConvertError::WrongExtensionType(
                format!("{}", field_name),
                format!("{}", field.get_extendee()),
                format!("{}", expected_extendee),
            )
            .into());
        }

        Ok(field)
    }

    fn ext_resolve_field(
        &self,
        scope: &ProtobufAbsPathRef,
        message: &WithFullName<&DescriptorProto>,
        field: &ProtobufOptionNamePart,
    ) -> anyhow::Result<FieldDescriptorProto> {
        match field {
            ProtobufOptionNamePart::Direct(field) => {
                match message.t.field.iter().find(|f| f.get_name() == field.get()) {
                    Some(field) => Ok(field.clone()),
                    None => Err(ConvertError::UnknownFieldName(field.to_string()).into()),
                }
            }
            ProtobufOptionNamePart::Ext(field) => self.ext_resolve_field_ext(scope, message, field),
        }
    }

    fn custom_option_ext_step(
        &self,
        scope: &ProtobufAbsPathRef,
        options_type: &WithFullName<&DescriptorProto>,
        options: &mut UnknownFields,
        option_name: &ProtobufOptionNamePart,
        option_name_rem: &[ProtobufOptionNamePart],
        option_value: &ProtobufConstant,
    ) -> anyhow::Result<()> {
        let field = self.ext_resolve_field(scope, options_type, option_name)?;

        let field_type = TypeResolved::from_field(&field);

        if !option_name_rem.is_empty() {
            match field_type {
                TypeResolved::Message(message_name) => {
                    let m = self.find_message_by_abs_name(&message_name)?;
                    let message_proto = self.message(&message_name.parent().unwrap(), m.t)?;
                    let mut unknown_fields = UnknownFields::new();
                    self.custom_option_ext_step(
                        scope,
                        &WithFullName {
                            full_name: message_name.clone(),
                            t: &message_proto,
                        },
                        &mut unknown_fields,
                        &option_name_rem[0],
                        &option_name_rem[1..],
                        option_value,
                    )?;
                    options.add_length_delimited(
                        field.get_number() as u32,
                        unknown_fields.write_to_bytes(),
                    );
                    return Ok(());
                }
                TypeResolved::Group(..) => {
                    // TODO: implement
                    return Ok(());
                }
                _ => {
                    return Err(
                        ConvertError::ExtensionIsNotMessage(format!("{}", option_name)).into(),
                    )
                }
            }
        }

        let value = match self.option_value_to_unknown_value(
            &field_type,
            option_value,
            &format!("{}", option_name),
        ) {
            Ok(value) => value,
            Err(ConvertError::ConstantsOfTypeMessageEnumGroupNotImplemented) => {
                // TODO: return error
                return Ok(());
            }
            Err(e) => return Err(e.into()),
        };

        options.add_value(field.get_number() as u32, value);
        Ok(())
    }

    fn custom_option_ext<M>(
        &self,
        scope: &ProtobufAbsPathRef,
        options: &mut M,
        option_name: &ProtobufOptionNameExt,
        option_value: &ProtobufConstant,
    ) -> anyhow::Result<()>
    where
        M: Message,
    {
        self.custom_option_ext_step(
            scope,
            &WithFullName {
                full_name: ProtobufAbsPath::from_path_without_dot(
                    M::descriptor_static().full_name(),
                ),
                t: M::descriptor_static().get_proto(),
            },
            options.mut_unknown_fields(),
            &option_name.0[0],
            &option_name.0[1..],
            option_value,
        )
    }

    fn custom_option<M>(
        &self,
        scope: &ProtobufAbsPathRef,
        options: &mut M,
        option: &model::ProtobufOption,
    ) -> anyhow::Result<()>
    where
        M: Message,
    {
        match &option.name {
            ProtobufOptionName::Builtin(simple) => {
                self.custom_option_builtin(scope, options, simple, &option.value)
            }
            ProtobufOptionName::Ext(e) => self.custom_option_ext(scope, options, e, &option.value),
        }
    }

    fn custom_options<M>(
        &self,
        scope: &ProtobufAbsPathRef,
        input: &[model::ProtobufOption],
    ) -> anyhow::Result<Option<M>>
    where
        M: Message,
    {
        if input.is_empty() {
            // Empty options do not have to represented to unset message field,
            // but this is what Google's parser does.
            return Ok(None);
        }

        let mut options = M::new();

        for option in input {
            self.custom_option(scope, &mut options, option)?;
        }
        Ok(Some(options))
    }

    fn field_options(
        &self,
        scope: &ProtobufAbsPathRef,
        input: &[model::ProtobufOption],
    ) -> anyhow::Result<Option<protobuf::descriptor::FieldOptions>> {
        self.custom_options(scope, input)
    }

    fn field(
        &self,
        scope: &ProtobufAbsPathRef,
        input: &model::WithLoc<model::Field>,
        oneof_index: Option<i32>,
    ) -> anyhow::Result<protobuf::descriptor::FieldDescriptorProto> {
        let mut output = protobuf::descriptor::FieldDescriptorProto::new();
        output.set_name(input.t.name.clone());

        if let model::FieldType::Map(..) = input.t.typ {
            output.set_label(protobuf::descriptor::field_descriptor_proto::Label::LABEL_REPEATED);
        } else {
            output.set_label(label(input.t.rule));
        }

        let t = self.field_type(scope, &input.t.name, &input.t.typ)?;
        output.set_field_type(t.type_enum());
        if let Some(t_name) = t.type_name() {
            output.set_type_name(t_name.path.clone());
        }

        output.set_number(input.t.number);
        if let Some(ref default) = input.t.options.as_slice().by_name("default") {
            let default = match output.get_field_type() {
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING => {
                    if let &model::ProtobufConstant::String(ref s) = default {
                        s.decode_utf8()?
                    } else {
                        return Err(ConvertError::DefaultValueIsNotStringLiteral.into());
                    }
                }
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_BYTES => {
                    if let &model::ProtobufConstant::String(ref s) = default {
                        let mut buf = String::new();
                        quote_bytes_to(&s.decode_bytes()?, &mut buf);
                        buf
                    } else {
                        return Err(ConvertError::DefaultValueIsNotStringLiteral.into());
                    }
                }
                _ => default.format(),
            };
            output.set_default_value(default);
        }

        output.options = self.field_options(scope, &input.t.options)?.into();

        if let Some(oneof_index) = oneof_index {
            output.set_oneof_index(oneof_index);
        }

        if let Some(json_name) = input.t.options.as_slice().by_name_string("json_name")? {
            output.set_json_name(json_name);
        } else {
            output.set_json_name(json_name(&input.t.name));
        }

        Ok(output)
    }

    fn root_scope(&self) -> LookupScopeUnion {
        self.type_resolver.root_scope()
    }

    fn lookup(&self, path: &ProtobufAbsPath) -> LookupScopeUnion {
        self.root_scope().lookup(&path.to_root_rel())
    }

    fn find_message_by_abs_name(
        &self,
        abs_path: &ProtobufAbsPath,
    ) -> anyhow::Result<WithFullName<&'a model::Message>> {
        let with_full_name = self
            .type_resolver
            .find_message_or_enum_by_abs_name(abs_path)?;
        match with_full_name.t {
            MessageOrEnum::Message(m) => Ok(WithFullName {
                t: m,
                full_name: with_full_name.full_name,
            }),
            MessageOrEnum::Enum(..) => Err(ConvertError::ExpectingMessage(abs_path.clone()).into()),
        }
    }

    fn find_enum_by_abs_name(
        &self,
        abs_path: &ProtobufAbsPath,
    ) -> anyhow::Result<&'a model::Enumeration> {
        match self
            .type_resolver
            .find_message_or_enum_by_abs_name(abs_path)?
            .t
        {
            MessageOrEnum::Enum(e) => Ok(e),
            MessageOrEnum::Message(..) => Err(ConvertError::ExpectingEnum(abs_path.clone()).into()),
        }
    }

    fn scope_resolved_candidates_rel(
        scope: &ProtobufAbsPathRef,
        rel: &ProtobufRelPathRef,
    ) -> Vec<ProtobufAbsPath> {
        scope
            .self_and_parents()
            .into_iter()
            .map(|a| {
                let mut a = a.to_owned();
                a.push_relative(rel);
                a
            })
            .collect()
    }

    fn scope_resolved_candidates(
        scope: &ProtobufAbsPathRef,
        path: &ProtobufPath,
    ) -> Vec<ProtobufAbsPath> {
        match path {
            ProtobufPath::Abs(p) => vec![p.clone()],
            ProtobufPath::Rel(p) => Self::scope_resolved_candidates_rel(scope, p),
        }
    }

    fn field_type(
        &self,
        scope: &ProtobufAbsPathRef,
        name: &str,
        input: &model::FieldType,
    ) -> anyhow::Result<TypeResolved> {
        Ok(match *input {
            model::FieldType::Bool => TypeResolved::Bool,
            model::FieldType::Int32 => TypeResolved::Int32,
            model::FieldType::Int64 => TypeResolved::Int64,
            model::FieldType::Uint32 => TypeResolved::Uint32,
            model::FieldType::Uint64 => TypeResolved::Uint64,
            model::FieldType::Sint32 => TypeResolved::Sint32,
            model::FieldType::Sint64 => TypeResolved::Sint64,
            model::FieldType::Fixed32 => TypeResolved::Fixed32,
            model::FieldType::Fixed64 => TypeResolved::Fixed64,
            model::FieldType::Sfixed32 => TypeResolved::Sfixed32,
            model::FieldType::Sfixed64 => TypeResolved::Sfixed64,
            model::FieldType::Float => TypeResolved::Float,
            model::FieldType::Double => TypeResolved::Double,
            model::FieldType::String => TypeResolved::String,
            model::FieldType::Bytes => TypeResolved::Bytes,
            model::FieldType::MessageOrEnum(ref name) => {
                let t = self.type_resolver.resolve_message_or_enum(scope, &name)?;
                match t.t {
                    MessageOrEnum::Message(..) => TypeResolved::Message(t.full_name),
                    MessageOrEnum::Enum(..) => TypeResolved::Enum(t.full_name),
                }
            }
            model::FieldType::Map(..) => {
                let mut type_name = scope.to_owned();
                type_name.push_simple(&Resolver::map_entry_name_for_field_name(name));
                TypeResolved::Message(type_name)
            }
            model::FieldType::Group(model::Group {
                name: ref group_name,
                ..
            }) => {
                let mut type_name = scope.to_owned();
                type_name.push_simple(ProtobufIdentRef::new(group_name));
                TypeResolved::Group(type_name)
            }
        })
    }

    fn _runtime_type_for_field_type(&self, ft: &TypeResolved) -> anyhow::Result<RuntimeTypeBox> {
        Ok(match ft {
            TypeResolved::Bool => RuntimeTypeBox::Bool,
            TypeResolved::Int32 | TypeResolved::Sint32 | TypeResolved::Sfixed32 => {
                RuntimeTypeBox::I32
            }
            TypeResolved::Int64 | TypeResolved::Sint64 | TypeResolved::Sfixed64 => {
                RuntimeTypeBox::I64
            }
            TypeResolved::Uint32 | TypeResolved::Fixed32 => RuntimeTypeBox::U32,
            TypeResolved::Uint64 | TypeResolved::Fixed64 => RuntimeTypeBox::U64,
            TypeResolved::Float => RuntimeTypeBox::F32,
            TypeResolved::Double => RuntimeTypeBox::F64,
            TypeResolved::String => RuntimeTypeBox::String,
            TypeResolved::Bytes => RuntimeTypeBox::VecU8,
            TypeResolved::Message(_) | TypeResolved::Enum(_) | TypeResolved::Group(_) => {
                return Err(ConvertError::ConstantsOfTypeMessageEnumGroupNotImplemented.into())
            }
        })
    }

    fn enum_value(
        &self,
        scope: &ProtobufAbsPathRef,
        input: &model::EnumValue,
    ) -> anyhow::Result<protobuf::descriptor::EnumValueDescriptorProto> {
        let mut output = protobuf::descriptor::EnumValueDescriptorProto::new();
        output.set_name(input.name.clone());
        output.set_number(input.number);
        output.options = self.enum_value_options(scope, &input.options)?.into();
        Ok(output)
    }

    fn enum_options(
        &self,
        scope: &ProtobufAbsPathRef,
        input: &[model::ProtobufOption],
    ) -> anyhow::Result<Option<protobuf::descriptor::EnumOptions>> {
        self.custom_options(scope, input)
    }

    fn enum_value_options(
        &self,
        scope: &ProtobufAbsPathRef,
        input: &[model::ProtobufOption],
    ) -> anyhow::Result<Option<protobuf::descriptor::EnumValueOptions>> {
        self.custom_options(scope, input)
    }

    fn enumeration(
        &self,
        scope: &ProtobufAbsPathRef,
        input: &model::Enumeration,
    ) -> anyhow::Result<protobuf::descriptor::EnumDescriptorProto> {
        let mut output = protobuf::descriptor::EnumDescriptorProto::new();
        output.set_name(input.name.clone());
        output.value = input
            .values
            .iter()
            .map(|v| self.enum_value(scope, &v))
            .collect::<Result<_, _>>()?;
        output.options = self.enum_options(scope, &input.options)?.into();
        Ok(output)
    }

    fn oneof_options(
        &self,
        scope: &ProtobufAbsPathRef,
        input: &[model::ProtobufOption],
    ) -> anyhow::Result<Option<protobuf::descriptor::OneofOptions>> {
        self.custom_options(scope, input)
    }

    fn oneof(
        &self,
        scope: &ProtobufAbsPathRef,
        input: &model::OneOf,
    ) -> anyhow::Result<protobuf::descriptor::OneofDescriptorProto> {
        let mut output = protobuf::descriptor::OneofDescriptorProto::new();
        output.set_name(input.name.clone());
        output.options = self.oneof_options(scope, &input.options)?.into();
        Ok(output)
    }

    fn find_extension_by_abs_path(
        &self,
        path: &ProtobufAbsPathRef,
    ) -> anyhow::Result<Option<(&model::Extension, FieldDescriptorProto)>> {
        let mut path = path.to_owned();
        let extension = path.pop().unwrap();

        let scope = self.lookup(&path);

        for ext in scope.extensions() {
            if ext.field.t.name == extension.get() {
                let (resolved_ext, _) = self.extension(&path, &ext)?;
                return Ok(Some((&ext, resolved_ext)));
            }
        }

        Ok(None)
    }

    fn find_extension_by_path(
        &self,
        scope: &ProtobufAbsPathRef,
        path: &ProtobufPath,
    ) -> anyhow::Result<(&model::Extension, FieldDescriptorProto)> {
        for candidate in Self::scope_resolved_candidates(scope, path) {
            if let Some(e) = self.find_extension_by_abs_path(&candidate)? {
                return Ok(e);
            }
        }

        Err(ConvertError::ExtensionNotFound(path.to_string()).into())
    }

    fn option_value_to_unknown_value(
        &self,
        field_type: &TypeResolved,
        value: &model::ProtobufConstant,
        option_name_for_diag: &str,
    ) -> Result<UnknownValue, ConvertError> {
        match value {
            &model::ProtobufConstant::Bool(b) => {
                if field_type != &TypeResolved::Bool {
                    {}
                } else {
                    return Ok(UnknownValue::Varint(if b { 1 } else { 0 }));
                }
            }
            // TODO: check overflow
            &model::ProtobufConstant::U64(v) => match field_type {
                TypeResolved::Fixed64 | TypeResolved::Sfixed64 => {
                    return Ok(UnknownValue::Fixed64(v))
                }
                TypeResolved::Fixed32 | TypeResolved::Sfixed32 => {
                    return Ok(UnknownValue::Fixed32(v as u32))
                }
                TypeResolved::Int64
                | TypeResolved::Int32
                | TypeResolved::Uint64
                | TypeResolved::Uint32 => return Ok(UnknownValue::Varint(v)),
                TypeResolved::Sint64 => return Ok(UnknownValue::sint64(v as i64)),
                TypeResolved::Sint32 => return Ok(UnknownValue::sint32(v as i32)),
                TypeResolved::Float => return Ok(UnknownValue::float(v as f32)),
                TypeResolved::Double => return Ok(UnknownValue::double(v as f64)),
                _ => {}
            },
            &model::ProtobufConstant::I64(v) => match field_type {
                TypeResolved::Fixed64 | TypeResolved::Sfixed64 => {
                    return Ok(UnknownValue::Fixed64(v as u64))
                }
                TypeResolved::Fixed32 | TypeResolved::Sfixed32 => {
                    return Ok(UnknownValue::Fixed32(v as u32))
                }
                TypeResolved::Int64
                | TypeResolved::Int32
                | TypeResolved::Uint64
                | TypeResolved::Uint32 => return Ok(UnknownValue::Varint(v as u64)),
                TypeResolved::Sint64 => return Ok(UnknownValue::sint64(v as i64)),
                TypeResolved::Sint32 => return Ok(UnknownValue::sint32(v as i32)),
                TypeResolved::Float => return Ok(UnknownValue::float(v as f32)),
                TypeResolved::Double => return Ok(UnknownValue::double(v as f64)),
                _ => {}
            },
            &model::ProtobufConstant::F64(f) => match field_type {
                TypeResolved::Float => return Ok(UnknownValue::float(f as f32)),
                TypeResolved::Double => return Ok(UnknownValue::double(f)),
                TypeResolved::Fixed32 => return Ok(UnknownValue::Fixed32(f as u32)),
                TypeResolved::Fixed64 => return Ok(UnknownValue::Fixed64(f as u64)),
                TypeResolved::Sfixed32 => return Ok(UnknownValue::sfixed32(f as i32)),
                TypeResolved::Sfixed64 => return Ok(UnknownValue::sfixed64(f as i64)),
                TypeResolved::Int32 | TypeResolved::Int64 => {
                    return Ok(UnknownValue::int64(f as i64))
                }
                TypeResolved::Uint32 | TypeResolved::Uint64 => {
                    return Ok(UnknownValue::Varint(f as u64))
                }
                TypeResolved::Sint64 => return Ok(UnknownValue::sint64(f as i64)),
                TypeResolved::Sint32 => return Ok(UnknownValue::sint32(f as i32)),
                _ => {}
            },
            &model::ProtobufConstant::String(ref s) => match field_type {
                TypeResolved::String => {
                    return Ok(UnknownValue::LengthDelimited(s.decode_utf8()?.into_bytes()))
                }
                TypeResolved::Bytes => return Ok(UnknownValue::LengthDelimited(s.decode_bytes()?)),
                _ => {}
            },
            model::ProtobufConstant::Ident(ident) => match &field_type {
                TypeResolved::Enum(e) => {
                    let e = self
                        .find_enum_by_abs_name(e)
                        .map_err(ConvertError::OtherError)?;
                    let n = match e
                        .values
                        .iter()
                        .find(|v| v.name == format!("{}", ident))
                        .map(|v| v.number)
                    {
                        Some(n) => n,
                        None => return Err(ConvertError::UnknownEnumValue(ident.to_string())),
                    };
                    return Ok(UnknownValue::int32(n));
                }
                _ => {}
            },
            model::ProtobufConstant::Message(mo) => match &field_type {
                TypeResolved::Message(ma) => {
                    let m = self
                        .find_message_by_abs_name(ma)
                        .map_err(ConvertError::OtherError)?
                        .t;
                    let mut unknown_fields = UnknownFields::new();
                    for (n, v) in &mo.fields {
                        match n {
                            ProtobufConstantMessageFieldName::Regular(n) => {
                                let f = match m.field_by_name(n.as_str()) {
                                    Some(f) => f,
                                    None => return Err(ConvertError::UnknownFieldName(n.clone())),
                                };
                                let u = self
                                    .option_value_field_to_unknown_value(
                                        ma,
                                        v,
                                        n,
                                        &f.typ,
                                        option_name_for_diag,
                                    )
                                    .map_err(ConvertError::OtherError)?;
                                unknown_fields.add_value(f.number as u32, u);
                            }
                            ProtobufConstantMessageFieldName::Extension(..) => {
                                // TODO: implement extension fields in constants
                            }
                            ProtobufConstantMessageFieldName::AnyTypeUrl(..) => {
                                // TODO: implement any type url in constants
                            }
                        }
                    }
                    return Ok(UnknownValue::LengthDelimited(
                        unknown_fields.write_to_bytes(),
                    ));
                }
                _ => {}
            },
        };

        Err(match field_type {
            TypeResolved::Message(..) | TypeResolved::Enum(..) | TypeResolved::Group(..) => {
                ConvertError::ConstantsOfTypeMessageEnumGroupNotImplemented
            }
            _ => ConvertError::UnsupportedExtensionType(
                option_name_for_diag.to_owned(),
                format!("{:?}", field_type),
                value.clone(),
            ),
        })
    }

    fn option_value_field_to_unknown_value(
        &self,
        scope: &ProtobufAbsPath,
        value: &model::ProtobufConstant,
        name: &str,
        field_type: &model::FieldType,
        option_name_for_diag: &str,
    ) -> anyhow::Result<UnknownValue> {
        let field_type = self.field_type(&scope, name, field_type)?;
        Ok(self.option_value_to_unknown_value(&field_type, value, option_name_for_diag)?)
    }

    fn file_options(
        &self,
        scope: &ProtobufAbsPath,
        input: &[model::ProtobufOption],
    ) -> anyhow::Result<Option<protobuf::descriptor::FileOptions>> {
        self.custom_options(scope, input)
    }

    fn extension(
        &self,
        scope: &ProtobufAbsPath,
        input: &model::Extension,
    ) -> anyhow::Result<(
        protobuf::descriptor::FieldDescriptorProto,
        Option<protobuf::descriptor::DescriptorProto>,
    )> {
        let mut field = self.field(scope, &input.field, None)?;
        field.set_extendee(
            self.type_resolver
                .resolve_message_or_enum(scope, &input.extendee)?
                .full_name
                .to_string(),
        );
        let group_messages = if let model::FieldType::Group(g) = &input.field.t.typ {
            Some(self.group_message(scope, &g.name, &g.fields)?)
        } else {
            None
        };
        Ok((field, group_messages))
    }
}

fn syntax(input: model::Syntax) -> String {
    match input {
        model::Syntax::Proto2 => "proto2".to_owned(),
        model::Syntax::Proto3 => "proto3".to_owned(),
    }
}

fn label(input: model::Rule) -> protobuf::descriptor::field_descriptor_proto::Label {
    match input {
        model::Rule::Optional => {
            protobuf::descriptor::field_descriptor_proto::Label::LABEL_OPTIONAL
        }
        model::Rule::Required => {
            protobuf::descriptor::field_descriptor_proto::Label::LABEL_REQUIRED
        }
        model::Rule::Repeated => {
            protobuf::descriptor::field_descriptor_proto::Label::LABEL_REPEATED
        }
    }
}

pub(crate) fn populate_dependencies(
    input: &model::FileDescriptor,
    output: &mut protobuf::descriptor::FileDescriptorProto,
) {
    for import in &input.imports {
        if import.vis == model::ImportVis::Public {
            output
                .public_dependency
                .push(output.dependency.len() as i32);
        } else if import.vis == model::ImportVis::Weak {
            output.weak_dependency.push(output.dependency.len() as i32);
        }
        output.dependency.push(import.path.to_string());
    }
}

pub(crate) fn file_descriptor(
    name: &ProtoPath,
    input: &model::FileDescriptor,
    deps: &[FileDescriptorPair],
) -> anyhow::Result<protobuf::descriptor::FileDescriptorProto> {
    let resolver = Resolver {
        current_file: &input,
        type_resolver: TypeResolver {
            current_file: &input,
            deps,
        },
    };

    let mut output = protobuf::descriptor::FileDescriptorProto::new();
    output.set_name(fs_path_to_proto_path(name));
    output.set_syntax(syntax(input.syntax));

    if input.package != ProtobufAbsPath::root() {
        output.set_package(input.package.to_root_rel().to_string());
    }

    populate_dependencies(&input, &mut output);

    let mut messages = Vec::new();
    let mut services = Vec::new();

    let mut extensions = Vec::new();
    for e in &input.extensions {
        let (ext, group_messages) = resolver.extension(&resolver.current_file.package, &e.t)?;
        extensions.push(ext);
        messages.extend(group_messages.map(model::WithLoc::with_loc(e.loc)));
    }
    output.extension = extensions;

    for m in &input.messages {
        let message = resolver.message(&resolver.current_file.package, &m.t)?;
        messages.push(model::WithLoc {
            t: message,
            loc: m.loc,
        });
    }

    for s in &input.services {
        let service = resolver.service(&s.t)?;
        services.push(model::WithLoc {
            t: service,
            loc: s.loc,
        })
    }

    // Preserve declaration order
    messages.sort_by_key(|m| m.loc);
    output.message_type = messages
        .into_iter()
        .map(|model::WithLoc { t, .. }| t)
        .collect();

    output.enum_type = input
        .enums
        .iter()
        .map(|e| resolver.enumeration(&resolver.current_file.package, e))
        .collect::<Result<_, _>>()?;

    output.service = services
        .into_iter()
        .map(|model::WithLoc { t, .. }| t)
        .collect();

    output.options = resolver
        .file_options(&resolver.current_file.package, &input.options)?
        .into();

    Ok(output)
}

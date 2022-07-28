//! Convert parser model to rust-protobuf model

mod option_resolver;
mod type_resolver;

use protobuf::descriptor::descriptor_proto::ReservedRange;
use protobuf::descriptor::enum_descriptor_proto::EnumReservedRange;
use protobuf::descriptor::field_descriptor_proto;
use protobuf::descriptor::field_descriptor_proto::Type;
use protobuf::descriptor::FieldDescriptorProto;
use protobuf::descriptor::OneofDescriptorProto;
use protobuf::reflect::FileDescriptor;
use protobuf_support::json_name::json_name;
use protobuf_support::text_format::escape_bytes_to;

use crate::case_convert::camel_case;
use crate::path::fs_path_to_proto_path;
use crate::proto_path::ProtoPath;
use crate::protobuf_abs_path::ProtobufAbsPath;
use crate::protobuf_ident::ProtobufIdent;
use crate::pure::convert::option_resolver::OptionResoler;
use crate::pure::convert::option_resolver::ProtobufOptions;
use crate::pure::convert::type_resolver::MessageOrEnum;
use crate::pure::convert::type_resolver::TypeResolver;
use crate::pure::model;
use crate::FileDescriptorPair;
use crate::ProtobufAbsPathRef;
use crate::ProtobufIdentRef;

#[derive(Debug, thiserror::Error)]
enum ConvertError {
    #[error("default value is not a string literal")]
    DefaultValueIsNotStringLiteral,
    #[error("expecting a message for name {0}")]
    ExpectingMessage(ProtobufAbsPath),
    #[error("expecting an enum for name {0}")]
    ExpectingEnum(ProtobufAbsPath),
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
        match field.type_() {
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
                assert!(!field.type_name().is_empty());
                TypeResolved::Group(ProtobufAbsPath::new(field.type_name()))
            }
            Type::TYPE_ENUM => {
                assert!(!field.type_name().is_empty());
                TypeResolved::Enum(ProtobufAbsPath::new(field.type_name()))
            }
            Type::TYPE_MESSAGE => {
                assert!(!field.type_name().is_empty());
                TypeResolved::Message(ProtobufAbsPath::new(field.type_name()))
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

pub(crate) struct Resolver<'a> {
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
        output.set_type(t.type_enum());
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

        output.options.mut_or_insert_default().set_map_entry(true);
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
                        let oneof_index = if self.is_proto3_optional(f) {
                            let oneof_index = output.oneof_decl.len() as i32;
                            let mut oneof = OneofDescriptorProto::new();
                            oneof.set_name(format!("_{}", f.name));
                            output.oneof_decl.push(oneof);
                            Some(oneof_index)
                        } else {
                            None
                        };
                        fields.push(self.field(&nested_scope, f, oneof_index)?);
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

        for ext in &input.extension_ranges {
            let mut extension_range = protobuf::descriptor::descriptor_proto::ExtensionRange::new();
            extension_range.set_start(*ext.start());
            extension_range.set_end(*ext.end() + 1);
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
            reserved_range.set_start(*reserved.start());
            reserved_range.set_end(*reserved.end() + 1);
            output.reserved_range.push(reserved_range);
        }
        output.reserved_name = input.reserved_names.clone().into();

        Ok(output)
    }

    fn service_method(
        &self,
        input: &model::Method,
    ) -> anyhow::Result<protobuf::descriptor::MethodDescriptorProto> {
        let scope = &self.current_file.package;
        let mut output = protobuf::descriptor::MethodDescriptorProto::new();
        output.set_name(input.name.clone());
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

        if input.client_streaming {
            output.set_client_streaming(input.client_streaming);
        }

        if input.server_streaming {
            output.set_server_streaming(input.server_streaming);
        }
        Ok(output)
    }

    fn service(
        &self,
        input: &model::Service,
    ) -> anyhow::Result<protobuf::descriptor::ServiceDescriptorProto> {
        let mut output = protobuf::descriptor::ServiceDescriptorProto::new();
        output.set_name(input.name.clone());

        output.method = input
            .methods
            .iter()
            .map(|m| self.service_method(m))
            .collect::<Result<_, _>>()?;

        Ok(output)
    }

    fn is_proto3_optional(&self, input: &model::WithLoc<model::Field>) -> bool {
        (self.current_file.syntax, input.t.rule)
            == (model::Syntax::Proto3, Some(model::Rule::Optional))
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

            if self.is_proto3_optional(input) {
                output.set_proto3_optional(true);
            }
        }

        let t = self.field_type(scope, &input.t.name, &input.t.typ)?;
        output.set_type(t.type_enum());
        if let Some(t_name) = t.type_name() {
            output.set_type_name(t_name.path.clone());
        }

        output.set_number(input.t.number);
        // TODO: move default to option parser
        if let Some(ref default) = input.t.options.as_slice().by_name("default") {
            let default = match output.type_() {
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
                        escape_bytes_to(&s.decode_bytes()?, &mut buf);
                        buf
                    } else {
                        return Err(ConvertError::DefaultValueIsNotStringLiteral.into());
                    }
                }
                _ => default.format(),
            };
            output.set_default_value(default);
        }

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

    fn enum_value(
        &self,
        _scope: &ProtobufAbsPathRef,
        input: &model::EnumValue,
    ) -> anyhow::Result<protobuf::descriptor::EnumValueDescriptorProto> {
        let mut output = protobuf::descriptor::EnumValueDescriptorProto::new();
        output.set_name(input.name.clone());
        output.set_number(input.number);
        Ok(output)
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

        for reserved in &input.reserved_nums {
            let mut reserved_range = EnumReservedRange::new();
            reserved_range.set_start(*reserved.start());
            // EnumReservedRange is inclusive, not like ExtensionRange and
            // ReservedRange, which are exclusive.
            reserved_range.set_end(*reserved.end());
            output.reserved_range.push(reserved_range);
        }

        output.reserved_name = input.reserved_names.clone().into();

        Ok(output)
    }

    fn oneof(
        &self,
        _scope: &ProtobufAbsPathRef,
        input: &model::OneOf,
    ) -> anyhow::Result<protobuf::descriptor::OneofDescriptorProto> {
        let mut output = protobuf::descriptor::OneofDescriptorProto::new();
        output.set_name(input.name.clone());
        Ok(output)
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

fn label(input: Option<model::Rule>) -> protobuf::descriptor::field_descriptor_proto::Label {
    match input {
        Some(model::Rule::Optional) => {
            protobuf::descriptor::field_descriptor_proto::Label::LABEL_OPTIONAL
        }
        Some(model::Rule::Required) => {
            protobuf::descriptor::field_descriptor_proto::Label::LABEL_REQUIRED
        }
        Some(model::Rule::Repeated) => {
            protobuf::descriptor::field_descriptor_proto::Label::LABEL_REPEATED
        }
        None => protobuf::descriptor::field_descriptor_proto::Label::LABEL_OPTIONAL,
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

    let descriptor_without_options = FileDescriptor::new_dynamic(
        output.clone(),
        &deps
            .iter()
            .map(|d| d.descriptor.clone())
            .collect::<Vec<_>>(),
    )?;

    let option_resolver = OptionResoler {
        resolver: &resolver,
        descriptor_without_options,
    };

    option_resolver.file(&mut output)?;

    Ok(output)
}

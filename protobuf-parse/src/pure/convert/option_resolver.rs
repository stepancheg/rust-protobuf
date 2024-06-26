use anyhow::Context;
use protobuf::descriptor::DescriptorProto;
use protobuf::descriptor::EnumDescriptorProto;
use protobuf::descriptor::EnumValueDescriptorProto;
use protobuf::descriptor::FieldDescriptorProto;
use protobuf::descriptor::FileDescriptorProto;
use protobuf::descriptor::MethodDescriptorProto;
use protobuf::descriptor::OneofDescriptorProto;
use protobuf::descriptor::ServiceDescriptorProto;
use protobuf::reflect::FieldDescriptor;
use protobuf::reflect::FileDescriptor;
use protobuf::reflect::MessageDescriptor;
use protobuf::MessageFull;
use protobuf::UnknownFields;
use protobuf::UnknownValue;
use protobuf_support::lexer::str_lit::StrLitDecodeError;

use crate::model;
use crate::model::ProtobufConstant;
use crate::model::ProtobufConstantMessage;
use crate::model::ProtobufConstantMessageFieldName;
use crate::model::ProtobufOptionName;
use crate::model::ProtobufOptionNameExt;
use crate::model::ProtobufOptionNamePart;
use crate::model::WithLoc;
use crate::protobuf_path::ProtobufPath;
use crate::pure::convert::Resolver;
use crate::pure::convert::TypeResolved;
use crate::ProtobufAbsPath;
use crate::ProtobufAbsPathRef;
use crate::ProtobufIdent;
use crate::ProtobufIdentRef;
use crate::ProtobufRelPath;
use crate::ProtobufRelPathRef;

#[derive(Debug, thiserror::Error)]
enum OptionResolverError {
    #[error(transparent)]
    OtherError(anyhow::Error),
    #[error("extension is not a message: {0}")]
    ExtensionIsNotMessage(String),
    #[error("unknown field name: {0}")]
    UnknownFieldName(String),
    #[error("wrong extension type: option {0} extendee {1} expected extendee {2}")]
    WrongExtensionType(String, String, String),
    #[error("extension not found: {0}")]
    ExtensionNotFound(String),
    #[error("unknown enum value: {0}")]
    UnknownEnumValue(String),
    #[error("unsupported extension type: {0} {1} {2}")]
    UnsupportedExtensionType(String, String, model::ProtobufConstant),
    #[error("builtin option {0} not found for options {1}")]
    BuiltinOptionNotFound(String, String),
    #[error("builtin option {0} points to a non-singular field of {1}")]
    BuiltinOptionPointsToNonSingularField(String, String),
    #[error("incorrect string literal: {0}")]
    StrLitDecodeError(#[source] StrLitDecodeError),
    #[error("wrong option type, expecting {0}, got `{1}`")]
    WrongOptionType(&'static str, String),
    #[error("Message field requires a message constant")]
    MessageFieldRequiresMessageConstant,
    #[error("message not found by name {0}")]
    MessageNotFound(ProtobufAbsPath),
    #[error("message not found by name {0}")]
    MessageFoundMoreThanOnce(ProtobufAbsPath),
}

#[derive(Clone)]
enum LookupScope2 {
    File(FileDescriptor),
    Message(MessageDescriptor, ProtobufAbsPath),
}

impl LookupScope2 {
    fn current_path(&self) -> ProtobufAbsPath {
        match self {
            LookupScope2::File(f) => ProtobufAbsPath::package_from_file_descriptor(f),
            LookupScope2::Message(_, p) => p.clone(),
        }
    }

    fn messages(&self) -> Vec<MessageDescriptor> {
        match self {
            LookupScope2::File(file) => file.messages().collect(),
            LookupScope2::Message(message, _) => message.nested_messages().collect(),
        }
    }

    fn down(&self, name: &ProtobufIdentRef) -> Option<LookupScope2> {
        match self.messages().iter().find(|m| m.name() == name.as_str()) {
            Some(m) => {
                let mut path = self.current_path();
                path.push_simple(name);
                Some(LookupScope2::Message(m.clone(), path))
            }
            None => None,
        }
    }

    fn extensions(&self) -> Vec<FieldDescriptor> {
        match self {
            LookupScope2::File(f) => f.extensions().collect(),
            LookupScope2::Message(m, _) => m.extensions().collect(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct LookupScopeUnion2 {
    path: ProtobufAbsPath,
    scopes: Vec<LookupScope2>,
    partial_scopes: Vec<FileDescriptor>,
}

impl LookupScopeUnion2 {
    fn down(&self, name: &ProtobufIdentRef) -> LookupScopeUnion2 {
        let mut path: ProtobufAbsPath = self.path.clone();
        path.push_simple(name);

        let mut scopes: Vec<_> = self.scopes.iter().flat_map(|f| f.down(name)).collect();
        let mut partial_scopes = Vec::new();

        for partial_scope in &self.partial_scopes {
            let package = ProtobufAbsPath::package_from_file_descriptor(partial_scope);
            if package.as_ref() == path.as_ref() {
                scopes.push(LookupScope2::File(partial_scope.clone()));
            } else if package.starts_with(&path) {
                partial_scopes.push(partial_scope.clone());
            }
        }
        LookupScopeUnion2 {
            path,
            scopes,
            partial_scopes,
        }
    }

    fn lookup(&self, path: &ProtobufRelPath) -> LookupScopeUnion2 {
        let mut scope = self.clone();
        for c in path.components() {
            scope = scope.down(c);
        }
        scope
    }

    fn extensions(&self) -> Vec<FieldDescriptor> {
        self.scopes.iter().flat_map(|s| s.extensions()).collect()
    }

    fn as_message(&self) -> anyhow::Result<MessageDescriptor> {
        let mut messages: Vec<MessageDescriptor> = self
            .scopes
            .iter()
            .filter_map(|s| match s {
                LookupScope2::Message(m, _) => Some(m.clone()),
                _ => None,
            })
            .collect();
        let message = match messages.pop() {
            Some(m) => m,
            None => return Err(OptionResolverError::MessageNotFound(self.path.clone()).into()),
        };
        if !messages.is_empty() {
            return Err(OptionResolverError::MessageFoundMoreThanOnce(self.path.clone()).into());
        }
        Ok(message)
    }
}

pub(crate) trait ProtobufOptions {
    fn by_name(&self, name: &str) -> Option<&model::ProtobufConstant>;

    fn _by_name_bool(&self, name: &str) -> anyhow::Result<Option<bool>> {
        match self.by_name(name) {
            Some(model::ProtobufConstant::Bool(b)) => Ok(Some(*b)),
            Some(c) => Err(OptionResolverError::WrongOptionType("bool", c.to_string()).into()),
            None => Ok(None),
        }
    }

    fn by_name_string(&self, name: &str) -> anyhow::Result<Option<String>> {
        match self.by_name(name) {
            Some(model::ProtobufConstant::String(s)) => s
                .decode_utf8()
                .map(Some)
                .map_err(|e| OptionResolverError::StrLitDecodeError(e).into()),
            Some(c) => Err(OptionResolverError::WrongOptionType("string", c.to_string()).into()),
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

pub(crate) struct OptionResoler<'a> {
    pub(crate) resolver: &'a Resolver<'a>,
    pub(crate) descriptor_without_options: FileDescriptor,
}

impl<'a> OptionResoler<'a> {
    fn all_files(&self) -> Vec<FileDescriptor> {
        let mut files = Vec::new();
        files.push(self.descriptor_without_options.clone());
        files.extend(
            self.resolver
                .type_resolver
                .deps
                .iter()
                .map(|p| p.descriptor.clone()),
        );
        files
    }

    fn root_scope(&self) -> LookupScopeUnion2 {
        let (scopes, partial_scopes) = self
            .all_files()
            .into_iter()
            .partition::<Vec<_>, _>(|f| ProtobufAbsPath::package_from_file_descriptor(f).is_root());
        LookupScopeUnion2 {
            path: ProtobufAbsPath::root(),
            scopes: scopes.into_iter().map(LookupScope2::File).collect(),
            partial_scopes,
        }
    }

    fn lookup(&self, path: &ProtobufAbsPath) -> LookupScopeUnion2 {
        self.root_scope().lookup(&path.to_root_rel())
    }

    fn find_message_by_abs_name(
        &self,
        path: &ProtobufAbsPath,
    ) -> anyhow::Result<MessageDescriptor> {
        let scope = self.lookup(path);
        scope.as_message()
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

    fn find_extension_by_abs_path(
        &self,
        path: &ProtobufAbsPathRef,
    ) -> anyhow::Result<Option<FieldDescriptor>> {
        let mut path = path.to_owned();
        let extension = path.pop().unwrap();

        let scope = self.lookup(&path);

        for ext in scope.extensions() {
            if ext.name() == extension.get() {
                return Ok(Some(ext.clone()));
            }
        }

        Ok(None)
    }

    fn find_extension_by_path(
        &self,
        scope: &ProtobufAbsPathRef,
        path: &ProtobufPath,
    ) -> anyhow::Result<FieldDescriptor> {
        for candidate in Self::scope_resolved_candidates(scope, path) {
            if let Some(e) = self.find_extension_by_abs_path(&candidate)? {
                return Ok(e);
            }
        }

        Err(OptionResolverError::ExtensionNotFound(path.to_string()).into())
    }

    fn ext_resolve_field_ext(
        &self,
        scope: &ProtobufAbsPathRef,
        message: &MessageDescriptor,
        field_name: &ProtobufPath,
    ) -> anyhow::Result<FieldDescriptor> {
        let expected_extendee = ProtobufAbsPath::from_message(message);
        let field = self.find_extension_by_path(scope, field_name)?;
        if ProtobufAbsPath::new(field.proto().extendee()) != expected_extendee {
            return Err(OptionResolverError::WrongExtensionType(
                format!("{}", field_name),
                format!("{}", field.proto().extendee()),
                format!("{}", expected_extendee),
            )
            .into());
        }

        Ok(field)
    }

    fn ext_resolve_field(
        &self,
        scope: &ProtobufAbsPathRef,
        message: &MessageDescriptor,
        field: &ProtobufOptionNamePart,
    ) -> anyhow::Result<FieldDescriptor> {
        match field {
            ProtobufOptionNamePart::Direct(field) => match message.field_by_name(field.get()) {
                Some(field) => Ok(field),
                None => Err(OptionResolverError::UnknownFieldName(field.to_string()).into()),
            },
            ProtobufOptionNamePart::Ext(field) => {
                Ok(self.ext_resolve_field_ext(scope, message, field)?)
            }
        }
    }

    fn custom_option_ext_step(
        &self,
        scope: &ProtobufAbsPathRef,
        options_type: &MessageDescriptor,
        options: &mut UnknownFields,
        option_name: &ProtobufOptionNamePart,
        option_name_rem: &[ProtobufOptionNamePart],
        option_value: &ProtobufConstant,
    ) -> anyhow::Result<()> {
        let field = self.ext_resolve_field(scope, options_type, option_name)?;

        let field_type = TypeResolved::from_field(field.proto());

        match option_name_rem.split_first() {
            Some((first, rem)) => {
                match field_type {
                    TypeResolved::Message(message_name) => {
                        let m = self.find_message_by_abs_name(&message_name)?;
                        let mut unknown_fields = UnknownFields::new();
                        self.custom_option_ext_step(
                            scope,
                            &m,
                            &mut unknown_fields,
                            first,
                            rem,
                            option_value,
                        )?;
                        options.add_length_delimited(
                            field.proto().number() as u32,
                            unknown_fields.write_to_bytes(),
                        );
                        Ok(())
                    }
                    TypeResolved::Group(..) => {
                        // TODO: implement
                        Ok(())
                    }
                    _ => Err(OptionResolverError::ExtensionIsNotMessage(format!(
                        "scope: {}, option name: {}",
                        scope, option_name
                    ))
                    .into()),
                }
            }
            None => {
                let value = match self.option_value_to_unknown_value(
                    &field_type,
                    option_value,
                    &format!("{}", option_name),
                ) {
                    Ok(value) => value,
                    Err(e) => {
                        let e = e.context(format!(
                            "parsing custom option `{}` value `{}` at `{}`",
                            option_name, option_value, scope
                        ));
                        return Err(e.into());
                    }
                };

                options.add_value(field.proto().number() as u32, value);
                Ok(())
            }
        }
    }

    fn custom_option_ext<M>(
        &self,
        scope: &ProtobufAbsPathRef,
        options: &mut M,
        option_name: &ProtobufOptionNameExt,
        option_value: &ProtobufConstant,
    ) -> anyhow::Result<()>
    where
        M: MessageFull,
    {
        self.custom_option_ext_step(
            scope,
            &M::descriptor(),
            options.mut_unknown_fields(),
            &option_name.0[0],
            &option_name.0[1..],
            option_value,
        )
    }

    fn fixed32(
        v: impl TryInto<u32, Error = impl std::error::Error + Send + Sync + 'static>,
    ) -> anyhow::Result<UnknownValue> {
        Ok(UnknownValue::Fixed32(v.try_into()?))
    }

    fn sfixed32(
        v: impl TryInto<i32, Error = impl std::error::Error + Send + Sync + 'static>,
    ) -> anyhow::Result<UnknownValue> {
        Ok(UnknownValue::sfixed32(v.try_into()?))
    }

    fn fixed64(
        v: impl TryInto<u64, Error = impl std::error::Error + Send + Sync + 'static>,
    ) -> anyhow::Result<UnknownValue> {
        Ok(UnknownValue::Fixed64(v.try_into()?))
    }

    fn sfixed64(
        v: impl TryInto<i64, Error = impl std::error::Error + Send + Sync + 'static>,
    ) -> anyhow::Result<UnknownValue> {
        Ok(UnknownValue::sfixed64(v.try_into()?))
    }

    fn int32(
        v: impl TryInto<i32, Error = impl std::error::Error + Send + Sync + 'static>,
    ) -> anyhow::Result<UnknownValue> {
        Ok(UnknownValue::int32(v.try_into()?))
    }

    fn int64(
        v: impl TryInto<i64, Error = impl std::error::Error + Send + Sync + 'static>,
    ) -> anyhow::Result<UnknownValue> {
        Ok(UnknownValue::int64(v.try_into()?))
    }

    fn uint32(
        v: impl TryInto<u32, Error = impl std::error::Error + Send + Sync + 'static>,
    ) -> anyhow::Result<UnknownValue> {
        Ok(UnknownValue::Varint(v.try_into()? as u64))
    }

    fn uint64(
        v: impl TryInto<u64, Error = impl std::error::Error + Send + Sync + 'static>,
    ) -> anyhow::Result<UnknownValue> {
        Ok(UnknownValue::Varint(v.try_into()?))
    }

    fn sint32(
        v: impl TryInto<i32, Error = impl std::error::Error + Send + Sync + 'static>,
    ) -> anyhow::Result<UnknownValue> {
        Ok(UnknownValue::sint32(v.try_into()?))
    }

    fn sint64(
        v: impl TryInto<i64, Error = impl std::error::Error + Send + Sync + 'static>,
    ) -> anyhow::Result<UnknownValue> {
        Ok(UnknownValue::sint64(v.try_into()?))
    }

    fn option_value_message_to_unknown_value(
        &self,
        field_type: &TypeResolved,
        value: &ProtobufConstantMessage,
        option_name_for_diag: &str,
    ) -> anyhow::Result<UnknownValue> {
        match &field_type {
            TypeResolved::Message(ma) => {
                let m = self
                    .resolver
                    .find_message_by_abs_name(ma)
                    .map_err(OptionResolverError::OtherError)?
                    .t;
                let mut unknown_fields = UnknownFields::new();
                for (n, v) in &value.fields {
                    match n {
                        ProtobufConstantMessageFieldName::Regular(n) => {
                            let f = match m.field_by_name(n.as_str()) {
                                Some(f) => f,
                                None => {
                                    return Err(
                                        OptionResolverError::UnknownFieldName(n.clone()).into()
                                    )
                                }
                            };
                            let u = self
                                .option_value_field_to_unknown_value(
                                    ma,
                                    v,
                                    n,
                                    &f.typ,
                                    option_name_for_diag,
                                )
                                .map_err(OptionResolverError::OtherError)?;
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
                Ok(UnknownValue::LengthDelimited(
                    unknown_fields.write_to_bytes(),
                ))
            }
            _ => Err(OptionResolverError::MessageFieldRequiresMessageConstant.into()),
        }
    }

    fn option_value_to_unknown_value(
        &self,
        field_type: &TypeResolved,
        value: &model::ProtobufConstant,
        option_name_for_diag: &str,
    ) -> anyhow::Result<UnknownValue> {
        match value {
            &model::ProtobufConstant::Bool(b) => {
                if field_type != &TypeResolved::Bool {
                    {}
                } else {
                    return Ok(UnknownValue::Varint(if b { 1 } else { 0 }));
                }
            }
            &model::ProtobufConstant::U64(v) => match field_type {
                TypeResolved::Fixed64 => return Self::fixed64(v),
                TypeResolved::Sfixed64 => return Self::sfixed64(v),
                TypeResolved::Fixed32 => return Self::fixed32(v),
                TypeResolved::Sfixed32 => return Self::sfixed32(v),
                TypeResolved::Int32 => return Self::int32(v),
                TypeResolved::Int64 => return Self::int64(v),
                TypeResolved::Uint64 => return Self::uint64(v),
                TypeResolved::Uint32 => return Self::uint32(v),
                TypeResolved::Sint64 => return Self::sint64(v),
                TypeResolved::Sint32 => return Self::sint32(v),
                TypeResolved::Float => return Ok(UnknownValue::float(v as f32)),
                TypeResolved::Double => return Ok(UnknownValue::double(v as f64)),
                _ => {}
            },
            &model::ProtobufConstant::I64(v) => match field_type {
                TypeResolved::Fixed64 => return Self::fixed64(v),
                TypeResolved::Sfixed64 => return Self::sfixed64(v),
                TypeResolved::Fixed32 => return Self::fixed32(v),
                TypeResolved::Sfixed32 => return Self::sfixed32(v),
                TypeResolved::Int64 => return Self::int64(v),
                TypeResolved::Int32 => return Self::int32(v),
                TypeResolved::Uint64 => return Self::uint64(v),
                TypeResolved::Uint32 => return Self::uint32(v),
                TypeResolved::Sint64 => return Self::sint64(v),
                TypeResolved::Sint32 => return Self::sint32(v),
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
                        .resolver
                        .find_enum_by_abs_name(e)
                        .map_err(OptionResolverError::OtherError)?;
                    let n = match e
                        .values
                        .iter()
                        .find(|v| v.name == format!("{}", ident))
                        .map(|v| v.number)
                    {
                        Some(n) => n,
                        None => {
                            return Err(
                                OptionResolverError::UnknownEnumValue(ident.to_string()).into()
                            )
                        }
                    };
                    return Ok(UnknownValue::int32(n));
                }
                _ => {}
            },
            model::ProtobufConstant::Message(mo) => {
                return self.option_value_message_to_unknown_value(
                    &field_type,
                    mo,
                    option_name_for_diag,
                );
            }
        };

        Err(match field_type {
            _ => OptionResolverError::UnsupportedExtensionType(
                option_name_for_diag.to_owned(),
                format!("{:?}", field_type),
                value.clone(),
            )
            .into(),
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
        let field_type = self.resolver.field_type(&scope, name, field_type)?;
        Ok(self
            .option_value_to_unknown_value(&field_type, value, option_name_for_diag)
            .context("parsing custom option value")?)
    }

    fn custom_option_builtin<M>(
        &self,
        _scope: &ProtobufAbsPathRef,
        options: &mut M,
        option: &ProtobufIdent,
        option_value: &ProtobufConstant,
    ) -> anyhow::Result<()>
    where
        M: MessageFull,
    {
        if M::descriptor().full_name() == "google.protobuf.FieldOptions" {
            if option.get() == "default" || option.get() == "json_name" {
                // some options are written to non-options message and handled outside
                return Ok(());
            }
        }
        match M::descriptor().field_by_name(option.get()) {
            Some(field) => {
                if field.is_repeated_or_map() {
                    return Err(OptionResolverError::BuiltinOptionPointsToNonSingularField(
                        M::descriptor().full_name().to_owned(),
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
                return Err(OptionResolverError::BuiltinOptionNotFound(
                    M::descriptor().full_name().to_owned(),
                    option.get().to_owned(),
                )
                .into())
            }
        }
    }

    fn custom_option<M>(
        &self,
        scope: &ProtobufAbsPathRef,
        options: &mut M,
        option: &model::ProtobufOption,
    ) -> anyhow::Result<()>
    where
        M: MessageFull,
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
        M: MessageFull,
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

    fn file_options(
        &self,
        scope: &ProtobufAbsPath,
        input: &[model::ProtobufOption],
    ) -> anyhow::Result<Option<protobuf::descriptor::FileOptions>> {
        self.custom_options(scope, input)
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

    fn field_options(
        &self,
        scope: &ProtobufAbsPathRef,
        input: &[model::ProtobufOption],
    ) -> anyhow::Result<Option<protobuf::descriptor::FieldOptions>> {
        self.custom_options(scope, input)
    }

    fn message_options(
        &self,
        scope: &ProtobufAbsPathRef,
        input: &[model::ProtobufOption],
    ) -> anyhow::Result<Option<protobuf::descriptor::MessageOptions>> {
        self.custom_options(scope, input)
    }

    fn oneof_options(
        &self,
        scope: &ProtobufAbsPathRef,
        input: &[model::ProtobufOption],
    ) -> anyhow::Result<Option<protobuf::descriptor::OneofOptions>> {
        self.custom_options(scope, input)
    }

    fn method(
        &self,
        method_proto: &mut MethodDescriptorProto,
        method_model: &model::Method,
    ) -> anyhow::Result<()> {
        method_proto.options = self.service_method_options(&method_model.options)?.into();
        Ok(())
    }

    fn service_options(
        &self,
        input: &[model::ProtobufOption],
    ) -> anyhow::Result<Option<protobuf::descriptor::ServiceOptions>> {
        self.custom_options(&self.resolver.current_file.package, input)
    }

    fn service_method_options(
        &self,
        input: &[model::ProtobufOption],
    ) -> anyhow::Result<Option<protobuf::descriptor::MethodOptions>> {
        self.custom_options(&self.resolver.current_file.package, input)
    }

    fn service(
        &self,
        service_proto: &mut ServiceDescriptorProto,
        service_model: &WithLoc<model::Service>,
    ) -> anyhow::Result<()> {
        service_proto.options = self.service_options(&service_model.options)?.into();

        for service_method_model in &service_model.methods {
            let mut method_proto = service_proto
                .method
                .iter_mut()
                .find(|method| method.name() == service_method_model.name)
                .unwrap();
            self.method(&mut method_proto, service_method_model)?;
        }

        Ok(())
    }

    fn enum_value(
        &self,
        scope: &ProtobufAbsPathRef,
        enum_value_proto: &mut EnumValueDescriptorProto,
        enum_value_model: &model::EnumValue,
    ) -> anyhow::Result<()> {
        enum_value_proto.options = self
            .enum_value_options(scope, &enum_value_model.options)?
            .into();
        Ok(())
    }

    fn enumeration(
        &self,
        scope: &ProtobufAbsPathRef,
        enum_proto: &mut EnumDescriptorProto,
        enum_model: &WithLoc<model::Enumeration>,
    ) -> anyhow::Result<()> {
        enum_proto.options = self.enum_options(scope, &enum_model.options)?.into();

        for enum_value_model in &enum_model.values {
            let mut enum_value_proto = enum_proto
                .value
                .iter_mut()
                .find(|v| v.name() == enum_value_model.name)
                .unwrap();
            self.enum_value(scope, &mut enum_value_proto, enum_value_model)?;
        }

        Ok(())
    }

    fn oneof(
        &self,
        scope: &ProtobufAbsPathRef,
        oneof_proto: &mut OneofDescriptorProto,
        oneof_model: &model::OneOf,
    ) -> anyhow::Result<()> {
        oneof_proto.options = self.oneof_options(scope, &oneof_model.options)?.into();
        Ok(())
    }

    fn field(
        &self,
        scope: &ProtobufAbsPathRef,
        field_proto: &mut FieldDescriptorProto,
        field_model: &model::Field,
    ) -> anyhow::Result<()> {
        field_proto.options = self.field_options(scope, &field_model.options)?.into();
        Ok(())
    }

    fn message(
        &self,
        scope: &ProtobufAbsPathRef,
        message_proto: &mut DescriptorProto,
        message_model: &WithLoc<model::Message>,
    ) -> anyhow::Result<()> {
        message_proto.options = self.message_options(scope, &message_model.options)?.into();

        let mut nested_scope = scope.to_owned();
        nested_scope.push_simple(ProtobufIdentRef::new(&message_proto.name()));

        for field_model in &message_model.regular_fields_including_in_oneofs() {
            let mut field_proto = message_proto
                .field
                .iter_mut()
                .find(|field| field.name() == field_model.name)
                .unwrap();
            self.field(&nested_scope, &mut field_proto, field_model)?;
        }
        for field_model in &message_model.extensions {
            let field_proto = message_proto
                .extension
                .iter_mut()
                .find(|field| field.name() == field_model.field.name)
                .unwrap();
            self.field(&nested_scope, field_proto, &field_model.field)?;
        }

        for nested_message_model in &message_model.messages {
            let nested_message_proto = message_proto
                .nested_type
                .iter_mut()
                .find(|nested_message_proto| {
                    nested_message_proto.name() == nested_message_model.name
                })
                .unwrap();
            self.message(&nested_scope, nested_message_proto, nested_message_model)?;
        }

        for nested_enum_model in &message_model.enums {
            let nested_enum_proto = message_proto
                .enum_type
                .iter_mut()
                .find(|nested_enum_proto| nested_enum_proto.name() == nested_enum_model.name)
                .unwrap();
            self.enumeration(&nested_scope, nested_enum_proto, nested_enum_model)?;
        }

        for oneof_model in &message_model.oneofs() {
            let oneof_proto = message_proto
                .oneof_decl
                .iter_mut()
                .find(|oneof_proto| oneof_proto.name() == oneof_model.name)
                .unwrap();
            self.oneof(&nested_scope, oneof_proto, oneof_model)?;
        }

        Ok(())
    }

    pub(crate) fn file(&self, output: &mut FileDescriptorProto) -> anyhow::Result<()> {
        // TODO: use it to resolve messages.
        let _ = &self.descriptor_without_options;

        for message_model in &self.resolver.current_file.messages {
            let message_proto = output
                .message_type
                .iter_mut()
                .find(|m| m.name() == message_model.name)
                .unwrap();
            self.message(
                &self.resolver.current_file.package,
                message_proto,
                message_model,
            )?;
        }

        for enum_model in &self.resolver.current_file.enums {
            let enum_proto = output
                .enum_type
                .iter_mut()
                .find(|e| e.name() == enum_model.name)
                .unwrap();
            self.enumeration(&self.resolver.current_file.package, enum_proto, enum_model)?;
        }

        for service_proto in &mut output.service {
            let service_model = self
                .resolver
                .current_file
                .services
                .iter()
                .find(|s| s.name == service_proto.name())
                .unwrap();
            self.service(service_proto, service_model)?;
        }

        for extension_model in &self.resolver.current_file.extensions {
            let extension_proto = output
                .extension
                .iter_mut()
                .find(|e| e.name() == extension_model.field.name)
                .unwrap();
            self.field(
                &self.resolver.current_file.package,
                extension_proto,
                &extension_model.field,
            )?;
        }

        output.options = self
            .file_options(
                &self.resolver.current_file.package,
                &self.resolver.current_file.options,
            )?
            .into();

        Ok(())
    }
}

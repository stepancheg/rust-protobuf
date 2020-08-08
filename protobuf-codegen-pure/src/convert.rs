//! Convert parser model to rust-protobuf model

use std::iter;
use std::path::Path;

use crate::model;

use protobuf;
use protobuf::descriptor::field_descriptor_proto;
use protobuf::json::json_name;
use protobuf::Message;
use protobuf::UnknownFields;
use protobuf::UnknownValue;

use crate::model::ProtobufOptionName;
use crate::protobuf_codegen::case_convert::camel_case;
use crate::protobuf_codegen::ProtobufAbsolutePath;
use crate::protobuf_codegen::ProtobufIdent;
use crate::protobuf_codegen::ProtobufRelativePath;
use protobuf::descriptor::descriptor_proto::ReservedRange;
use protobuf::descriptor::field_descriptor_proto::Type;
use protobuf::reflect::RuntimeTypeBox;
use protobuf::text_format::lexer::StrLitDecodeError;
use protobuf::text_format::quote_bytes_to;

#[derive(Debug)]
pub enum ConvertError {
    UnsupportedOption(String),
    ExtensionNotFound(String),
    WrongExtensionType(String, &'static str),
    UnsupportedExtensionType(String, String, model::ProtobufConstant),
    StrLitDecodeError(StrLitDecodeError),
    DefaultValueIsNotStringLiteral,
    WrongOptionType,
    InconvertibleValue(RuntimeTypeBox, model::ProtobufConstant),
    ConstantsOfTypeMessageEnumGroupNotImplemented,
    NotFoundByAbsPath(ProtobufAbsolutePath),
    NotFoundByRelPath(ProtobufRelativePath, ProtobufAbsolutePath),
    ExpectingMessage(ProtobufAbsolutePath),
    ExpectingEnum(ProtobufAbsolutePath),
    UnknownEnumValue(String),
    UnknownFieldName(String),
}

impl From<StrLitDecodeError> for ConvertError {
    fn from(e: StrLitDecodeError) -> Self {
        ConvertError::StrLitDecodeError(e)
    }
}

pub type ConvertResult<T> = Result<T, ConvertError>;

trait ProtobufOptions {
    fn by_name(&self, name: &str) -> Option<&model::ProtobufConstant>;

    fn by_name_bool(&self, name: &str) -> ConvertResult<Option<bool>> {
        match self.by_name(name) {
            Some(model::ProtobufConstant::Bool(b)) => Ok(Some(*b)),
            Some(_) => Err(ConvertError::WrongOptionType),
            None => Ok(None),
        }
    }

    fn by_name_string(&self, name: &str) -> ConvertResult<Option<String>> {
        match self.by_name(name) {
            Some(model::ProtobufConstant::String(s)) => s
                .decode_utf8()
                .map(Some)
                .map_err(ConvertError::StrLitDecodeError),
            Some(_) => Err(ConvertError::WrongOptionType),
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

enum MessageOrEnum<'a> {
    Message(&'a model::Message),
    Enum(&'a model::Enumeration),
}

impl MessageOrEnum<'_> {
    fn _descriptor_type(&self) -> protobuf::descriptor::field_descriptor_proto::Type {
        match *self {
            MessageOrEnum::Message(..) => {
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_MESSAGE
            }
            MessageOrEnum::Enum(..) => {
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_ENUM
            }
        }
    }
}

enum LookupScope<'a> {
    File(&'a model::FileDescriptor),
    Message(&'a model::Message, ProtobufAbsolutePath),
}

impl<'a> LookupScope<'a> {
    fn current_path(&self) -> ProtobufAbsolutePath {
        match self {
            LookupScope::File(f) => {
                ProtobufAbsolutePath::from_package_path(f.package.as_ref().map(|s| s.as_ref()))
            }
            LookupScope::Message(_, p) => p.clone(),
        }
    }

    fn messages(&self) -> &'a [model::WithLoc<model::Message>] {
        match self {
            &LookupScope::File(file) => &file.messages,
            &LookupScope::Message(messasge, _) => &messasge.messages,
        }
    }

    fn find_message(&self, simple_name: &ProtobufIdent) -> Option<&'a model::Message> {
        self.messages()
            .into_iter()
            .find(|m| m.t.name == simple_name.get())
            .map(|m| &m.t)
    }

    fn enums(&self) -> &'a [model::Enumeration] {
        match self {
            &LookupScope::File(file) => &file.enums,
            &LookupScope::Message(messasge, _) => &messasge.enums,
        }
    }

    fn members(&self) -> Vec<(ProtobufIdent, MessageOrEnum<'a>)> {
        let mut r = Vec::new();
        r.extend(
            self.enums()
                .into_iter()
                .map(|e| (ProtobufIdent::from(&e.name[..]), MessageOrEnum::Enum(e))),
        );
        r.extend(self.messages().into_iter().map(|m| {
            (
                ProtobufIdent::from(&m.t.name[..]),
                MessageOrEnum::Message(&m.t),
            )
        }));
        r
    }

    fn find_member(&self, simple_name: &ProtobufIdent) -> Option<MessageOrEnum<'a>> {
        self.members()
            .into_iter()
            .filter_map(|(member_name, message_or_enum)| {
                if &member_name == simple_name {
                    Some(message_or_enum)
                } else {
                    None
                }
            })
            .next()
    }

    fn find_message_or_enum(
        &self,
        path: &ProtobufRelativePath,
    ) -> Option<(ProtobufAbsolutePath, MessageOrEnum<'a>)> {
        let current_path = self.current_path();
        let (first, rem) = match path.split_first_rem() {
            Some(x) => x,
            None => return None,
        };

        if rem.is_empty() {
            match self.find_member(&first) {
                Some(message_or_enum) => {
                    let mut result_path = current_path.clone();
                    result_path.push_simple(first);
                    Some((result_path, message_or_enum))
                }
                None => None,
            }
        } else {
            match self.find_message(&first) {
                Some(message) => {
                    let mut message_path = current_path.clone();
                    message_path.push_simple(ProtobufIdent::from(message.name.clone()));
                    let message_scope = LookupScope::Message(message, message_path);
                    message_scope.find_message_or_enum(&rem)
                }
                None => None,
            }
        }
    }
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
    Message(ProtobufAbsolutePath),
    Enum(ProtobufAbsolutePath),
    Group(ProtobufAbsolutePath),
}

impl TypeResolved {
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

    fn type_name(&self) -> Option<&ProtobufAbsolutePath> {
        match self {
            TypeResolved::Message(t) | TypeResolved::Enum(t) | TypeResolved::Group(t) => Some(t),
            _ => None,
        }
    }
}

struct Resolver<'a> {
    current_file: &'a model::FileDescriptor,
    deps: &'a [model::FileDescriptor],
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
        name: &str,
        number: i32,
        field_type: &model::FieldType,
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<protobuf::descriptor::FieldDescriptorProto> {
        // should be consisent with DescriptorBuilder::ValidateMapEntry

        let mut output = protobuf::descriptor::FieldDescriptorProto::new();

        output.set_name(name.to_owned());
        output.set_number(number);

        let t = self.field_type_leg(name, field_type, path_in_file)?;
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
        field_name: &str,
        key: &model::FieldType,
        value: &model::FieldType,
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<protobuf::descriptor::DescriptorProto> {
        let mut output = protobuf::descriptor::DescriptorProto::new();

        output.options.mut_message().set_map_entry(true);
        output.set_name(Resolver::map_entry_name_for_field_name(field_name).into_string());
        output
            .field
            .push(self.map_entry_field("key", 1, key, path_in_file)?);
        output
            .field
            .push(self.map_entry_field("value", 2, value, path_in_file)?);

        Ok(output)
    }

    fn group_message(
        &self,
        name: &str,
        fields: &[model::WithLoc<model::Field>],
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<protobuf::descriptor::DescriptorProto> {
        let mut output = protobuf::descriptor::DescriptorProto::new();

        output.set_name(name.to_owned());

        for f in fields {
            output.field.push(self.field(f, None, path_in_file)?);
        }

        Ok(output)
    }

    fn message_options(
        &self,
        input: &[model::ProtobufOption],
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<protobuf::descriptor::MessageOptions> {
        self.custom_options(input, path_in_file)
    }

    fn message(
        &self,
        input: &model::Message,
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<protobuf::descriptor::DescriptorProto> {
        let nested_path_in_file = path_in_file.append_ident(&ProtobufIdent::from(&input.name[..]));

        let mut output = protobuf::descriptor::DescriptorProto::new();
        output.set_name(input.name.clone());

        let mut nested_messages = Vec::new();

        for m in &input.messages {
            let message = self.message(&m.t, &nested_path_in_file)?;
            nested_messages.push(model::WithLoc {
                t: message,
                loc: m.loc,
            });
        }

        for f in input.regular_fields_including_in_oneofs() {
            match &f.t.typ {
                model::FieldType::Map(t) => {
                    let message = self.map_entry_message(&f.t.name, &t.0, &t.1, path_in_file)?;
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
                    let message = self.group_message(group_name, fields, &nested_path_in_file)?;
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
            .map(|e| self.enumeration(e, path_in_file))
            .collect::<Result<_, _>>()?;

        {
            let mut fields = Vec::new();

            for fo in &input.fields {
                match &fo.t {
                    model::FieldOrOneOf::Field(f) => {
                        fields.push(self.field(f, None, &nested_path_in_file)?);
                    }
                    model::FieldOrOneOf::OneOf(o) => {
                        let oneof_index = output.oneof_decl.len();
                        for f in &o.fields {
                            fields.push(self.field(
                                f,
                                Some(oneof_index as i32),
                                &nested_path_in_file,
                            )?);
                        }
                        output.oneof_decl.push(self.oneof(o, path_in_file)?);
                    }
                }
            }

            output.field = fields;
        }

        output
            .options
            .set_message(self.message_options(&input.options, path_in_file)?);

        for ext in &input.extension_ranges {
            let mut extension_range = protobuf::descriptor::descriptor_proto::ExtensionRange::new();
            extension_range.set_start(ext.from);
            extension_range.set_end(ext.to + 1);
            output.extension_range.push(extension_range);
        }
        for ext in &input.extensions {
            let mut extension = self.field(&ext.t.field, None, path_in_file)?;
            extension.set_extendee(
                self.resolve_message_or_enum_leg(&ext.t.extendee, path_in_file)?
                    .0
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

    fn custom_options<M>(
        &self,
        input: &[model::ProtobufOption],
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<M>
    where
        M: Message,
    {
        let mut options = M::new();
        let extendee = M::descriptor_static().full_name();

        for option in input {
            match option.name.get_simple() {
                Some(simple) => {
                    if let Some(field) = M::descriptor_static().get_field_by_name(simple.get()) {
                        if field.is_repeated() || field.is_map() {
                            continue;
                        }

                        field.set_singular_field(
                            &mut options,
                            option
                                .value
                                .as_type(field.singular_runtime_type().to_box())?,
                        );
                    }
                    continue;
                }
                None => {
                    // ?
                }
            }

            let extension = match self.find_extension(&option.name.full_name()) {
                Ok(e) => e,
                // TODO: return error
                Err(_) => continue,
            };
            if extension.extendee != extendee {
                return Err(ConvertError::WrongExtensionType(
                    format!("{}", option.name),
                    extendee,
                ));
            }

            let value = match self.option_value_to_unknown_value_leg(
                &option.value,
                &extension.field.t.name,
                &extension.field.t.typ,
                &format!("{}", option.name),
                path_in_file,
            ) {
                Ok(value) => value,
                Err(ConvertError::ConstantsOfTypeMessageEnumGroupNotImplemented) => {
                    // TODO: return error
                    continue;
                }
                Err(e) => return Err(e),
            };

            options
                .mut_unknown_fields()
                .add_value(extension.field.t.number as u32, value);
        }
        Ok(options)
    }

    fn field_options(
        &self,
        input: &[model::ProtobufOption],
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<protobuf::descriptor::FieldOptions> {
        self.custom_options(input, path_in_file)
    }

    fn field(
        &self,
        input: &model::WithLoc<model::Field>,
        oneof_index: Option<i32>,
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<protobuf::descriptor::FieldDescriptorProto> {
        let mut output = protobuf::descriptor::FieldDescriptorProto::new();
        output.set_name(input.t.name.clone());

        if let model::FieldType::Map(..) = input.t.typ {
            output.set_label(protobuf::descriptor::field_descriptor_proto::Label::LABEL_REPEATED);
        } else {
            output.set_label(label(input.t.rule));
        }

        let t = self.field_type_leg(&input.t.name, &input.t.typ, path_in_file)?;
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
                        return Err(ConvertError::DefaultValueIsNotStringLiteral);
                    }
                }
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_BYTES => {
                    if let &model::ProtobufConstant::String(ref s) = default {
                        let mut buf = String::new();
                        quote_bytes_to(&s.decode_bytes()?, &mut buf);
                        buf
                    } else {
                        return Err(ConvertError::DefaultValueIsNotStringLiteral);
                    }
                }
                _ => default.format(),
            };
            output.set_default_value(default);
        }

        output
            .options
            .set_message(self.field_options(&input.t.options, path_in_file)?);

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

    fn all_files(&self) -> Vec<&'a model::FileDescriptor> {
        iter::once(self.current_file).chain(self.deps).collect()
    }

    fn package_files(&self, package: Option<&str>) -> Vec<&model::FileDescriptor> {
        self.all_files()
            .into_iter()
            .filter(|f| f.package.as_deref() == package)
            .collect()
    }

    fn find_message_or_enum_by_abs_name(
        &self,
        absolute_path: &ProtobufAbsolutePath,
    ) -> ConvertResult<MessageOrEnum<'a>> {
        for file in self.all_files() {
            let file_package = ProtobufAbsolutePath::from_package_path(file.package.as_deref());
            if let Some(relative) = absolute_path.remove_prefix(&file_package) {
                if let Some((_, t)) = LookupScope::File(file).find_message_or_enum(&relative) {
                    return Ok(t);
                }
            }
        }

        return Err(ConvertError::NotFoundByAbsPath(absolute_path.clone()));
    }

    fn find_message_by_abs_name(
        &self,
        abs_path: &ProtobufAbsolutePath,
    ) -> ConvertResult<&'a model::Message> {
        match self.find_message_or_enum_by_abs_name(abs_path)? {
            MessageOrEnum::Message(m) => Ok(m),
            MessageOrEnum::Enum(..) => Err(ConvertError::ExpectingMessage(abs_path.clone())),
        }
    }

    fn find_enum_by_abs_name(
        &self,
        abs_path: &ProtobufAbsolutePath,
    ) -> ConvertResult<&'a model::Enumeration> {
        match self.find_message_or_enum_by_abs_name(abs_path)? {
            MessageOrEnum::Enum(e) => Ok(e),
            MessageOrEnum::Message(..) => Err(ConvertError::ExpectingEnum(abs_path.clone())),
        }
    }

    fn resolve_message_or_enum(
        &self,
        name: &str,
        scope: &ProtobufAbsolutePath,
    ) -> ConvertResult<(ProtobufAbsolutePath, MessageOrEnum)> {
        if ProtobufAbsolutePath::is_abs(name) && !name.is_empty() {
            let abs_path = ProtobufAbsolutePath::new(name.to_owned());
            return Ok((
                abs_path.clone(),
                self.find_message_or_enum_by_abs_name(&abs_path)?,
            ));
        } else {
            let name = ProtobufRelativePath::from(name);

            // find message or enum in current package
            for p in scope.self_and_parents() {
                let mut fq = p;
                fq.push_relative(&name);
                if let Ok(me) = self.find_message_or_enum_by_abs_name(&fq) {
                    return Ok((fq, me));
                }
            }

            return Err(ConvertError::NotFoundByRelPath(name, scope.clone()));
        }
    }

    fn resolve_message_or_enum_leg(
        &self,
        name: &str,
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<(ProtobufAbsolutePath, MessageOrEnum)> {
        let mut scope = ProtobufAbsolutePath::from_package_path(
            self.current_file.package.as_ref().map(|s| s.as_str()),
        );
        scope.push_relative(path_in_file);
        self.resolve_message_or_enum(name, &scope)
    }

    fn field_type(
        &self,
        name: &str,
        input: &model::FieldType,
        scope: &ProtobufAbsolutePath,
    ) -> ConvertResult<TypeResolved> {
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
                let (name, me) = self.resolve_message_or_enum(&name, scope)?;
                match me {
                    MessageOrEnum::Message(..) => TypeResolved::Message(name),
                    MessageOrEnum::Enum(..) => TypeResolved::Enum(name),
                }
            }
            model::FieldType::Map(..) => {
                let mut type_name = scope.clone();
                type_name.push_simple(Resolver::map_entry_name_for_field_name(name));
                TypeResolved::Message(type_name)
            }
            model::FieldType::Group(model::Group {
                name: ref group_name,
                ..
            }) => {
                let mut type_name = scope.clone();
                type_name.push_simple(ProtobufIdent::from(group_name.clone()));
                TypeResolved::Group(type_name)
            }
        })
    }

    fn field_type_leg(
        &self,
        name: &str,
        input: &model::FieldType,
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<TypeResolved> {
        let mut scope =
            ProtobufAbsolutePath::from_package_path(self.current_file.package.as_deref());
        scope.push_relative(path_in_file);
        self.field_type(name, input, &scope)
    }

    fn _runtime_type_for_field_type(&self, ft: &TypeResolved) -> ConvertResult<RuntimeTypeBox> {
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
                return Err(ConvertError::ConstantsOfTypeMessageEnumGroupNotImplemented)
            }
        })
    }

    fn enum_value(
        &self,
        input: &model::EnumValue,
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<protobuf::descriptor::EnumValueDescriptorProto> {
        let mut output = protobuf::descriptor::EnumValueDescriptorProto::new();
        output.set_name(input.name.clone());
        output.set_number(input.number);
        output
            .options
            .set_message(self.enum_value_options(&input.options, path_in_file)?);
        Ok(output)
    }

    fn enum_options(
        &self,
        input: &[model::ProtobufOption],
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<protobuf::descriptor::EnumOptions> {
        self.custom_options(input, path_in_file)
    }

    fn enum_value_options(
        &self,
        input: &[model::ProtobufOption],
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<protobuf::descriptor::EnumValueOptions> {
        self.custom_options(input, path_in_file)
    }

    fn enumeration(
        &self,
        input: &model::Enumeration,
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<protobuf::descriptor::EnumDescriptorProto> {
        let mut output = protobuf::descriptor::EnumDescriptorProto::new();
        output.set_name(input.name.clone());
        output.value = input
            .values
            .iter()
            .map(|v| self.enum_value(&v, path_in_file))
            .collect::<Result<_, _>>()?;
        output
            .options
            .set_message(self.enum_options(&input.options, path_in_file)?);
        Ok(output)
    }

    fn oneof_options(
        &self,
        input: &[model::ProtobufOption],
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<protobuf::descriptor::OneofOptions> {
        self.custom_options(input, path_in_file)
    }

    fn oneof(
        &self,
        input: &model::OneOf,
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<protobuf::descriptor::OneofDescriptorProto> {
        let mut output = protobuf::descriptor::OneofDescriptorProto::new();
        output.set_name(input.name.clone());
        output
            .options
            .set_message(self.oneof_options(&input.options, path_in_file)?);
        Ok(output)
    }

    fn find_extension_by_path(&self, path: &str) -> ConvertResult<&model::Extension> {
        let (package, name) = match path.rfind('.') {
            Some(dot) => (Some(&path[..dot]), &path[dot + 1..]),
            None => (self.current_file.package.as_deref(), path),
        };

        for file in self.package_files(package) {
            for ext in &file.extensions {
                if ext.t.field.t.name == name {
                    return Ok(&ext.t);
                }
            }
        }

        Err(ConvertError::ExtensionNotFound(path.to_owned()))
    }

    fn find_extension(&self, option_name: &str) -> ConvertResult<&model::Extension> {
        if !option_name.starts_with('(') || !option_name.ends_with(')') {
            return Err(ConvertError::UnsupportedOption(option_name.to_owned()));
        }
        let path = &option_name[1..option_name.len() - 1];
        self.find_extension_by_path(path)
    }

    fn option_value_to_unknown_value_leg(
        &self,
        value: &model::ProtobufConstant,
        name: &str,
        field_type: &model::FieldType,
        option_name_for_diag: &str,
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<UnknownValue> {
        let mut scope = ProtobufAbsolutePath::from_package_path(
            self.current_file.package.as_ref().map(|s| s.as_ref()),
        );
        scope.push_relative(path_in_file);

        self.option_value_to_unknown_value(value, name, field_type, option_name_for_diag, &scope)
    }

    fn option_value_to_unknown_value(
        &self,
        value: &model::ProtobufConstant,
        name: &str,
        field_type: &model::FieldType,
        option_name_for_diag: &str,
        scope: &ProtobufAbsolutePath,
    ) -> ConvertResult<UnknownValue> {
        let field_type = self.field_type(name, field_type, &scope)?;

        match value {
            &model::ProtobufConstant::Bool(b) => {
                if field_type != TypeResolved::Bool {
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
                    let e = self.find_enum_by_abs_name(e)?;
                    let n = match e.values.iter().find(|v| v.name == *ident).map(|v| v.number) {
                        Some(n) => n,
                        None => return Err(ConvertError::UnknownEnumValue(ident.clone())),
                    };
                    return Ok(UnknownValue::int32(n));
                }
                _ => {}
            },
            model::ProtobufConstant::Message(mo) => match &field_type {
                TypeResolved::Message(ma) => {
                    let m = self.find_message_by_abs_name(ma)?;
                    let mut unknown_fields = UnknownFields::new();
                    for (n, v) in &mo.fields {
                        let f = match m.field_by_name(n.as_str()) {
                            Some(f) => f,
                            None => return Err(ConvertError::UnknownFieldName(n.clone())),
                        };
                        let u = self.option_value_to_unknown_value(
                            v,
                            n,
                            &f.typ,
                            option_name_for_diag,
                            ma,
                        )?;
                        unknown_fields.add_value(f.number as u32, u);
                    }
                    for (_n, _v) in &mo.extensions {
                        // TODO
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

    fn file_options(
        &self,
        input: &[model::ProtobufOption],
        path_in_file: &ProtobufRelativePath,
    ) -> ConvertResult<protobuf::descriptor::FileOptions> {
        self.custom_options(input, path_in_file)
    }

    fn extension(
        &self,
        input: &model::Extension,
    ) -> ConvertResult<(
        protobuf::descriptor::FieldDescriptorProto,
        Option<protobuf::descriptor::DescriptorProto>,
    )> {
        let relative_path = ProtobufRelativePath::new("".to_owned());
        let mut field = self.field(&input.field, None, &relative_path)?;
        field.set_extendee(
            self.resolve_message_or_enum_leg(&input.extendee, &relative_path)?
                .0
                .path,
        );
        let group_messages = if let model::FieldType::Group(g) = &input.field.t.typ {
            Some(self.group_message(&g.name, &g.fields, &ProtobufRelativePath::empty())?)
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

pub fn file_descriptor(
    name: &Path,
    input: &model::FileDescriptor,
    deps: &[model::FileDescriptor],
) -> ConvertResult<protobuf::descriptor::FileDescriptorProto> {
    let name = name.to_str().expect("not a valid UTF-8 name");

    let resolver = Resolver {
        current_file: &input,
        deps,
    };

    let mut output = protobuf::descriptor::FileDescriptorProto::new();
    output.set_name(name.to_owned());
    output.set_syntax(syntax(input.syntax));

    if let Some(package) = &input.package {
        output.set_package(package.clone());
    }

    for import in &input.imports {
        if import.vis == model::ImportVis::Public {
            output
                .public_dependency
                .push(output.dependency.len() as i32);
        } else if import.vis == model::ImportVis::Weak {
            output.weak_dependency.push(output.dependency.len() as i32);
        }
        output.dependency.push(import.path.clone());
    }

    let mut messages = Vec::new();

    let mut extensions = Vec::new();
    for e in &input.extensions {
        let (ext, group_messages) = resolver.extension(&e.t)?;
        extensions.push(ext);
        messages.extend(group_messages.map(model::WithLoc::with_loc(e.loc)));
    }
    output.extension = extensions;

    for m in &input.messages {
        let message = resolver.message(&m.t, &ProtobufRelativePath::empty())?;
        messages.push(model::WithLoc {
            t: message,
            loc: m.loc,
        });
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
        .map(|e| resolver.enumeration(e, &ProtobufRelativePath::empty()))
        .collect::<Result<_, _>>()?;

    output
        .options
        .set_message(resolver.file_options(&input.options, &ProtobufRelativePath::empty())?);

    Ok(output)
}

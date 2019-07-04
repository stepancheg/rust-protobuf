//! Oneof-related codegen functions.

use crate::code_writer::CodeWriter;
use crate::customize::Customize;
use crate::field::FieldElem;
use crate::field::FieldGen;
use crate::file_and_mod::FileAndMod;
use crate::inside::protobuf_crate_path;
use crate::message::MessageGen;
use crate::rust_name::{RustIdent, RustIdentWithPath, RustPath};
use crate::rust_types_values::make_path;
use crate::rust_types_values::RustType;
use crate::scope::WithScope;
use crate::scope::{FieldWithContext, OneofVariantWithContext};
use crate::scope::{OneofWithContext, RootScope};
use crate::{serde, ProtobufAbsolutePath};
use protobuf::descriptor::field_descriptor_proto;
use std::collections::HashSet;

// oneof one { ... }
#[derive(Clone)]
pub(crate) struct OneofField<'a> {
    pub elem: FieldElem<'a>,
    pub oneof_variant_rust_name: RustIdent,
    pub oneof_field_name: RustIdent,
    pub type_name: RustIdentWithPath,
    pub boxed: bool,
}

impl<'a> OneofField<'a> {
    // Detecting recursion: if oneof fields contains a self-reference
    // or another message which has a reference to self,
    // put oneof variant into a box.
    fn need_boxed(
        field: &FieldWithContext,
        root_scope: &RootScope,
        owner_name: &ProtobufAbsolutePath,
    ) -> bool {
        let mut visited_messages = HashSet::new();
        let mut fields = vec![field.clone()];
        while let Some(field) = fields.pop() {
            if field.field.get_field_type() == field_descriptor_proto::Type::TYPE_MESSAGE {
                let message_name = ProtobufAbsolutePath::from(field.field.get_type_name());
                if !visited_messages.insert(message_name.clone()) {
                    continue;
                }
                if message_name == *owner_name {
                    return true;
                }
                let message = root_scope.find_message(&message_name);
                fields.extend(message.fields().into_iter().filter(|f| f.is_oneof()));
            }
        }
        false
    }

    pub fn parse(
        oneof: &OneofWithContext<'a>,
        field: &FieldWithContext<'a>,
        elem: FieldElem<'a>,
        root_scope: &RootScope,
    ) -> OneofField<'a> {
        let boxed = OneofField::need_boxed(field, root_scope, &oneof.message.name_absolute());

        OneofField {
            elem,
            type_name: oneof.rust_name(),
            boxed,
            oneof_variant_rust_name: field.rust_name(),
            oneof_field_name: oneof.field_name(),
        }
    }

    pub fn rust_type(&self, reference: &FileAndMod) -> RustType {
        let t = self.elem.rust_storage_elem_type(reference);

        if self.boxed {
            RustType::Uniq(Box::new(t))
        } else {
            t
        }
    }

    pub fn variant_path(&self, reference: &RustPath) -> RustIdentWithPath {
        make_path(
            reference,
            &self
                .type_name
                .to_path()
                .with_ident(self.oneof_variant_rust_name.clone()),
        )
    }
}

#[derive(Clone)]
pub(crate) struct OneofVariantGen<'a> {
    oneof: &'a OneofGen<'a>,
    variant: OneofVariantWithContext<'a>,
    oneof_field: OneofField<'a>,
    pub field: FieldGen<'a>,
    path: String,
}

impl<'a> OneofVariantGen<'a> {
    fn parse(
        oneof: &'a OneofGen<'a>,
        variant: OneofVariantWithContext<'a>,
        field: &'a FieldGen,
        _root_scope: &RootScope,
    ) -> OneofVariantGen<'a> {
        OneofVariantGen {
            oneof,
            variant: variant.clone(),
            field: field.clone(),
            path: format!(
                "{}::{}",
                oneof.type_name_relative(
                    &oneof
                        .oneof
                        .message
                        .scope
                        .rust_path_to_file()
                        .clone()
                        .into_path()
                ),
                field.rust_name
            ),
            oneof_field: OneofField::parse(
                variant.oneof,
                &field.proto_field,
                field.elem().clone(),
                oneof.message.root_scope,
            ),
        }
    }

    pub fn rust_type(&self, reference: &FileAndMod) -> RustType {
        self.oneof_field.rust_type(reference)
    }

    pub fn path(&self, reference: &FileAndMod) -> RustPath {
        RustPath::from(format!(
            "{}::{}",
            self.oneof
                .type_name_relative(&reference.relative_mod.clone().into_path()),
            self.field.rust_name
        ))
    }
}

#[derive(Clone)]
pub(crate) struct OneofGen<'a> {
    // Message containing this oneof
    message: &'a MessageGen<'a>,
    pub oneof: OneofWithContext<'a>,
    lite_runtime: bool,
    customize: Customize,
}

impl<'a> OneofGen<'a> {
    pub fn parse(
        message: &'a MessageGen,
        oneof: OneofWithContext<'a>,
        customize: &Customize,
    ) -> OneofGen<'a> {
        OneofGen {
            message,
            oneof,
            lite_runtime: message.lite_runtime,
            customize: customize.clone(),
        }
    }

    pub fn type_name_relative(&self, source: &RustPath) -> RustIdentWithPath {
        make_path(source, &self.oneof.rust_name())
    }

    pub fn variants_except_group(&'a self) -> Vec<OneofVariantGen<'a>> {
        self.oneof
            .variants()
            .into_iter()
            .filter_map(|v| {
                let field = self
                    .message
                    .fields
                    .iter()
                    .filter(|f| f.proto_field.name() == v.field.get_name())
                    .next()
                    .expect(&format!("field not found by name: {}", v.field.get_name()));
                match field.proto_type {
                    field_descriptor_proto::Type::TYPE_GROUP => None,
                    _ => Some(OneofVariantGen::parse(
                        self,
                        v,
                        field,
                        self.message.root_scope,
                    )),
                }
            })
            .collect()
    }

    pub fn full_storage_type(&self) -> RustType {
        RustType::Option(Box::new(RustType::Oneof(
            self.type_name_relative(
                &self
                    .oneof
                    .message
                    .scope
                    .get_file_and_mod(self.customize.clone())
                    .relative_mod
                    .into_path(),
            )
            .clone(),
        )))
    }

    fn get_file_and_mod(&self) -> FileAndMod {
        let mut file_and_mod = self
            .message
            .message
            .scope
            .get_file_and_mod(self.customize.clone());
        file_and_mod
            .relative_mod
            .push_ident(self.message.message.mod_name());
        file_and_mod
    }

    fn write_enum(&self, w: &mut CodeWriter) {
        let derive = vec!["Clone", "PartialEq", "Debug"];
        w.derive(&derive);
        serde::write_serde_attr(w, &self.customize, "derive(Serialize, Deserialize)");
        w.pub_enum(&self.oneof.rust_name().ident.to_string(), |w| {
            for variant in self.variants_except_group() {
                w.write_line(&format!(
                    "{}({}),",
                    variant.field.rust_name,
                    &variant
                        .rust_type(&self.get_file_and_mod())
                        .to_code(&self.customize)
                ));
            }
        });
    }

    fn write_impl_oneof(&self, w: &mut CodeWriter) {
        w.impl_for_block(
            &format!("{}::Oneof", protobuf_crate_path(&self.customize)),
            self.oneof.rust_name().ident.to_string(),
            |_w| {
                // nothing here yet
            },
        );
    }

    pub fn write(&self, w: &mut CodeWriter) {
        self.write_enum(w);
        w.write_line("");
        self.write_impl_oneof(w);
    }
}

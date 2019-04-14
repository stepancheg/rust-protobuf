//! Oneof-related codegen functions.

use code_writer::CodeWriter;
use field::FieldElem;
use field::FieldGen;
use message::MessageGen;
use protobuf::descriptor::field_descriptor_proto;
use scope::{OneofVariantWithContext, FieldWithContext};
use scope::OneofWithContext;
use scope::WithScope;
use rust_types_values::RustType;
use rust_types_values::make_path;
use serde;
use Customize;
use rust_name::{RustIdent, RustIdentWithPath, RustPath};

// oneof one { ... }
#[derive(Clone)]
pub(crate) struct OneofField {
    pub elem: FieldElem,
    pub oneof_variant_rust_name: RustIdent,
    pub oneof_field_name: RustIdent,
    pub type_name: RustIdentWithPath,
    pub boxed: bool,
}

impl OneofField {
    pub fn parse(
        oneof: &OneofWithContext,
        field: &FieldWithContext,
        elem: FieldElem,
    ) -> OneofField {
        // detecting recursion
        let boxed = if let &FieldElem::Message(ref name, ..) = &elem {
            // TODO: compare protobuf names
            if name == &oneof.message.rust_name_to_file() {
                true
            } else {
                false
            }
        } else {
            false
        };

        OneofField {
            elem,
            type_name: oneof.rust_name(),
            boxed,
            oneof_variant_rust_name: field.rust_name(),
            oneof_field_name: oneof.field_name(),
        }
    }

    pub fn rust_type(&self) -> RustType {
        let t = self.elem.rust_storage_elem_type();

        if self.boxed {
            RustType::Uniq(Box::new(t))
        } else {
            t
        }
    }

    pub fn variant_path(&self, reference: &RustPath) -> RustIdentWithPath {
        make_path(
            reference,
            &self.type_name.to_path().with_ident(self.oneof_variant_rust_name.clone()))
    }
}

#[derive(Clone)]
pub(crate) struct OneofVariantGen<'a> {
    oneof: &'a OneofGen<'a>,
    variant: OneofVariantWithContext<'a>,
    oneof_field: OneofField,
    pub field: FieldGen<'a>,
    path: String,
}

impl<'a> OneofVariantGen<'a> {
    fn parse(
        oneof: &'a OneofGen<'a>,
        variant: OneofVariantWithContext<'a>,
        field: &'a FieldGen,
    ) -> OneofVariantGen<'a> {
        OneofVariantGen {
            oneof,
            variant: variant.clone(),
            field: field.clone(),
            path: format!("{}::{}", oneof.type_name_relative(&oneof.oneof.message.scope.rust_path_to_file()), field.rust_name),
            oneof_field: OneofField::parse(
                variant.oneof,
                &field.proto_field,
                field.elem().clone(),
            ),
        }
    }

    pub fn rust_type(&self) -> RustType {
        self.oneof_field.rust_type()
    }

    pub fn path(&self) -> String {
        self.path.clone()
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
                    _ => Some(OneofVariantGen::parse(self, v, field)),
                }
            }).collect()
    }

    pub fn full_storage_type(&self) -> RustType {
        RustType::Option(Box::new(RustType::Oneof(self.type_name_relative(&self.oneof.rust_name().path).clone())))
    }

    fn write_enum(&self, w: &mut CodeWriter) {
        let mut derive = vec!["Clone", "PartialEq"];
        if self.lite_runtime {
            derive.push("Debug");
        }
        w.derive(&derive);
        serde::write_serde_attr(w, &self.customize, "derive(Serialize, Deserialize)");
        w.pub_enum(&self.oneof.rust_name().ident.to_string(), |w| {
            for variant in self.variants_except_group() {
                w.write_line(&format!(
                    "{}({}),",
                    variant.field.rust_name,
                    &variant.rust_type().to_string()
                ));
            }
        });
    }

    fn write_impl_oneof(&self, w: &mut CodeWriter) {
        w.impl_for_block("::protobuf::Oneof", self.oneof.rust_name().ident.to_string(), |_w| {
            // nothing here yet
        });
    }

    pub fn write(&self, w: &mut CodeWriter) {
        self.write_enum(w);
        w.write_line("");
        self.write_impl_oneof(w);
    }
}

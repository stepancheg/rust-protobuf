//! Oneof-related codegen functions.

use code_writer::CodeWriter;
use field::FieldElem;
use field::FieldGen;
use message::MessageGen;
use protobuf::descriptor::FieldDescriptorProto;
use protobuf::descriptor::field_descriptor_proto;
use scope::OneofVariantWithContext;
use scope::OneofWithContext;
use scope::WithScope;
use rust_types_values::RustType;
use serde;
use Customize;

// oneof one { ... }
#[derive(Clone)]
pub(crate) struct OneofField {
    pub elem: FieldElem,
    pub oneof_name: String,
    pub oneof_type_name: RustType,
    pub boxed: bool,
}

impl OneofField {
    pub fn parse(
        oneof: &OneofWithContext,
        _field: &FieldDescriptorProto,
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
            oneof_name: oneof.name().to_string(),
            oneof_type_name: RustType::Oneof(oneof.rust_name().to_path()),
            boxed,
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
            oneof: oneof,
            variant: variant.clone(),
            field: field.clone(),
            path: format!("{}::{}", oneof.type_name, field.rust_name),
            oneof_field: OneofField::parse(
                variant.oneof,
                variant.field,
                field.oneof().elem.clone(),
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
    oneof: OneofWithContext<'a>,
    type_name: RustType,
    lite_runtime: bool,
    customize: Customize,
}

impl<'a> OneofGen<'a> {
    pub fn parse(
        message: &'a MessageGen,
        oneof: OneofWithContext<'a>,
        customize: &Customize,
    ) -> OneofGen<'a> {
        let rust_name = oneof.rust_name();
        OneofGen {
            message,
            oneof,
            type_name: RustType::Oneof(rust_name.to_path()),
            lite_runtime: message.lite_runtime,
            customize: customize.clone(),
        }
    }

    pub fn name(&self) -> &str {
        match self.oneof.oneof.get_name() {
            "type" => "field_type",
            "box" => "field_box",
            x => x,
        }
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
        RustType::Option(Box::new(self.type_name.clone()))
    }

    fn write_enum(&self, w: &mut CodeWriter) {
        let mut derive = vec!["Clone", "PartialEq"];
        if self.lite_runtime {
            derive.push("Debug");
        }
        w.derive(&derive);
        serde::write_serde_attr(w, &self.customize, "derive(Serialize, Deserialize)");
        w.pub_enum(&self.type_name.to_string(), |w| {
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
        w.impl_for_block("::protobuf::Oneof", self.type_name.to_string(), |_w| {
            // nothing here yet
        });
    }

    pub fn write(&self, w: &mut CodeWriter) {
        self.write_enum(w);
        w.write_line("");
        self.write_impl_oneof(w);
    }
}

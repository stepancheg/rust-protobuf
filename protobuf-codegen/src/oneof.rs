//! Oneof-related codegen functions.

use code_writer::CodeWriter;
use field::FieldElem;
use field::FieldGen;
use message::MessageGen;
use protobuf::descriptor::FieldDescriptorProto_Type;
use protobuf::descriptorx::OneofWithContext;
use protobuf::descriptorx::WithScope;
use protobuf::descriptorx::{FieldWithContext, OneofVariantWithContext, RootScope};
use rust_types_values::RustType;
use serde;
use std::collections::HashSet;
use Customize;

// oneof one { ... }
#[derive(Clone)]
pub struct OneofField {
    pub elem: FieldElem,
    pub oneof_name: String,
    pub oneof_type_name: RustType,
    pub boxed: bool,
}

impl OneofField {
    // Detecting recursion: if oneof fields contains a self-reference
    // or another message which has a reference to self,
    // put oneof variant into a box.
    fn need_boxed(field: &FieldWithContext, root_scope: &RootScope, owner_name: &str) -> bool {
        let mut visited_messages = HashSet::new();
        let mut fields = vec![field.clone()];
        while let Some(field) = fields.pop() {
            if field.field.get_field_type() == FieldDescriptorProto_Type::TYPE_MESSAGE {
                let message_name = field.field.get_type_name().to_owned();
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
        oneof: &OneofWithContext,
        field: &FieldWithContext,
        elem: FieldElem,
        root_scope: &RootScope,
    ) -> OneofField {
        let boxed = OneofField::need_boxed(field, root_scope, &oneof.message.name_absolute());

        OneofField {
            elem: elem,
            oneof_name: oneof.name().to_string(),
            oneof_type_name: RustType::Oneof(oneof.rust_name()),
            boxed: boxed,
        }
    }

    pub fn rust_type(&self) -> RustType {
        let t = self.elem.rust_storage_type();

        if self.boxed {
            RustType::Uniq(Box::new(t))
        } else {
            t
        }
    }
}

#[derive(Clone)]
pub struct OneofVariantGen<'a> {
    oneof: &'a OneofGen<'a>,
    variant: OneofVariantWithContext<'a>,
    oneof_field: OneofField,
    pub field: FieldGen<'a>,
    path: String,
    customize: Customize,
}

impl<'a> OneofVariantGen<'a> {
    fn parse(
        oneof: &'a OneofGen<'a>,
        variant: OneofVariantWithContext<'a>,
        field: &'a FieldGen,
        _root_scope: &RootScope,
        customize: Customize,
    ) -> OneofVariantGen<'a> {
        OneofVariantGen {
            oneof: oneof,
            variant: variant.clone(),
            field: field.clone(),
            path: format!(
                "{}::{}",
                oneof.type_name.to_code(&field.customize),
                field.rust_name
            ),
            oneof_field: OneofField::parse(
                variant.oneof,
                &field.proto_field,
                field.oneof().elem.clone(),
                oneof.message.root_scope,
            ),
            customize,
        }
    }

    fn rust_type(&self) -> RustType {
        self.oneof_field.rust_type()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }
}

#[derive(Clone)]
pub struct OneofGen<'a> {
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
            message: message,
            oneof: oneof,
            type_name: RustType::Oneof(rust_name),
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
                    FieldDescriptorProto_Type::TYPE_GROUP => None,
                    _ => Some(OneofVariantGen::parse(
                        self,
                        v,
                        field,
                        self.message.root_scope,
                        self.customize.clone(),
                    )),
                }
            })
            .collect()
    }

    pub fn full_storage_type(&self) -> RustType {
        RustType::Option(Box::new(self.type_name.clone()))
    }

    pub fn write_enum(&self, w: &mut CodeWriter) {
        let derive = vec!["Clone", "PartialEq", "Debug"];
        w.derive(&derive);
        serde::write_serde_attr(w, &self.customize, "derive(Serialize, Deserialize)");
        w.pub_enum(&self.type_name.to_code(&self.customize), |w| {
            for variant in self.variants_except_group() {
                w.write_line(&format!(
                    "{}({}),",
                    variant.field.rust_name,
                    &variant.rust_type().to_code(&self.customize)
                ));
            }
        });
    }
}

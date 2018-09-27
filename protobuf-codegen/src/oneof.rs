//! Oneof-related codegen functions.

use protobuf::descriptorx::OneofVariantWithContext;
use protobuf::descriptorx::WithScope;
use field::FieldGen;
use field::FieldElem;
use rust_types_values::RustType;
use protobuf::descriptorx::OneofWithContext;
use protobuf::descriptor::FieldDescriptorProto;
use message::MessageGen;
use Customize;
use code_writer::CodeWriter;
use protobuf::descriptor::FieldDescriptorProto_Type;


// oneof one { ... }
#[derive(Clone)]
pub struct OneofField {
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
            if *name == oneof.message.rust_name() {
                true
            } else {
                false
            }
        } else {
            false
        };

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
    pub fn parse(message: &'a MessageGen, oneof: OneofWithContext<'a>, customize: &Customize)
        -> OneofGen<'a>
    {
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
                let field = self.message
                    .fields
                    .iter()
                    .filter(|f| f.proto_field.name() == v.field.get_name())
                    .next()
                    .expect(&format!("field not found by name: {}", v.field.get_name()));
                match field.proto_type {
                    FieldDescriptorProto_Type::TYPE_GROUP => None,
                    _ => Some(OneofVariantGen::parse(self, v, field)),
                }

            })
            .collect()
    }

    pub fn full_storage_type(&self) -> RustType {
        RustType::Option(Box::new(self.type_name.clone()))
    }

    pub fn write_enum(&self, w: &mut CodeWriter) {
        let mut derive = vec!["Clone", "PartialEq"];
        if self.lite_runtime {
            derive.push("Debug");
        }
        w.derive(&derive);
        w.write_line("#[cfg_attr(feature = \"with-serde\", derive(Serialize, Deserialize))]");
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
}

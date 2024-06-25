use crate::gen::code_writer::CodeWriter;
use crate::gen::field::elem::FieldElem;
use crate::gen::field::elem::FieldElemEnum;
use crate::gen::field::option_kind::OptionKind;
use crate::gen::field::repeated::RepeatedField;
use crate::gen::field::repeated::RepeatedFieldKind;
use crate::gen::field::singular::SingularField;
use crate::gen::field::singular::SingularFieldFlag;
use crate::gen::field::FieldGen;
use crate::gen::field::FieldKind;
use crate::gen::field::MapField;
use crate::gen::inside::protobuf_crate_path;
use crate::gen::oneof::OneofField;
use crate::gen::rust_types_values::RustType;
use crate::gen::scope::WithScope;

struct AccessorFn {
    name: String,
    // function type params after first underscore
    type_params: Vec<String>,
    callback_params: Vec<String>,
}

impl AccessorFn {
    fn sig(&self) -> String {
        let mut s = self.name.clone();
        s.push_str("::<_");
        for p in &self.type_params {
            s.push_str(", ");
            s.push_str(&p);
        }
        s.push_str(">");
        s
    }
}

impl FieldGen<'_> {
    fn make_accessor_fns_lambda(&self) -> Vec<String> {
        let message = self.proto_field.message.rust_name();
        vec![
            format!("|m: &{}| {{ &m.{} }}", message, self.rust_name),
            format!("|m: &mut {}| {{ &mut m.{} }}", message, self.rust_name),
        ]
    }

    fn make_accessor_fns_has_get_set(&self) -> Vec<String> {
        let message = self.proto_field.message.rust_name();
        vec![
            format!("{}::{}", message, self.has_name()),
            format!("{}::{}", message, self.rust_name),
            format!("{}::{}", message, self.set_name()),
        ]
    }

    fn make_accessor_fns_has_get_mut_set(&self) -> Vec<String> {
        let message = self.proto_field.message.rust_name();
        vec![
            format!("{}::{}", message, self.has_name()),
            format!("{}::{}", message, self.rust_name),
            format!("{}::{}", message, self.mut_name()),
            format!("{}::{}", message, self.set_name()),
        ]
    }

    fn accessor_fn_map(&self, map_field: &MapField) -> AccessorFn {
        let MapField { .. } = map_field;
        AccessorFn {
            name: "make_map_simpler_accessor_new".to_owned(),
            type_params: vec![format!("_")],
            callback_params: self.make_accessor_fns_lambda(),
        }
    }

    fn accessor_fn_repeated(&self, repeated_field: &RepeatedField) -> AccessorFn {
        let RepeatedField { .. } = repeated_field;
        let name = match repeated_field.kind() {
            RepeatedFieldKind::Vec => "make_vec_simpler_accessor",
        };
        AccessorFn {
            name: name.to_owned(),
            type_params: vec![format!("_")],
            callback_params: self.make_accessor_fns_lambda(),
        }
    }

    fn accessor_fn_oneof_enum(&self, oneof: &OneofField, en: &FieldElemEnum) -> AccessorFn {
        let message = self.proto_field.message.rust_name();

        let variant_path = oneof.variant_path(&self.proto_field.message.scope.rust_path_to_file());

        let getter = CodeWriter::with_no_error(|w| {
            w.expr_block(
                &format!(
                    "|message: &{}| match &message.{}",
                    message, oneof.oneof_field_name
                ),
                |w| {
                    w.case_expr(
                        &format!("::std::option::Option::Some({}(e))", variant_path),
                        "::std::option::Option::Some(*e)",
                    );
                    w.case_expr("_", "::std::option::Option::None");
                },
            );
        });

        let setter = CodeWriter::with_no_error(|w| {
            w.expr_block(
                &format!(
                    "|message: &mut {}, e: {}::EnumOrUnknown<{}>|",
                    message,
                    protobuf_crate_path(&self.customize),
                    en.enum_rust_type(&self.file_and_mod())
                        .to_code(&self.customize)
                ),
                |w| {
                    w.write_line(&format!(
                        "message.{} = ::std::option::Option::Some({}(e));",
                        oneof.oneof_field_name, variant_path
                    ));
                },
            )
        });

        let default = self.xxx_default_value_rust();

        AccessorFn {
            name: "make_oneof_enum_accessors".to_owned(),
            type_params: vec![format!("_")],
            callback_params: vec![getter, setter, default],
        }
    }

    fn accessor_fn_singular_without_flag(&self, _elem: &FieldElem) -> AccessorFn {
        AccessorFn {
            name: "make_simpler_field_accessor".to_owned(),
            type_params: vec![format!("_")],
            callback_params: self.make_accessor_fns_lambda(),
        }
    }

    fn accessor_fn_singular_with_flag(
        &self,
        elem: &FieldElem,
        _option_kind: OptionKind,
    ) -> AccessorFn {
        match elem {
            FieldElem::Message(m) => AccessorFn {
                name: "make_message_field_accessor".to_owned(),
                type_params: vec![format!("{}", m.rust_name_relative(&self.file_and_mod()))],
                callback_params: self.make_accessor_fns_lambda(),
            },
            FieldElem::Primitive(..) | FieldElem::Enum(..) => AccessorFn {
                name: "make_option_accessor".to_owned(),
                type_params: vec!["_".to_owned()],
                callback_params: self.make_accessor_fns_lambda(),
            },
            FieldElem::Group => {
                unreachable!("no accessor for group field");
            }
        }
    }

    fn accessor_fn_oneof(&self, oneof: &OneofField) -> AccessorFn {
        let OneofField { ref elem, .. } = oneof;

        let reference = self
            .proto_field
            .message
            .scope
            .file_and_mod(self.customize.clone());

        if let FieldElem::Enum(en) = &oneof.elem {
            return self.accessor_fn_oneof_enum(oneof, en);
        }

        if elem.is_copy() {
            return AccessorFn {
                name: "make_oneof_copy_has_get_set_simpler_accessors".to_owned(),
                type_params: vec![format!("_")],
                callback_params: self.make_accessor_fns_has_get_set(),
            };
        }

        if let RustType::Message(name) = elem.rust_storage_elem_type(&reference) {
            return AccessorFn {
                name: "make_oneof_message_has_get_mut_set_accessor".to_owned(),
                type_params: vec![format!("{}", name)],
                callback_params: self.make_accessor_fns_has_get_mut_set(),
            };
        }

        // string or bytes
        AccessorFn {
            name: "make_oneof_deref_has_get_set_simpler_accessor".to_owned(),
            type_params: vec![format!("_")],
            callback_params: self.make_accessor_fns_has_get_set(),
        }
    }

    fn accessor_fn(&self) -> AccessorFn {
        match self.kind {
            FieldKind::Repeated(ref repeated_field) => self.accessor_fn_repeated(repeated_field),
            FieldKind::Map(ref map_field) => self.accessor_fn_map(map_field),
            FieldKind::Singular(SingularField {
                ref elem,
                flag: SingularFieldFlag::WithoutFlag,
            }) => self.accessor_fn_singular_without_flag(elem),
            FieldKind::Singular(SingularField {
                ref elem,
                flag: SingularFieldFlag::WithFlag { option_kind, .. },
            }) => self.accessor_fn_singular_with_flag(elem, option_kind),
            FieldKind::Oneof(ref oneof) => self.accessor_fn_oneof(oneof),
        }
    }

    pub fn write_push_accessor(&self, fields_var: &str, w: &mut CodeWriter) {
        let accessor_fn = self.accessor_fn();
        w.write_line(&format!(
            "{}.push({}::reflect::rt::v2::{}(",
            fields_var,
            protobuf_crate_path(&self.customize),
            accessor_fn.sig()
        ));
        w.indented(|w| {
            w.write_line(&format!("\"{}\",", self.proto_field.name()));
            for callback in &accessor_fn.callback_params {
                let callback_lines: Vec<&str> = callback.lines().collect();
                for (i, callback_line) in callback_lines.iter().enumerate() {
                    let comma = if i == callback_lines.len() - 1 {
                        ","
                    } else {
                        ""
                    };
                    w.write_line(&format!("{}{}", callback_line, comma));
                }
            }
        });
        w.write_line("));");
    }
}

use crate::code_writer::CodeWriter;
use crate::field::FieldElem;
use crate::field::FieldGen;
use crate::field::FieldKind;
use crate::field::MapField;
use crate::field::OptionKind;
use crate::field::RepeatedField;
use crate::field::SingularField;
use crate::field::SingularFieldFlag;
use crate::inside::protobuf_crate_path;
use crate::oneof::OneofField;
use crate::rust_types_values::ProtobufTypeGen;
use crate::rust_types_values::RustType;
use protobuf::descriptor::field_descriptor_proto;

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
    fn accessor_fn_map(&self, map_field: &MapField) -> AccessorFn {
        let MapField {
            ref key, ref value, ..
        } = map_field;
        AccessorFn {
            name: "make_map_accessor".to_owned(),
            type_params: vec![
                key.lib_protobuf_type(&self.get_file_and_mod()),
                value.lib_protobuf_type(&self.get_file_and_mod()),
            ],
            callback_params: self.make_accessor_fns_lambda(),
        }
    }

    fn accessor_fn_repeated(&self, repeated_field: &RepeatedField) -> AccessorFn {
        let RepeatedField { ref elem, .. } = repeated_field;
        let coll = match self.full_storage_type(
            &self
                .proto_field
                .message
                .scope
                .get_file_and_mod(self.customize.clone()),
        ) {
            RustType::Vec(..) => "vec",
            RustType::RepeatedField(..) => "repeated_field",
            _ => unreachable!(),
        };
        let name = format!("make_{}_accessor", coll);
        AccessorFn {
            name: name,
            type_params: vec![elem.lib_protobuf_type(&self.get_file_and_mod())],
            callback_params: self.make_accessor_fns_lambda(),
        }
    }

    fn accessor_fn_singular_without_flag(&self, elem: &FieldElem) -> AccessorFn {
        if let &FieldElem::Message(ref m, ..) = elem {
            // TODO: old style, needed because of default instance

            AccessorFn {
                name: "make_singular_message_accessor".to_owned(),
                type_params: vec![format!(
                    "{}",
                    m.rust_name_relative(&self.get_file_and_mod())
                )],
                callback_params: self.make_accessor_fns_has_get(),
            }
        } else {
            AccessorFn {
                name: "make_simple_field_accessor".to_owned(),
                type_params: vec![elem.lib_protobuf_type(&self.get_file_and_mod())],
                callback_params: self.make_accessor_fns_lambda(),
            }
        }
    }

    fn accessor_fn_singular_with_flag(
        &self,
        elem: &FieldElem,
        _option_kind: OptionKind,
    ) -> AccessorFn {
        match elem {
            FieldElem::Message(..) => AccessorFn {
                name: "make_option_accessor".to_owned(),
                type_params: vec![
                    elem.lib_protobuf_type(&self.get_file_and_mod()),
                    "_".to_owned(),
                ],
                callback_params: self.make_accessor_fns_lambda(),
            },
            FieldElem::Primitive(field_descriptor_proto::Type::TYPE_STRING, ..)
            | FieldElem::Primitive(field_descriptor_proto::Type::TYPE_BYTES, ..) => AccessorFn {
                name: "make_option_get_ref_accessor".to_owned(),
                type_params: vec![
                    elem.lib_protobuf_type(&self.get_file_and_mod()),
                    "_".to_owned(),
                ],
                callback_params: self.make_accessor_fns_lambda_get(),
            },
            FieldElem::Primitive(..) => AccessorFn {
                name: "make_option_get_copy_accessor".to_owned(),
                type_params: vec![
                    elem.lib_protobuf_type(&self.get_file_and_mod()),
                    "_".to_owned(),
                ],
                callback_params: self.make_accessor_fns_lambda_get(),
            },
            FieldElem::Enum(ref en) => AccessorFn {
                name: "make_option_enum_accessor".to_owned(),
                type_params: vec![format!(
                    "{}",
                    en.rust_name_relative(&self.get_file_and_mod())
                )],
                callback_params: self.make_accessor_fns_lambda_default_value(),
            },
            FieldElem::Group => {
                unreachable!("no accessor for group field");
            }
        }
    }

    fn accessor_fn_oneof(&self, oneof: &OneofField) -> AccessorFn {
        let OneofField { ref elem, .. } = oneof;
        // TODO: uses old style

        let reference = self
            .proto_field
            .message
            .scope
            .get_file_and_mod(self.customize.clone());

        if let FieldElem::Enum(ref en) = oneof.elem {
            return AccessorFn {
                name: "make_oneof_copy_has_get_set_accessors".to_owned(),
                type_params: vec![ProtobufTypeGen::Enum(
                    en.rust_name_relative(&self.get_file_and_mod()),
                )
                .rust_type(&self.customize)],
                callback_params: self.make_accessor_fns_has_get_set(),
            };
        }

        if elem.rust_storage_elem_type(&reference).is_copy() {
            return AccessorFn {
                name: "make_oneof_copy_has_get_set_accessors".to_owned(),
                type_params: vec![elem
                    .protobuf_type_gen(&self.get_file_and_mod())
                    .rust_type(&self.customize)],
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
            name: "make_oneof_deref_has_get_set_accessor".to_owned(),
            type_params: vec![elem
                .protobuf_type_gen(&self.get_file_and_mod())
                .rust_type(&self.customize)],
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

    pub fn write_descriptor_field(&self, fields_var: &str, w: &mut CodeWriter) {
        let accessor_fn = self.accessor_fn();
        w.write_line(&format!(
            "{}.push({}::reflect::rt::{}(",
            fields_var,
            protobuf_crate_path(&self.customize),
            accessor_fn.sig()
        ));
        w.indented(|w| {
            w.write_line(&format!("\"{}\",", self.proto_field.name()));
            for callback in accessor_fn.callback_params {
                w.write_line(&format!("{},", callback));
            }
        });
        w.write_line("));");
    }
}

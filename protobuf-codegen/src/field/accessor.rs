use crate::code_writer::CodeWriter;
use crate::field::FieldElem;
use crate::field::FieldGen;
use crate::field::FieldKind;
use crate::field::MapField;
use crate::field::RepeatedField;
use crate::field::SingularField;
use crate::field::SingularFieldFlag;
use crate::inside::protobuf_crate_path;
use crate::oneof::OneofField;
use crate::rust_types_values::RustType;

enum AccessorStyle {
    Lambda,
    HasGet,
}

struct AccessorFn {
    name: String,
    // function type params after first underscore
    type_params: Vec<String>,
    pub style: AccessorStyle,
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
    fn accessor_fn(&self) -> AccessorFn {
        match self.kind {
            FieldKind::Repeated(RepeatedField { ref elem, .. }) => {
                let coll = match self.full_storage_type() {
                    RustType::Vec(..) => "vec",
                    RustType::RepeatedField(..) => "repeated_field",
                    _ => unreachable!(),
                };
                let name = format!("make_{}_accessor", coll);
                AccessorFn {
                    name: name,
                    type_params: vec![elem.lib_protobuf_type(&self.customize)],
                    style: AccessorStyle::Lambda,
                }
            }
            FieldKind::Map(MapField {
                ref key, ref value, ..
            }) => AccessorFn {
                name: "make_map_accessor".to_owned(),
                type_params: vec![
                    key.lib_protobuf_type(&self.customize),
                    value.lib_protobuf_type(&self.customize),
                ],
                style: AccessorStyle::Lambda,
            },
            FieldKind::Singular(SingularField {
                ref elem,
                flag: SingularFieldFlag::WithoutFlag,
            }) => {
                if let &FieldElem::Message(ref name, ..) = elem {
                    // TODO: old style, needed because of default instance

                    AccessorFn {
                        name: "make_singular_message_accessor".to_owned(),
                        type_params: vec![name.clone()],
                        style: AccessorStyle::HasGet,
                    }
                } else {
                    AccessorFn {
                        name: "make_simple_field_accessor".to_owned(),
                        type_params: vec![elem.lib_protobuf_type(&self.customize)],
                        style: AccessorStyle::Lambda,
                    }
                }
            }
            FieldKind::Singular(SingularField {
                ref elem,
                flag: SingularFieldFlag::WithFlag { .. },
            }) => {
                let coll = match self.full_storage_type() {
                    RustType::Option(..) => "option",
                    RustType::SingularField(..) => "singular_field",
                    RustType::SingularPtrField(..) => "singular_ptr_field",
                    _ => unreachable!(),
                };
                let name = format!("make_{}_accessor", coll);
                AccessorFn {
                    name: name,
                    type_params: vec![elem.lib_protobuf_type(&self.customize)],
                    style: AccessorStyle::Lambda,
                }
            }
            FieldKind::Oneof(OneofField { ref elem, .. }) => {
                // TODO: uses old style

                let suffix = match &self.elem().rust_storage_type() {
                    t if t.is_primitive() => t.to_code(&self.customize),
                    &RustType::String | &RustType::Chars => "string".to_string(),
                    &RustType::Vec(ref t) if t.is_u8() => "bytes".to_string(),
                    &RustType::Bytes => "bytes".to_string(),
                    &RustType::Enum(..) => "enum".to_string(),
                    &RustType::Message(..) => "message".to_string(),
                    t => panic!("unexpected field type: {:?}", t),
                };

                let name = format!("make_singular_{}_accessor", suffix);

                let mut type_params = Vec::new();
                match elem {
                    &FieldElem::Message(ref name, ..) | &FieldElem::Enum(ref name, ..) => {
                        type_params.push(name.to_owned());
                    }
                    _ => (),
                }

                AccessorFn {
                    name,
                    type_params,
                    style: AccessorStyle::HasGet,
                }
            }
        }
    }

    pub fn write_descriptor_field(
        &self,
        message_type_name: &str,
        fields_var: &str,
        w: &mut CodeWriter,
    ) {
        let accessor_fn = self.accessor_fn();
        w.write_line(&format!(
            "{}.push({}::reflect::rt::{}(",
            fields_var,
            protobuf_crate_path(&self.customize),
            accessor_fn.sig()
        ));
        w.indented(|w| {
            w.write_line(&format!("\"{}\",", self.proto_field.name()));
            match accessor_fn.style {
                AccessorStyle::Lambda => {
                    w.write_line(&format!(
                        "|m: &{}| {{ &m.{} }},",
                        message_type_name, self.rust_name
                    ));
                    w.write_line(&format!(
                        "|m: &mut {}| {{ &mut m.{} }},",
                        message_type_name, self.rust_name
                    ));
                }
                AccessorStyle::HasGet => {
                    w.write_line(&format!("{}::has_{},", message_type_name, self.rust_name));
                    w.write_line(&format!("{}::get_{},", message_type_name, self.rust_name));
                }
            }
        });
        w.write_line("));");
    }
}

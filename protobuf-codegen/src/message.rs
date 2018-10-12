use protobuf::descriptor::*;
use protobuf::descriptorx::*;

use protobuf::prelude::*;

use super::enums::*;
use super::rust_types_values::*;
use super::field::*;
use super::code_writer::*;
use super::customize::Customize;
use super::customize::customize_from_rustproto_for_message;
use oneof::OneofGen;
use oneof::OneofVariantGen;
use map::map_entry;
use serde;


/// Message info for codegen
pub struct MessageGen<'a> {
    message: &'a MessageWithScope<'a>,
    root_scope: &'a RootScope<'a>,
    type_name: String,
    pub fields: Vec<FieldGen<'a>>,
    pub lite_runtime: bool,
    customize: Customize,
}

impl<'a> MessageGen<'a> {
    pub fn new(
        message: &'a MessageWithScope<'a>,
        root_scope: &'a RootScope<'a>,
        customize: &Customize)
        -> MessageGen<'a>
    {
        let mut customize = customize.clone();
        customize.update_with(&customize_from_rustproto_for_message(
            message.message.options.get_message()));

        let fields: Vec<_> = message
            .fields()
            .into_iter()
            .map(|field| FieldGen::parse(field, root_scope, &customize))
            .collect();
        MessageGen {
            message: message,
            root_scope: root_scope,
            type_name: message.rust_name(),
            fields: fields,
            lite_runtime: message
                .get_file_descriptor()
                .options.get_message()
                .get_optimize_for() == FileOptions_OptimizeMode::LITE_RUNTIME,
            customize,
        }
    }

    fn expose_oneof(&self) -> bool {
        self.customize.expose_oneof.unwrap_or(true)
    }

    fn oneofs(&'a self) -> Vec<OneofGen<'a>> {
        self.message
            .oneofs()
            .into_iter()
            .map(|oneof| OneofGen::parse(self, oneof, &self.customize))
            .collect()
    }

    fn required_fields(&'a self) -> Vec<&'a FieldGen> {
        self.fields
            .iter()
            .filter(|f| match f.kind {
                FieldKind::Singular(ref singular) => singular.flag.is_required(),
                _ => false,
            })
            .collect()
    }

    fn message_fields(&'a self) -> Vec<&'a FieldGen> {
        self.fields
            .iter()
            .filter(|f| f.proto_type == FieldDescriptorProto_Type::TYPE_MESSAGE)
            .collect()
    }

    fn fields_except_oneof(&'a self) -> Vec<&'a FieldGen> {
        self.fields.iter().filter(|f| !f.is_oneof()).collect()
    }

    fn fields_except_group(&'a self) -> Vec<&'a FieldGen> {
        self.fields
            .iter()
            .filter(|f| f.proto_type != FieldDescriptorProto_Type::TYPE_GROUP)
            .collect()
    }

    fn fields_except_oneof_and_group(&'a self) -> Vec<&'a FieldGen> {
        self.fields
            .iter()
            .filter(|f| {
                !f.is_oneof() && f.proto_type != FieldDescriptorProto_Type::TYPE_GROUP
            })
            .collect()
    }


    fn write_match_each_oneof_variant<F>(&self, w: &mut CodeWriter, cb: F)
    where
        F: Fn(&mut CodeWriter, &OneofVariantGen, &str, &RustType),
    {
        for oneof in self.oneofs() {
            let variants = oneof.variants_except_group();
            if variants.is_empty() {
                // Special case because
                // https://github.com/rust-lang/rust/issues/50642
                continue;
            }
            w.if_let_stmt("::std::option::Option::Some(ref v)", &format!("self.{}", oneof.name())[..], |w| {
                w.match_block("v", |w| {
                    for variant in variants {
                        let ref field = variant.field;

                        let (refv, vtype) =
                            if field.elem_type_is_copy() {
                                ("v", variant.rust_type())
                            } else {
                                ("ref v", variant.rust_type().ref_type())
                            };
                            w.case_block(format!("&{}({})", variant.path(), refv), |w| {
                                cb(w, &variant, "v", &vtype);
                            });
                        }
                    });
                },
            );
        }
    }

    fn write_write_to_with_cached_sizes(&self, w: &mut CodeWriter) {
        w.def_fn("write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()>", |w| {
            // To have access to its methods but not polute the name space.
            for f in self.fields_except_oneof_and_group() {
                f.write_message_write_field(w);
            }
            self.write_match_each_oneof_variant(w, |w, variant, v, v_type| {
                let v = RustValueTyped { value: v.to_owned(), rust_type: v_type.clone() };
                variant.field.write_write_element(w, "os", &v);
            });
            w.write_line("os.write_unknown_fields(self.get_unknown_fields())?;");
            w.write_line("::std::result::Result::Ok(())");
        });
    }

    fn write_get_cached_size(&self, w: &mut CodeWriter) {
        w.def_fn("get_cached_size(&self) -> u32", |w| {
            w.write_line("self.cached_size.get()");
        });
    }

    fn write_default_instance(&self, w: &mut CodeWriter) {
        w.def_fn(&format!("default_instance() -> &'static {}", self.type_name), |w| {
            w.lazy_static_decl_get_simple(
                "instance",
                &self.type_name,
                &format!("{}::new", self.type_name));
        });
    }

    fn write_compute_size(&self, w: &mut CodeWriter) {
        // Append sizes of messages in the tree to the specified vector.
        // First appended element is size of self, and then nested message sizes.
        // in serialization order are appended recursively.");
        w.comment("Compute sizes of nested messages");
        // there are unused variables in oneof
        w.allow(&["unused_variables"]);
        w.def_fn("compute_size(&self) -> u32", |w| {
            // To have access to its methods but not polute the name space.
            w.write_line("let mut my_size = 0;");
            for field in self.fields_except_oneof_and_group() {
                field.write_message_compute_field_size("my_size", w);
            }
            self.write_match_each_oneof_variant(w, |w, variant, v, vtype| {
                variant.field.write_element_size(w, v, vtype, "my_size");
            });
            w.write_line(
                "my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());",
            );
            w.write_line("self.cached_size.set(my_size);");
            w.write_line("my_size");
        });
    }

    fn write_field_accessors(&self, w: &mut CodeWriter) {
        for f in self.fields_except_group() {
            f.write_message_single_field_accessors(w);
        }
    }

    fn write_impl_self(&self, w: &mut CodeWriter) {
        w.impl_self_block(&self.type_name, |w| {
            // TODO: new should probably be a part of Message trait
            w.pub_fn(&format!("new() -> {}", self.type_name), |w| {
                w.write_line("::std::default::Default::default()");
            });

            self.write_field_accessors(w);
        });
    }

    fn write_unknown_fields(&self, w: &mut CodeWriter) {
        w.def_fn(
            "get_unknown_fields(&self) -> &::protobuf::UnknownFields",
            |w| {
                w.write_line("&self.unknown_fields");
            },
        );
        w.write_line("");
        w.def_fn(
            "mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields",
            |w| {
                w.write_line("&mut self.unknown_fields");
            },
        );
    }

    fn write_merge_from(&self, w: &mut CodeWriter) {
        w.def_fn(&format!("merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()>"), |w| {
            w.while_block("!is.eof()?", |w| {
                w.write_line(&format!("let (field_number, wire_type) = is.read_tag_unpack()?;"));
                w.match_block("field_number", |w| {
                    for f in &self.fields_except_group() {
                        let number = f.proto_field.number();
                        w.case_block(number.to_string(), |w| {
                            f.write_merge_from_field("wire_type", w);
                        });
                    }
                    w.case_block("_", |w| {
                        w.write_line("::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;");
                    });
                });
            });
            w.write_line("::std::result::Result::Ok(())");
        });
    }

    fn write_descriptor_static(&self, w: &mut CodeWriter) {
        w.def_fn(&format!("descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor"), |w| {
            w.lazy_static_decl_get("descriptor", "::protobuf::reflect::MessageDescriptor", |w| {
                let fields = self.fields_except_group();
                if fields.is_empty() {
                    w.write_line(&format!("let fields = ::std::vec::Vec::new();"));
                } else {
                    w.write_line(&format!("let mut fields = ::std::vec::Vec::new();"));
                }
                for field in fields {
                    field.write_descriptor_field("fields", w);;
                }
                w.write_line(&format!(
                    "::protobuf::reflect::MessageDescriptor::new::<{}>(", self.type_name));
                w.indented(|w| {
                    w.write_line(&format!("\"{}\",", self.type_name));
                    w.write_line("fields,");
                    w.write_line("file_descriptor_proto()");
                });
                w.write_line(")");
            });
        });
    }

    fn write_is_initialized(&self, w: &mut CodeWriter) {
        w.def_fn(&format!("is_initialized(&self) -> bool"), |w| {
            // TODO: use single loop

            for f in self.required_fields() {
                f.write_if_self_field_is_none(w, |w| {
                    w.write_line("return false;");
                });
            }

            for f in self.message_fields() {
                if let FieldKind::Map(..) = f.kind {
                    // TODO: check values
                    continue;
                }

                // TODO:
                // if message is declared in this file and has no message fields,
                // we could skip the check here
                f.write_for_self_field(w, "v", |w, _t| {
                    w.if_stmt("!v.is_initialized()", |w| {
                        w.write_line("return false;");
                    });
                });
            }
            w.write_line("true");
        });
    }

    fn write_impl_message(&self, w: &mut CodeWriter) {
        w.impl_for_block("::protobuf::Message", &self.type_name, |w| {
            self.write_is_initialized(w);
            w.write_line("");
            self.write_merge_from(w);
            w.write_line("");
            self.write_compute_size(w);
            w.write_line("");
            self.write_write_to_with_cached_sizes(w);
            w.write_line("");
            self.write_get_cached_size(w);
            w.write_line("");
            self.write_unknown_fields(w);
            w.write_line("");
            w.def_fn("descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor", |w| {
                w.write_line("Self::descriptor_static()");
            });
            w.write_line("");
            w.def_fn(&format!("new() -> {}", self.type_name), |w| {
                w.write_line(&format!("{}::new()", self.type_name));
            });
            if !self.lite_runtime {
                w.write_line("");
                self.write_descriptor_static(w);
            }
            w.write_line("");
            self.write_default_instance(w);
        });
    }

    fn write_impl_value(&self, w: &mut CodeWriter) {
        w.impl_for_block("::protobuf::reflect::ProtobufValue", &self.type_name, |_w| {
        });
    }

    fn write_impl_show(&self, w: &mut CodeWriter) {
        w.impl_for_block("::std::fmt::Debug", &self.type_name, |w| {
            w.def_fn(
                "fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result",
                |w| {
                    w.write_line("::protobuf::text_format::fmt(self, f)");
                },
            );
        });
    }

    fn write_impl_clear(&self, w: &mut CodeWriter) {
        w.impl_for_block("::protobuf::Clear", &self.type_name, |w| {
            w.def_fn("clear(&mut self)", |w| {
                for f in self.fields_except_group() {
                    f.write_clear(w);
                }
                w.write_line("self.unknown_fields.clear();");
            });
        });
    }

    fn supports_derive_partial_eq(&self) -> bool {
        // There's stack overflow in the compiler when struct has too many fields
        // https://github.com/rust-lang/rust/issues/40119
        self.fields.len() <= 500
    }

    fn write_struct(&self, w: &mut CodeWriter) {
        let mut derive = Vec::new();
        if self.supports_derive_partial_eq() {
            derive.push("PartialEq");
        }
        derive.extend(&["Clone", "Default"]);
        if self.lite_runtime {
            derive.push("Debug");
        }
        w.derive(&derive);
        serde::write_serde_attr(w, &self.customize, "derive(Serialize, Deserialize)");
        w.pub_struct(&self.type_name, |w| {
            if !self.fields_except_oneof().is_empty() {
                w.comment("message fields");
                for field in self.fields_except_oneof() {
                    field.write_struct_field(w);
                }
            }
            if !self.oneofs().is_empty() {
                w.comment("message oneof groups");
                for oneof in self.oneofs() {
                    let vis = match self.expose_oneof() {
                        true => Visibility::Public,
                        false => Visibility::Default,
                    };
                    w.field_decl_vis(vis, oneof.name(), &oneof.full_storage_type().to_string());
                }
            }
            w.comment("special fields");

            serde::write_serde_attr(w, &self.customize, "serde(skip)");
            w.pub_field_decl("unknown_fields", "::protobuf::UnknownFields");
            serde::write_serde_attr(w, &self.customize, "serde(skip)");
            w.field_decl("cached_size", "::protobuf::CachedSize");
        });
    }

    fn write_dummy_impl_partial_eq(&self, w: &mut CodeWriter) {
        w.impl_for_block("::std::cmp::PartialEq", &self.type_name, |w| {
            w.def_fn("eq(&self, _: &Self) -> bool", |w| {
                w.comment("https://github.com/rust-lang/rust/issues/40119");
                w.unimplemented();
            });
        });
    }

    pub fn write(&self, w: &mut CodeWriter) {
        self.write_struct(w);

        if !self.supports_derive_partial_eq() {
            w.write_line("");
            self.write_dummy_impl_partial_eq(w);
        }

        for oneof in self.oneofs() {
            w.write_line("");
            oneof.write(w);
        }

        w.write_line("");
        self.write_impl_self(w);
        w.write_line("");
        self.write_impl_message(w);
        w.write_line("");
        self.write_impl_clear(w);
        if !self.lite_runtime {
            w.write_line("");
            self.write_impl_show(w);
        }
        w.write_line("");
        self.write_impl_value(w);

        let mut nested_prefix = self.type_name.to_string();
        nested_prefix.push_str("_");

        for nested in &self.message.to_scope().get_messages() {
            // ignore map entries, because they are not used in map fields
            if map_entry(nested).is_none() {
                w.write_line("");
                MessageGen::new(nested, self.root_scope, &self.customize).write(w);
            }
        }

        for enum_type in &self.message.to_scope().get_enums() {
            w.write_line("");
            let current_file = self.message.get_scope().get_file_descriptor();
            EnumGen::new(enum_type, current_file, &self.customize, self.root_scope).write(w);
        }
    }
}

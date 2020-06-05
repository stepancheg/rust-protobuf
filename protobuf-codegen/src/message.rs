use protobuf::descriptor::*;

use protobuf::prelude::*;

use super::code_writer::*;
use super::customize::customize_from_rustproto_for_message;
use super::customize::Customize;
use super::enums::*;
use super::field::*;
use super::rust_types_values::*;
use crate::case_convert::snake_case;
use crate::file_and_mod::FileAndMod;
use crate::file_descriptor::file_descriptor_proto_expr;
use crate::inside::protobuf_crate_path;
use crate::map::map_entry;
use crate::oneof::OneofGen;
use crate::oneof::OneofVariantGen;
use crate::rust::is_rust_keyword;
use crate::rust_name::RustIdent;
use crate::rust_name::RustIdentWithPath;
use crate::scope::MessageWithScope;
use crate::scope::RootScope;
use crate::scope::WithScope;
use crate::serde;

/// Message info for codegen
pub(crate) struct MessageGen<'a> {
    pub message: &'a MessageWithScope<'a>,
    pub root_scope: &'a RootScope<'a>,
    type_name: RustIdentWithPath,
    pub fields: Vec<FieldGen<'a>>,
    pub lite_runtime: bool,
    customize: Customize,
    path: &'a [i32],
    info: Option<&'a SourceCodeInfo>,
}

impl<'a> MessageGen<'a> {
    pub fn new(
        message: &'a MessageWithScope<'a>,
        root_scope: &'a RootScope<'a>,
        customize: &Customize,
        path: &'a [i32],
        info: Option<&'a SourceCodeInfo>,
    ) -> MessageGen<'a> {
        let mut customize = customize.clone();
        customize.update_with(&customize_from_rustproto_for_message(
            message.message.options.get_message(),
        ));

        static FIELD_NUMBER: protobuf::rt::Lazy<i32> = protobuf::rt::Lazy::INIT;
        let field_number = *FIELD_NUMBER.get(|| {
            protobuf::reflect::MessageDescriptor::for_type::<DescriptorProto>()
                .get_field_by_name("field")
                .expect("`field` must exist")
                .proto()
                .get_number()
        });

        let fields: Vec<_> = message
            .fields()
            .into_iter()
            .enumerate()
            .map(|(id, field)| {
                let mut path = path.to_vec();
                path.extend_from_slice(&[field_number, id as i32]);
                FieldGen::parse(field, root_scope, &customize, path, info)
            })
            .collect();
        let lite_runtime = customize.lite_runtime.unwrap_or_else(|| {
            message
                .get_file_descriptor()
                .options
                .get_message()
                .get_optimize_for()
                == file_options::OptimizeMode::LITE_RUNTIME
        });
        MessageGen {
            message,
            root_scope,
            type_name: message.rust_name().to_path(),
            fields,
            lite_runtime,
            customize,
            path,
            info,
        }
    }

    pub fn get_file_and_mod(&self) -> FileAndMod {
        self.message.scope.get_file_and_mod(self.customize.clone())
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
            .filter(|f| f.proto_type == field_descriptor_proto::Type::TYPE_MESSAGE)
            .collect()
    }

    fn fields_except_oneof(&'a self) -> Vec<&'a FieldGen> {
        self.fields
            .iter()
            .filter(|f| match f.kind {
                FieldKind::Oneof(..) => false,
                _ => true,
            })
            .collect()
    }

    fn fields_except_group(&'a self) -> Vec<&'a FieldGen> {
        self.fields
            .iter()
            .filter(|f| f.proto_type != field_descriptor_proto::Type::TYPE_GROUP)
            .collect()
    }

    fn fields_except_oneof_and_group(&'a self) -> Vec<&'a FieldGen> {
        self.fields
            .iter()
            .filter(|f| match f.kind {
                FieldKind::Oneof(..) => false,
                _ => f.proto_type != field_descriptor_proto::Type::TYPE_GROUP,
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
            w.if_let_stmt(
                "::std::option::Option::Some(ref v)",
                &format!("self.{}", oneof.oneof.field_name())[..],
                |w| {
                    w.match_block("v", |w| {
                        for variant in variants {
                            let ref field = variant.field;

                            let (refv, vtype) = if field.elem_type_is_copy() {
                                ("v", variant.rust_type(&self.get_file_and_mod()))
                            } else {
                                (
                                    "ref v",
                                    variant.rust_type(&self.get_file_and_mod()).ref_type(),
                                )
                            };
                            w.case_block(
                                format!("&{}({})", variant.path(&self.get_file_and_mod()), refv),
                                |w| {
                                    cb(w, &variant, "v", &vtype);
                                },
                            );
                        }
                    });
                },
            );
        }
    }

    fn write_write_to_with_cached_sizes(&self, w: &mut CodeWriter) {
        let sig = format!(
            "write_to_with_cached_sizes(&self, os: &mut {}::CodedOutputStream<'_>) -> {}::ProtobufResult<()>",
            protobuf_crate_path(&self.customize),
            protobuf_crate_path(&self.customize),
        );
        w.def_fn(&sig, |w| {
            // To have access to its methods but not polute the name space.
            for f in self.fields_except_oneof_and_group() {
                f.write_message_write_field(w);
            }
            self.write_match_each_oneof_variant(w, |w, variant, v, v_type| {
                let v = RustValueTyped {
                    value: v.to_owned(),
                    rust_type: v_type.clone(),
                };
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
        w.def_fn(
            &format!("default_instance() -> &'static {}", self.type_name),
            |w| {
                w.lazy_static_decl_get_simple(
                    "instance",
                    &format!("{}", self.type_name),
                    &format!("{}::new", self.type_name),
                    protobuf_crate_path(&self.customize),
                );
            },
        );
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
            w.write_line(&format!(
                "my_size += {}::rt::unknown_fields_size(self.get_unknown_fields());",
                protobuf_crate_path(&self.customize)
            ));
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
        w.impl_self_block(&format!("{}", self.type_name), |w| {
            // TODO: new should probably be a part of Message trait
            w.pub_fn(&format!("new() -> {}", self.type_name), |w| {
                w.write_line("::std::default::Default::default()");
            });

            self.write_field_accessors(w);
        });
    }

    fn write_unknown_fields(&self, w: &mut CodeWriter) {
        let sig = format!(
            "get_unknown_fields(&self) -> &{}::UnknownFields",
            protobuf_crate_path(&self.customize)
        );
        w.def_fn(&sig, |w| {
            w.write_line("&self.unknown_fields");
        });
        w.write_line("");
        let sig = format!(
            "mut_unknown_fields(&mut self) -> &mut {}::UnknownFields",
            protobuf_crate_path(&self.customize)
        );
        w.def_fn(&sig, |w| {
            w.write_line("&mut self.unknown_fields");
        });
    }

    fn write_merge_from(&self, w: &mut CodeWriter) {
        let sig = format!(
            "merge_from(&mut self, is: &mut {}::CodedInputStream<'_>) -> {}::ProtobufResult<()>",
            protobuf_crate_path(&self.customize),
            protobuf_crate_path(&self.customize),
        );
        w.def_fn(&sig, |w| {
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
                        w.write_line(&format!("{}::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;", protobuf_crate_path(&self.customize)));
                    });
                });
            });
            w.write_line("::std::result::Result::Ok(())");
        });
    }

    fn write_descriptor_static(&self, w: &mut CodeWriter) {
        let sig = format!(
            "descriptor_static() -> &'static {}::reflect::MessageDescriptor",
            protobuf_crate_path(&self.customize)
        );
        w.def_fn(&sig, |w| {
            w.lazy_static_decl_get(
                "descriptor",
                &format!(
                    "{}::reflect::MessageDescriptor",
                    protobuf_crate_path(&self.customize)
                ),
                protobuf_crate_path(&self.customize),
                |w| {
                    let fields = self.fields_except_group();
                    if fields.is_empty() {
                        w.write_line(&format!("let fields = ::std::vec::Vec::new();"));
                    } else {
                        w.write_line(&format!("let mut fields = ::std::vec::Vec::new();"));
                    }
                    for field in fields {
                        field.write_descriptor_field("fields", w);
                    }
                    w.write_line(&format!(
                        "{}::reflect::MessageDescriptor::new::<{}>(",
                        protobuf_crate_path(&self.customize),
                        self.type_name,
                    ));
                    w.indented(|w| {
                        w.write_line(&format!("\"{}\",", self.message.name_to_package()));
                        w.write_line("fields,");
                        w.write_line(&file_descriptor_proto_expr(&self.message.scope));
                    });
                    w.write_line(")");
                },
            );
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
        w.impl_for_block(
            &format!("{}::Message", protobuf_crate_path(&self.customize)),
            &format!("{}", self.type_name),
            |w| {
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
                w.def_fn(
                    &format!(
                        "descriptor(&self) -> &'static {}::reflect::MessageDescriptor",
                        protobuf_crate_path(&self.customize)
                    ),
                    |w| {
                        w.write_line("Self::descriptor_static()");
                    },
                );
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
            },
        );
    }

    fn write_impl_value(&self, w: &mut CodeWriter) {
        w.impl_for_block(
            &format!(
                "{}::reflect::ProtobufValue",
                protobuf_crate_path(&self.customize)
            ),
            &format!("{}", self.type_name),
            |_w| {},
        );
    }

    fn write_impl_show(&self, w: &mut CodeWriter) {
        w.impl_for_block("::std::fmt::Debug", &format!("{}", self.type_name), |w| {
            w.def_fn(
                "fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result",
                |w| {
                    w.write_line(&format!(
                        "{}::text_format::fmt(self, f)",
                        protobuf_crate_path(&self.customize)
                    ));
                },
            );
        });
    }

    fn write_impl_clear(&self, w: &mut CodeWriter) {
        w.impl_for_block(
            &format!("{}::Clear", protobuf_crate_path(&self.customize)),
            &format!("{}", self.type_name),
            |w| {
                w.def_fn("clear(&mut self)", |w| {
                    for f in self.fields_except_group() {
                        f.write_clear(w);
                    }
                    w.write_line("self.unknown_fields.clear();");
                });
            },
        );
    }

    fn supports_derive_partial_eq(&self) -> bool {
        // There's stack overflow in the compiler when struct has too many fields
        // https://github.com/rust-lang/rust/issues/40119
        self.fields.len() <= 500
    }

    fn write_struct(&self, w: &mut CodeWriter, customize: &Customize) {
        let mut derive = Vec::new();
        if self.supports_derive_partial_eq() {
            derive.push("PartialEq");
        }
        if let Some(ref d) = customize.derives {
            derive.push(d.as_str());
        }
        derive.extend(&["Clone", "Default"]);

        if self.lite_runtime {
            derive.push("Debug");
        }
        w.derive(&derive);
        serde::write_serde_attr(w, &self.customize, "derive(Serialize, Deserialize)");
        w.pub_struct(&format!("{}", self.type_name), |w| {
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
                    w.field_decl_vis(
                        vis,
                        &oneof.oneof.field_name().to_string(),
                        &oneof.full_storage_type().to_code(&self.customize),
                    );
                }
            }
            w.comment("special fields");

            serde::write_serde_attr(w, &self.customize, "serde(skip)");
            w.pub_field_decl(
                "unknown_fields",
                &format!("{}::UnknownFields", protobuf_crate_path(&self.customize)),
            );
            serde::write_serde_attr(w, &self.customize, "serde(skip)");
            w.pub_field_decl(
                "cached_size",
                &format!("{}::rt::CachedSize", protobuf_crate_path(&self.customize)),
            );
        });
    }

    fn write_impl_default_for_amp(&self, w: &mut CodeWriter) {
        w.impl_args_for_block(
            &["'a"],
            "::std::default::Default",
            &format!("&'a {}", self.type_name),
            |w| {
                w.def_fn(&format!("default() -> &'a {}", self.type_name), |w| {
                    w.write_line(&format!(
                        "<{} as {}::Message>::default_instance()",
                        self.type_name,
                        protobuf_crate_path(&self.customize),
                    ));
                });
            },
        );
    }

    fn write_dummy_impl_partial_eq(&self, w: &mut CodeWriter) {
        w.impl_for_block(
            "::std::cmp::PartialEq",
            &format!("{}", self.type_name),
            |w| {
                w.def_fn("eq(&self, _: &Self) -> bool", |w| {
                    w.comment("https://github.com/rust-lang/rust/issues/40119");
                    w.unimplemented();
                });
            },
        );
    }

    pub fn write(&self, w: &mut CodeWriter, customize: &Customize) {
        w.all_documentation(self.info, self.path);
        self.write_struct(w, customize);

        w.write_line("");
        self.write_impl_default_for_amp(w);

        if !self.supports_derive_partial_eq() {
            w.write_line("");
            self.write_dummy_impl_partial_eq(w);
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

        let mod_name = message_name_to_nested_mod_name(&self.message.message.get_name());

        let oneofs = self.oneofs();
        let nested_messages: Vec<_> = self
            .message
            .to_scope()
            .get_messages()
            .into_iter()
            .filter(|nested| {
                // ignore map entries, because they are not used in map fields
                map_entry(nested).is_none()
            })
            .collect();
        let nested_enums = self.message.to_scope().get_enums();

        if !oneofs.is_empty() || !nested_messages.is_empty() || !nested_enums.is_empty() {
            w.write_line("");
            w.write_line(&format!(
                "/// Nested message and enums of message `{}`",
                self.message.message.get_name()
            ));
            w.pub_mod(mod_name.get(), |w| {
                let mut first = true;

                for oneof in &oneofs {
                    w.write_line("");
                    oneof.write(w, customize);
                }

                static NESTED_TYPE_NUMBER: protobuf::rt::Lazy<i32> = protobuf::rt::Lazy::INIT;
                let nested_type_number = *NESTED_TYPE_NUMBER.get(|| {
                    protobuf::reflect::MessageDescriptor::for_type::<DescriptorProto>()
                        .get_field_by_name("nested_type")
                        .expect("`nested_type` must exist")
                        .proto()
                        .get_number()
                });

                let mut path = self.path.to_vec();
                path.extend(&[nested_type_number, 0]);
                for (id, nested) in nested_messages.iter().enumerate() {
                    let len = path.len() - 1;
                    path[len] = id as i32;

                    if !first {
                        w.write_line("");
                    }
                    first = false;
                    MessageGen::new(nested, self.root_scope, &self.customize, &path, self.info)
                        .write(w, customize);
                }

                static ENUM_TYPE_NUMBER: protobuf::rt::Lazy<i32> = protobuf::rt::Lazy::INIT;
                let enum_type_number = *ENUM_TYPE_NUMBER.get(|| {
                    protobuf::reflect::MessageDescriptor::for_type::<DescriptorProto>()
                        .get_field_by_name("enum_type")
                        .expect("`enum_type` must exist")
                        .proto()
                        .get_number()
                });

                let len = path.len() - 2;
                path[len] = enum_type_number;
                for (id, enum_type) in self.message.to_scope().get_enums().iter().enumerate() {
                    let len = path.len() - 1;
                    path[len] = id as i32;

                    if !first {
                        w.write_line("");
                    }
                    first = false;
                    EnumGen::new(
                        enum_type,
                        &self.customize,
                        self.root_scope,
                        &path,
                        self.info,
                    )
                    .write(w, customize);
                }
            });
        }
    }
}

pub(crate) fn message_name_to_nested_mod_name(message_name: &str) -> RustIdent {
    let mut mod_name = snake_case(message_name);
    if is_rust_keyword(&mod_name) {
        mod_name.insert_str(0, "mod_");
    }
    RustIdent::new(&mod_name)
}

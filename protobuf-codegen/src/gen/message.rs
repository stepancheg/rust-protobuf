use std::fmt;

use protobuf::descriptor::*;
use protobuf::reflect::FileDescriptor;
use protobuf::reflect::MessageDescriptor;
use protobuf_parse::snake_case;

use crate::customize::ctx::CustomizeElemCtx;
use crate::customize::ctx::SpecialFieldPseudoDescriptor;
use crate::customize::rustproto_proto::customize_from_rustproto_for_message;
use crate::gen::code_writer::*;
use crate::gen::descriptor::write_fn_descriptor;
use crate::gen::enums::*;
use crate::gen::field::FieldGen;
use crate::gen::field::FieldKind;
use crate::gen::file_and_mod::FileAndMod;
use crate::gen::inside::protobuf_crate_path;
use crate::gen::oneof::OneofGen;
use crate::gen::oneof::OneofVariantGen;
use crate::gen::protoc_insertion_point::write_protoc_insertion_point_for_message;
use crate::gen::protoc_insertion_point::write_protoc_insertion_point_for_special_field;
use crate::gen::rust::ident::RustIdent;
use crate::gen::rust::ident_with_path::RustIdentWithPath;
use crate::gen::rust::rel_path::RustRelativePath;
use crate::gen::rust::snippets::expr_vec_with_capacity_const;
use crate::gen::rust::snippets::EXPR_NONE;
use crate::gen::rust_types_values::*;
use crate::gen::scope::MessageWithScope;
use crate::gen::scope::RootScope;
use crate::gen::scope::WithScope;
use crate::Customize;

/// Protobuf message Rust type name
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RustTypeMessage(pub RustIdentWithPath);

impl fmt::Display for RustTypeMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<S: Into<RustIdentWithPath>> From<S> for RustTypeMessage {
    fn from(s: S) -> Self {
        RustTypeMessage(s.into())
    }
}

impl RustTypeMessage {
    /// Code which emits default instance.
    pub fn default_instance(&self, customize: &Customize) -> String {
        format!(
            "<{} as {}::Message>::default_instance()",
            self.0,
            protobuf_crate_path(customize)
        )
    }
}

/// Message info for codegen
pub(crate) struct MessageGen<'a> {
    file_descriptor: &'a FileDescriptor,
    message_descriptor: MessageDescriptor,
    pub message: &'a MessageWithScope<'a>,
    pub root_scope: &'a RootScope<'a>,
    pub fields: Vec<FieldGen<'a>>,
    pub lite_runtime: bool,
    customize: CustomizeElemCtx<'a>,
    path: &'a [i32],
    info: Option<&'a SourceCodeInfo>,
}

impl<'a> MessageGen<'a> {
    pub fn new(
        file_descriptor: &'a FileDescriptor,
        message: &'a MessageWithScope<'a>,
        root_scope: &'a RootScope<'a>,
        parent_customize: &CustomizeElemCtx<'a>,
        path: &'a [i32],
        info: Option<&'a SourceCodeInfo>,
    ) -> anyhow::Result<MessageGen<'a>> {
        let message_descriptor = file_descriptor
            .message_by_package_relative_name(&format!("{}", message.protobuf_name_to_package()))
            .unwrap();

        let customize = parent_customize.child(
            &customize_from_rustproto_for_message(message.message.proto().options.get_or_default()),
            &message.message,
        );

        static FIELD_NUMBER: protobuf::rt::Lazy<i32> = protobuf::rt::Lazy::new();
        let field_number = *FIELD_NUMBER.get(|| {
            protobuf::reflect::MessageDescriptor::for_type::<DescriptorProto>()
                .field_by_name("field")
                .expect("`field` must exist")
                .proto()
                .number()
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
            .collect::<anyhow::Result<Vec<_>>>()?;
        let lite_runtime = customize.for_elem.lite_runtime.unwrap_or_else(|| {
            message.file_descriptor().proto().options.optimize_for()
                == file_options::OptimizeMode::LITE_RUNTIME
        });
        Ok(MessageGen {
            message_descriptor,
            file_descriptor,
            message,
            root_scope,
            fields,
            lite_runtime,
            customize,
            path,
            info,
        })
    }

    fn rust_name(&self) -> RustIdent {
        self.message.rust_name()
    }

    fn mod_name(&self) -> RustRelativePath {
        self.message.scope.rust_path_to_file()
    }

    pub fn file_and_mod(&self) -> FileAndMod {
        self.message
            .scope
            .file_and_mod(self.customize.for_elem.clone())
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
        F: Fn(&mut CodeWriter, &OneofVariantGen, &RustValueTyped),
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
                                ("v", variant.rust_type(&self.file_and_mod()))
                            } else {
                                ("ref v", variant.rust_type(&self.file_and_mod()).ref_type())
                            };
                            w.case_block(
                                format!("&{}({})", variant.path(&self.file_and_mod()), refv),
                                |w| {
                                    cb(
                                        w,
                                        &variant,
                                        &RustValueTyped {
                                            value: "v".to_owned(),
                                            rust_type: vtype.clone(),
                                        },
                                    );
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
            "write_to_with_cached_sizes(&self, os: &mut {protobuf_crate}::CodedOutputStream<'_>) -> {protobuf_crate}::Result<()>",
            protobuf_crate=protobuf_crate_path(&self.customize.for_elem),
        );
        w.def_fn(&sig, |w| {
            // To have access to its methods but not polute the name space.
            for f in self.fields_except_oneof_and_group() {
                f.write_message_write_field("os", w);
            }
            self.write_match_each_oneof_variant(w, |w, variant, v| {
                variant
                    .field
                    .write_write_element(variant.elem(), w, "os", v);
            });
            w.write_line("os.write_unknown_fields(self.special_fields.unknown_fields())?;");
            w.write_line("::std::result::Result::Ok(())");
        });
    }

    fn write_default_instance_lazy(&self, w: &mut CodeWriter) {
        w.lazy_static_decl_get_simple(
            "instance",
            &format!("{}", self.rust_name()),
            &format!("{}::new", self.rust_name()),
            &format!("{}", protobuf_crate_path(&self.customize.for_elem)),
        );
    }

    fn write_default_instance_static(&self, w: &mut CodeWriter) {
        w.stmt_block(
            &format!(
                "static instance: {} = {}",
                self.rust_name(),
                self.rust_name()
            ),
            |w| {
                for f in &self.fields_except_oneof_and_group() {
                    w.field_entry(
                        &f.rust_name.to_string(),
                        &f.kind
                            .default(&self.customize.for_elem, &self.file_and_mod(), true),
                    );
                }
                for o in &self.oneofs() {
                    w.field_entry(&o.oneof.field_name().to_string(), EXPR_NONE);
                }
                w.field_entry(
                    "special_fields",
                    &format!(
                        "{}::SpecialFields::new()",
                        protobuf_crate_path(&self.customize.for_elem)
                    ),
                );
            },
        );
        w.write_line("&instance");
    }

    fn write_default_instance(&self, w: &mut CodeWriter) {
        w.def_fn(
            &format!("default_instance() -> &'static {}", self.rust_name()),
            |w| {
                let has_map_field = self.fields.iter().any(|f| match f.kind {
                    FieldKind::Map(..) => true,
                    _ => false,
                });
                if has_map_field {
                    self.write_default_instance_lazy(w)
                } else {
                    self.write_default_instance_static(w)
                }
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
        w.def_fn("compute_size(&self) -> u64", |w| {
            // To have access to its methods but not polute the name space.
            w.write_line("let mut my_size = 0;");
            for field in self.fields_except_oneof_and_group() {
                field.write_message_compute_field_size("my_size", w);
            }
            self.write_match_each_oneof_variant(w, |w, variant, v| {
                variant
                    .field
                    .write_element_size(variant.elem(), w, v, "my_size");
            });
            w.write_line(&format!(
                "my_size += {}::rt::unknown_fields_size(self.special_fields.unknown_fields());",
                protobuf_crate_path(&self.customize.for_elem)
            ));
            w.write_line("self.special_fields.cached_size().set(my_size as u32);");
            w.write_line("my_size");
        });
    }

    fn write_field_accessors(&self, w: &mut CodeWriter) {
        for f in self.fields_except_group() {
            f.write_message_single_field_accessors(w);
        }
    }

    fn write_impl_self(&self, w: &mut CodeWriter) {
        w.impl_self_block(&format!("{}", self.rust_name()), |w| {
            w.pub_fn(&format!("new() -> {}", self.rust_name()), |w| {
                w.write_line("::std::default::Default::default()");
            });

            self.write_field_accessors(w);

            if !self.lite_runtime {
                w.write_line("");
                self.write_generated_message_descriptor_data(w);
            }
        });
    }

    fn write_unknown_fields(&self, w: &mut CodeWriter) {
        let sig = format!(
            "special_fields(&self) -> &{}::SpecialFields",
            protobuf_crate_path(&self.customize.for_elem)
        );
        w.def_fn(&sig, |w| {
            w.write_line("&self.special_fields");
        });
        w.write_line("");
        let sig = format!(
            "mut_special_fields(&mut self) -> &mut {}::SpecialFields",
            protobuf_crate_path(&self.customize.for_elem)
        );
        w.def_fn(&sig, |w| {
            w.write_line("&mut self.special_fields");
        });
    }

    fn write_merge_from(&self, w: &mut CodeWriter) {
        let sig = format!(
            "merge_from(&mut self, is: &mut {}::CodedInputStream<'_>) -> {}::Result<()>",
            protobuf_crate_path(&self.customize.for_elem),
            protobuf_crate_path(&self.customize.for_elem),
        );
        w.def_fn(&sig, |w| {
            w.while_block("let Some(tag) = is.read_raw_tag_or_eof()?", |w| {
                w.match_block("tag", |w| {
                    for f in &self.fields_except_group() {
                        f.write_merge_from_field_case_block(w);
                    }
                    w.case_block("tag", |w| {
                        w.write_line(&format!("{}::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;", protobuf_crate_path(&self.customize.for_elem)));
                    });
                });
            });
            w.write_line("::std::result::Result::Ok(())");
        });
    }

    fn write_impl_message_full_fn_descriptor(&self, w: &mut CodeWriter) {
        write_fn_descriptor(
            &self.message.message,
            self.message.scope(),
            &self.customize.for_elem,
            w,
        );
    }

    fn write_generated_message_descriptor_data(&self, w: &mut CodeWriter) {
        let sig = format!(
            "generated_message_descriptor_data() -> {}::reflect::GeneratedMessageDescriptorData",
            protobuf_crate_path(&self.customize.for_elem)
        );
        w.fn_block(
            Visibility::Path(self.message.scope().rust_path_to_file().to_reverse()),
            &sig,
            |w| {
                let fields = self.fields_except_group();
                let oneofs = self.oneofs();
                w.write_line(&format!(
                    "let mut fields = {};",
                    expr_vec_with_capacity_const(fields.len())
                ));
                w.write_line(&format!(
                    "let mut oneofs = {};",
                    expr_vec_with_capacity_const(oneofs.len())
                ));
                for field in fields {
                    field.write_push_accessor("fields", w);
                }
                for oneof in oneofs {
                    w.write_line(&format!(
                        "oneofs.push({}::generated_oneof_descriptor_data());",
                        oneof.type_name_relative(&self.mod_name())
                    ));
                }
                w.write_line(&format!(
                    "{}::reflect::GeneratedMessageDescriptorData::new_2::<{}>(",
                    protobuf_crate_path(&self.customize.for_elem),
                    self.rust_name(),
                ));
                w.indented(|w| {
                    w.write_line(&format!("\"{}\",", self.message.name_to_package()));
                    w.write_line("fields,");
                    w.write_line("oneofs,");
                });
                w.write_line(")");
            },
        );
    }

    fn write_is_initialized(&self, w: &mut CodeWriter) {
        w.def_fn(&format!("is_initialized(&self) -> bool"), |w| {
            if !self.message.message.is_initialized_is_always_true() {
                // TODO: use single loop

                for f in self.required_fields() {
                    f.write_if_self_field_is_none(w, |w| {
                        w.write_line("return false;");
                    });
                }

                for f in self.message_fields() {
                    if let FieldKind::Map(..) = f.kind {
                        // TODO
                        w.comment("TODO: check map values are initialized");
                        continue;
                    }

                    f.write_for_self_field(w, "v", |w, _t| {
                        w.if_stmt("!v.is_initialized()", |w| {
                            w.write_line("return false;");
                        });
                    });
                }
            }
            w.write_line("true");
        });
    }

    fn write_impl_message(&self, w: &mut CodeWriter) {
        w.impl_for_block(
            &format!("{}::Message", protobuf_crate_path(&self.customize.for_elem),),
            &format!("{}", self.rust_name()),
            |w| {
                w.write_line(&format!(
                    "const NAME: &'static str = \"{}\";",
                    self.message.message.name()
                ));
                w.write_line("");
                self.write_is_initialized(w);
                w.write_line("");
                self.write_merge_from(w);
                w.write_line("");
                self.write_compute_size(w);
                w.write_line("");
                self.write_write_to_with_cached_sizes(w);
                w.write_line("");
                self.write_unknown_fields(w);
                w.write_line("");
                w.def_fn(&format!("new() -> {}", self.rust_name()), |w| {
                    w.write_line(&format!("{}::new()", self.rust_name()));
                });
                w.write_line("");
                w.def_fn("clear(&mut self)", |w| {
                    for f in self.fields_except_group() {
                        f.write_clear(w);
                    }
                    w.write_line("self.special_fields.clear();");
                });
                w.write_line("");
                self.write_default_instance(w);
            },
        );
    }

    fn write_impl_message_full(&self, w: &mut CodeWriter) {
        w.impl_for_block(
            &format!(
                "{}::MessageFull",
                protobuf_crate_path(&self.customize.for_elem),
            ),
            &format!("{}", self.rust_name()),
            |w| {
                self.write_impl_message_full_fn_descriptor(w);
            },
        );
    }

    fn write_impl_value(&self, w: &mut CodeWriter) {
        w.impl_for_block(
            &format!(
                "{}::reflect::ProtobufValue",
                protobuf_crate_path(&self.customize.for_elem)
            ),
            &format!("{}", self.rust_name()),
            |w| {
                w.write_line(&format!(
                    "type RuntimeType = {}::reflect::rt::RuntimeTypeMessage<Self>;",
                    protobuf_crate_path(&self.customize.for_elem)
                ));
            },
        )
    }

    fn write_impl_display(&self, w: &mut CodeWriter) {
        w.impl_for_block(
            "::std::fmt::Display",
            &format!("{}", self.rust_name()),
            |w| {
                w.def_fn(
                    "fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result",
                    |w| {
                        w.write_line(&format!(
                            "{}::text_format::fmt(self, f)",
                            protobuf_crate_path(&self.customize.for_elem)
                        ));
                    },
                );
            },
        );
    }

    fn supports_derive_partial_eq(&self) -> bool {
        // There's stack overflow in the compiler when struct has too many fields
        // https://github.com/rust-lang/rust/issues/40119
        self.fields.len() <= 500
    }

    fn write_struct(&self, w: &mut CodeWriter) {
        write_protoc_insertion_point_for_message(
            w,
            &self.customize.for_elem,
            &self.message_descriptor,
        );
        let mut derive = Vec::new();
        if self.supports_derive_partial_eq() {
            derive.push("PartialEq");
        }
        derive.extend(&["Clone", "Default", "Debug"]);
        w.derive(&derive);

        w.pub_struct(&format!("{}", self.rust_name()), |w| {
            if !self.fields_except_oneof().is_empty() {
                w.comment("message fields");
                for field in self.fields_except_oneof() {
                    field.write_struct_field(w);
                }
            }
            if !self.oneofs().is_empty() {
                w.comment("message oneof groups");
                for oneof in self.oneofs() {
                    w.field_decl_vis(
                        Visibility::Public,
                        &oneof.oneof.field_name().to_string(),
                        &oneof.full_storage_type().to_code(&self.customize.for_elem),
                    );
                }
            }
            w.comment("special fields");

            let customize_special_fields = self
                .customize
                .child(
                    &Customize::default(),
                    &SpecialFieldPseudoDescriptor {
                        message: &self.message.message,
                        field: "special_fields",
                    },
                )
                .for_elem;

            write_protoc_insertion_point_for_special_field(
                w,
                &customize_special_fields,
                &self.message_descriptor,
                "special_fields",
            );
            w.pub_field_decl(
                "special_fields",
                &format!(
                    "{}::SpecialFields",
                    protobuf_crate_path(&self.customize.for_elem)
                ),
            );
        });
    }

    fn write_impl_default_for_amp(&self, w: &mut CodeWriter) {
        w.impl_args_for_block(
            &["'a"],
            "::std::default::Default",
            &format!("&'a {}", self.rust_name()),
            |w| {
                w.def_fn(&format!("default() -> &'a {}", self.rust_name()), |w| {
                    w.write_line(&format!(
                        "<{} as {}::Message>::default_instance()",
                        self.rust_name(),
                        protobuf_crate_path(&self.customize.for_elem),
                    ));
                });
            },
        );
    }

    fn write_dummy_impl_partial_eq(&self, w: &mut CodeWriter) {
        w.impl_for_block(
            "::std::cmp::PartialEq",
            &format!("{}", self.rust_name()),
            |w| {
                w.def_fn("eq(&self, _: &Self) -> bool", |w| {
                    w.comment("https://github.com/rust-lang/rust/issues/40119");
                    w.unimplemented();
                });
            },
        );
    }

    pub fn write(&self, w: &mut CodeWriter) -> anyhow::Result<()> {
        w.all_documentation(self.info, self.path);
        self.write_struct(w);

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
        if !self.lite_runtime {
            w.write_line("");
            self.write_impl_message_full(w);
        }
        if !self.lite_runtime {
            w.write_line("");
            self.write_impl_display(w);

            w.write_line("");
            self.write_impl_value(w);
        }

        let mod_name = message_name_to_nested_mod_name(&self.message.message.name());

        let oneofs = self.oneofs();
        let nested_messages: Vec<_> = self
            .message
            .to_scope()
            .messages()
            .into_iter()
            .filter(|nested| {
                // ignore map entries, because they are not used in map fields
                !nested.is_map()
            })
            .collect();
        let nested_enums = self.message.to_scope().enums();

        if !oneofs.is_empty() || !nested_messages.is_empty() || !nested_enums.is_empty() {
            w.write_line("");
            w.write_line(&format!(
                "/// Nested message and enums of message `{}`",
                self.message.message.name()
            ));
            w.pub_mod(&mod_name.to_string(), |w| {
                let mut first = true;

                for oneof in &oneofs {
                    w.write_line("");
                    oneof.write(w);
                }

                static NESTED_TYPE_NUMBER: protobuf::rt::Lazy<i32> = protobuf::rt::Lazy::new();
                let nested_type_number = *NESTED_TYPE_NUMBER.get(|| {
                    MessageDescriptor::for_type::<DescriptorProto>()
                        .field_by_name("nested_type")
                        .expect("`nested_type` must exist")
                        .proto()
                        .number()
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
                    MessageGen::new(
                        &self.file_descriptor,
                        nested,
                        self.root_scope,
                        &self.customize,
                        &path,
                        self.info,
                    )
                    // TODO: do not unwrap.
                    .unwrap()
                    .write(w)
                    // TODO: do not unwrap.
                    .unwrap();
                }

                static ENUM_TYPE_NUMBER: protobuf::rt::Lazy<i32> = protobuf::rt::Lazy::new();
                let enum_type_number = *ENUM_TYPE_NUMBER.get(|| {
                    MessageDescriptor::for_type::<DescriptorProto>()
                        .field_by_name("enum_type")
                        .expect("`enum_type` must exist")
                        .proto()
                        .number()
                });

                let len = path.len() - 2;
                path[len] = enum_type_number;
                for (id, enum_type) in self.message.to_scope().enums().iter().enumerate() {
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
                    .write(w);
                }
            });
        }
        Ok(())
    }
}

pub(crate) fn message_name_to_nested_mod_name(message_name: &str) -> RustIdent {
    let mod_name = snake_case(message_name);
    RustIdent::new(&mod_name)
}

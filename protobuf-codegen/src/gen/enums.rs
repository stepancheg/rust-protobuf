use std::collections::HashSet;

use protobuf::descriptor::*;

use crate::customize::ctx::CustomizeElemCtx;
use crate::customize::rustproto_proto::customize_from_rustproto_for_enum;
use crate::gen::code_writer::CodeWriter;
use crate::gen::code_writer::Visibility;
use crate::gen::descriptor::write_fn_descriptor;
use crate::gen::inside::protobuf_crate_path;
use crate::gen::protoc_insertion_point::write_protoc_insertion_point_for_enum;
use crate::gen::protoc_insertion_point::write_protoc_insertion_point_for_enum_value;
use crate::gen::rust::ident::RustIdent;
use crate::gen::rust::ident_with_path::RustIdentWithPath;
use crate::gen::rust::snippets::EXPR_NONE;
use crate::gen::scope::EnumValueWithContext;
use crate::gen::scope::EnumWithScope;
use crate::gen::scope::RootScope;
use crate::gen::scope::WithScope;

#[derive(Clone)]
pub(crate) struct EnumValueGen<'a> {
    value: EnumValueWithContext<'a>,
    enum_rust_name: RustIdentWithPath,
}

impl<'a> EnumValueGen<'a> {
    fn parse(
        value: EnumValueWithContext<'a>,
        enum_rust_name: &RustIdentWithPath,
    ) -> EnumValueGen<'a> {
        EnumValueGen {
            value: value.clone(),
            enum_rust_name: enum_rust_name.clone(),
        }
    }

    // enum value
    fn number(&self) -> i32 {
        self.value.proto.proto().number()
    }

    // name of enum variant in generated rust code
    pub fn rust_name_inner(&self) -> RustIdent {
        self.value.rust_name()
    }

    pub fn rust_name_outer(&self) -> RustIdentWithPath {
        self.enum_rust_name
            .to_path()
            .with_ident(self.rust_name_inner())
    }
}

// Codegen for enum definition
pub(crate) struct EnumGen<'a> {
    enum_with_scope: &'a EnumWithScope<'a>,
    type_name: RustIdentWithPath,
    lite_runtime: bool,
    customize: CustomizeElemCtx<'a>,
    path: &'a [i32],
    info: Option<&'a SourceCodeInfo>,
}

impl<'a> EnumGen<'a> {
    pub fn new(
        enum_with_scope: &'a EnumWithScope<'a>,
        customize: &CustomizeElemCtx<'a>,
        _root_scope: &RootScope,
        path: &'a [i32],
        info: Option<&'a SourceCodeInfo>,
    ) -> EnumGen<'a> {
        let customize = customize.child(
            &customize_from_rustproto_for_enum(enum_with_scope.en.proto().options.get_or_default()),
            &enum_with_scope.en,
        );
        let lite_runtime = customize.for_elem.lite_runtime.unwrap_or_else(|| {
            enum_with_scope
                .file_descriptor()
                .proto()
                .options
                .optimize_for()
                == file_options::OptimizeMode::LITE_RUNTIME
        });

        EnumGen {
            enum_with_scope,
            type_name: enum_with_scope.rust_name().to_path(),
            lite_runtime,
            customize,
            path,
            info,
        }
    }

    fn allow_alias(&self) -> bool {
        self.enum_with_scope
            .en
            .proto()
            .options
            .get_or_default()
            .allow_alias()
    }

    fn values_all(&self) -> Vec<EnumValueGen> {
        let mut r = Vec::new();
        for p in self.enum_with_scope.values() {
            r.push(EnumValueGen::parse(p, &self.type_name));
        }
        r
    }

    fn values_unique(&self) -> Vec<EnumValueGen> {
        let mut used = HashSet::new();
        let mut r = Vec::new();
        for p in self.enum_with_scope.values() {
            if !used.insert(p.proto.proto().number()) {
                continue;
            }
            r.push(EnumValueGen::parse(p, &self.type_name));
        }
        r
    }

    pub fn write(&self, w: &mut CodeWriter) {
        self.write_enum(w);
        if self.allow_alias() {
            w.write_line("");
            self.write_impl_eq(w);
            w.write_line("");
            self.write_impl_hash(w);
        }
        w.write_line("");
        self.write_impl_enum(w);
        if !self.lite_runtime {
            w.write_line("");
            self.write_impl_enum_full(w);
        }
        w.write_line("");
        self.write_impl_default(w);
        w.write_line("");
        self.write_impl_self(w);
    }

    fn write_impl_self(&self, w: &mut CodeWriter) {
        if !self.lite_runtime {
            w.impl_self_block(&format!("{}", self.type_name), |w| {
                self.write_generated_enum_descriptor_data(w);
            });
        }
    }

    fn write_enum(&self, w: &mut CodeWriter) {
        w.all_documentation(self.info, self.path);

        let mut derive = Vec::new();
        derive.push("Clone");
        derive.push("Copy");
        if !self.allow_alias() {
            derive.push("PartialEq");
        }
        derive.push("Eq");
        derive.push("Debug");
        if !self.allow_alias() {
            derive.push("Hash");
        } else {
            w.comment("Note: you cannot use pattern matching for enums with allow_alias option");
        }
        w.derive(&derive);
        let ref type_name = self.type_name;
        write_protoc_insertion_point_for_enum(
            w,
            &self.customize.for_elem,
            &self.enum_with_scope.en,
        );
        w.expr_block(&format!("pub enum {}", type_name), |w| {
            for value in self.values_all() {
                write_protoc_insertion_point_for_enum_value(
                    w,
                    &self.customize.for_children,
                    &value.value.proto,
                );
                if self.allow_alias() {
                    w.write_line(&format!(
                        "{}, // {}",
                        value.rust_name_inner(),
                        value.number()
                    ));
                } else {
                    w.write_line(&format!(
                        "{} = {},",
                        value.rust_name_inner(),
                        value.number()
                    ));
                }
            }
        });
    }

    fn write_impl_enum_fn_value(&self, w: &mut CodeWriter) {
        w.def_fn("value(&self) -> i32", |w| {
            if self.allow_alias() {
                w.match_expr("*self", |w| {
                    for value in self.values_all() {
                        w.case_expr(
                            &format!("{}", value.rust_name_outer()),
                            &format!("{}", value.number()),
                        );
                    }
                });
            } else {
                w.write_line("*self as i32")
            }
        });
    }

    fn write_impl_enum_const_name(&self, w: &mut CodeWriter) {
        w.write_line(&format!(
            "const NAME: &'static str = \"{}\";",
            self.enum_with_scope.en.name()
        ));
    }

    fn write_impl_enum_fn_from_i32(&self, w: &mut CodeWriter) {
        w.def_fn(
            &format!(
                "from_i32(value: i32) -> ::std::option::Option<{}>",
                self.type_name
            ),
            |w| {
                w.match_expr("value", |w| {
                    let values = self.values_unique();
                    for value in values {
                        w.write_line(&format!(
                            "{} => ::std::option::Option::Some({}),",
                            value.number(),
                            value.rust_name_outer()
                        ));
                    }
                    w.write_line(&format!("_ => {}", EXPR_NONE));
                });
            },
        );
    }

    fn write_impl_enum_fn_from_str(&self, w: &mut CodeWriter) {
        w.def_fn(
            &format!(
                "from_str(str: &str) -> ::std::option::Option<{}>",
                self.type_name
            ),
            |w| {
                w.match_expr("str", |w| {
                    let values = self.values_unique();
                    for value in values {
                        w.write_line(&format!(
                            "\"{}\" => ::std::option::Option::Some({}),",
                            value.value.proto.name(),
                            value.rust_name_outer()
                        ));
                    }
                    w.write_line(&format!("_ => {}", EXPR_NONE));
                });
            },
        );
    }

    fn write_impl_enum_const_values(&self, w: &mut CodeWriter) {
        w.write_line(&format!("const VALUES: &'static [{}] = &[", self.type_name));
        w.indented(|w| {
            for value in self.values_all() {
                w.write_line(&format!("{},", value.rust_name_outer()));
            }
        });
        w.write_line("];");
    }

    fn write_impl_enum(&self, w: &mut CodeWriter) {
        w.impl_for_block(
            &format!("{}::Enum", protobuf_crate_path(&self.customize.for_elem)),
            &format!("{}", self.type_name),
            |w| {
                self.write_impl_enum_const_name(w);
                w.write_line("");
                self.write_impl_enum_fn_value(w);
                w.write_line("");
                self.write_impl_enum_fn_from_i32(w);
                w.write_line("");
                self.write_impl_enum_fn_from_str(w);
                w.write_line("");
                self.write_impl_enum_const_values(w);
            },
        );
    }

    fn write_impl_enum_full(&self, w: &mut CodeWriter) {
        let ref type_name = self.type_name;
        w.impl_for_block(
            &format!(
                "{}::EnumFull",
                protobuf_crate_path(&self.customize.for_elem)
            ),
            &format!("{}", type_name),
            |w| {
                self.write_impl_enum_full_fn_enum_descriptor(w);
                w.write_line("");
                self.write_impl_enum_full_fn_descriptor(w);
            },
        );
    }

    fn write_impl_enum_full_fn_enum_descriptor(&self, w: &mut CodeWriter) {
        write_fn_descriptor(
            &self.enum_with_scope.en,
            self.enum_with_scope.scope(),
            &self.customize.for_elem,
            w,
        );
    }

    fn rust_enum_descriptor_is_enum_index(&self) -> bool {
        if self.allow_alias() {
            false
        } else {
            self.values_all()
                .into_iter()
                .enumerate()
                .all(|(i, value)| (i as i32) == value.number())
        }
    }

    fn write_impl_enum_full_fn_descriptor(&self, w: &mut CodeWriter) {
        let sig = format!(
            "descriptor(&self) -> {}::reflect::EnumValueDescriptor",
            protobuf_crate_path(&self.customize.for_elem)
        );
        w.def_fn(&sig, |w| {
            if self.rust_enum_descriptor_is_enum_index() {
                w.write_line("let index = *self as usize;");
            } else {
                w.write_line("let index = match self {");
                w.indented(|w| {
                    for (i, value) in self.values_all().into_iter().enumerate() {
                        w.write_line(&format!(
                            "{}::{} => {},",
                            self.type_name,
                            value.rust_name_inner(),
                            i
                        ));
                    }
                });
                w.write_line("};");
            }
            w.write_line(&format!("Self::enum_descriptor().value_by_index(index)"));
        });
    }

    fn write_generated_enum_descriptor_data(&self, w: &mut CodeWriter) {
        let sig = format!(
            "generated_enum_descriptor_data() -> {}::reflect::GeneratedEnumDescriptorData",
            protobuf_crate_path(&self.customize.for_elem)
        );
        w.fn_block(
            Visibility::Path(
                self.enum_with_scope
                    .scope()
                    .rust_path_to_file()
                    .to_reverse(),
            ),
            &sig,
            |w| {
                w.write_line(&format!(
                    "{}::reflect::GeneratedEnumDescriptorData::new::<{}>(\"{}\")",
                    protobuf_crate_path(&self.customize.for_elem),
                    self.type_name,
                    self.enum_with_scope.name_to_package(),
                ));
            },
        );
    }

    fn write_impl_eq(&self, w: &mut CodeWriter) {
        assert!(self.allow_alias());
        w.impl_for_block(
            "::std::cmp::PartialEq",
            &format!("{}", self.type_name),
            |w| {
                w.def_fn("eq(&self, other: &Self) -> bool", |w| {
                    w.write_line(&format!(
                        "{}::Enum::value(self) == {}::Enum::value(other)",
                        protobuf_crate_path(&self.customize.for_elem),
                        protobuf_crate_path(&self.customize.for_elem)
                    ));
                });
            },
        );
    }

    fn write_impl_hash(&self, w: &mut CodeWriter) {
        assert!(self.allow_alias());
        w.impl_for_block("::std::hash::Hash", &format!("{}", self.type_name), |w| {
            w.def_fn("hash<H : ::std::hash::Hasher>(&self, state: &mut H)", |w| {
                w.write_line(&format!(
                    "state.write_i32({}::Enum::value(self))",
                    protobuf_crate_path(&self.customize.for_elem)
                ));
            });
        });
    }

    fn write_impl_default(&self, w: &mut CodeWriter) {
        let first_value = &self.enum_with_scope.values()[0];
        if first_value.proto.proto().number() != 0 {
            // This warning is emitted only for proto2
            // (because in proto3 first enum variant number is always 0).
            // `Default` implemented unconditionally to simplify certain
            // generic operations, e. g. reading a map.
            // Also, note that even in proto2 some operations fallback to
            // first enum value, e. g. `get_xxx` for unset field,
            // so this implementation is not completely unreasonable.
            w.comment("Note, `Default` is implemented although default value is not 0");
        }
        w.impl_for_block(
            "::std::default::Default",
            &format!("{}", self.type_name),
            |w| {
                w.def_fn("default() -> Self", |w| {
                    w.write_line(&format!(
                        "{}::{}",
                        &self.type_name,
                        &first_value.rust_name()
                    ))
                });
            },
        );
    }
}

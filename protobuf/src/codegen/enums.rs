use std::collections::HashSet;

use descriptor::*;
use descriptorx::*;
use code_writer::*;


#[derive(Clone)]
pub struct EnumValueGen {
    proto: EnumValueDescriptorProto,
    enum_rust_name: String,
}

impl EnumValueGen {
    fn parse(proto: &EnumValueDescriptorProto, enum_rust_name: &str) -> EnumValueGen {
        EnumValueGen {
            proto: proto.clone(),
            enum_rust_name: enum_rust_name.to_string(),
        }
    }

    // enum value
    fn number(&self) -> i32 {
        self.proto.get_number()
    }

    // name of enum variant in generated rust code
    fn rust_name_inner(&self) -> String {
        self.proto.rust_name()
    }

    pub fn rust_name_outer(&self) -> String {
        let mut r = String::new();
        r.push_str(&self.enum_rust_name);
        r.push_str("::");
        r.push_str(&self.rust_name_inner());
        r
    }
}


pub struct EnumGen<'a> {
    enum_with_scope: &'a EnumWithScope<'a>,
    type_name: String,
    lite_runtime: bool,
}

impl<'a> EnumGen<'a> {
    pub fn new(
        enum_with_scope: &'a EnumWithScope<'a>,
        current_file: &FileDescriptorProto,
    ) -> EnumGen<'a> {
        let rust_name = if enum_with_scope.get_scope().get_file_descriptor().get_name() ==
            current_file.get_name()
        {
            // field type is a message or enum declared in the same file
            enum_with_scope.rust_name()
        } else {
            format!(
                "super::{}::{}",
                proto_path_to_rust_mod(
                    enum_with_scope.get_scope().get_file_descriptor().get_name(),
                ),
                enum_with_scope.rust_name()
            )
        };
        EnumGen {
            enum_with_scope: enum_with_scope,
            type_name: rust_name,
            lite_runtime: enum_with_scope
                .get_scope()
                .get_file_descriptor()
                .get_options()
                .get_optimize_for() ==
                FileOptions_OptimizeMode::LITE_RUNTIME,
        }
    }

    fn allow_alias(&self) -> bool {
        self.enum_with_scope.en.get_options().get_allow_alias()
    }

    fn values_all(&self) -> Vec<EnumValueGen> {
        let mut r = Vec::new();
        for p in self.enum_with_scope.values() {
            r.push(EnumValueGen::parse(p, &self.type_name));
        }
        r
    }

    pub fn values_unique(&self) -> Vec<EnumValueGen> {
        let mut used = HashSet::new();
        let mut r = Vec::new();
        for p in self.enum_with_scope.values() {
            // skipping non-unique enums
            // TODO: should support it
            if !used.insert(p.get_number()) {
                continue;
            }
            r.push(EnumValueGen::parse(p, &self.type_name));
        }
        r
    }

    // find enum value by name
    pub fn value_by_name(&'a self, name: &str) -> EnumValueGen {
        EnumValueGen::parse(self.enum_with_scope.value_by_name(name), &self.type_name)
    }

    pub fn write(&self, w: &mut CodeWriter) {
        self.write_struct(w);
        if self.allow_alias() {
            w.write_line("");
            self.write_impl_eq(w);
            w.write_line("");
            self.write_impl_hash(w);
        }
        w.write_line("");
        self.write_impl_enum(w);
        w.write_line("");
        self.write_impl_copy(w);
        if self.enum_with_scope.scope.file_scope.syntax() == Syntax::PROTO3 {
            w.write_line("");
            self.write_impl_default(w);
        }
        w.write_line("");
        self.write_impl_value(w);
    }

    fn write_struct(&self, w: &mut CodeWriter) {
        let mut derive = Vec::new();
        derive.push("Clone");
        if !self.allow_alias() {
            derive.push("PartialEq");
        }
        derive.push("Eq");
        derive.push("Debug");
        if !self.allow_alias() {
            derive.push("Hash");
        } else {
            w.comment(
                "Note: you cannot use pattern matching for enums with allow_alias option",
            );
        }
        w.derive(&derive);
        let ref type_name = self.type_name;
        w.expr_block(&format!("pub enum {}", type_name), |w| {
            for value in self.values_all() {
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

    fn write_fn_value(&self, w: &mut CodeWriter) {
        w.def_fn("value(&self) -> i32", |w| if self.allow_alias() {
            w.match_expr("*self", |w| for value in self.values_all() {
                w.case_expr(value.rust_name_outer(), format!("{}", value.number()));
            });
        } else {
            w.write_line("*self as i32")
        });
    }

    fn write_impl_enum(&self, w: &mut CodeWriter) {
        let ref type_name = self.type_name;
        w.impl_for_block("::protobuf::ProtobufEnum", &type_name, |w| {
            self.write_fn_value(w);

            w.write_line("");
            let ref type_name = self.type_name;
            w.def_fn(&format!("from_i32(value: i32) -> ::std::option::Option<{}>", type_name), |w| {
                w.match_expr("value", |w| {
                    let values = self.values_unique();
                    for value in values {
                        w.write_line(&format!("{} => ::std::option::Option::Some({}),",
                            value.number(), value.rust_name_outer()));
                    }
                    w.write_line(&format!("_ => ::std::option::Option::None"));
                });
            });

            w.write_line("");
            w.def_fn(&format!("values() -> &'static [Self]"), |w| {
                w.write_line(&format!("static values: &'static [{}] = &[", type_name));
                w.indented(|w| {
                    for value in self.values_all() {
                        w.write_line(&format!("{},", value.rust_name_outer()));
                    }
                });
                w.write_line("];");
                w.write_line("values");
            });

            if !self.lite_runtime {
                w.write_line("");
                let ref type_name = self.type_name;
                w.def_fn(&format!("enum_descriptor_static(_: ::std::option::Option<{}>) -> &'static ::protobuf::reflect::EnumDescriptor", type_name), |w| {
                    w.lazy_static_decl_get("descriptor", "::protobuf::reflect::EnumDescriptor", |w| {
                        let ref type_name = self.type_name;
                        w.write_line(&format!("::protobuf::reflect::EnumDescriptor::new(\"{}\", file_descriptor_proto())", type_name));
                    });
                });
            }
        });
    }

    fn write_impl_value(&self, w: &mut CodeWriter) {
        w.impl_for_block("::protobuf::reflect::ProtobufValue", &self.type_name, |w| {
            w.def_fn(
                "as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef",
                |w| {
                    w.write_line(
                        "::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())",
                    )
                },
            )
        })
    }

    fn write_impl_copy(&self, w: &mut CodeWriter) {
        w.impl_for_block("::std::marker::Copy", &self.type_name, |_w| {});
    }

    fn write_impl_eq(&self, w: &mut CodeWriter) {
        assert!(self.allow_alias());
        w.impl_for_block("::std::cmp::PartialEq", &self.type_name, |w| {
            w.def_fn("eq(&self, other: &Self) -> bool", |w| {
                w.write_line("self.value() == other.value()");
            });
        });
    }

    fn write_impl_hash(&self, w: &mut CodeWriter) {
        assert!(self.allow_alias());
        w.impl_for_block("::std::hash::Hash", &self.type_name, |w| {
            w.def_fn("hash<H : ::std::hash::Hasher>(&self, state: &mut H)", |w| {
                w.write_line("state.write_i32(self.value())");
            });
        });
    }

    fn write_impl_default(&self, w: &mut CodeWriter) {
        assert!(self.enum_with_scope.scope.file_scope.syntax() == Syntax::PROTO3);
        w.impl_for_block("::std::default::Default", &self.type_name, |w| {
            w.def_fn("default() -> Self", |w| {
                w.write_line(&format!(
                    "{}::{}",
                    &self.type_name,
                    &self.enum_with_scope.values()[0].rust_name()
                ))
            });
        });
    }
}

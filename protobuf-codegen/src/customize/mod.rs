pub(crate) mod by_path;
pub(crate) mod custom_attr;

use protobuf::descriptor::EnumOptions;
use protobuf::descriptor::FieldOptions;
use protobuf::descriptor::FileOptions;
use protobuf::descriptor::MessageOptions;
use protobuf::rustproto;

use crate::customize::custom_attr::CustomAttr;

/// Specifies style of generated code.
/// Generated files can be customized using this proto
/// or using `rustproto.proto` options.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Customize {
    /// Make oneof enum public.
    pub(crate) expose_oneof: Option<bool>,
    /// When true all fields are public, and accessors are not generated
    pub(crate) expose_fields: Option<bool>,
    /// When false, `get_`, `set_`, `mut_` etc. accessors are not generated
    pub(crate) generate_accessors: Option<bool>,
    /// When false, `get_` is not generated even if `syntax = "proto2"`
    pub(crate) generate_getter: Option<bool>,
    /// Use `bytes::Bytes` for `bytes` fields
    pub(crate) carllerche_bytes_for_bytes: Option<bool>,
    /// Use `bytes::Bytes` for `string` fields
    pub(crate) carllerche_bytes_for_string: Option<bool>,
    /// Custom attributes for types.
    pub(crate) type_attrs: Option<Vec<CustomAttr>>,
    /// Custom attributes for fields.
    pub(crate) field_attrs: Option<Vec<CustomAttr>>,
    /// Custom attributes for `unknown_fields` and `computed_size`.
    pub(crate) special_field_attrs: Option<Vec<CustomAttr>>,
    /// Implement serde_derive for messages
    pub(crate) serde_derive: Option<bool>,
    /// When `serde_derive` is set, serde annotations will be guarded with `#[cfg(cfg, ...)]`.
    pub(crate) serde_derive_cfg: Option<String>,
    /// When `serde_derive` is set, use attribute rename_all
    pub(crate) serde_rename_all: Option<String>,
    /// Enable lite runtime
    pub(crate) lite_runtime: Option<bool>,
    /// Generate `mod.rs` in the output directory.
    ///
    /// This option allows inclusion of generated files from cargo output directory.
    ///
    /// This option will likely be on by default in rust-protobuf version 3.
    pub(crate) gen_mod_rs: Option<bool>,
    /// Used internally to generate protos bundled in protobuf crate
    /// like `descriptor.proto`
    pub(crate) inside_protobuf: Option<bool>,
}

#[derive(Debug)]
pub enum CustomizeParseParameterError {
    EqNotFound,
    CannotParseBool,
    UnknownOptionName(String),
}

pub type CustomizeParseParameterResult<T> = Result<T, CustomizeParseParameterError>;

impl Customize {
    pub fn expose_oneof(mut self, expose_oneof: bool) -> Self {
        self.expose_oneof = Some(expose_oneof);
        self
    }

    pub fn expose_fields(mut self, expose_fields: bool) -> Self {
        self.expose_fields = Some(expose_fields);
        self
    }

    pub fn generate_accessors(mut self, generate_accessors: bool) -> Self {
        self.generate_accessors = Some(generate_accessors);
        self
    }

    pub fn generate_getter(mut self, generate_getter: bool) -> Self {
        self.generate_getter = Some(generate_getter);
        self
    }

    pub fn carllerche_bytes_for_bytes(mut self, carllerche_bytes_for_bytes: bool) -> Self {
        self.carllerche_bytes_for_bytes = Some(carllerche_bytes_for_bytes);
        self
    }

    pub fn carllerche_bytes_for_string(mut self, carllerche_bytes_for_string: bool) -> Self {
        self.carllerche_bytes_for_string = Some(carllerche_bytes_for_string);
        self
    }

    fn add_type_attr(&mut self, type_attr: &str) {
        if let None = self.type_attrs {
            self.type_attrs = Some(Vec::with_capacity(1));
        };
        self.type_attrs
            .as_mut()
            .unwrap()
            .push(CustomAttr::from_str(type_attr));
    }

    pub fn type_attr(mut self, type_attr: &str) -> Self {
        self.add_type_attr(type_attr);
        self
    }

    fn add_field_attr(&mut self, field_attr: &str) {
        if let None = self.field_attrs {
            self.field_attrs = Some(Vec::with_capacity(1));
        };
        self.field_attrs
            .as_mut()
            .unwrap()
            .push(CustomAttr::from_str(field_attr));
    }

    pub fn field_attr(mut self, field_attr: &str) -> Self {
        self.add_field_attr(field_attr);
        self
    }

    fn add_special_field_attr(&mut self, special_field_attr: &str) {
        if let None = self.special_field_attrs {
            self.special_field_attrs = Some(Vec::with_capacity(1));
        };
        self.special_field_attrs
            .as_mut()
            .unwrap()
            .push(CustomAttr::from_str(special_field_attr));
    }

    pub fn special_field_attr(mut self, special_field_attr: &str) -> Self {
        self.add_special_field_attr(special_field_attr);
        self
    }

    pub fn serde_derive(mut self, serde_derive: bool) -> Self {
        self.serde_derive = Some(serde_derive);
        self
    }

    pub fn serde_derive_cfg(mut self, serde_derive_cfg: &str) -> Self {
        self.serde_derive_cfg = Some(serde_derive_cfg.to_owned());
        self
    }

    pub fn serde_rename_all(mut self, serde_rename_all: &str) -> Self {
        self.serde_rename_all = Some(serde_rename_all.to_owned());
        self
    }

    pub fn lite_runtime(mut self, lite_runtime: bool) -> Self {
        self.lite_runtime = Some(lite_runtime);
        self
    }

    pub fn gen_mod_rs(mut self, gen_mod_rs: bool) -> Self {
        self.gen_mod_rs = Some(gen_mod_rs);
        self
    }

    pub fn inside_protobuf(mut self, inside_protobuf: bool) -> Self {
        self.inside_protobuf = Some(inside_protobuf);
        self
    }

    /// Update fields of self with fields defined in other customize
    pub fn update_with(&mut self, that: &Customize) {
        if let Some(v) = that.expose_oneof {
            self.expose_oneof = Some(v);
        }
        if let Some(v) = that.expose_fields {
            self.expose_fields = Some(v);
        }
        if let Some(v) = that.generate_accessors {
            self.generate_accessors = Some(v);
        }
        if let Some(v) = that.generate_getter {
            self.generate_getter = Some(v);
        }
        if let Some(v) = that.carllerche_bytes_for_bytes {
            self.carllerche_bytes_for_bytes = Some(v);
        }
        if let Some(v) = that.carllerche_bytes_for_string {
            self.carllerche_bytes_for_string = Some(v);
        }
        if let Some(v) = &that.type_attrs {
            self.type_attrs = Some(v.clone());
        }
        if let Some(v) = that.serde_derive {
            self.serde_derive = Some(v);
        }
        if let Some(ref v) = that.serde_derive_cfg {
            self.serde_derive_cfg = Some(v.clone());
        }
        if let Some(ref v) = that.serde_rename_all {
            self.serde_rename_all = Some(v.clone());
        }
        if let Some(v) = that.lite_runtime {
            self.lite_runtime = Some(v);
        }
        if let Some(v) = that.gen_mod_rs {
            self.gen_mod_rs = Some(v);
        }
        if let Some(v) = that.inside_protobuf {
            self.inside_protobuf = Some(v);
        }
    }

    /// Update unset fields of self with fields from other customize
    pub fn set_defaults_from(&mut self, other: &Customize) {
        let mut tmp = other.clone();
        tmp.update_with(self);
        *self = tmp;
    }

    /// Parse customize options from a string passed via protoc flag.
    pub fn parse_from_parameter(parameter: &str) -> CustomizeParseParameterResult<Customize> {
        fn parse_bool(v: &str) -> CustomizeParseParameterResult<bool> {
            v.parse()
                .map_err(|_| CustomizeParseParameterError::CannotParseBool)
        }

        let mut r = Customize::default();
        for nv in parameter.split_whitespace() {
            let eq = match nv.find('=') {
                Some(eq) => eq,
                None => return Err(CustomizeParseParameterError::EqNotFound),
            };

            let n = &nv[..eq];
            let v = &nv[eq + 1..];

            if n == "expose_oneof" {
                r.expose_oneof = Some(parse_bool(v)?);
            } else if n == "expose_fields" {
                r.expose_fields = Some(parse_bool(v)?);
            } else if n == "generate_accessors" {
                r.generate_accessors = Some(parse_bool(v)?);
            } else if n == "generate_getter" {
                r.generate_getter = Some(parse_bool(v)?);
            } else if n == "carllerche_bytes_for_bytes" {
                r.carllerche_bytes_for_bytes = Some(parse_bool(v)?);
            } else if n == "carllerche_bytes_for_string" {
                r.carllerche_bytes_for_string = Some(parse_bool(v)?);
            } else if n == "serde_derive" {
                r.serde_derive = Some(parse_bool(v)?);
            } else if n == "serde_derive_cfg" {
                r.serde_derive_cfg = Some(v.to_owned());
            } else if n == "serde_rename_all" {
                r.serde_rename_all = Some(v.to_owned());
            } else if n == "lite_runtime" {
                r.lite_runtime = Some(parse_bool(v)?);
            } else if n == "gen_mod_rs" {
                r.gen_mod_rs = Some(parse_bool(v)?);
            } else if n == "inside_protobuf" {
                r.inside_protobuf = Some(parse_bool(v)?);
            } else if n == "type_attr" {
                r.add_type_attr(v);
            } else if n == "field_attr" {
                r.add_field_attr(v);
            } else if n == "special_field_attr" {
                r.add_special_field_attr(v);
            } else {
                return Err(CustomizeParseParameterError::UnknownOptionName(
                    n.to_owned(),
                ));
            }
        }
        Ok(r)
    }
}

pub fn customize_from_rustproto_for_message(source: &MessageOptions) -> Customize {
    let expose_oneof = rustproto::exts::expose_oneof.get(source);
    let expose_fields = rustproto::exts::expose_fields.get(source);
    let generate_accessors = rustproto::exts::generate_accessors.get(source);
    let generate_getter = rustproto::exts::generate_getter.get(source);
    let carllerche_bytes_for_bytes = rustproto::exts::carllerche_bytes_for_bytes.get(source);
    let carllerche_bytes_for_string = rustproto::exts::carllerche_bytes_for_string.get(source);
    let type_attrs = None; // TODO
    let field_attrs = None; // TODO
    let special_field_attrs = None; // TODO
    let serde_derive = rustproto::exts::serde_derive.get(source);
    let serde_derive_cfg = rustproto::exts::serde_derive_cfg.get(source);
    let lite_runtime = None;
    let gen_mod_rs = None;
    let inside_protobuf = None;
    let serde_rename_all = None;
    Customize {
        expose_oneof,
        expose_fields,
        generate_accessors,
        generate_getter,
        carllerche_bytes_for_bytes,
        carllerche_bytes_for_string,
        type_attrs,
        field_attrs,
        special_field_attrs,
        serde_derive,
        serde_derive_cfg,
        serde_rename_all,
        lite_runtime,
        gen_mod_rs,
        inside_protobuf,
    }
}

pub fn customize_from_rustproto_for_enum(source: &EnumOptions) -> Customize {
    let serde_rename_all = rustproto::exts::serde_rename_all.get(source);
    let mut r = Customize::default();
    r.serde_rename_all = serde_rename_all;
    return r;
}

pub fn customize_from_rustproto_for_field(source: &FieldOptions) -> Customize {
    let expose_oneof = None;
    let expose_fields = rustproto::exts::expose_fields_field.get(source);
    let generate_accessors = rustproto::exts::generate_accessors_field.get(source);
    let generate_getter = rustproto::exts::generate_getter_field.get(source);
    let carllerche_bytes_for_bytes = rustproto::exts::carllerche_bytes_for_bytes_field.get(source);
    let carllerche_bytes_for_string =
        rustproto::exts::carllerche_bytes_for_string_field.get(source);
    let type_attrs = None;
    let field_attrs = None; // TODO
    let special_field_attrs = None; // TODO
    let serde_rename_all = None;
    let serde_derive = None;
    let serde_derive_cfg = None;
    let lite_runtime = None;
    let gen_mod_rs = None;
    let inside_protobuf = None;
    Customize {
        expose_oneof,
        expose_fields,
        generate_accessors,
        generate_getter,
        carllerche_bytes_for_bytes,
        carllerche_bytes_for_string,
        type_attrs,
        field_attrs,
        special_field_attrs,
        serde_derive,
        serde_derive_cfg,
        serde_rename_all,
        lite_runtime,
        gen_mod_rs,
        inside_protobuf,
    }
}

pub fn customize_from_rustproto_for_file(source: &FileOptions) -> Customize {
    let expose_oneof = rustproto::exts::expose_oneof_all.get(source);
    let expose_fields = rustproto::exts::expose_fields_all.get(source);
    let generate_accessors = rustproto::exts::generate_accessors_all.get(source);
    let generate_getter = rustproto::exts::generate_getter_all.get(source);
    let carllerche_bytes_for_bytes = rustproto::exts::carllerche_bytes_for_bytes_all.get(source);
    let carllerche_bytes_for_string = rustproto::exts::carllerche_bytes_for_string_all.get(source);
    let type_attrs = None; // TODO
    let field_attrs = None; // TODO
    let special_field_attrs = None; // TODO
    let serde_derive = rustproto::exts::serde_derive_all.get(source);
    let serde_derive_cfg = rustproto::exts::serde_derive_cfg_all.get(source);
    let lite_runtime = rustproto::exts::lite_runtime_all.get(source);
    let gen_mod_rs = None;
    let inside_protobuf = None;
    let serde_rename_all = None;
    Customize {
        expose_oneof,
        expose_fields,
        generate_accessors,
        generate_getter,
        carllerche_bytes_for_bytes,
        carllerche_bytes_for_string,
        type_attrs,
        field_attrs,
        special_field_attrs,
        serde_derive,
        serde_derive_cfg,
        serde_rename_all,
        lite_runtime,
        inside_protobuf,
        gen_mod_rs,
    }
}

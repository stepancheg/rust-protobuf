pub(crate) mod ctx;
pub(crate) mod rustproto_proto;

use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use protobuf::reflect::EnumDescriptor;
use protobuf::reflect::FieldDescriptor;
use protobuf::reflect::FileDescriptor;
use protobuf::reflect::MessageDescriptor;
use protobuf::reflect::OneofDescriptor;

/// Dynamic callback to customize code generation.
pub trait CustomizeCallback: 'static {
    fn file(&self, file: &FileDescriptor) -> Customize {
        let _ = file;
        Customize::default()
    }

    fn message(&self, message: &MessageDescriptor) -> Customize {
        let _ = message;
        Customize::default()
    }

    fn field(&self, field: &FieldDescriptor) -> Customize {
        let _ = field;
        Customize::default()
    }

    fn special_field(&self, message: &MessageDescriptor, field: &str) -> Customize {
        let _ = (message, field);
        Customize::default()
    }

    fn enumeration(&self, enum_type: &EnumDescriptor) -> Customize {
        let _ = enum_type;
        Customize::default()
    }

    fn oneof(&self, oneof: &OneofDescriptor) -> Customize {
        let _ = oneof;
        Customize::default()
    }
}

pub(crate) struct CustomizeCallbackDefault;
impl CustomizeCallback for CustomizeCallbackDefault {}

#[derive(Clone)]
pub(crate) struct CustomizeCallbackHolder(pub(crate) Rc<dyn CustomizeCallback>);

impl CustomizeCallbackHolder {
    pub(crate) fn new(callback: impl CustomizeCallback) -> CustomizeCallbackHolder {
        CustomizeCallbackHolder(Rc::new(callback))
    }
}

impl Deref for CustomizeCallbackHolder {
    type Target = dyn CustomizeCallback;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl Default for CustomizeCallbackHolder {
    fn default() -> Self {
        CustomizeCallbackHolder(Rc::new(CustomizeCallbackDefault))
    }
}

impl PartialEq for CustomizeCallbackHolder {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl fmt::Debug for CustomizeCallbackHolder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CustomizeCallbackWrapper")
            .finish_non_exhaustive()
    }
}

/// Specifies style of generated code.
/// Generated files can be customized using this proto
/// or using `rustproto.proto` options.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Customize {
    /// Code to insert before the element in the generated file.
    pub(crate) before: Option<String>,
    /// When false, `get_`, `set_`, `mut_` etc. accessors are not generated
    pub(crate) generate_accessors: Option<bool>,
    /// When false, `get_` is not generated even if `syntax = "proto2"`
    pub(crate) generate_getter: Option<bool>,
    /// Use `bytes::Bytes` for `bytes` fields
    pub(crate) tokio_bytes: Option<bool>,
    /// Use `bytes::Bytes` for `string` fields
    pub(crate) tokio_bytes_for_string: Option<bool>,
    /// When false, `#[non_exhaustive]` is not generated for `oneof` fields.
    pub(crate) oneofs_non_exhaustive: Option<bool>,
    /// Enable lite runtime.
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
    /// When true, protobuf maps are represented with `std::collections::BTreeMap`
    pub(crate) btreemap: Option<bool>,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum CustomizeParseParameterError {
    #[error("Cannot parse bool option value: {:?}", .0)]
    CannotParseBool(String),
    #[error("Unknown option name: {:?}", .0)]
    UnknownOptionName(String),
}

impl Customize {
    /// Insert code before the element in the generated file
    /// (e. g. serde annotations, see
    /// [example here](https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-examples/customize-serde)).
    pub fn before(mut self, before: &str) -> Self {
        self.before = Some(before.to_owned());
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

    pub fn tokio_bytes(mut self, tokio_bytes: bool) -> Self {
        self.tokio_bytes = Some(tokio_bytes);
        self
    }

    pub fn tokio_bytes_for_string(mut self, tokio_bytes_for_string: bool) -> Self {
        self.tokio_bytes_for_string = Some(tokio_bytes_for_string);
        self
    }

    pub fn oneofs_non_exhaustive(mut self, non_exhaustive: bool) -> Self {
        self.oneofs_non_exhaustive = Some(non_exhaustive);
        self
    }

    /// Generate code for "lite runtime". Generated code contains no code for reflection.
    /// So the generated code (and more importantly, generated binary size) is smaller,
    /// but reflection, text format, JSON serialization won't work.
    ///
    /// Note when using `protoc` plugin `protoc-gen-rust`, the option name is just `lite`.
    pub fn lite_runtime(mut self, lite_runtime: bool) -> Self {
        self.lite_runtime = Some(lite_runtime);
        self
    }

    /// Generate `mod.rs` with all the generated modules.
    /// This option is on by default in rust-protobuf version 3.
    pub fn gen_mod_rs(mut self, gen_mod_rs: bool) -> Self {
        self.gen_mod_rs = Some(gen_mod_rs);
        self
    }

    /// Generate code bundled in protobuf crate. Regular users don't need this option.
    pub fn inside_protobuf(mut self, inside_protobuf: bool) -> Self {
        self.inside_protobuf = Some(inside_protobuf);
        self
    }

    /// Use btreemaps for maps representation
    pub fn btreemaps(self, use_btreemaps: bool) -> Self {
        Self {
            btreemap: Some(use_btreemaps),
            ..self
        }
    }

    /// Update fields of self with fields defined in other customize
    pub fn update_with(&mut self, that: &Customize) {
        if let Some(v) = &that.before {
            self.before = Some(v.clone());
        }
        if let Some(v) = that.generate_accessors {
            self.generate_accessors = Some(v);
        }
        if let Some(v) = that.generate_getter {
            self.generate_getter = Some(v);
        }
        if let Some(v) = that.tokio_bytes {
            self.tokio_bytes = Some(v);
        }
        if let Some(v) = that.tokio_bytes_for_string {
            self.tokio_bytes_for_string = Some(v);
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
        if let Some(v) = that.btreemap {
            self.btreemap = Some(v);
        }
    }

    /// Update unset fields of self with fields from other customize
    pub fn set_defaults_from(&mut self, other: &Customize) {
        let mut tmp = other.clone();
        tmp.update_with(self);
        *self = tmp;
    }

    /// Parse customize options from a string passed via protoc flag.
    pub fn parse_from_parameter(parameter: &str) -> anyhow::Result<Customize> {
        fn parse_bool(v: &str) -> anyhow::Result<bool> {
            v.parse()
                .map_err(|_| CustomizeParseParameterError::CannotParseBool(v.to_owned()).into())
        }

        let mut r = Customize::default();
        for nv in parameter.split_whitespace() {
            let (n, v) = match nv.find('=') {
                Some(eq) => {
                    let n = &nv[..eq];
                    let v = &nv[eq + 1..];
                    (n, v)
                }
                None => (nv, "true"),
            };

            if n == "generate_accessors" {
                r.generate_accessors = Some(parse_bool(v)?);
            } else if n == "generate_getter" {
                r.generate_getter = Some(parse_bool(v)?);
            } else if n == "tokio_bytes" {
                r.tokio_bytes = Some(parse_bool(v)?);
            } else if n == "tokio_bytes_for_string" {
                r.tokio_bytes_for_string = Some(parse_bool(v)?);
            } else if n == "lite_runtime" {
                r.lite_runtime = Some(parse_bool(v)?);
            } else if n == "gen_mod_rs" {
                r.gen_mod_rs = Some(parse_bool(v)?);
            } else if n == "btreemap" {
                r.btreemap = Some(parse_bool(v)?);
            } else if n == "inside_protobuf" {
                r.inside_protobuf = Some(parse_bool(v)?);
            } else if n == "lite" {
                // Support Java and C++ protoc plugin syntax:
                // https://github.com/protocolbuffers/protobuf/issues/6489
                r.lite_runtime = Some(parse_bool(v)?);
            } else {
                return Err(CustomizeParseParameterError::UnknownOptionName(n.to_owned()).into());
            }
        }
        Ok(r)
    }
}

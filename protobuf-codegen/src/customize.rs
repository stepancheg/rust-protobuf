use protobuf::rustproto;
use protobuf::descriptor::FieldOptions;
use protobuf::descriptor::MessageOptions;
use protobuf::descriptor::FileOptions;


/// Specifies style of generated code.
/// Generated files can be customized using this proto
/// or using `rustproto.proto` options.
#[derive(Default, Debug, Clone)]
pub struct Customize {
    /// Make oneof enum public.
    pub expose_oneof: Option<bool>,
    /// When true all fields are public, and accessors are not generated
    pub expose_fields: Option<bool>,
    /// When false, `get_`, `set_`, `mut_` etc. accessors are not generated
    pub generate_accessors: Option<bool>,
    /// When false, `get_` is not generated even if `syntax = "proto2"`
    pub generate_getter: Option<bool>,
    /// Use `bytes::Bytes` for `bytes` fields
    pub carllerche_bytes_for_bytes: Option<bool>,
    /// Use `bytes::Bytes` for `string` fields
    pub carllerche_bytes_for_string: Option<bool>,
    /// Use `std::Vec<T>` to store repeated messages fields
    pub repeated_field_vec: Option<bool>,
    /// Use `std::Option<std::Box<T>>` to store singular messages fields
    pub singular_field_option_box: Option<bool>,
    /// Use `std::Option<T>` to store singular messages fields.
    /// Note, it's not possible to have recursive messages with this option enabled.
    pub singular_field_option: Option<bool>,
    /// Implement serde_derive for messages
    pub serde_derive: Option<bool>,
    /// Make sure `Customize` is always used with `..Default::default()`
    /// for future compatibility.
    pub _future_options: (),
}

impl Customize {
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
        if let Some(v) = that.repeated_field_vec {
            self.repeated_field_vec = Some(v);
        }
        if let Some(v) = that.singular_field_option_box {
            self.singular_field_option_box = Some(v);
        }
        if let Some(v) = that.singular_field_option {
            self.singular_field_option = Some(v);
        }
        if let Some(v) = that.serde_derive {
            self.serde_derive = Some(v);
        }
    }

    /// Update unset fields of self with fields from other customize
    pub fn set_defaults_from(&mut self, other: &Customize) {
        let mut tmp = other.clone();
        tmp.update_with(self);
        *self = tmp;
    }
}


pub fn customize_from_rustproto_for_message(source: &MessageOptions) -> Customize {
    let expose_oneof = rustproto::exts::expose_oneof.get(source);
    let expose_fields = rustproto::exts::expose_fields.get(source);
    let generate_accessors = rustproto::exts::generate_accessors.get(source);
    let generate_getter = rustproto::exts::generate_getter.get(source);
    let carllerche_bytes_for_bytes = rustproto::exts::carllerche_bytes_for_bytes.get(source);
    let carllerche_bytes_for_string = rustproto::exts::carllerche_bytes_for_string.get(source);
    let repeated_field_vec = rustproto::exts::repeated_field_vec.get(source);
    let singular_field_option_box = rustproto::exts::singular_field_option_box.get(source);
    let singular_field_option = rustproto::exts::singular_field_option.get(source);
    let serde_derive = rustproto::exts::serde_derive.get(source);
    Customize {
        expose_oneof,
        expose_fields,
        generate_accessors,
        generate_getter,
        carllerche_bytes_for_bytes,
        carllerche_bytes_for_string,
        repeated_field_vec,
        singular_field_option_box,
        singular_field_option,
        serde_derive,
        _future_options: (),
    }
}

pub fn customize_from_rustproto_for_field(source: &FieldOptions) -> Customize {
    let expose_oneof = None;
    let expose_fields = rustproto::exts::expose_fields_field.get(source);
    let generate_accessors = rustproto::exts::generate_accessors_field.get(source);
    let generate_getter = rustproto::exts::generate_getter_field.get(source);
    let carllerche_bytes_for_bytes = rustproto::exts::carllerche_bytes_for_bytes_field.get(source);
    let carllerche_bytes_for_string = rustproto::exts::carllerche_bytes_for_string_field.get(source);
    let repeated_field_vec = rustproto::exts::repeated_field_vec_field.get(source);
    let singular_field_option_box = rustproto::exts::singular_field_option_box_field.get(source);
    let singular_field_option = rustproto::exts::singular_field_option_field.get(source);
    let serde_derive = None;
    Customize {
        expose_oneof,
        expose_fields,
        generate_accessors,
        generate_getter,
        carllerche_bytes_for_bytes,
        carllerche_bytes_for_string,
        repeated_field_vec,
        singular_field_option_box,
        singular_field_option,
        serde_derive,
        _future_options: (),
    }
}

pub fn customize_from_rustproto_for_file(source: &FileOptions) -> Customize {
    let expose_oneof = rustproto::exts::expose_oneof_all.get(source);
    let expose_fields = rustproto::exts::expose_fields_all.get(source);
    let generate_accessors = rustproto::exts::generate_accessors_all.get(source);
    let generate_getter = rustproto::exts::generate_getter_all.get(source);
    let carllerche_bytes_for_bytes = rustproto::exts::carllerche_bytes_for_bytes_all.get(source);
    let carllerche_bytes_for_string = rustproto::exts::carllerche_bytes_for_string_all.get(source);
    let repeated_field_vec = rustproto::exts::repeated_field_vec_all.get(source);
    let singular_field_option_box = rustproto::exts::singular_field_option_box_all.get(source);
    let singular_field_option = rustproto::exts::singular_field_option_all.get(source);
    let serde_derive = rustproto::exts::serde_derive_all.get(source);
    Customize {
        expose_oneof,
        expose_fields,
        generate_accessors,
        generate_getter,
        carllerche_bytes_for_bytes,
        carllerche_bytes_for_string,
        repeated_field_vec,
        singular_field_option_box,
        singular_field_option,
        serde_derive,
        _future_options: (),
    }
}

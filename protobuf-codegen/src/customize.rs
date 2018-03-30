use protobuf::descriptorx::FieldWithContext;
use protobuf::rustproto;
use protobuf::reflect::ProtobufValue;
use protobuf::types::ProtobufType;
use protobuf::ext::ExtFieldOptional;
use protobuf::descriptor::FieldOptions;
use protobuf::descriptor::MessageOptions;
use protobuf::descriptor::FileOptions;


/// Specifies style of generated code.
#[derive(Default, Debug, Clone)]
pub struct Customize {
    /// Make oneof enum public.
    pub expose_oneof: Option<bool>,
    /// When true all fields are public, and accessors are not generated
    pub expose_fields: Option<bool>,
    /// When false, `get_`, `set_`, `mut_` etc. accessors are not generated
    pub generate_accessors: Option<bool>,
    /// Use `bytes::Bytes` for `bytes` fields
    pub carllerche_bytes_for_bytes: Option<bool>,
    /// Use `bytes::Bytes` for `string` fields
    pub carllerche_bytes_for_string: Option<bool>,
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
        if let Some(v) = that.carllerche_bytes_for_bytes {
            self.carllerche_bytes_for_bytes = Some(v);
        }
        if let Some(v) = that.carllerche_bytes_for_string {
            self.carllerche_bytes_for_string = Some(v);
        }
    }

    /// Update unset fields of self with fields from other customize
    pub fn set_defaults_from(&mut self, other: &Customize) {
        let mut tmp = other.clone();
        tmp.update_with(self);
        *self = tmp;
    }
}


pub fn customize_from_rustproto_for_field(source: &FieldWithContext) -> Customize {
    // doesn't have sense for field
    let expose_oneof = None;
    let expose_fields = join_field_ext(
        source,
        rustproto::exts::expose_fields_field,
        rustproto::exts::expose_fields,
        rustproto::exts::expose_fields_all,
    );
    let generate_accessors = join_field_ext(
        source,
        rustproto::exts::generate_accessors_field,
        rustproto::exts::generate_accessors,
        rustproto::exts::generate_accessors_all,
    );
    let carllerche_bytes_for_bytes = join_field_ext(
        source,
        rustproto::exts::carllerche_bytes_for_bytes_field,
        rustproto::exts::carllerche_bytes_for_bytes,
        rustproto::exts::carllerche_bytes_for_bytes_all,
    );
    let carllerche_bytes_for_string = join_field_ext(
        source,
        rustproto::exts::carllerche_bytes_for_string_field,
        rustproto::exts::carllerche_bytes_for_string,
        rustproto::exts::carllerche_bytes_for_string_all,
    );

    Customize {
        expose_oneof,
        expose_fields,
        generate_accessors,
        carllerche_bytes_for_bytes,
        carllerche_bytes_for_string,
    }
}

pub fn customize_from_rustproto_for_file(source: &FileOptions) -> Customize {
    let expose_oneof = rustproto::exts::expose_oneof_all.get(source);
    let expose_fields = rustproto::exts::expose_fields_all.get(source);
    let generate_accessors = rustproto::exts::generate_accessors_all.get(source);
    let carllerche_bytes_for_bytes = rustproto::exts::carllerche_bytes_for_bytes_all.get(source);
    let carllerche_bytes_for_string = rustproto::exts::carllerche_bytes_for_string_all.get(source);
    Customize {
        expose_oneof,
        expose_fields,
        generate_accessors,
        carllerche_bytes_for_bytes,
        carllerche_bytes_for_string,
    }
}

fn join_field_ext<A : ProtobufValue + Clone, T : ProtobufType<Value = A>>(
    source: &FieldWithContext,
    field_ext: ExtFieldOptional<FieldOptions, T>,
    message_ext: ExtFieldOptional<MessageOptions, T>,
    file_ext: ExtFieldOptional<FileOptions, T>,
) -> Option<A> {
    if let Some(v) = field_ext.get(source.field.get_options()) {
        return Some(v);
    }
    for m in source.containing_messages() {
        if let Some(v) = message_ext.get(m.get_options()) {
            return Some(v);
        }
    }
    return file_ext.get(source.message.scope.get_file_descriptor().get_options());
}


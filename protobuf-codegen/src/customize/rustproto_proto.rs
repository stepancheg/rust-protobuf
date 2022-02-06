use protobuf::descriptor::EnumOptions;
use protobuf::descriptor::FieldOptions;
use protobuf::descriptor::FileOptions;
use protobuf::descriptor::MessageOptions;
use protobuf::rustproto;

use crate::Customize;

pub(crate) fn customize_from_rustproto_for_message(source: &MessageOptions) -> Customize {
    let before = None;
    let expose_oneof = rustproto::exts::expose_oneof.get(source);
    let expose_fields = rustproto::exts::expose_fields.get(source);
    let generate_accessors = rustproto::exts::generate_accessors.get(source);
    let generate_getter = rustproto::exts::generate_getter.get(source);
    let carllerche_bytes_for_bytes = rustproto::exts::carllerche_bytes_for_bytes.get(source);
    let carllerche_bytes_for_string = rustproto::exts::carllerche_bytes_for_string.get(source);
    let lite_runtime = None;
    let gen_mod_rs = None;
    let inside_protobuf = None;
    Customize {
        before,
        expose_oneof,
        expose_fields,
        generate_accessors,
        generate_getter,
        carllerche_bytes_for_bytes,
        carllerche_bytes_for_string,
        lite_runtime,
        gen_mod_rs,
        inside_protobuf,
    }
}

pub(crate) fn customize_from_rustproto_for_enum(_source: &EnumOptions) -> Customize {
    Customize::default()
}

pub(crate) fn customize_from_rustproto_for_field(source: &FieldOptions) -> Customize {
    let before = None;
    let expose_oneof = None;
    let expose_fields = rustproto::exts::expose_fields_field.get(source);
    let generate_accessors = rustproto::exts::generate_accessors_field.get(source);
    let generate_getter = rustproto::exts::generate_getter_field.get(source);
    let carllerche_bytes_for_bytes = rustproto::exts::carllerche_bytes_for_bytes_field.get(source);
    let carllerche_bytes_for_string =
        rustproto::exts::carllerche_bytes_for_string_field.get(source);
    let lite_runtime = None;
    let gen_mod_rs = None;
    let inside_protobuf = None;
    Customize {
        before,
        expose_oneof,
        expose_fields,
        generate_accessors,
        generate_getter,
        carllerche_bytes_for_bytes,
        carllerche_bytes_for_string,
        lite_runtime,
        gen_mod_rs,
        inside_protobuf,
    }
}

pub(crate) fn customize_from_rustproto_for_file(source: &FileOptions) -> Customize {
    let before = None;
    let expose_oneof = rustproto::exts::expose_oneof_all.get(source);
    let expose_fields = rustproto::exts::expose_fields_all.get(source);
    let generate_accessors = rustproto::exts::generate_accessors_all.get(source);
    let generate_getter = rustproto::exts::generate_getter_all.get(source);
    let carllerche_bytes_for_bytes = rustproto::exts::carllerche_bytes_for_bytes_all.get(source);
    let carllerche_bytes_for_string = rustproto::exts::carllerche_bytes_for_string_all.get(source);
    let lite_runtime = rustproto::exts::lite_runtime_all.get(source);
    let gen_mod_rs = None;
    let inside_protobuf = None;
    Customize {
        before,
        expose_oneof,
        expose_fields,
        generate_accessors,
        generate_getter,
        carllerche_bytes_for_bytes,
        carllerche_bytes_for_string,
        lite_runtime,
        inside_protobuf,
        gen_mod_rs,
    }
}

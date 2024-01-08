use protobuf::descriptor::EnumOptions;
use protobuf::descriptor::FieldOptions;
use protobuf::descriptor::FileOptions;
use protobuf::descriptor::MessageOptions;
use protobuf::rustproto;

use crate::Customize;

pub(crate) fn customize_from_rustproto_for_message(source: &MessageOptions) -> Customize {
    let before = None;
    let generate_accessors = rustproto::exts::generate_accessors.get(source);
    let generate_getter = rustproto::exts::generate_getter.get(source);
    let tokio_bytes = rustproto::exts::tokio_bytes.get(source);
    let tokio_bytes_for_string = rustproto::exts::tokio_bytes_for_string.get(source);
    let oneofs_non_exhaustive = rustproto::exts::oneofs_non_exhaustive.get(source);
    let btreemap = rustproto::exts::btreemap.get(source);
    let lite_runtime = None;
    let gen_mod_rs = None;
    let inside_protobuf = None;
    Customize {
        before,
        generate_accessors,
        generate_getter,
        tokio_bytes,
        tokio_bytes_for_string,
        oneofs_non_exhaustive,
        lite_runtime,
        gen_mod_rs,
        inside_protobuf,
        btreemap,
    }
}

pub(crate) fn customize_from_rustproto_for_enum(_source: &EnumOptions) -> Customize {
    Customize::default()
}

pub(crate) fn customize_from_rustproto_for_field(source: &FieldOptions) -> Customize {
    let before = None;
    let generate_accessors = rustproto::exts::generate_accessors_field.get(source);
    let generate_getter = rustproto::exts::generate_getter_field.get(source);
    let tokio_bytes = rustproto::exts::tokio_bytes_field.get(source);
    let tokio_bytes_for_string = rustproto::exts::tokio_bytes_for_string_field.get(source);
    let oneofs_non_exhaustive = rustproto::exts::oneofs_non_exhaustive_field.get(source);
    let btreemap = rustproto::exts::btreemap_field.get(source);
    let lite_runtime = None;
    let gen_mod_rs = None;
    let inside_protobuf = None;
    Customize {
        before,
        generate_accessors,
        generate_getter,
        tokio_bytes,
        tokio_bytes_for_string,
        oneofs_non_exhaustive,
        lite_runtime,
        gen_mod_rs,
        inside_protobuf,
        btreemap,
    }
}

pub(crate) fn customize_from_rustproto_for_file(source: &FileOptions) -> Customize {
    let before = None;
    let generate_accessors = rustproto::exts::generate_accessors_all.get(source);
    let generate_getter = rustproto::exts::generate_getter_all.get(source);
    let tokio_bytes = rustproto::exts::tokio_bytes_all.get(source);
    let tokio_bytes_for_string = rustproto::exts::tokio_bytes_for_string_all.get(source);
    let oneofs_non_exhaustive = rustproto::exts::oneofs_non_exhaustive_all.get(source);
    let lite_runtime = rustproto::exts::lite_runtime_all.get(source);
    let btreemap = rustproto::exts::btreemap_all.get(source);
    let gen_mod_rs = None;
    let inside_protobuf = None;
    Customize {
        before,
        generate_accessors,
        generate_getter,
        tokio_bytes,
        tokio_bytes_for_string,
        oneofs_non_exhaustive,
        lite_runtime,
        inside_protobuf,
        gen_mod_rs,
        btreemap,
    }
}

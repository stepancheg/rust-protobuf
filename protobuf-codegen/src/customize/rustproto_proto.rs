use protobuf::descriptor::EnumOptions;
use protobuf::descriptor::FieldOptions;
use protobuf::descriptor::FileOptions;
use protobuf::descriptor::MessageOptions;
use protobuf::rustproto;

use crate::Customize;

pub(crate) fn customize_from_rustproto_for_message(source: &MessageOptions) -> Customize {
    let before = None;
    let tokio_bytes = rustproto::exts::tokio_bytes.get(source);
    let tokio_bytes_for_string = rustproto::exts::tokio_bytes_for_string.get(source);
    let lite_runtime = None;
    let gen_mod_rs = None;
    let inside_protobuf = None;
    Customize {
        before,
        tokio_bytes,
        tokio_bytes_for_string,
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
    let tokio_bytes = rustproto::exts::tokio_bytes_field.get(source);
    let tokio_bytes_for_string = rustproto::exts::tokio_bytes_for_string_field.get(source);
    let lite_runtime = None;
    let gen_mod_rs = None;
    let inside_protobuf = None;
    Customize {
        before,
        tokio_bytes,
        tokio_bytes_for_string,
        lite_runtime,
        gen_mod_rs,
        inside_protobuf,
    }
}

pub(crate) fn customize_from_rustproto_for_file(source: &FileOptions) -> Customize {
    let before = None;
    let tokio_bytes = rustproto::exts::tokio_bytes_all.get(source);
    let tokio_bytes_for_string = rustproto::exts::tokio_bytes_for_string_all.get(source);
    let lite_runtime = rustproto::exts::lite_runtime_all.get(source);
    let gen_mod_rs = None;
    let inside_protobuf = None;
    Customize {
        before,
        tokio_bytes,
        tokio_bytes_for_string,
        lite_runtime,
        inside_protobuf,
        gen_mod_rs,
    }
}

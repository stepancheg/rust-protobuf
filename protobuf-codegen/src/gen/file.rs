use std::collections::HashMap;

use protobuf::descriptor::file_options;
use protobuf::descriptor::FileDescriptorProto;
use protobuf::reflect::FileDescriptor;
use protobuf_parse::ProtoPath;

use crate::compiler_plugin;
use crate::customize::ctx::CustomizeElemCtx;
use crate::customize::rustproto_proto::customize_from_rustproto_for_file;
use crate::gen::code_writer::CodeWriter;
use crate::gen::enums::EnumGen;
use crate::gen::extensions::write_extensions;
use crate::gen::file_descriptor::write_file_descriptor_data;
use crate::gen::inside::protobuf_crate_path;
use crate::gen::message::MessageGen;
use crate::gen::paths::proto_path_to_rust_mod;
use crate::gen::scope::FileScope;
use crate::gen::scope::RootScope;
use crate::proto_name_to_rs;

pub(crate) struct GenFileResult {
    pub(crate) compiler_plugin_result: compiler_plugin::GenResult,
    pub(crate) mod_name: String,
}

pub(crate) fn gen_file(
    file_descriptor: &FileDescriptor,
    _files_map: &HashMap<&ProtoPath, &FileDescriptor>,
    root_scope: &RootScope,
    parent_customize: &CustomizeElemCtx,
    parser: &str,
) -> anyhow::Result<GenFileResult> {
    let lite_runtime_from_builtin_option = file_descriptor
        .proto()
        .options
        .get_or_default()
        .optimize_for()
        == file_options::OptimizeMode::LITE_RUNTIME;

    let mut customize_from_proto =
        customize_from_rustproto_for_file(file_descriptor.proto().options.get_or_default());
    if customize_from_proto.lite_runtime.is_none()
        && parent_customize.for_elem.lite_runtime.is_none()
    {
        customize_from_proto.lite_runtime = Some(lite_runtime_from_builtin_option);
    }

    let customize = parent_customize.child(&customize_from_proto, file_descriptor);

    let file_scope = FileScope { file_descriptor };
    let scope = file_scope.to_scope();

    let lite_runtime = customize.for_elem.lite_runtime.unwrap_or(false);

    let v = CodeWriter::with(|w| {
        w.write_generated_by("rust-protobuf", env!("CARGO_PKG_VERSION"), parser);

        w.write_line("");
        w.write_line(&format!(
            "//! Generated file from `{}`",
            file_descriptor.proto().name()
        ));

        if customize.for_elem.lite_runtime.unwrap_or(false) {
            w.comment("Generated for lite runtime");
        }

        if customize.for_elem.inside_protobuf != Some(true) {
            w.write_line("");
            w.write_line("/// Generated files are compatible only with the same version");
            w.write_line("/// of protobuf runtime.");
            w.write_line(&format!(
                "const _PROTOBUF_VERSION_CHECK: () = {}::{};",
                protobuf_crate_path(&customize.for_elem),
                protobuf::VERSION_IDENT
            ));
        }

        static NESTED_TYPE_NUMBER: protobuf::rt::Lazy<i32> = protobuf::rt::Lazy::new();
        let message_type_number = *NESTED_TYPE_NUMBER.get(|| {
            protobuf::reflect::MessageDescriptor::for_type::<FileDescriptorProto>()
                .field_by_name("message_type")
                .expect("`message_type` must exist")
                .proto()
                .number()
        });

        let mut path = vec![message_type_number, 0];
        for (id, message) in scope.messages().iter().enumerate() {
            // ignore map entries, because they are not used in map fields
            if !message.is_map() {
                path[1] = id as i32;

                w.write_line("");
                MessageGen::new(
                    file_descriptor,
                    message,
                    &root_scope,
                    &customize,
                    &path,
                    file_descriptor.proto().source_code_info.as_ref(),
                )?
                .write(w)?;
            }
        }

        static ENUM_TYPE_NUMBER: protobuf::rt::Lazy<i32> = protobuf::rt::Lazy::new();
        let enum_type_number = *ENUM_TYPE_NUMBER.get(|| {
            protobuf::reflect::MessageDescriptor::for_type::<FileDescriptorProto>()
                .field_by_name("enum_type")
                .expect("`enum_type` must exist")
                .proto()
                .number()
        });

        let mut path = vec![enum_type_number, 0];
        for (id, enum_type) in scope.enums().iter().enumerate() {
            path[1] = id as i32;

            w.write_line("");
            EnumGen::new(
                enum_type,
                &customize,
                root_scope,
                &path,
                file_descriptor.proto().source_code_info.as_ref(),
            )
            .write(w);
        }

        write_extensions(file_descriptor, &root_scope, w, &customize);

        if !lite_runtime {
            w.write_line("");
            write_file_descriptor_data(file_descriptor, &customize.for_elem, w);
        }

        Ok(())
    })?;

    Ok(GenFileResult {
        compiler_plugin_result: compiler_plugin::GenResult {
            name: proto_name_to_rs(file_descriptor.proto().name()),
            content: v.into_bytes(),
        },
        mod_name: proto_path_to_rust_mod(file_descriptor.proto().name()).into_string(),
    })
}

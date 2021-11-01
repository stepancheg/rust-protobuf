use std::collections::HashMap;

use protobuf::descriptor::file_options;
use protobuf::descriptor::FileDescriptorProto;
use protobuf::reflect::FileDescriptor;
use protobuf_parse::ProtoPath;

use crate::compiler_plugin;
use crate::customize::customize_from_rustproto_for_file;
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
use crate::Customize;
use crate::FileIndex;
use crate::GenFileResult;

pub(crate) fn gen_file(
    file_descriptor: &FileDescriptor,
    _files_map: &HashMap<&ProtoPath, &FileDescriptor>,
    root_scope: &RootScope,
    customize: &Customize,
    parser: &str,
) -> GenFileResult {
    // TODO: use it
    let mut customize = customize.clone();
    // options specified in invocation have precedence over options specified in file
    customize.update_with(&customize_from_rustproto_for_file(
        file_descriptor.proto().options.get_or_default(),
    ));

    let file_scope = FileScope { file_descriptor };
    let scope = file_scope.to_scope();
    let lite_runtime = customize.lite_runtime.unwrap_or_else(|| {
        file_descriptor
            .proto()
            .options
            .get_or_default()
            .get_optimize_for()
            == file_options::OptimizeMode::LITE_RUNTIME
    });

    let file_index = FileIndex::index(&file_scope);

    let mut v = Vec::new();

    {
        let mut w = CodeWriter::new(&mut v);

        w.write_generated_by("rust-protobuf", env!("CARGO_PKG_VERSION"), parser);

        w.write_line("");
        w.write_line(&format!(
            "//! Generated file from `{}`",
            file_descriptor.proto().get_name()
        ));
        if customize.inside_protobuf != Some(true) {
            w.write_line("");
            w.write_line("/// Generated files are compatible only with the same version");
            w.write_line("/// of protobuf runtime.");
            w.write_line(&format!(
                "const _PROTOBUF_VERSION_CHECK: () = {}::{};",
                protobuf_crate_path(&customize),
                protobuf::VERSION_IDENT
            ));
        }

        static NESTED_TYPE_NUMBER: protobuf::rt::LazyV2<i32> = protobuf::rt::LazyV2::INIT;
        let message_type_number = *NESTED_TYPE_NUMBER.get(|| {
            protobuf::reflect::MessageDescriptor::for_type::<FileDescriptorProto>()
                .get_field_by_name("message_type")
                .expect("`message_type` must exist")
                .get_proto()
                .get_number()
        });

        let mut path = vec![message_type_number, 0];
        for (id, message) in scope.get_messages().iter().enumerate() {
            // ignore map entries, because they are not used in map fields
            if !message.is_map() {
                path[1] = id as i32;

                w.write_line("");
                MessageGen::new(
                    file_descriptor,
                    message,
                    &file_index,
                    &root_scope,
                    &customize,
                    &path,
                    file_descriptor.proto().source_code_info.as_ref(),
                )
                .write(&mut w);
            }
        }

        static ENUM_TYPE_NUMBER: protobuf::rt::LazyV2<i32> = protobuf::rt::LazyV2::INIT;
        let enum_type_number = *ENUM_TYPE_NUMBER.get(|| {
            protobuf::reflect::MessageDescriptor::for_type::<FileDescriptorProto>()
                .get_field_by_name("enum_type")
                .expect("`enum_type` must exist")
                .get_proto()
                .get_number()
        });

        let mut path = vec![enum_type_number, 0];
        for (id, enum_type) in scope.get_enums().iter().enumerate() {
            path[1] = id as i32;

            w.write_line("");
            EnumGen::new(
                enum_type,
                &file_index,
                &customize,
                root_scope,
                &path,
                file_descriptor.proto().source_code_info.as_ref(),
            )
            .write(&mut w);
        }

        write_extensions(file_descriptor, &root_scope, &mut w, &customize);

        if !lite_runtime {
            w.write_line("");
            write_file_descriptor_data(file_descriptor, &customize, &mut w);
        }
    }

    GenFileResult {
        compiler_plugin_result: compiler_plugin::GenResult {
            name: proto_name_to_rs(file_descriptor.proto().get_name()),
            content: v,
        },
        mod_name: proto_path_to_rust_mod(file_descriptor.proto().get_name()).into_string(),
    }
}

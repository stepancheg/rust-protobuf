use std::collections::HashMap;

use protobuf::descriptor::FileDescriptorProto;
use protobuf::reflect::FileDescriptor;
use protobuf_parse::ProtoPath;
use protobuf_parse::ProtoPathBuf;

use crate::compiler_plugin;
use crate::customize::ctx::CustomizeElemCtx;
use crate::customize::CustomizeCallback;
use crate::gen::file::gen_file;
use crate::gen::mod_rs::gen_mod_rs;
use crate::gen::scope::RootScope;
use crate::gen::well_known_types::gen_well_known_types_mod;
use crate::Customize;

pub(crate) fn gen_all(
    file_descriptors: &[FileDescriptorProto],
    parser: &str,
    files_to_generate: &[ProtoPathBuf],
    customize: &Customize,
    customize_callback: &dyn CustomizeCallback,
) -> anyhow::Result<Vec<compiler_plugin::GenResult>> {
    let file_descriptors = FileDescriptor::new_dynamic_fds(file_descriptors.to_vec(), &[])?;

    let root_scope = RootScope {
        file_descriptors: &file_descriptors,
    };

    let mut results: Vec<compiler_plugin::GenResult> = Vec::new();
    let files_map: HashMap<&ProtoPath, &FileDescriptor> = file_descriptors
        .iter()
        .map(|f| Ok((ProtoPath::new(f.proto().name())?, f)))
        .collect::<Result<_, anyhow::Error>>()?;

    let mut mods = Vec::new();

    let customize = CustomizeElemCtx {
        for_elem: customize.clone(),
        for_children: customize.clone(),
        callback: customize_callback,
    };

    for file_name in files_to_generate {
        let file = files_map.get(file_name.as_path()).expect(&format!(
            "file not found in file descriptors: {:?}, files: {:?}",
            file_name,
            files_map.keys()
        ));
        let gen_file_result = gen_file(file, &files_map, &root_scope, &customize, parser)?;
        results.push(gen_file_result.compiler_plugin_result);
        mods.push(gen_file_result.mod_name);
    }

    if customize.for_elem.inside_protobuf.unwrap_or(false) {
        results.push(gen_well_known_types_mod());
    }

    if customize.for_elem.gen_mod_rs.unwrap_or(true) {
        results.push(gen_mod_rs(&mods));
    }

    Ok(results)
}

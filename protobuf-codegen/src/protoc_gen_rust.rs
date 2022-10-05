#![doc(hidden)]

use crate::compiler_plugin;
use crate::customize::CustomizeCallbackDefault;
use crate::gen::all::gen_all;
use crate::Customize;

#[doc(hidden)]
pub fn protoc_gen_rust_main() {
    compiler_plugin::plugin_main(|r| {
        let customize = Customize::parse_from_parameter(r.parameter).expect("parse options");
        gen_all(
            r.file_descriptors,
            "protoc --rust_out=...",
            r.files_to_generate,
            &customize,
            &CustomizeCallbackDefault,
        )
    })
    .expect("plugin failed");
}

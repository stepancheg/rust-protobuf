use protobuf::reflect::EnumDescriptor;
use protobuf::reflect::MessageDescriptor;

use crate::gen::code_writer::CodeWriter;
use crate::gen::file_descriptor::file_descriptor_call_expr;
use crate::gen::inside::protobuf_crate_path;
use crate::gen::scope::Scope;
use crate::Customize;

/// Abstract message or enum descriptor.
pub(crate) trait Descriptor {
    const DESCRIPTOR_FN: &'static str;
    const TYPE_NAME: &'static str;
    const GET_BY_RELATIVE_NAME_NAME: &'static str;
    fn name_to_package(&self) -> &str;
}

impl Descriptor for MessageDescriptor {
    const DESCRIPTOR_FN: &'static str = "descriptor";
    const TYPE_NAME: &'static str = "MessageDescriptor";
    const GET_BY_RELATIVE_NAME_NAME: &'static str = "message_by_package_relative_name";

    fn name_to_package(&self) -> &str {
        self.name_to_package()
    }
}

impl Descriptor for EnumDescriptor {
    const DESCRIPTOR_FN: &'static str = "enum_descriptor";
    const TYPE_NAME: &'static str = "EnumDescriptor";
    const GET_BY_RELATIVE_NAME_NAME: &'static str = "enum_by_package_relative_name";

    fn name_to_package(&self) -> &str {
        self.name_to_package()
    }
}

pub(crate) fn write_fn_descriptor<D: Descriptor>(
    descriptor: &D,
    scope: &Scope,
    customize: &Customize,
    w: &mut CodeWriter,
) {
    let sig = format!(
        "{}() -> {}::reflect::{}",
        D::DESCRIPTOR_FN,
        protobuf_crate_path(customize),
        D::TYPE_NAME,
    );
    w.def_fn(&sig, |w| {
        let expr = format!(
            "{}.{}(\"{}\").unwrap()",
            file_descriptor_call_expr(scope),
            D::GET_BY_RELATIVE_NAME_NAME,
            descriptor.name_to_package()
        );
        w.lazy_static(
            "descriptor",
            &format!(
                "{}::reflect::{}",
                protobuf_crate_path(customize),
                D::TYPE_NAME,
            ),
            &protobuf_crate_path(customize).to_string(),
        );
        w.write_line(&format!("descriptor.get(|| {}).clone()", expr));
    });
}

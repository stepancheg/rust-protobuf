use crate::rust_name::RustIdent;
use crate::scope::Scope;

pub(crate) fn file_descriptor_proto_expr(scope: &Scope) -> String {
    let file_descriptor_proto_path = scope
        .rust_path_to_file()
        .to_reverse()
        .into_path()
        .append_ident(RustIdent::from("file_descriptor_proto"));
    format!("{}()", file_descriptor_proto_path)
}

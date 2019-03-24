use scope::Scope;
use rust_name::RustIdent;

pub(crate) fn file_descriptor_proto_expr(scope: &Scope) -> String {
    let file_descriptor_proto_path = scope.rust_path_to_file().to_reverse()
        .append_ident(RustIdent::from("file_descriptor_proto"));
    format!("{}()", file_descriptor_proto_path)
}

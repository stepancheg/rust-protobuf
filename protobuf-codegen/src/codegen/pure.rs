use protobuf_parse::pure;
use protobuf_parse::ParsedAndTypechecked;

use crate::codegen::Codegen;

pub(crate) fn parse_and_typecheck(
    codegen: &Codegen,
) -> anyhow::Result<(ParsedAndTypechecked, String)> {
    let p = pure::parse_and_typecheck(&codegen.includes, &codegen.inputs)?;
    Ok((p, format!("protobuf-codegen={}", env!("CARGO_PKG_VERSION"))))
}

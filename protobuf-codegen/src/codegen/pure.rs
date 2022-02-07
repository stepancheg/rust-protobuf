use protobuf_parse::Parser;
use protobuf_parse::ParsedAndTypechecked;

use crate::codegen::Codegen;

pub(crate) fn parse_and_typecheck(
    codegen: &Codegen,
) -> anyhow::Result<(ParsedAndTypechecked, String)> {
    let p = Parser::new()
        .pure()
        .includes(&codegen.includes)
        .inputs(&codegen.inputs)
        .parse_and_typecheck()?;
    Ok((p, format!("protobuf-codegen={}", env!("CARGO_PKG_VERSION"))))
}

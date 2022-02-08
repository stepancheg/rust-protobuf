use protobuf_parse::ParsedAndTypechecked;
use protobuf_parse::Parser;

use crate::codegen::Codegen;

pub(crate) fn parse_and_typecheck(codegen: &Codegen) -> anyhow::Result<ParsedAndTypechecked> {
    let p = Parser::new()
        .pure()
        .includes(&codegen.includes)
        .inputs(&codegen.inputs)
        .parse_and_typecheck()?;
    Ok(p)
}

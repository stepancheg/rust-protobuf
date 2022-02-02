pub(crate) mod attr;
pub(crate) mod field;

use crate::gen::code_writer::CodeWriter;

pub(crate) trait RustAst {
    fn write_to(&self, w: &mut CodeWriter);
}

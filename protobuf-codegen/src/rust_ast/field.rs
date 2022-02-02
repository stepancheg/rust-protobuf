use crate::gen::code_writer::CodeWriter;
use crate::gen::code_writer::Visibility;
use crate::rust_ast::RustAst;

pub(crate) struct RustField {
    pub(crate) vis: Visibility,
    pub(crate) name: String,
    pub(crate) ty: String,
}

impl RustAst for RustField {
    fn write_to(&self, w: &mut CodeWriter) {
        match self.vis {
            Visibility::Public => w.write_line(&format!("pub {}: {},", self.name, self.ty)),
            Visibility::Default => w.write_line(&format!("{}: {},", self.name, self.ty)),
            Visibility::Path(..) => unimplemented!(),
        }
    }
}

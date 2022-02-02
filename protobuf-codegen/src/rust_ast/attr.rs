use crate::gen::code_writer::CodeWriter;
use crate::rust_ast::RustAst;

pub(crate) enum RustAttrArg {
    Attr(RustAttr),
}

pub(crate) struct RustAttr {
    pub(crate) name: String,
    pub(crate) args: Option<Vec<RustAttrArg>>,
}

impl RustAttrArg {
    fn to_string(&self) -> String {
        match self {
            RustAttrArg::Attr(attr) => attr.to_string(),
        }
    }

    fn ident(ident: &str) -> RustAttrArg {
        RustAttrArg::Attr(RustAttr::ident(ident))
    }
}

impl RustAttr {
    pub(crate) fn ident(name: &str) -> RustAttr {
        RustAttr {
            name: name.to_owned(),
            args: None,
        }
    }

    pub(crate) fn derive(traits: &[&str]) -> RustAttr {
        RustAttr {
            name: "derive".to_string(),
            args: Some(traits.iter().map(|t| RustAttrArg::ident(t)).collect()),
        }
    }

    fn to_string(&self) -> String {
        match &self.args {
            Some(args) => format!(
                "{}({})",
                self.name,
                args.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            None => format!("{}", self.name),
        }
    }
}

impl RustAst for RustAttr {
    fn write_to(&self, w: &mut CodeWriter) {
        w.write_line(&format!("#[{}]", self.to_string()));
    }
}

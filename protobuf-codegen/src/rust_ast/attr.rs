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
        RustAttrArg::Attr(RustAttr::no_args(ident))
    }
}

impl RustAttr {
    pub(crate) fn no_args(name: &str) -> RustAttr {
        RustAttr {
            name: name.to_owned(),
            args: None,
        }
    }

    pub(crate) fn ident_args(name: &str, args: &[&str]) -> RustAttr {
        RustAttr {
            name: name.to_owned(),
            args: Some(args.iter().map(|s| RustAttrArg::ident(s)).collect()),
        }
    }

    pub(crate) fn derive(traits: &[&str]) -> RustAttr {
        RustAttr::ident_args("derive", traits)
    }

    pub(crate) fn allow(attrs: &[&str]) -> RustAttr {
        RustAttr::ident_args("allow", attrs)
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

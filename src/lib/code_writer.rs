use std::io::Write;

// TODO: should not use wire_format here
use wire_format;

pub struct CodeWriter<'a> {
    writer: &'a mut (Write + 'a),
    indent: String,
}

impl<'a> CodeWriter<'a> {
    pub fn new(writer: &'a mut Write) -> CodeWriter<'a> {
        CodeWriter {
            writer: writer,
            indent: "".to_string(),
        }
    }

    pub fn write_line<S : AsRef<str>>(&mut self, line: S) {
        (if line.as_ref().is_empty() {
            self.writer.write_all("\n".as_bytes())
        } else {
            let s: String = [self.indent.as_ref(), line.as_ref(), "\n"].concat();
            self.writer.write_all(s.as_bytes())
        }).unwrap();
    }

    pub fn todo(&mut self, message: &str) {
        self.write_line(format!("panic!(\"TODO: {}\");", message));
    }

    pub fn indented<F>(&mut self, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        cb(&mut CodeWriter {
            writer: self.writer,
            indent: format!("{}    ", self.indent),
        });
    }

    #[allow(dead_code)]
    pub fn commented<F>(&mut self, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        cb(&mut CodeWriter {
            writer: self.writer,
            indent: format!("// {}", self.indent),
        });
    }

    pub fn lazy_static<S1 : AsRef<str>, S2 : AsRef<str>>(&mut self, name: S1, ty: S2) {
        self.stmt_block(format!("static mut {}: ::protobuf::lazy::Lazy<{}> = ::protobuf::lazy::Lazy", name.as_ref(), ty.as_ref()), |w| {
            w.field_entry("lock", "::protobuf::lazy::ONCE_INIT");
            w.field_entry("ptr", format!("0 as *const {}", ty.as_ref()));
        });
    }

    pub fn lazy_static_decl_get<S1 : AsRef<str>, S2 : AsRef<str>, F>(&mut self, name: S1, ty: S2, init: F)
        where F : Fn(&mut CodeWriter)
    {
        self.lazy_static(name.as_ref(), ty);
        self.unsafe_expr(|w| {
            w.write_line(format!("{}.get(|| {{", name.as_ref()));
            w.indented(|w| init(w));
            w.write_line(format!("}})"));
        });
    }

    pub fn block<S1 : AsRef<str>, S2 : AsRef<str>, F>(&mut self, first_line: S1, last_line: S2, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.write_line(first_line.as_ref());
        self.indented(cb);
        self.write_line(last_line.as_ref());
    }

    pub fn expr_block<S : AsRef<str>, F>(&mut self, prefix: S, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.block(format!("{} {{", prefix.as_ref()), "}", cb);
    }

    pub fn stmt_block<S : AsRef<str>, F>(&mut self, prefix: S, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.block(format!("{} {{", prefix.as_ref()), "};", cb);
    }

    pub fn unsafe_expr<F>(&mut self, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.expr_block("unsafe", cb);
    }

    pub fn impl_self_block<S : AsRef<str>, F>(&mut self, name: S, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.expr_block(format!("impl {}", name.as_ref()), cb);
    }

    pub fn impl_for_block<S1 : AsRef<str>, S2 : AsRef<str>, F>(&mut self, tr: S1, ty: S2, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.expr_block(format!("impl {} for {}", tr.as_ref(), ty.as_ref()), cb);
    }

    pub fn pub_struct<S : AsRef<str>, F>(&mut self, name: S, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.expr_block(format!("pub struct {}", name.as_ref()), cb);
    }

    pub fn pub_enum<F>(&mut self, name: &str, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.expr_block(format!("pub enum {}", name), cb);
    }

    pub fn field_entry<S1 : AsRef<str>, S2 : AsRef<str>>(&mut self, name: S1, value: S2) {
        self.write_line(format!("{}: {},", name.as_ref(), value.as_ref()));
    }

    pub fn field_decl<S : AsRef<str>>(&mut self, name: S, field_type: &str) {
        self.field_entry(name, field_type);
    }

    pub fn derive(&mut self, derive: &[&str]) {
        let v: Vec<String> = derive.iter().map(|&s| s.to_string()).collect();
        self.write_line(format!("#[derive({})]", v.connect(",")));
    }

    pub fn allow(&mut self, what: &[&str]) {
        let v: Vec<String> = what.iter().map(|&s| s.to_string()).collect();
        self.write_line(format!("#[allow({})]", v.connect(",")));
    }

    pub fn comment(&mut self, comment: &str) {
        if comment.is_empty() {
            self.write_line("//");
        } else {
            self.write_line(format!("// {}", comment));
        }
    }

    pub fn pub_fn<S : AsRef<str>, F>(&mut self, sig: S, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.expr_block(format!("pub fn {}", sig.as_ref()), cb);
    }

    pub fn def_fn<S : AsRef<str>, F>(&mut self, sig: S, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.expr_block(format!("fn {}", sig.as_ref()), cb);
    }

    pub fn while_block<S : AsRef<str>, F>(&mut self, cond: S, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.expr_block(format!("while {}", cond.as_ref()), cb);
    }

    // if ... { ... }
    pub fn if_stmt<S : AsRef<str>, F>(&mut self, cond: S, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.stmt_block(format!("if {}", cond.as_ref()), cb);
    }

    // if ... {} else { ... }
    pub fn if_else_stmt<S : AsRef<str>, F>(&mut self, cond: S, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.write_line(format!("if {} {{", cond.as_ref()));
        self.write_line("} else {");
        self.indented(cb);
        self.write_line("}");
    }

    // if let ... = ... { ... }
    pub fn if_let_stmt<F>(&mut self, decl: &str, expr: &str, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.if_stmt(format!("let {} = {}", decl, expr), cb);
    }

    // if let ... = ... { } else { ... }
    pub fn if_let_else_stmt<F>(&mut self, decl: &str, expr: &str, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.if_else_stmt(format!("let {} = {}", decl, expr), cb);
    }

    pub fn for_stmt<S1 : AsRef<str>, S2 : AsRef<str>, F>(&mut self, over: S1, varn: S2, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.stmt_block(format!("for {} in {}", varn.as_ref(), over.as_ref()), cb)
    }

    pub fn match_block<S : AsRef<str>, F>(&mut self, value: S, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.stmt_block(format!("match {}", value.as_ref()), cb);
    }

    pub fn match_expr<S : AsRef<str>, F>(&mut self, value: S, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.expr_block(format!("match {}", value.as_ref()), cb);
    }

    pub fn case_block<S : AsRef<str>, F>(&mut self, cond: S, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        self.block(format!("{} => {{", cond.as_ref()), "},", cb);
    }

    pub fn case_expr<S1 : AsRef<str>, S2 : AsRef<str>>(&mut self, cond: S1, body: S2) {
        self.write_line(format!("{} => {},", cond.as_ref(), body.as_ref()));
    }

    pub fn error_wire_type(&mut self, _wire_type: wire_format::WireType) {
        // TODO: write wire type
        let message = "\"unexpected wire type\".to_string()";
        self.write_line(format!(
                "return ::std::result::Result::Err(::protobuf::ProtobufError::WireError({}));",
                message));
    }

    pub fn assert_wire_type(&mut self, wire_type: wire_format::WireType) {
        self.if_stmt(format!("wire_type != ::protobuf::wire_format::{:?}", wire_type), |w| {
            w.error_wire_type(wire_type);
        });
    }
}

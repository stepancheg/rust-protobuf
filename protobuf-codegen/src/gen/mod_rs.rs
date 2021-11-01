use crate::compiler_plugin;
use crate::gen::code_writer::CodeWriter;

pub(crate) fn gen_mod_rs(mods: &[String]) -> compiler_plugin::GenResult {
    let mut v = Vec::new();
    let mut w = CodeWriter::new(&mut v);
    w.comment(&format!("{}generated", "@"));
    w.write_line("");
    for m in mods {
        w.write_line(&format!("pub mod {};", m));
    }
    drop(w);
    compiler_plugin::GenResult {
        name: "mod.rs".to_owned(),
        content: v,
    }
}

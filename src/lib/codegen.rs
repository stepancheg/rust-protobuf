use std::rt::io::Writer;
use std::cast;

use descriptor::*;
use misc::*;
use core::*;
use rt;

#[deriving(Eq)]
enum RustVersion {
    Rust07,
    RustMaster,
}

static USE_RUST_VERSION: RustVersion = RustMaster;

fn rust_name(field_type: FieldDescriptorProto_Type) -> &'static str {
    match field_type {
        TYPE_DOUBLE   => "f64",
        TYPE_FLOAT    => "f32",
        TYPE_INT32    => "i32",
        TYPE_INT64    => "i64",
        TYPE_UINT32   => "u32",
        TYPE_UINT64   => "u64",
        TYPE_SINT32   => "i32",
        TYPE_SINT64   => "i64",
        TYPE_FIXED32  => "u32",
        TYPE_FIXED64  => "u64",
        TYPE_SFIXED32 => "s32",
        TYPE_SFIXED64 => "s64",
        TYPE_BOOL     => "bool",
        TYPE_STRING   => "~str",
        TYPE_BYTES    => "~[u8]",
        TYPE_ENUM | TYPE_GROUP | TYPE_MESSAGE => fail!()
    }
}

fn protobuf_name(field_type: FieldDescriptorProto_Type) -> &'static str {
    match field_type {
        TYPE_DOUBLE   => "double",
        TYPE_FLOAT    => "float",
        TYPE_INT32    => "int32",
        TYPE_INT64    => "int64",
        TYPE_UINT32   => "uint32",
        TYPE_UINT64   => "uint64",
        TYPE_SINT32   => "sint32",
        TYPE_SINT64   => "sint64",
        TYPE_FIXED32  => "fixed32",
        TYPE_FIXED64  => "fixed64",
        TYPE_SFIXED32 => "sfixed32",
        TYPE_SFIXED64 => "sfixed64",
        TYPE_BOOL     => "bool",
        TYPE_STRING   => "string",
        TYPE_BYTES    => "bytes",
        TYPE_ENUM | TYPE_GROUP | TYPE_MESSAGE => fail!()
    }
}

fn field_type_wire_type(field_type: FieldDescriptorProto_Type) -> wire_format::WireType {
    use core::wire_format::*;
    match field_type {
        TYPE_INT32    => WireTypeVarint,
        TYPE_INT64    => WireTypeVarint,
        TYPE_UINT32   => WireTypeVarint,
        TYPE_UINT64   => WireTypeVarint,
        TYPE_SINT32   => WireTypeVarint,
        TYPE_SINT64   => WireTypeVarint,
        TYPE_BOOL     => WireTypeVarint,
        TYPE_ENUM     => WireTypeVarint,
        TYPE_FIXED32  => WireTypeFixed32,
        TYPE_FIXED64  => WireTypeFixed64,
        TYPE_SFIXED32 => WireTypeFixed32,
        TYPE_SFIXED64 => WireTypeFixed64,
        TYPE_FLOAT    => WireTypeFixed32,
        TYPE_DOUBLE   => WireTypeFixed64,
        TYPE_STRING   => WireTypeLengthDelimited,
        TYPE_BYTES    => WireTypeLengthDelimited,
        TYPE_MESSAGE  => WireTypeLengthDelimited,
        TYPE_GROUP    => fail!()
    }
}

fn field_type_size(field_type: FieldDescriptorProto_Type) -> Option<u32> {
    match field_type {
        TYPE_BOOL => Some(1),
        t if field_type_wire_type(t) == wire_format::WireTypeFixed32 => Some(4),
        t if field_type_wire_type(t) == wire_format::WireTypeFixed64 => Some(8),
        _ => None
    }
}

fn field_type_name(field: &FieldDescriptorProto, pkg: &str) -> ~str {
    match field.type_name {
        Some(ref type_name) => {
            let current_pkg_prefix = "." + pkg + ".";
            if (*type_name).starts_with(current_pkg_prefix) {
                remove_prefix(*type_name, current_pkg_prefix).to_owned()
            } else {
                remove_to(*type_name, '.').to_owned()
            }
        },
        None =>
            rust_name(field.field_type.unwrap()).to_owned()
    }
}

#[deriving(Clone)]
enum RepeatMode {
    Single,
    RepeatRegular,
    RepeatPacked,
}

#[deriving(Clone)]
struct Field {
    proto_field: FieldDescriptorProto,
    name: ~str,
    field_type: FieldDescriptorProto_Type,
    wire_type: wire_format::WireType,
    type_name: ~str,
    number: u32,
    repeated: bool,
    packed: bool,
    repeat_mode: RepeatMode,
}

impl Field {
    fn parse(field: &FieldDescriptorProto, pkg: &str) -> Option<Field> {
        let type_name = field_type_name(field, pkg).replace(".", "_");
        let repeated = match field.label.unwrap() {
            LABEL_REPEATED => true,
            LABEL_OPTIONAL | LABEL_REQUIRED => false,
        };
        let name = match field.name.get_ref().to_owned() {
            ~"type" => ~"field_type",
            x => x,
        };
        let packed = match field.options {
            Some(ref options) => options.packed.unwrap_or(false),
            None => false
        };
        let repeat_mode =
            if repeated {
                if packed { RepeatPacked } else { RepeatRegular }
            } else {
                Single
            };
        Some(Field {
            proto_field: field.clone(),
            name: name,
            field_type: field.field_type.unwrap(),
            wire_type: field_type_wire_type(field.field_type.unwrap()),
            type_name: type_name,
            number: field.number.unwrap() as u32,
            repeated: repeated,
            packed: packed,
            repeat_mode: repeat_mode,
        })
    }

    fn is_primitive(&self) -> bool {
        match self.field_type {
            TYPE_STRING | TYPE_BYTES | TYPE_MESSAGE => false,
            _ => true,
        }
    }

    fn stored_as_pointer(&self) -> bool {
        match self.field_type {
            TYPE_STRING | TYPE_BYTES => true,
            _ => false,
        }
    }

    fn full_type(&self) -> ~str {
        match self.repeated {
            true  => format!("~[{:s}]", self.type_name),
            false => format!("Option<{:s}>", self.type_name),
        }
    }
}

#[deriving(Clone)]
struct Message {
    proto_message: DescriptorProto,
    pkg: ~str,
    prefix: ~str,
    type_name: ~str,
    fields: ~[Field],
}

impl<'self> Message {
    fn parse(proto_message: &DescriptorProto, pkg: &str, prefix: &str) -> Message {
        Message {
            proto_message: proto_message.clone(),
            pkg: pkg.to_owned(),
            prefix: prefix.to_owned(),
            type_name: prefix + proto_message.name.get_ref().to_owned(),
            fields: do proto_message.field.flat_map |field| {
                match Field::parse(field, pkg) {
                    Some(field) => ~[field],
                    None => ~[]
                }
            },
        }
    }

    fn has_any_message_field(&self) -> bool {
        for field in self.fields.iter() {
            if field.field_type == TYPE_MESSAGE {
                return true;
            }
        }
        false
    }

    fn has_any_repeated_field(&self) -> bool {
        for field in self.fields.iter() {
            if field.repeated {
                return true;
            }
        }
        false
    }

    fn required_fields(&'self self) -> ~[&'self Field] {
        let mut r = ~[];
        for field in self.fields.iter() {
            if field.proto_field.label.unwrap() == LABEL_REQUIRED {
                r.push(field);
            }
        }
        r
    }
}


struct IndentWriter<'self> {
    writer: @Writer,
    indent: ~str,
    msg: Option<&'self Message>,
    field: Option<&'self Field>,
}

impl<'self> IndentWriter<'self> {
    fn new(writer: @Writer) -> IndentWriter {
        IndentWriter {
            writer: writer,
            indent: ~"",
            msg: None,
            field: None,
        }
    }

    fn bind_message<T>(&self, msg: &Message, cb: &fn(&IndentWriter) -> T) -> T {
        cb(&IndentWriter {
            writer: self.writer,
            indent: self.indent.to_owned(),
            msg: Some(msg),
            field: None,
        })
    }

    fn bind_field<T>(&self, field: &'self Field, cb: &fn(&IndentWriter) -> T) -> T {
        assert!(self.msg.is_some());
        cb(&IndentWriter {
            writer: self.writer,
            indent: self.indent.to_owned(),
            msg: self.msg,
            field: Some(field),
        })
    }

    fn fields(&self, cb: &fn(&IndentWriter)) {
        let fields = &self.msg.get_ref().fields;
        let mut iter = fields.iter();
        for field in iter {
            self.bind_field(field, |w| cb(w));
        }
    }

    fn required_fields(&self, cb: &fn(&IndentWriter)) {
        let fields = &self.msg.get_ref().required_fields();
        let mut iter = fields.iter();
        for field in iter {
            self.bind_field(*field, |w| cb(w));
        }
    }
    /*
    fn fields(&'self self) -> FieldsIter<'self> {
        FieldsIter { parent: self }
    }
    fn required_fields(&'self self) -> FieldsIter<'self> {
        FieldsIter { parent: self }
    }
    */


    fn field(&self) -> &'self Field {
        assert!(self.field.is_some());
        self.field.unwrap()
    }

    fn self_field(&self) -> ~str {
        format!("self.{:s}", self.field().name)
    }

    fn self_field_is_some(&self) -> ~str {
        assert!(!self.field().repeated);
        format!("{:s}.is_some()", self.self_field())
    }

    fn self_field_is_none(&self) -> ~str {
        assert!(!self.field().repeated);
        format!("{:s}.is_none()", self.self_field())
    }

    fn if_self_field_is_some(&self, cb: &fn(&IndentWriter)) {
        self.if_stmt(self.self_field_is_some(), cb);
    }

    fn if_self_field_is_none(&self, cb: &fn(&IndentWriter)) {
        self.if_stmt(self.self_field_is_none(), cb);
    }

    fn self_field_assign(&self, value: &str) {
        self.write_line(format!("{:s} = {:s};", self.self_field(), value));
    }

    fn self_field_assign_none(&self) {
        assert!(!self.field().repeated);
        self.self_field_assign("None");
    }

    fn self_field_assign_some(&self, value: &str) {
        assert!(!self.field().repeated);
        self.self_field_assign(format!("Some({:s})", value));
    }

    fn self_field_push(&self, value: &str) {
        assert!(self.field().repeated);
        self.write_line(format!("{:s}.push({:s});", self.self_field(), value));
    }

    fn field_default(&self) {
        let init =
            if self.field().repeated {
                "~[]"
            } else {
                "None"
            };
        self.field_entry(self.field().name, init);
    }

    fn field_type_default(&self) -> ~str {
        match self.field().field_type {
            TYPE_MESSAGE => format!("{:s}::new()", self.field().type_name),
            // TODO: use hardcoded constant
            TYPE_ENUM    => format!("{:s}::new(0)", self.field().type_name),
            TYPE_STRING  => ~"~\"\"",
            TYPE_BYTES   => ~"~[]",
            TYPE_BOOL    => ~"false",
            TYPE_FLOAT | TYPE_DOUBLE => ~"0.",
            _            => ~"0",
        }
    }

    fn write_line(&self, line: &str) {
        let mut_writer: @mut Writer = unsafe { cast::transmute(self.writer) };
        if line.is_empty() {
            mut_writer.write("\n".as_bytes())
        } else {
            let s = self.indent + line + "\n";
            mut_writer.write(s.as_bytes());
        }
    }

    fn write_lines(&self, lines: &[~str]) {
        for line in lines.iter() {
            self.write_line(*line);
        }
    }

    fn indented(&self, cb: &fn(&IndentWriter)) {
        cb(&IndentWriter {
            writer: self.writer,
            indent: self.indent + "    ",
            msg: self.msg,
            field: self.field,
        });
    }

    fn commented(&self, cb: &fn(&IndentWriter)) {
        cb(&IndentWriter {
            writer: self.writer,
            indent: "// " + self.indent,
            msg: self.msg,
            field: self.field.clone(),
        });
    }

    fn block(&self, first_line: &str, last_line: &str, cb: &fn(&IndentWriter)) {
        self.write_line(first_line);
        self.indented(cb);
        self.write_line(last_line);
    }

    fn expr_block(&self, prefix: &str, cb: &fn(&IndentWriter)) {
        self.block(prefix + " {", "}", cb);
    }

    fn stmt_block(&self, prefix: &str, cb: &fn(&IndentWriter)) {
        self.block(prefix + " {", "};", cb);
    }

    fn impl_block(&self, name: &str, cb: &fn(&IndentWriter)) {
        self.expr_block(format!("impl {:s}", name), cb);
    }

    fn impl_self_block(&self, name: &str, cb: &fn(&IndentWriter)) {
        self.expr_block(format!("impl<'self> {:s}", name), cb);
    }

    fn impl_for_block(&self, tr: &str, ty: &str, cb: &fn(&IndentWriter)) {
        self.expr_block(format!("impl {:s} for {:s}", tr, ty), cb);
    }

    fn pub_struct(&self, name: &str, cb: &fn(&IndentWriter)) {
        self.expr_block("pub struct " + name, cb);
    }

    fn def_struct(&self, name: &str, cb: &fn(&IndentWriter)) {
        self.expr_block("struct " + name, cb);
    }

    fn def_mod(&self, name: &str, cb: &fn(&IndentWriter)) {
        self.expr_block("mod " + name, cb);
    }

    fn field_entry(&self, name: &str, value: &str) {
        self.write_line(format!("{:s}: {:s},", name, value));
    }

    fn fail(&self, reason: &str) {
        self.write_line(format!("fail!({:?});", reason));
    }

    fn todo(&self) {
        self.fail("TODO");
    }

    fn comment(&self, comment: &str) {
        if comment.is_empty() {
            self.write_line("//");
        } else {
            self.write_line("// " + comment);
        }
    }

    fn pub_fn(&self, sig: &str, cb: &fn(&IndentWriter)) {
        self.expr_block(format!("pub fn {:s}", sig), cb);
    }

    fn def_fn(&self, sig: &str, cb: &fn(&IndentWriter)) {
        self.expr_block(format!("fn {:s}", sig), cb);
    }

    fn while_block(&self, cond: &str, cb: &fn(&IndentWriter)) {
        self.expr_block(format!("while {:s}", cond), cb);
    }

    fn if_stmt(&self, cond: &str, cb: &fn(&IndentWriter)) {
        self.stmt_block(format!("if {:s}", cond), cb);
    }

    fn for_stmt(&self, over: &str, varn: &str, cb: &fn(&IndentWriter)) {
        match USE_RUST_VERSION {
            Rust07     => self.stmt_block(format!("for {:s}.advance |{:s}|", over, varn), cb),
            RustMaster => self.stmt_block(format!("for {:s} in {:s}", varn, over), cb),
        }
    }

    fn match_block(&self, value: &str, cb: &fn(&IndentWriter)) {
        self.stmt_block(format!("match {:s}", value), cb);
    }

    fn match_expr(&self, value: &str, cb: &fn(&IndentWriter)) {
        self.expr_block(format!("match {:s}", value), cb);
    }

    fn case_block(&self, cond: &str, cb: &fn(&IndentWriter)) {
        self.block(format!("{:s} => \\{", cond), "},", cb);
    }

    fn case_expr(&self, cond: &str, body: &str) {
        self.write_line(format!("{:s} => {:s},", cond, body));
    }

    fn clear_field_func(&self) -> ~str {
        "clear_" + self.field.get_ref().name
    }

    fn clear_field(&self) {
        if self.field().repeated {
            self.write_line(format!("{:s}.clear();", self.self_field()));
        } else {
            self.self_field_assign_none();
        }
    }

}

fn write_merge_from_field(w: &IndentWriter) {
    let field = w.field();
    let wire_type = field_type_wire_type(field.field_type);
    let repeat_mode =
        if field.repeated {
            if wire_type == wire_format::WireTypeLengthDelimited {
                RepeatRegular
            } else {
                RepeatPacked // may be both regular or packed
            }
        } else {
            Single
        };

    let read_proc = match field.field_type {
        TYPE_MESSAGE => None,
        TYPE_ENUM => Some(format!("{:s}::new(is.read_int32())", field.type_name)),
        t => Some(format!("is.read_{:s}()", protobuf_name(t))),
    };

    match repeat_mode {
        Single | RepeatRegular => {
            w.write_line(format!("assert_eq!(wire_format::{:?}, wire_type);", wire_type));
            match field.field_type {
                TYPE_MESSAGE => {
                    w.write_line(format!("let mut tmp = {:s}::new();", field.type_name));
                    w.write_line(format!("is.merge_message(&mut tmp);"));
                },
                _ => {
                    w.write_line(format!("let tmp = {:s};", *read_proc.get_ref()));
                },
            };
            match repeat_mode {
                Single => w.self_field_assign_some("tmp"),
                RepeatRegular => w.self_field_push("tmp"),
                _ => fail!()
            }
        },
        RepeatPacked => {
            w.write_line(format!("if wire_type == wire_format::{:?} \\{", wire_format::WireTypeLengthDelimited));
            do w.indented |w| {
                w.write_line("let len = is.read_raw_varint32();");
                w.write_line("let old_limit = is.push_limit(len);");
                do w.while_block("!is.eof()") |w| {
                    w.self_field_push(*read_proc.get_ref());
                };
                w.write_line("is.pop_limit(old_limit);");
            }
            w.write_line("} else {");
            do w.indented |w| {
                w.write_line(format!("assert_eq!(wire_format::{:?}, wire_type);", wire_type));
                w.self_field_push(*read_proc.get_ref());
            }
            w.write_line("}");
        },
    };
}

fn write_message(msg: &Message, w: &IndentWriter) {
    let pkg = msg.pkg.as_slice();
    let message_type = &msg.proto_message;

    do w.bind_message(msg) |w| {
        w.write_line(format!("\\#[deriving(Clone,Eq)]"));
        do w.pub_struct(msg.type_name) |w| {
            do w.fields |w| {
                let field = w.field.unwrap();
                if !field.type_name.contains_char('.') {
                    w.field_entry(field.name, field.full_type());
                }
            }
            if msg.fields.is_empty() {
                w.field_entry("dummy", "bool");
            }
        }

        w.write_line("");
        do w.impl_self_block(msg.type_name) |w| {
            do w.pub_fn(format!("new() -> {:s}", msg.type_name)) |w| {
                do w.expr_block(msg.type_name) |w| {
                    do w.fields |w| {
                        w.field_default();
                    }
                    if msg.fields.is_empty() {
                        w.field_entry("dummy", "false");
                    }
                }
            }

            w.write_line("");
            do w.pub_fn(format!("default_instance() -> &'static {:s}", msg.type_name)) |w| {
                fn write_body(w: &IndentWriter) {
                    let msg = w.msg.get_ref();
                    do w.stmt_block(format!("static instance: {:s} = {:s}", msg.type_name, msg.type_name)) |w| {
                        do w.fields |w| {
                            w.field_default();
                        }
                        if msg.fields.is_empty() {
                            w.field_entry("dummy", "false");
                        }
                    }
                    w.write_line("&'static instance");
                }
                if msg.has_any_repeated_field() {
                    do w.commented |w| {
                        w.comment("doesn't work, because rust doen't implement \
                                static constants of types like ~str");
                        w.comment("https://github.com/mozilla/rust/issues/8406");
                        write_body(w);
                    }
                    w.todo();
                } else if USE_RUST_VERSION == RustMaster {
                    do w.commented |w| {
                        w.comment("doesn't work, because rust master has broken \
                                static constants that contains None of ~str");
                        w.comment("https://github.com/mozilla/rust/issues/8578");
                        w.comment("TODO: should at least keep static without ~str");
                        write_body(w);
                    }
                    w.todo();
                } else {
                    write_body(w)
                }
            }

            w.write_line("");
            if !msg.has_any_message_field() {
                // `sizes` and `sizes_pos` are unused
                w.write_line("#[allow(unused_variable)]");
            }
            do w.pub_fn("write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint)") |w| {
                do w.fields |w| {
                    let field = w.field();
                    let field_type = field.field_type;
                    let write_method_suffix = match field_type {
                        TYPE_MESSAGE => "message",
                        TYPE_ENUM => "enum",
                        t => protobuf_name(t),
                    };
                    let field_number = field.proto_field.number.unwrap();
                    let vv = match field.field_type {
                        TYPE_MESSAGE => "v", // TODO: as &Message
                        TYPE_ENUM => "*v as i32",
                        _ => "*v",
                    };
                    let write_value_lines = match field.field_type {
                        TYPE_MESSAGE => ~[
                            format!("os.write_tag({:d}, wire_format::{:?});",
                                    field_number as int, wire_format::WireTypeLengthDelimited),
                            format!("os.write_raw_varint32(sizes[*sizes_pos]);"),
                            format!("*sizes_pos += 1;"),
                            format!("v.write_to_with_computed_sizes(os, sizes, sizes_pos);"),
                        ],
                        _ => ~[
                            format!("os.write_{:s}({:d}, {:s});", write_method_suffix, field_number as int, vv),
                        ],
                    };
                    match field.repeat_mode {
                        Single => {
                            do w.match_block(w.self_field()) |w| {
                                do w.case_block("Some(ref v)") |w| {
                                    w.write_lines(write_value_lines);
                                };
                                w.case_expr("None", "{}");
                            }
                        },
                        RepeatPacked => {
                            do w.if_stmt(format!("!{:s}.is_empty()", w.self_field())) |w| {
                                w.write_line(format!("os.write_tag({:d}, wire_format::{:?});", field_number as int, wire_format::WireTypeLengthDelimited));
                                w.write_line(format!("os.write_raw_varint32(rt::vec_packed_data_size({:s}, wire_format::{:?}));", w.self_field(), field_type_wire_type(field.field_type)));
                                do w.for_stmt(format!("{:s}.iter()", w.self_field()), "v") |w| {
                                    w.write_line(format!("os.write_{:s}_no_tag({:s});", write_method_suffix, vv));
                                }
                            }
                        },
                        RepeatRegular => {
                            do w.for_stmt(format!("{:s}.iter()", w.self_field()), "v") |w| {
                                w.write_lines(write_value_lines);
                            }
                        },
                    };
                }
            }
            do w.fields |w| {
                w.write_line("");
                do w.pub_fn(format!("{:s}(&mut self)", w.clear_field_func())) |w| {
                    w.clear_field();
                }

                if !w.field().repeated {
                    w.write_line("");
                    do w.pub_fn(format!("has_{:s}(&self) -> bool", w.field().name)) |w| {
                        w.write_line(w.self_field_is_some());
                    }
                }

                let set_param_type = if w.field().repeated {
                    w.field().full_type()
                } else {
                    w.field().type_name.to_owned()
                };

                w.write_line("");
                w.comment("Param is passed by value, moved");
                do w.pub_fn(format!("set_{:s}(&mut self, v: {:s})", w.field().name, set_param_type)) |w| {
                    if w.field().repeated {
                        w.self_field_assign("v");
                    } else {
                        w.self_field_assign_some("v");
                    }
                }

                w.write_line("");
                w.comment("Mutable pointer to the field.");
                if !w.field().repeated {
                    w.comment("If field is not initialized, it is initialized with default value first.");
                }
                do w.pub_fn(format!("mut_{:s}(&'self mut self) -> &'self mut {:s}", w.field().name, set_param_type))
                |w| {
                    if !w.field().repeated {
                        do w.if_self_field_is_none |w| {
                            w.self_field_assign_some(w.field_type_default());
                        }
                        w.write_line(format!("{:s}.get_mut_ref()", w.self_field()));
                    } else {
                        w.write_line(format!("&mut {:s}", w.self_field()));
                    }
                }

                w.write_line("");
                let return_reference = w.field().repeated || match w.field().field_type {
                    TYPE_MESSAGE | TYPE_STRING | TYPE_BYTES => true,
                    _ => false,
                };
                let get_xxx_return_type = match w.field().repeated {
                    true => format!("&'self [{:s}]", w.field().type_name),
                    false => match return_reference {
                        true => {
                            format!("&'self {:s}", match w.field().field_type {
                                TYPE_BYTES  => ~"[u8]",
                                TYPE_STRING => ~"str",
                                _ => set_param_type,
                            })
                        }
                        false => set_param_type.to_owned(),
                    }
                };
                let self_param = match return_reference {
                    true  => "&'self self",
                    false => "&self",
                };
                do w.pub_fn(format!("get_{:s}({:s}) -> {:s}", w.field().name, self_param, get_xxx_return_type))
                |w| {
                    if !w.field().repeated {
                        if return_reference {
                            do w.match_expr(w.self_field()) |w| {
                                w.case_expr(
                                    "Some(ref v)",
                                    match w.field().field_type {
                                        TYPE_STRING => "v.as_slice()",
                                        TYPE_BYTES => "rt::as_slice_tmp(v)",
                                        _ => "v",
                                    }
                                );
                                w.case_expr(
                                    "None",
                                    match w.field().field_type {
                                        TYPE_MESSAGE => format!("{:s}::default_instance()", w.field().type_name),
                                        TYPE_BYTES   => ~"&'self []",
                                        TYPE_STRING  => ~"&'self \"\"",
                                        _            => fail!(),
                                    }
                                );
                            }
                        } else {
                            let get_name = match USE_RUST_VERSION {
                                Rust07     => "get_or_default",
                                RustMaster => "unwrap_or",
                            };
                            w.write_line(format!(
                                    "{:s}.{:s}({:s})",
                                    w.self_field(),
                                    get_name,
                                    w.field_type_default()));
                        }
                    } else {
                        w.write_line(format!("rt::as_slice_tmp(&{:s})", w.self_field()));
                    }
                }

                if w.field().repeated {
                    w.write_line("");
                    do w.pub_fn(format!("add_{:s}(&mut self, v: {:s})",
                            w.field().name, w.field().type_name.to_owned()))
                    |w| {
                        w.self_field_push("v");
                    }
                }
            }
        }

        w.write_line("");

        do w.impl_for_block("Message", msg.type_name) |w| {
            do w.def_fn(format!("new() -> {:s}", msg.type_name)) |w| {
                w.write_line(format!("{:s}::new()", msg.type_name));
            }
            w.write_line("");
            do w.def_fn("clear(&mut self)") |w| {
                do w.fields |w| {
                    w.write_line(format!("self.{:s}();", w.clear_field_func()));
                }
            }
            w.write_line("");
            do w.def_fn(format!("is_initialized(&self) -> bool")) |w| {
                do w.required_fields |w| {
                    do w.if_self_field_is_none |w| {
                        w.write_line("return false;");
                    }
                }
                w.write_line("true");
            }
            w.write_line("");
            do w.def_fn(format!("merge_from(&mut self, is: &mut CodedInputStream)")) |w| {
                do w.while_block("!is.eof()") |w| {
                    w.write_line(format!("let (field_number, wire_type) = is.read_tag_unpack();"));
                    do w.match_block("field_number") |w| {
                        do w.fields |w| {
                            do w.case_block(w.field().number.to_str()) |w| {
                                write_merge_from_field(w);
                            }
                        }
                        do w.case_block("_") |w| {
                            w.write_line(format!("// TODO: store in unknown fields"));
                            w.write_line(format!("is.skip_field(wire_type);"));
                        }
                    }
                }
            }
            w.write_line("");
            // Append sizes of messages in the tree to the specified vector.
            // First appended element is size of self, and then nested message sizes.
            // in serialization order are appended recursively.");
            w.comment("Compute sizes of nested messages");
            do w.def_fn("compute_sizes(&self, sizes: &mut ~[u32]) -> u32") |w| {
                w.write_line("let pos = sizes.len();");
                w.write_line("sizes.push(0);");
                w.write_line("let mut my_size = 0;");
                do w.fields |w| {
                    let field = w.field();
                    match field.repeat_mode {
                        Single | RepeatRegular => {
                            match field_type_size(field.field_type) {
                                Some(s) => {
                                    if field.repeated {
                                        w.write_line(format!(
                                                "my_size += {:d} * {:s}.len();",
                                                (s + rt::tag_size(field.number)) as int,
                                                w.self_field()));
                                    } else {
                                        do w.if_self_field_is_some |w| {
                                            w.write_line(format!(
                                                    "my_size += {:d};",
                                                    (s + rt::tag_size(field.number)) as int));
                                        }
                                    }
                                },
                                None => {
                                    do w.for_stmt(format!("{:s}.iter()", w.self_field()), "value") |w| {
                                        match field.field_type {
                                            TYPE_MESSAGE => {
                                                w.write_line("let len = value.compute_sizes(sizes);");
                                                w.write_line(format!(
                                                        "my_size += {:u} + rt::compute_raw_varint32_size(len) + len;",
                                                        rt::tag_size(field.number) as uint));
                                            },
                                            TYPE_BYTES | TYPE_STRING => {
                                                let pn = protobuf_name(field.field_type);
                                                w.write_line(format!(
                                                        "my_size += rt::{:s}_size({:d}, *value);",
                                                        pn,
                                                        field.number as int));
                                            },
                                            TYPE_ENUM => {
                                                w.write_line(format!(
                                                        "my_size += rt::enum_size({:d}, *value);",
                                                        field.number as int));
                                            },
                                            _ => {
                                                w.write_line(format!(
                                                        "my_size += rt::value_size({:d}, *value, wire_format::{:?});",
                                                        field.number as int, field.wire_type));
                                            },
                                        }
                                    }
                                },
                            };
                        },
                        RepeatPacked => {
                            w.write_line(format!(
                                    "my_size += rt::vec_packed_size({:d}, {:s}, wire_format::{:?});",
                                    field.number as int, w.self_field(), field.wire_type));
                        },
                    };
                }
                w.write_line("sizes[pos] = my_size;");
                w.comment("value is returned for convenience");
                w.write_line("my_size");
            }
            w.write_line("");
            do w.def_fn("write_to(&self, os: &mut CodedOutputStream)") |w| {
                w.write_line("self.check_initialized();");
                w.write_line("let mut sizes: ~[u32] = ~[];");
                w.write_line("self.compute_sizes(&mut sizes);");
                w.write_line("let mut sizes_pos = 1; // first element is self");
                w.write_line("self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);");
                w.write_line("assert_eq!(sizes_pos, sizes.len());");
            }
        }

        for nested_type in message_type.nested_type.iter() {
            w.write_line("");
            write_message(&Message::parse(nested_type, pkg, msg.type_name + "_"), w);
        }

        for enum_type in message_type.enum_type.iter() {
            w.write_line("");
            write_enum(msg.type_name + "_", w, enum_type);
        }
    }
}

fn write_enum(prefix: &str, w: &IndentWriter, enum_type: &EnumDescriptorProto) {
    let enum_type_name = prefix + enum_type.name.get_ref().to_owned();
    w.write_line(format!("\\#[deriving(Clone,Eq)]"));
    w.write_line(format!("pub enum {:s} \\{", enum_type_name));
    for value in enum_type.value.iter() {
        w.write_line(format!("    {:s} = {:d},", value.name.get_ref().to_owned(), value.number.unwrap() as int));
    }
    w.write_line(format!("\\}"));
    w.write_line("");
    do w.impl_block(enum_type_name) |w| {
        do w.pub_fn(format!("new(value: i32) -> {:s}", enum_type_name)) |w| {
            do w.match_expr("value") |w| {
                for value in enum_type.value.iter() {
                    let value_number = value.number.unwrap();
                    let value_name = value.name.get_ref().to_owned();
                    w.write_line(format!("{:d} => {:s},", value_number as int, value_name));
                }
                w.write_line(format!("_ => fail!()"));
            }
        }
    }
    w.write_line("");
    do w.impl_for_block("ProtobufEnum", enum_type_name) |w| {
        do w.def_fn("value(&self) -> i32") |w| {
            w.write_line("*self as i32")
        }
    }
}

fn remove_to<'s>(s: &'s str, c: char) -> &'s str {
    match s.rfind(c) {
        Some(pos) => s.slice_from(pos + 1),
        None => s
    }
}

fn remove_from(s: &str, c: char) -> ~str {
    match s.find(c) {
        Some(pos) => s.slice_to(pos).to_owned(),
        None => s.to_owned()
    }
}

fn remove_suffix<'s>(s: &'s str, suffix: &str) -> &'s str {
    if !s.ends_with(suffix) {
        fail!();
    }
    s.slice_to(s.len() - suffix.len())
}

fn remove_prefix<'s>(s: &'s str, prefix: &str) -> &'s str {
    if !s.starts_with(prefix) {
        fail!();
    }
    s.slice_from(prefix.len())
}

fn remove_prefix_if_present<'s>(s: &'s str, prefix: &str) -> &'s str {
    if s.starts_with(prefix) {
        remove_prefix(s, prefix)
    } else {
        s
    }
}


fn last_part_of_package<'s>(pkg: &'s str) -> &'s str {
    remove_to(pkg, '.')
}

fn proto_path_to_rust_base<'s>(path: &'s str) -> &'s str {
    remove_suffix(remove_to(path, '/'), ".proto")
}

struct GenResult {
    name: ~str,
    content: ~[u8],
}

pub struct GenOptions {
    dummy: bool,
}

pub fn gen(files: &[FileDescriptorProto], _: &GenOptions) -> ~[GenResult] {
    let mut results: ~[GenResult] = ~[];
    for file in files.iter() {
        let base = proto_path_to_rust_base(*file.name.get_ref());

        let os0 = VecWriter::new();
        let os = os0 as @Writer;

        let w = IndentWriter::new(os);

        w.write_line("// This file is generated. Do not edit");
        w.write_line("");

        w.write_line("use protobuf::*;");
        w.write_line("use protobuf::rt;");
        for dep in file.dependency.iter() {
            w.write_line(format!("use {:s}::*;", proto_path_to_rust_base(*dep)));
        }

        for message_type in file.message_type.iter() {
            w.write_line("");
            write_message(&Message::parse(message_type, *file.package.get_ref(), ""), &w);
        }
        for enum_type in file.enum_type.iter() {
            w.write_line("");
            write_enum("", &w, enum_type);
        }

        results.push(GenResult {
            name: base + ".rs",
            content: os0.vec.to_owned(),
        });
    }
    results
}


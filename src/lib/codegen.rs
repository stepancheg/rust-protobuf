use std::io::Writer;
use std::cast;

use descriptor::*;
use misc::*;
use core::*;
use rt;
use paginate::PaginatableIterator;
use strx::*;

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
        TYPE_SFIXED32 => "i32",
        TYPE_SFIXED64 => "i64",
        TYPE_BOOL     => "bool",
        TYPE_STRING   => "~str",
        TYPE_BYTES    => "Vec<u8>",
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
    if field.has_type_name() {
        let current_pkg_prefix = if pkg.is_empty() { ~"." } else { "." + pkg + "." };
        if field.get_type_name().starts_with(current_pkg_prefix) {
            remove_prefix(field.get_type_name(), current_pkg_prefix).to_owned()
        } else {
            remove_to(field.get_type_name(), '.').to_owned()
        }
    } else if field.has_field_type() {
        rust_name(field.get_field_type()).to_owned()
    } else {
        fail!("neither type_name, nor field_type specified for field: {}", field.get_name());
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
        let repeated = match field.get_label() {
            LABEL_REPEATED => true,
            LABEL_OPTIONAL | LABEL_REQUIRED => false,
        };
        let name = match field.get_name() {
            "type" => ~"field_type",
            x => x.to_owned(),
        };
        let packed =
            if field.has_options() {
                field.get_options().get_packed()
            } else {
                false
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
            field_type: field.get_field_type(),
            wire_type: field_type_wire_type(field.get_field_type()),
            type_name: type_name,
            number: field.get_number() as u32,
            repeated: repeated,
            packed: packed,
            repeat_mode: repeat_mode,
        })
    }

    fn full_type(&self) -> ~str {
        match self.repeated {
            true  => format!("Vec<{:s}>", self.type_name),
            false => format!("Option<{:s}>", self.type_name),
        }
    }

    fn is_fixed(&self) -> bool {
        field_type_size(self.field_type).is_some()
    }

    fn is_zigzag(&self) -> bool {
        match self.field_type {
            TYPE_SINT32 | TYPE_SINT64 => true,
            _ => false,
        }
    }
}

#[deriving(Clone)]
struct Message {
    proto_message: DescriptorProto,
    pkg: ~str,
    prefix: ~str,
    type_name: ~str,
    fields: Vec<Field>,
}

impl<'a> Message {
    fn parse(proto_message: &DescriptorProto, pkg: &str, prefix: &str) -> Message {
        Message {
            proto_message: proto_message.clone(),
            pkg: pkg.to_owned(),
            prefix: prefix.to_owned(),
            type_name: prefix + proto_message.get_name().to_owned(),
            fields: proto_message.get_field().iter().flat_map(|field| {
                Field::parse(field, pkg).move_iter()
            }).collect(),
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

    fn required_fields(&'a self) -> Vec<&'a Field> {
        let mut r = Vec::new();
        for field in self.fields.iter() {
            if field.proto_field.get_label() == LABEL_REQUIRED {
                r.push(field);
            }
        }
        r
    }
}


struct IndentWriter<'a> {
    // TODO: add mut
    writer: &'a Writer,
    indent: ~str,
    msg: Option<&'a Message>,
    field: Option<&'a Field>,
}

impl<'a> IndentWriter<'a> {
    fn new(writer: &'a mut Writer) -> IndentWriter<'a> {
        IndentWriter {
            writer: writer,
            indent: ~"",
            msg: None,
            field: None,
        }
    }

    fn bind_message<T>(&self, msg: &Message, cb: |&mut IndentWriter| -> T) -> T {
        cb(&mut IndentWriter {
            writer: unsafe { cast::transmute(self.writer) },
            indent: self.indent.to_owned(),
            msg: Some(msg),
            field: None,
        })
    }

    fn bind_field<T>(&self, field: &'a Field, cb: |&mut IndentWriter| -> T) -> T {
        assert!(self.msg.is_some());
        cb(&mut IndentWriter {
            writer: self.writer,
            indent: self.indent.to_owned(),
            msg: self.msg,
            field: Some(field),
        })
    }

    fn fields(&self, cb: |&mut IndentWriter|) {
        let fields = &self.msg.get_ref().fields;
        let mut iter = fields.iter();
        for field in iter {
            self.bind_field(field, |w| cb(w));
        }
    }

    fn required_fields(&self, cb: |&mut IndentWriter|) {
        let fields = &self.msg.get_ref().required_fields();
        let mut iter = fields.iter();
        for field in iter {
            self.bind_field(*field, |w| cb(w));
        }
    }
    /*
    fn fields(&'a self) -> FieldsIter<'a> {
        FieldsIter { parent: self }
    }
    fn required_fields(&'a self) -> FieldsIter<'a> {
        FieldsIter { parent: self }
    }
    */


    fn field(&self) -> &'a Field {
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

    fn self_field_is_not_empty(&self) -> ~str {
        assert!(self.field().repeated);
        format!("!{:s}.is_empty()", self.self_field())
    }

    fn self_field_is_none(&self) -> ~str {
        assert!(!self.field().repeated);
        format!("{:s}.is_none()", self.self_field())
    }

    fn if_self_field_is_some(&self, cb: |&mut IndentWriter|) {
        self.if_stmt(self.self_field_is_some(), cb);
    }

    fn if_self_field_is_not_empty(&self, cb: |&mut IndentWriter|) {
        self.if_stmt(self.self_field_is_not_empty(), cb);
    }

    fn if_self_field_is_none(&self, cb: |&mut IndentWriter|) {
        self.if_stmt(self.self_field_is_none(), cb);
    }

    fn for_self_field(&mut self, varn: &str, cb: |&mut IndentWriter|) {
        self.for_stmt(format!("{}.iter()", self.self_field()), varn, cb);
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

    fn self_field_tag_size(&self) -> u32 {
        rt::tag_size(self.field().number)
    }

    fn self_field_vec_packed_fixed_data_size(&self) -> ~str {
        assert!(self.field().is_fixed());
        format!("({}.len() * {}) as u32",
            self.self_field(), field_type_size(self.field().field_type).unwrap())
    }

    fn self_field_vec_packed_varint_data_size(&self) -> ~str {
        assert!(!self.field().is_fixed());
        let zigzag_suffix = if self.field().is_zigzag() { "_zigzag" } else { "" };
        format!("::protobuf::rt::vec_packed_varint{}_data_size({:s}.as_slice())",
            zigzag_suffix, self.self_field())
    }

    fn self_field_vec_packed_data_size(&self) -> ~str {
        assert!(self.field().repeated);
        if self.field().is_fixed() {
            self.self_field_vec_packed_fixed_data_size()
        } else {
            self.self_field_vec_packed_varint_data_size()
        }
    }

    fn self_field_vec_packed_fixed_size(&self) -> ~str {
        // zero is filtered outside
        format!("{} + ::protobuf::rt::compute_raw_varint32_size({}.len() as u32) + {}",
            self.self_field_tag_size(),
            self.self_field(),
            self.self_field_vec_packed_fixed_data_size())
    }

    fn self_field_vec_packed_varint_size(&self) -> ~str {
        // zero is filtered outside
        assert!(!self.field().is_fixed());
        let zigzag_suffix = if self.field().is_zigzag() { "_zigzag" } else { "" };
        format!("::protobuf::rt::vec_packed_varint{}_size({:u}, {:s}.as_slice())",
            zigzag_suffix, self.field().number, self.self_field())
    }

    fn self_field_vec_packed_size(&mut self) -> ~str {
        assert!(self.field.unwrap().packed);
        // zero is filtered outside
        if self.field.unwrap().is_fixed() {
            self.self_field_vec_packed_fixed_size()
        } else {
            self.self_field_vec_packed_varint_size()
        }
    }

    fn field_default(&self) {
        let init =
            if self.field().repeated {
                "Vec::new()"
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
            TYPE_BYTES   => ~"Vec::new()",
            TYPE_BOOL    => ~"false",
            TYPE_FLOAT | TYPE_DOUBLE => ~"0.",
            _            => ~"0",
        }
    }

    fn write_line(&self, line: &str) {
        let mut_writer: &mut Writer = unsafe { cast::transmute(self.writer) };
        (if line.is_empty() {
            mut_writer.write("\n".as_bytes())
        } else {
            let s = self.indent + line + "\n";
            mut_writer.write(s.as_bytes())
        }).unwrap();
    }

    fn write_lines(&self, lines: &[~str]) {
        for line in lines.iter() {
            self.write_line(*line);
        }
    }

    fn indented(&self, cb: |&mut IndentWriter|) {
        cb(&mut IndentWriter {
            writer: self.writer,
            indent: self.indent + "    ",
            msg: self.msg,
            field: self.field,
        });
    }

    fn commented(&self, cb: |&mut IndentWriter|) {
        cb(&mut IndentWriter {
            writer: self.writer,
            indent: "// " + self.indent,
            msg: self.msg,
            field: self.field.clone(),
        });
    }

    fn block(&self, first_line: &str, last_line: &str, cb: |&mut IndentWriter|) {
        self.write_line(first_line);
        self.indented(cb);
        self.write_line(last_line);
    }

    fn expr_block(&self, prefix: &str, cb: |&mut IndentWriter|) {
        self.block(prefix + " {", "}", cb);
    }

    fn stmt_block(&self, prefix: &str, cb: |&mut IndentWriter|) {
        self.block(prefix + " {", "};", cb);
    }

    fn unsafe_block(&self, cb: |&mut IndentWriter|) {
        self.stmt_block("unsafe", cb);
    }

    fn unsafe_expr(&self, cb: |&mut IndentWriter|) {
        self.expr_block("unsafe", cb);
    }

    fn impl_block(&self, name: &str, cb: |&mut IndentWriter|) {
        self.expr_block(format!("impl {:s}", name), cb);
    }

    fn impl_self_block(&self, name: &str, cb: |&mut IndentWriter|) {
        self.expr_block(format!("impl<'a> {:s}", name), cb);
    }

    fn impl_for_block(&self, tr: &str, ty: &str, cb: |&mut IndentWriter|) {
        self.expr_block(format!("impl {:s} for {:s}", tr, ty), cb);
    }

    fn pub_struct(&self, name: &str, cb: |&mut IndentWriter|) {
        self.expr_block("pub struct " + name, cb);
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

    fn deriving(&mut self, deriving: &[&str]) {
        let v: ~[~str] = deriving.iter().map(|&s| s.to_owned()).collect();
        self.write_line(format!("\\#[deriving({})]", v.connect(",")));
    }

    fn comment(&self, comment: &str) {
        if comment.is_empty() {
            self.write_line("//");
        } else {
            self.write_line("// " + comment);
        }
    }

    fn pub_fn(&self, sig: &str, cb: |&mut IndentWriter|) {
        self.expr_block(format!("pub fn {:s}", sig), cb);
    }

    fn def_fn(&self, sig: &str, cb: |&mut IndentWriter|) {
        self.expr_block(format!("fn {:s}", sig), cb);
    }

    fn while_block(&self, cond: &str, cb: |&mut IndentWriter|) {
        self.expr_block(format!("while {:s}", cond), cb);
    }

    fn if_stmt(&self, cond: &str, cb: |&mut IndentWriter|) {
        self.stmt_block(format!("if {:s}", cond), cb);
    }

    fn for_stmt(&self, over: &str, varn: &str, cb: |&mut IndentWriter|) {
        self.stmt_block(format!("for {:s} in {:s}", varn, over), cb)
    }

    fn match_block(&self, value: &str, cb: |&mut IndentWriter|) {
        self.stmt_block(format!("match {:s}", value), cb);
    }

    fn match_expr(&self, value: &str, cb: |&mut IndentWriter|) {
        self.expr_block(format!("match {:s}", value), cb);
    }

    fn case_block(&self, cond: &str, cb: |&mut IndentWriter|) {
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

fn write_merge_from_field(w: &mut IndentWriter) {
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
            w.write_line(format!("assert_eq!(::protobuf::wire_format::{:?}, wire_type);", wire_type));
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
            w.write_line(format!("if wire_type == ::protobuf::wire_format::{:?} \\{", wire_format::WireTypeLengthDelimited));
            w.indented(|w| {
                w.write_line("let len = is.read_raw_varint32();");
                w.write_line("let old_limit = is.push_limit(len);");
                w.while_block("!is.eof()", |w| {
                    w.self_field_push(*read_proc.get_ref());
                });
                w.write_line("is.pop_limit(old_limit);");
            });
            w.write_line("} else {");
            w.indented(|w| {
                w.write_line(format!("assert_eq!(::protobuf::wire_format::{:?}, wire_type);", wire_type));
                w.self_field_push(*read_proc.get_ref());
            });
            w.write_line("}");
        },
    };
}

fn write_message_struct(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    w.deriving(["Clone,Eq,Default"]);
    w.pub_struct(msg.type_name, |w| {
        w.fields(|w| {
            let field = w.field.unwrap();
            if !field.type_name.contains_char('.') {
                w.field_entry(field.name, field.full_type());
            }
        });
        w.field_entry("unknown_fields", "Option<~::protobuf::UnknownFields>");
    });
}

fn write_message_compute_sizes(w: &mut IndentWriter) {
    // Append sizes of messages in the tree to the specified vector.
    // First appended element is size of self, and then nested message sizes.
    // in serialization order are appended recursively.");
    w.comment("Compute sizes of nested messages");
    w.def_fn("compute_sizes(&self, sizes: &mut Vec<u32>) -> u32", |w| {
        // To have access to its methods but not polute the name space.
        w.write_line("use protobuf::{Message};");
        w.write_line("let pos = sizes.len();");
        w.write_line("sizes.push(0);");
        w.write_line("let mut my_size = 0;");
        w.fields(|w| {
            let field = w.field();
            match field.repeat_mode {
                Single | RepeatRegular => {
                    match field_type_size(field.field_type) {
                        Some(s) => {
                            if field.repeated {
                                w.write_line(format!(
                                        "my_size += {:d} * {:s}.len() as u32;",
                                        (s + w.self_field_tag_size()) as int,
                                        w.self_field()));
                            } else {
                                w.if_self_field_is_some(|w| {
                                    w.write_line(format!(
                                            "my_size += {:d};",
                                            (s + w.self_field_tag_size()) as int));
                                });
                            }
                        },
                        None => {
                            w.for_self_field("value", |w| {
                                match field.field_type {
                                    TYPE_MESSAGE => {
                                        w.write_line("let len = value.compute_sizes(sizes);");
                                        w.write_line(format!(
                                                "my_size += {:u} + ::protobuf::rt::compute_raw_varint32_size(len) + len;",
                                                w.self_field_tag_size() as uint));
                                    },
                                    TYPE_BYTES => {
                                        w.write_line(format!(
                                                "my_size += ::protobuf::rt::bytes_size({:d}, value.as_slice());",
                                                field.number as int));
                                    },
                                    TYPE_STRING => {
                                        w.write_line(format!(
                                                "my_size += ::protobuf::rt::string_size({:d}, *value);",
                                                field.number as int));
                                    },
                                    TYPE_ENUM => {
                                        w.write_line(format!(
                                                "my_size += ::protobuf::rt::enum_size({:d}, *value);",
                                                field.number as int));
                                    },
                                    _ => {
                                        w.write_line(format!(
                                                "my_size += ::protobuf::rt::value_size({:d}, *value, ::protobuf::wire_format::{:?});",
                                                field.number as int, field.wire_type));
                                    },
                                }
                            });
                        },
                    };
                },
                RepeatPacked => {
                    w.if_self_field_is_not_empty(|w| {
                        let size_expr = w.self_field_vec_packed_size();
                        w.write_line(format!("my_size += {};", size_expr));
                    });
                },
            };
        });
        w.write_line("my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());");
        w.write_line("*sizes.get_mut(pos) = my_size;");
        w.comment("value is returned for convenience");
        w.write_line("my_size");
    });
}

fn write_message_write_to_with_computed_sizes(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    if !msg.has_any_message_field() {
        // `sizes` and `sizes_pos` are unused
        w.write_line("#[allow(unused_variable)]");
    }
    w.pub_fn("write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint)", |w| {
        // To have access to its methods but not polute the name space.
        w.write_line("use protobuf::{Message};");
        w.fields(|w| {
            let field = w.field();
            let field_type = field.field_type;
            let write_method_suffix = match field_type {
                TYPE_MESSAGE => "message",
                TYPE_ENUM => "enum",
                t => protobuf_name(t),
            };
            let field_number = field.proto_field.get_number();
            let vv = match field.field_type {
                TYPE_MESSAGE => "v", // TODO: as &Message
                TYPE_ENUM => "*v as i32",
                TYPE_BYTES => "v.as_slice()",
                _ => "*v",
            };
            let write_value_lines = match field.field_type {
                TYPE_MESSAGE => ~[
                    format!("os.write_tag({:d}, ::protobuf::wire_format::{:?});",
                            field_number as int, wire_format::WireTypeLengthDelimited),
                    format!("os.write_raw_varint32(sizes[*sizes_pos]);"),
                    format!("*sizes_pos += 1;"),
                    format!("v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos);"),
                ],
                _ => ~[
                    format!("os.write_{:s}({:d}, {:s});", write_method_suffix, field_number as int, vv),
                ],
            };
            match field.repeat_mode {
                Single => {
                    w.match_block(w.self_field(), |w| {
                        w.case_block("Some(ref v)", |w| {
                            w.write_lines(write_value_lines);
                        });
                        w.case_expr("None", "{}");
                    });
                },
                RepeatPacked => {
                    w.if_self_field_is_not_empty(|w| {
                        w.write_line(format!("os.write_tag({:d}, ::protobuf::wire_format::{:?});", field_number as int, wire_format::WireTypeLengthDelimited));
                        let data_size_expr = w.self_field_vec_packed_data_size();
                        w.write_line(format!("os.write_raw_varint32({});", data_size_expr));
                        w.for_self_field("v", |w| {
                            w.write_line(format!("os.write_{:s}_no_tag({:s});", write_method_suffix, vv));
                        });
                    });
                },
                RepeatRegular => {
                    w.for_self_field("v", |w| {
                        w.write_lines(write_value_lines);
                    });
                },
            };
        });
        w.write_line("os.write_unknown_fields(self.get_unknown_fields());");
    });
}

fn write_message_default_instance(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    w.pub_fn(format!("default_instance() -> &'static {:s}", msg.type_name), |w| {
        fn write_body(w: &mut IndentWriter) {
            let msg = w.msg.get_ref();
            w.stmt_block(format!("static instance: {:s} = {:s}", msg.type_name, msg.type_name), |w| {
                w.fields(|w| {
                    w.field_default();
                });
                w.field_entry("unknown_fields", "None");
            });
            w.write_line("&'static instance");
        }
        if msg.has_any_repeated_field() {
            w.commented(|w| {
                w.comment("doesn't work, because rust doen't implement \
                        static constants of types like ~str");
                w.comment("https://github.com/mozilla/rust/issues/8406");
                write_body(w);
            });
            w.todo();
        } else {
            write_body(w)
        }
    });
}

fn write_message_field_accessors(w: &mut IndentWriter) {
    w.fields(|w| {
        w.write_line("");
        w.pub_fn(format!("{:s}(&mut self)", w.clear_field_func()), |w| {
            w.clear_field();
        });

        if !w.field().repeated {
            w.write_line("");
            w.pub_fn(format!("has_{:s}(&self) -> bool", w.field().name), |w| {
                w.write_line(w.self_field_is_some());
            });
        }

        let set_param_type = if w.field().repeated {
            w.field().full_type()
        } else {
            w.field().type_name.to_owned()
        };

        w.write_line("");
        w.comment("Param is passed by value, moved");
        w.pub_fn(format!("set_{:s}(&mut self, v: {:s})", w.field().name, set_param_type), |w| {
            if w.field().repeated {
                w.self_field_assign("v");
            } else {
                w.self_field_assign_some("v");
            }
        });

        w.write_line("");
        w.comment("Mutable pointer to the field.");
        if !w.field().repeated {
            w.comment("If field is not initialized, it is initialized with default value first.");
        }
        w.pub_fn(format!("mut_{:s}(&'a mut self) -> &'a mut {:s}", w.field().name, set_param_type),
        |w| {
            if !w.field().repeated {
                w.if_self_field_is_none(|w| {
                    w.self_field_assign_some(w.field_type_default());
                });
                w.write_line(format!("{:s}.get_mut_ref()", w.self_field()));
            } else {
                w.write_line(format!("&mut {:s}", w.self_field()));
            }
        });

        w.write_line("");
        let return_reference = w.field().repeated || match w.field().field_type {
            TYPE_MESSAGE | TYPE_STRING | TYPE_BYTES => true,
            _ => false,
        };
        let get_xxx_return_type = match w.field().repeated {
            true => format!("&'a [{:s}]", w.field().type_name),
            false => match return_reference {
                true => {
                    format!("&'a {:s}", match w.field().field_type {
                        TYPE_BYTES  => ~"[u8]",
                        TYPE_STRING => ~"str",
                        _ => set_param_type,
                    })
                }
                false => set_param_type.to_owned(),
            }
        };
        let self_param = match return_reference {
            true  => "&'a self",
            false => "&self",
        };
        w.pub_fn(format!("get_{:s}({:s}) -> {:s}", w.field().name, self_param, get_xxx_return_type),
        |w| {
            if !w.field().repeated {
                if return_reference {
                    w.match_expr(w.self_field(), |w| {
                        w.case_expr(
                            "Some(ref v)",
                            match w.field().field_type {
                                TYPE_STRING | TYPE_BYTES => "v.as_slice()",
                                _ => "v",
                            }
                        );
                        w.case_expr(
                            "None",
                            match w.field().field_type {
                                TYPE_MESSAGE => format!("{:s}::default_instance()", w.field().type_name),
                                TYPE_BYTES   => ~"&'a []",
                                TYPE_STRING  => ~"&'a \"\"",
                                _            => fail!(),
                            }
                        );
                    });
                } else {
                    w.write_line(format!(
                            "{:s}.unwrap_or_else(|| {:s})",
                            w.self_field(), w.field_type_default()));
                }
            } else {
                w.write_line(format!("{:s}.as_slice()", w.self_field()));
            }
        });

        if w.field().repeated {
            w.write_line("");
            w.pub_fn(format!("add_{:s}(&mut self, v: {:s})",
                    w.field().name, w.field().type_name),
            |w| {
                w.self_field_push("v");
            });
        }
    });
}

fn write_message_impl_self(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    w.impl_self_block(msg.type_name, |w| {
        w.pub_fn(format!("new() -> {:s}", msg.type_name), |w| {
            w.write_line("::std::default::Default::default()");
        });

        w.write_line("");
        write_message_default_instance(w);
        w.write_line("");
        write_message_write_to_with_computed_sizes(w);
        write_message_field_accessors(w);
    });
}

fn write_message_clear(w: &mut IndentWriter) {
    w.def_fn("clear(&mut self)", |w| {
        w.fields(|w| {
            w.write_line(format!("self.{:s}();", w.clear_field_func()));
        });
    });
}

fn write_message_unknown_fields(w: &mut IndentWriter) {
    w.def_fn("get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields", |w| {
        w.write_line("if self.unknown_fields.is_some() {");
        w.indented(|w| {
            w.write_line("&**self.unknown_fields.get_ref()");
        });
        w.write_line("} else {");
        w.indented(|w| {
            w.write_line("::protobuf::UnknownFields::default_instance()");
        });
        w.write_line("}");
    });
    w.write_line("");
    w.def_fn("mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields", |w| {
        w.write_line("if self.unknown_fields.is_none() {");
        w.indented(|w| {
            w.write_line("self.unknown_fields = Some(::std::default::Default::default())");
        });
        w.write_line("}");
        w.write_line("&mut **self.unknown_fields.get_mut_ref()");
    });
}

fn write_message_merge_from(w: &mut IndentWriter) {
    w.def_fn(format!("merge_from(&mut self, is: &mut ::protobuf::CodedInputStream)"), |w| {
        w.while_block("!is.eof()", |w| {
            w.write_line(format!("let (field_number, wire_type) = is.read_tag_unpack();"));
            w.match_block("field_number", |w| {
                w.fields(|w| {
                    w.case_block(w.field().number.to_str(), |w| {
                        write_merge_from_field(w);
                    });
                });
                w.case_block("_", |w| {
                    w.write_line("let unknown = is.read_unknown(wire_type);");
                    w.write_line("self.mut_unknown_fields().add_value(field_number, unknown);");
                });
            });
        });
    });
}

fn write_message_impl_message(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    w.impl_for_block("::protobuf::Message", msg.type_name, |w| {
        w.def_fn(format!("new() -> {:s}", msg.type_name), |w| {
            w.write_line(format!("{:s}::new()", msg.type_name));
        });
        w.write_line("");
        write_message_clear(w);
        w.write_line("");
        w.def_fn(format!("is_initialized(&self) -> bool"), |w| {
            w.required_fields(|w| {
                w.if_self_field_is_none(|w| {
                    w.write_line("return false;");
                });
            });
            w.write_line("true");
        });
        w.write_line("");
        write_message_merge_from(w);
        w.write_line("");
        write_message_compute_sizes(w);
        w.write_line("");
        w.def_fn("write_to(&self, os: &mut ::protobuf::CodedOutputStream)", |w| {
            w.write_line("self.check_initialized();");
            w.write_line("let mut sizes: Vec<u32> = Vec::new();");
            w.write_line("self.compute_sizes(&mut sizes);");
            w.write_line("let mut sizes_pos = 1; // first element is self");
            w.write_line("self.write_to_with_computed_sizes(os, sizes.as_slice(), &mut sizes_pos);");
            w.write_line("assert_eq!(sizes_pos, sizes.len());");
            w.comment("TODO: assert we've written same number of bytes as computed");
        });
        w.write_line("");
        write_message_unknown_fields(w);
    });
}

fn write_message(msg: &Message, w: &mut IndentWriter) {
    let pkg = msg.pkg.as_slice();
    let message_type = &msg.proto_message;

    w.bind_message(msg, |w| {
        write_message_struct(w);
        w.write_line("");
        write_message_impl_self(w);
        w.write_line("");
        write_message_impl_message(w);

        for nested_type in message_type.get_nested_type().iter() {
            w.write_line("");
            write_message(&Message::parse(nested_type, pkg, msg.type_name + "_"), w);
        }

        for enum_type in message_type.get_enum_type().iter() {
            w.write_line("");
            write_enum(msg.type_name + "_", w, enum_type);
        }
    });
}

fn write_enum(prefix: &str, w: &mut IndentWriter, enum_type: &EnumDescriptorProto) {
    let enum_type_name = prefix + enum_type.get_name().to_owned();
    w.deriving(["Clone", "Eq"]);
    w.write_line(format!("pub enum {:s} \\{", enum_type_name));
    for value in enum_type.get_value().iter() {
        w.write_line(format!("    {:s} = {:d},", value.get_name().to_owned(), value.get_number() as int));
    }
    w.write_line(format!("\\}"));
    w.write_line("");
    w.impl_block(enum_type_name, |w| {
        w.pub_fn(format!("new(value: i32) -> {:s}", enum_type_name), |w| {
            w.match_expr("value", |w| {
                for value in enum_type.get_value().iter() {
                    let value_number = value.get_number();
                    let value_name = value.get_name().to_owned();
                    w.write_line(format!("{:d} => {:s},", value_number as int, value_name));
                }
                w.write_line(format!("_ => fail!()"));
            });
        });
    });
    w.write_line("");
    w.impl_for_block("::protobuf::ProtobufEnum", enum_type_name, |w| {
        w.def_fn("value(&self) -> i32", |w| {
            w.write_line("*self as i32")
        });
    });
}

fn proto_path_to_rust_base<'s>(path: &'s str) -> &'s str {
    remove_suffix(remove_to(path, '/'), ".proto")
}

pub struct GenResult {
    pub name: ~str,
    pub content: Vec<u8>,
}

pub struct GenOptions {
    pub dummy: bool,
}

pub fn gen(files: &[FileDescriptorProto], _: &GenOptions) -> Vec<GenResult> {
    let mut results: Vec<GenResult> = Vec::new();
    for file in files.iter() {
        let base = proto_path_to_rust_base(file.get_name());

        let mut os = VecWriter::new();

        {
            let mut w = IndentWriter::new(&mut os as &mut Writer);

            w.write_line("// This file is generated. Do not edit");

            for dep in file.get_dependency().iter() {
                w.write_line(format!("use {:s}::*;", proto_path_to_rust_base(*dep)));
            }

            {
                w.write_line("");
                let fdp_bytes = file.write_to_bytes();
                w.write_line("static file_descriptor_proto_data: &'static [u8] = &[");
                for groups in fdp_bytes.iter().paginate(16) {
                    let fdp_bytes_str = groups.iter()
                            .map(|&b| format!("0x{:02x}", *b))
                            .collect::<~[~str]>()
                            .connect(", ");
                    w.write_line(format!("    {},", fdp_bytes_str));
                }
                w.write_line("];");
                w.write_line("");
                w.write_line("static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *::protobuf::descriptor::FileDescriptorProto };");
                w.write_line("");
                w.def_fn("parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto", |w| {
                    w.write_line("::protobuf::parse_from_bytes(file_descriptor_proto_data)");
                });
                w.write_line("");
                w.pub_fn("file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto", |w| {
                    w.unsafe_expr(|w| {
                        w.block("file_descriptor_proto_lazy.get(|| {", "})", |w| {
                            w.write_line("parse_descriptor_proto()");
                        });
                    });
                });
            }

            for message_type in file.get_message_type().iter() {
                w.write_line("");
                write_message(&Message::parse(message_type, file.get_package(), ""), &mut w);
            }
            for enum_type in file.get_enum_type().iter() {
                w.write_line("");
                write_enum("", &mut w, enum_type);
            }
        }

        results.push(GenResult {
            name: base + ".rs",
            content: os.vec,
        });
    }
    results
}


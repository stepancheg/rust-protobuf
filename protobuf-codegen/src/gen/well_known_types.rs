use protobuf_parse::ProtobufAbsPath;
use protobuf_parse::ProtobufRelPath;
use protobuf_parse::ProtobufRelPathRef;

use crate::compiler_plugin;
use crate::gen::code_writer::CodeWriter;
use crate::gen::paths::proto_path_to_rust_mod;

pub(crate) static WELL_KNOWN_TYPES_PROTO_FILE_NAMES: &[&str] = &[
    "any.proto",
    "api.proto",
    "duration.proto",
    "empty.proto",
    "field_mask.proto",
    "source_context.proto",
    "struct.proto",
    "timestamp.proto",
    "type.proto",
    "wrappers.proto",
];

pub(crate) static WELL_KNOWN_TYPES_PROTO_FILE_FULL_NAMES: &[&str] = &[
    "google/protobuf/any.proto",
    "google/protobuf/api.proto",
    "google/protobuf/duration.proto",
    "google/protobuf/empty.proto",
    "google/protobuf/field_mask.proto",
    "google/protobuf/source_context.proto",
    "google/protobuf/struct.proto",
    "google/protobuf/timestamp.proto",
    "google/protobuf/type.proto",
    "google/protobuf/wrappers.proto",
];

static NAMES: &'static [&'static str] = &[
    "Any",
    "Api",
    "BoolValue",
    "BytesValue",
    "DoubleValue",
    "Duration",
    "Empty",
    "Enum",
    "EnumValue",
    "Field",
    "Field.Cardinality",
    "Field.Kind",
    "FieldMask",
    "FloatValue",
    "Int32Value",
    "Int64Value",
    "ListValue",
    "Method",
    "Mixin",
    "NullValue",
    "Option",
    "SourceContext",
    "StringValue",
    "Struct",
    "Syntax",
    "Timestamp",
    "Type",
    "UInt32Value",
    "UInt64Value",
    "Value",
];

fn is_well_known_type(name: &ProtobufRelPathRef) -> bool {
    NAMES.iter().any(|&n| n == format!("{}", name))
}

pub(crate) fn is_well_known_type_full(name: &ProtobufAbsPath) -> Option<ProtobufRelPath> {
    if let Some(rem) = name.remove_prefix(&ProtobufAbsPath::from(".google.protobuf")) {
        if is_well_known_type(rem) {
            Some(rem.to_owned())
        } else {
            None
        }
    } else {
        None
    }
}

pub(crate) fn gen_well_known_types_mod() -> compiler_plugin::GenResult {
    let v = CodeWriter::with_no_error(|w| {
        w.comment("This file is generated. Do not edit");
        w.comment("@generated");
        w.mod_doc("Generated code for \"well known types\"");
        w.mod_doc("");
        w.mod_doc("[This document](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf) describes these types.");

        w.write_line("");
        w.write_line("#![allow(unused_attributes)]");
        w.write_line("#![cfg_attr(rustfmt, rustfmt::skip)]");

        w.write_line("");
        for m in WELL_KNOWN_TYPES_PROTO_FILE_NAMES {
            w.write_line(&format!("pub mod {};", proto_path_to_rust_mod(m)));
        }
    });

    compiler_plugin::GenResult {
        name: "well_known_types_mod.rs".to_string(),
        content: v.into_bytes(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_well_known_type_full() {
        assert_eq!(
            Some(ProtobufRelPath::from("BoolValue")),
            is_well_known_type_full(&ProtobufAbsPath::from(".google.protobuf.BoolValue"))
        );
        assert_eq!(
            None,
            is_well_known_type_full(&ProtobufAbsPath::from(".google.protobuf.Fgfg"))
        );
    }
}

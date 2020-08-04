use crate::protobuf_abs_path::ProtobufAbsolutePath;
use crate::protobuf_rel_path::ProtobufRelativePath;
use crate::compiler_plugin;
use protobuf::descriptor::FileDescriptorProto;
use crate::code_writer::CodeWriter;
use crate::file::proto_path_to_rust_mod;
use crate::scope::{FileScope, WithScope};

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
    // TODO: dotted names
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

fn is_well_known_type(name: &ProtobufRelativePath) -> bool {
    NAMES.iter().any(|&n| n == name.path)
}

pub fn is_well_known_type_full(name: &ProtobufAbsolutePath) -> Option<ProtobufRelativePath> {
    if let Some(ref rem) = name.remove_prefix(&ProtobufAbsolutePath::from(".google.protobuf")) {
        if is_well_known_type(rem) {
            Some(rem.clone())
        } else {
            None
        }
    } else {
        None
    }
}

pub(crate) fn gen_well_known_types_mod(
    file_descriptors: &[FileDescriptorProto],
) -> compiler_plugin::GenResult {
    let mut v = Vec::new();

    {
        let mut w = CodeWriter::new(&mut v);
        w.comment("This file is generated. Do not edit");
        w.comment("@generated");
        w.mod_doc("Generated code for \"well known types\"");
        w.mod_doc("");
        w.mod_doc("[This document](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf) describes these types.");

        w.write_line("");
        for m in WELL_KNOWN_TYPES_PROTO_FILE_NAMES {
            w.write_line(&format!("mod {};", proto_path_to_rust_mod(m)));
        }

        w.write_line("");
        for p in WELL_KNOWN_TYPES_PROTO_FILE_NAMES {
            let file_descriptor = match file_descriptors
                .iter()
                .find(|f| f.get_name() == &format!("google/protobuf/{}", p))
            {
                Some(f) => f,
                None => panic!(
                    "file descriptor not found for {}, all names: {}",
                    p,
                    file_descriptors
                        .iter()
                        .map(|f| f.get_name().to_owned())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            };

            let rust_mod = proto_path_to_rust_mod(p);

            let file_scope = FileScope { file_descriptor };

            for m in file_scope.to_scope().get_messages() {
                w.write_line(&format!("pub use self::{}::{};", rust_mod, m.rust_name()));
                if m.need_mod() {
                    // Does not correctly export oneofs,
                    // but that's not an issue for well_known_types
                    w.write_line(&format!("pub use self::{}::{};", rust_mod, m.mod_name()));
                }
            }
            for e in file_scope.to_scope().get_enums() {
                w.write_line(&format!("pub use self::{}::{};", rust_mod, e.rust_name()));
            }
        }
    }

    compiler_plugin::GenResult {
        name: "well_known_types_mod.rs".to_string(),
        content: v,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_well_known_type_full() {
        assert_eq!(
            Some(ProtobufRelativePath::from("BoolValue")),
            is_well_known_type_full(&ProtobufAbsolutePath::from(".google.protobuf.BoolValue"))
        );
        assert_eq!(
            None,
            is_well_known_type_full(&ProtobufAbsolutePath::from(".google.protobuf.Fgfg"))
        );
    }
}


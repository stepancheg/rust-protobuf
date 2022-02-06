use protobuf::reflect::EnumDescriptor;
use protobuf::reflect::EnumValueDescriptor;
use protobuf::reflect::FieldDescriptor;
use protobuf::reflect::MessageDescriptor;
use protobuf::reflect::OneofDescriptor;

use crate::gen::code_writer::CodeWriter;

/// Write `// @protoc_insertion_point(...)` before the element.
///
/// This is similar to what `protoc` codegen does for C++ or Java.
/// This can be used to modify the generated code.
pub(crate) fn write_protoc_insertion_point(w: &mut CodeWriter, arg: &str) {
    w.comment(&format!("@@protoc_insertion_point({})", arg));
}

pub(crate) fn write_protoc_insertion_point_for_message(
    w: &mut CodeWriter,
    message: &MessageDescriptor,
) {
    write_protoc_insertion_point(w, &format!("message:{}", message.full_name()));
}

pub(crate) fn write_protoc_insertion_point_for_field(w: &mut CodeWriter, field: &FieldDescriptor) {
    write_protoc_insertion_point(w, &format!("field:{}", field.full_name()));
}

pub(crate) fn write_protoc_insertion_point_for_special_field(
    w: &mut CodeWriter,
    message: &MessageDescriptor,
    field: &str,
) {
    write_protoc_insertion_point(
        w,
        &format!("special_field:{}.{}", message.full_name(), field),
    );
}

pub(crate) fn write_protoc_insertion_point_for_enum(
    w: &mut CodeWriter,
    enumeration: &EnumDescriptor,
) {
    write_protoc_insertion_point(w, &format!("enum:{}", enumeration.full_name()));
}

pub(crate) fn write_protoc_insertion_point_for_enum_value(
    w: &mut CodeWriter,
    value: &EnumValueDescriptor,
) {
    write_protoc_insertion_point(w, &format!("enum_value:{}", value.full_name()));
}

pub(crate) fn write_protoc_insertion_point_for_oneof(w: &mut CodeWriter, oneof: &OneofDescriptor) {
    write_protoc_insertion_point(w, &format!("oneof:{}", oneof.full_name()));
}

pub(crate) fn write_protoc_insertion_point_for_oneof_field(
    w: &mut CodeWriter,
    field: &FieldDescriptor,
) {
    write_protoc_insertion_point(w, &format!("oneof_field:{}", field.full_name()));
}

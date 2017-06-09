// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

pub mod exts {
    use protobuf::Message as Message_imported_for_functions;

    pub const expose_oneof_all: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17001, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes_all: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_all: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const expose_oneof: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17001, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes_field: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_field: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0frustproto.proto\x12\trustproto\x1a\x20google/protobuf/descriptor.p\
    roto:H\n\x10expose_oneof_all\x18\xe9\x84\x01\x20\x01(\x08\x12\x1c.google\
    .protobuf.FileOptionsR\x0eexposeOneofAll:b\n\x1ecarllerche_bytes_for_byt\
    es_all\x18\xf3\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\
    \x1acarllercheBytesForBytesAll:d\n\x1fcarllerche_bytes_for_string_all\
    \x18\xf4\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x1bca\
    rllercheBytesForStringAll:D\n\x0cexpose_oneof\x18\xe9\x84\x01\x20\x01(\
    \x08\x12\x1f.google.protobuf.MessageOptionsR\x0bexposeOneof:^\n\x1acarll\
    erche_bytes_for_bytes\x18\xf3\x84\x01\x20\x01(\x08\x12\x1f.google.protob\
    uf.MessageOptionsR\x17carllercheBytesForBytes:`\n\x1bcarllerche_bytes_fo\
    r_string\x18\xf4\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOpt\
    ionsR\x18carllercheBytesForString:g\n\x20carllerche_bytes_for_bytes_fiel\
    d\x18\xf3\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x1c\
    carllercheBytesForBytesField:i\n!carllerche_bytes_for_string_field\x18\
    \xf4\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x1dcarll\
    ercheBytesForStringFieldJ\xb5\x08\n\x06\x12\x04\0\0\x20\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\x07)\nh\n\x01\x02\x12\
    \x03\x07\x08\x112^\x20see\x20https://github.com/gogo/protobuf/blob/maste\
    r/gogoproto/gogo.proto\n\x20for\x20the\x20original\x20idea\n\n\t\n\x01\
    \x07\x12\x04\t\0\x10\x01\n7\n\x02\x07\0\x12\x03\x0b\x04+\x1a,\x20When\
    \x20true,\x20oneof\x20field\x20is\x20generated\x20public\n\n\n\n\x03\x07\
    \0\x02\x12\x03\t\x07\"\n\n\n\x03\x07\0\x04\x12\x03\x0b\x04\x0c\n\n\n\x03\
    \x07\0\x05\x12\x03\x0b\r\x11\n\n\n\x03\x07\0\x01\x12\x03\x0b\x12\"\n\n\n\
    \x03\x07\0\x03\x12\x03\x0b%*\n2\n\x02\x07\x01\x12\x03\r\x049\x1a'\x20Use\
    \x20`bytes::Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\x03\x07\x01\x02\
    \x12\x03\t\x07\"\n\n\n\x03\x07\x01\x04\x12\x03\r\x04\x0c\n\n\n\x03\x07\
    \x01\x05\x12\x03\r\r\x11\n\n\n\x03\x07\x01\x01\x12\x03\r\x120\n\n\n\x03\
    \x07\x01\x03\x12\x03\r38\n3\n\x02\x07\x02\x12\x03\x0f\x04:\x1a(\x20Use\
    \x20`bytes::Bytes`\x20for\x20`string`\x20fields\n\n\n\n\x03\x07\x02\x02\
    \x12\x03\t\x07\"\n\n\n\x03\x07\x02\x04\x12\x03\x0f\x04\x0c\n\n\n\x03\x07\
    \x02\x05\x12\x03\x0f\r\x11\n\n\n\x03\x07\x02\x01\x12\x03\x0f\x121\n\n\n\
    \x03\x07\x02\x03\x12\x03\x0f49\n\t\n\x01\x07\x12\x04\x12\0\x19\x01\n7\n\
    \x02\x07\x03\x12\x03\x14\x04'\x1a,\x20When\x20true,\x20oneof\x20field\
    \x20is\x20generated\x20public\n\n\n\n\x03\x07\x03\x02\x12\x03\x12\x07%\n\
    \n\n\x03\x07\x03\x04\x12\x03\x14\x04\x0c\n\n\n\x03\x07\x03\x05\x12\x03\
    \x14\r\x11\n\n\n\x03\x07\x03\x01\x12\x03\x14\x12\x1e\n\n\n\x03\x07\x03\
    \x03\x12\x03\x14!&\n2\n\x02\x07\x04\x12\x03\x16\x045\x1a'\x20Use\x20`byt\
    es::Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\x03\x07\x04\x02\x12\x03\
    \x12\x07%\n\n\n\x03\x07\x04\x04\x12\x03\x16\x04\x0c\n\n\n\x03\x07\x04\
    \x05\x12\x03\x16\r\x11\n\n\n\x03\x07\x04\x01\x12\x03\x16\x12,\n\n\n\x03\
    \x07\x04\x03\x12\x03\x16/4\n3\n\x02\x07\x05\x12\x03\x18\x046\x1a(\x20Use\
    \x20`bytes::Bytes`\x20for\x20`string`\x20fields\n\n\n\n\x03\x07\x05\x02\
    \x12\x03\x12\x07%\n\n\n\x03\x07\x05\x04\x12\x03\x18\x04\x0c\n\n\n\x03\
    \x07\x05\x05\x12\x03\x18\r\x11\n\n\n\x03\x07\x05\x01\x12\x03\x18\x12-\n\
    \n\n\x03\x07\x05\x03\x12\x03\x1805\n\t\n\x01\x07\x12\x04\x1b\0\x20\x01\n\
    2\n\x02\x07\x06\x12\x03\x1d\x04;\x1a'\x20Use\x20`bytes::Bytes`\x20for\
    \x20`bytes`\x20fields\n\n\n\n\x03\x07\x06\x02\x12\x03\x1b\x07#\n\n\n\x03\
    \x07\x06\x04\x12\x03\x1d\x04\x0c\n\n\n\x03\x07\x06\x05\x12\x03\x1d\r\x11\
    \n\n\n\x03\x07\x06\x01\x12\x03\x1d\x122\n\n\n\x03\x07\x06\x03\x12\x03\
    \x1d5:\n3\n\x02\x07\x07\x12\x03\x1f\x04<\x1a(\x20Use\x20`bytes::Bytes`\
    \x20for\x20`string`\x20fields\n\n\n\n\x03\x07\x07\x02\x12\x03\x1b\x07#\n\
    \n\n\x03\x07\x07\x04\x12\x03\x1f\x04\x0c\n\n\n\x03\x07\x07\x05\x12\x03\
    \x1f\r\x11\n\n\n\x03\x07\x07\x01\x12\x03\x1f\x123\n\n\n\x03\x07\x07\x03\
    \x12\x03\x1f6;\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}

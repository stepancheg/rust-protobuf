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

    pub const expose_fields_all: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors_all: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes_all: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_all: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const expose_oneof: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17001, phantom: ::std::marker::PhantomData };

    pub const expose_fields: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const expose_fields_field: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors_field: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes_field: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_field: ::protobuf::ext::ExtFieldOptional<super::super::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0frustproto.proto\x12\trustproto\x1a\x20google/protobuf/descriptor.p\
    roto:H\n\x10expose_oneof_all\x18\xe9\x84\x01\x20\x01(\x08\x12\x1c.google\
    .protobuf.FileOptionsR\x0eexposeOneofAll:J\n\x11expose_fields_all\x18\
    \xeb\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x0fexpose\
    FieldsAll:T\n\x16generate_accessors_all\x18\xec\x84\x01\x20\x01(\x08\x12\
    \x1c.google.protobuf.FileOptionsR\x14generateAccessorsAll:b\n\x1ecarller\
    che_bytes_for_bytes_all\x18\xf3\x84\x01\x20\x01(\x08\x12\x1c.google.prot\
    obuf.FileOptionsR\x1acarllercheBytesForBytesAll:d\n\x1fcarllerche_bytes_\
    for_string_all\x18\xf4\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.File\
    OptionsR\x1bcarllercheBytesForStringAll:D\n\x0cexpose_oneof\x18\xe9\x84\
    \x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x0bexposeOneof\
    :F\n\rexpose_fields\x18\xeb\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf\
    .MessageOptionsR\x0cexposeFields:P\n\x12generate_accessors\x18\xec\x84\
    \x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x11generateAcc\
    essors:^\n\x1acarllerche_bytes_for_bytes\x18\xf3\x84\x01\x20\x01(\x08\
    \x12\x1f.google.protobuf.MessageOptionsR\x17carllercheBytesForBytes:`\n\
    \x1bcarllerche_bytes_for_string\x18\xf4\x84\x01\x20\x01(\x08\x12\x1f.goo\
    gle.protobuf.MessageOptionsR\x18carllercheBytesForString:O\n\x13expose_f\
    ields_field\x18\xeb\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOp\
    tionsR\x11exposeFieldsField:Y\n\x18generate_accessors_field\x18\xec\x84\
    \x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x16generateAcces\
    sorsField:g\n\x20carllerche_bytes_for_bytes_field\x18\xf3\x84\x01\x20\
    \x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x1ccarllercheBytesForByt\
    esField:i\n!carllerche_bytes_for_string_field\x18\xf4\x84\x01\x20\x01(\
    \x08\x12\x1d.google.protobuf.FieldOptionsR\x1dcarllercheBytesForStringFi\
    eldJ\xf4\x0e\n\x06\x12\x04\0\0,\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\
    \n\x02\x03\0\x12\x03\x02\x07)\nh\n\x01\x02\x12\x03\x07\x08\x112^\x20see\
    \x20https://github.com/gogo/protobuf/blob/master/gogoproto/gogo.proto\n\
    \x20for\x20the\x20original\x20idea\n\n\t\n\x01\x07\x12\x04\t\0\x14\x01\n\
    7\n\x02\x07\0\x12\x03\x0b\x04+\x1a,\x20When\x20true,\x20oneof\x20field\
    \x20is\x20generated\x20public\n\n\n\n\x03\x07\0\x02\x12\x03\t\x07\"\n\n\
    \n\x03\x07\0\x04\x12\x03\x0b\x04\x0c\n\n\n\x03\x07\0\x05\x12\x03\x0b\r\
    \x11\n\n\n\x03\x07\0\x01\x12\x03\x0b\x12\"\n\n\n\x03\x07\0\x03\x12\x03\
    \x0b%*\nI\n\x02\x07\x01\x12\x03\r\x04,\x1a>\x20When\x20true\x20all\x20fi\
    elds\x20are\x20public,\x20and\x20not\x20accessors\x20generated\n\n\n\n\
    \x03\x07\x01\x02\x12\x03\t\x07\"\n\n\n\x03\x07\x01\x04\x12\x03\r\x04\x0c\
    \n\n\n\x03\x07\x01\x05\x12\x03\r\r\x11\n\n\n\x03\x07\x01\x01\x12\x03\r\
    \x12#\n\n\n\x03\x07\x01\x03\x12\x03\r&+\nP\n\x02\x07\x02\x12\x03\x0f\x04\
    1\x1aE\x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20acce\
    ssors\x20are\x20not\x20generated\n\n\n\n\x03\x07\x02\x02\x12\x03\t\x07\"\
    \n\n\n\x03\x07\x02\x04\x12\x03\x0f\x04\x0c\n\n\n\x03\x07\x02\x05\x12\x03\
    \x0f\r\x11\n\n\n\x03\x07\x02\x01\x12\x03\x0f\x12(\n\n\n\x03\x07\x02\x03\
    \x12\x03\x0f+0\n2\n\x02\x07\x03\x12\x03\x11\x049\x1a'\x20Use\x20`bytes::\
    Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\x03\x07\x03\x02\x12\x03\t\x07\
    \"\n\n\n\x03\x07\x03\x04\x12\x03\x11\x04\x0c\n\n\n\x03\x07\x03\x05\x12\
    \x03\x11\r\x11\n\n\n\x03\x07\x03\x01\x12\x03\x11\x120\n\n\n\x03\x07\x03\
    \x03\x12\x03\x1138\n3\n\x02\x07\x04\x12\x03\x13\x04:\x1a(\x20Use\x20`byt\
    es::Bytes`\x20for\x20`string`\x20fields\n\n\n\n\x03\x07\x04\x02\x12\x03\
    \t\x07\"\n\n\n\x03\x07\x04\x04\x12\x03\x13\x04\x0c\n\n\n\x03\x07\x04\x05\
    \x12\x03\x13\r\x11\n\n\n\x03\x07\x04\x01\x12\x03\x13\x121\n\n\n\x03\x07\
    \x04\x03\x12\x03\x1349\n\t\n\x01\x07\x12\x04\x16\0!\x01\n7\n\x02\x07\x05\
    \x12\x03\x18\x04'\x1a,\x20When\x20true,\x20oneof\x20field\x20is\x20gener\
    ated\x20public\n\n\n\n\x03\x07\x05\x02\x12\x03\x16\x07%\n\n\n\x03\x07\
    \x05\x04\x12\x03\x18\x04\x0c\n\n\n\x03\x07\x05\x05\x12\x03\x18\r\x11\n\n\
    \n\x03\x07\x05\x01\x12\x03\x18\x12\x1e\n\n\n\x03\x07\x05\x03\x12\x03\x18\
    !&\nI\n\x02\x07\x06\x12\x03\x1a\x04(\x1a>\x20When\x20true\x20all\x20fiel\
    ds\x20are\x20public,\x20and\x20not\x20accessors\x20generated\n\n\n\n\x03\
    \x07\x06\x02\x12\x03\x16\x07%\n\n\n\x03\x07\x06\x04\x12\x03\x1a\x04\x0c\
    \n\n\n\x03\x07\x06\x05\x12\x03\x1a\r\x11\n\n\n\x03\x07\x06\x01\x12\x03\
    \x1a\x12\x1f\n\n\n\x03\x07\x06\x03\x12\x03\x1a\"'\nP\n\x02\x07\x07\x12\
    \x03\x1c\x04-\x1aE\x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20\
    etc.\x20accessors\x20are\x20not\x20generated\n\n\n\n\x03\x07\x07\x02\x12\
    \x03\x16\x07%\n\n\n\x03\x07\x07\x04\x12\x03\x1c\x04\x0c\n\n\n\x03\x07\
    \x07\x05\x12\x03\x1c\r\x11\n\n\n\x03\x07\x07\x01\x12\x03\x1c\x12$\n\n\n\
    \x03\x07\x07\x03\x12\x03\x1c',\n2\n\x02\x07\x08\x12\x03\x1e\x045\x1a'\
    \x20Use\x20`bytes::Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\x03\x07\
    \x08\x02\x12\x03\x16\x07%\n\n\n\x03\x07\x08\x04\x12\x03\x1e\x04\x0c\n\n\
    \n\x03\x07\x08\x05\x12\x03\x1e\r\x11\n\n\n\x03\x07\x08\x01\x12\x03\x1e\
    \x12,\n\n\n\x03\x07\x08\x03\x12\x03\x1e/4\n3\n\x02\x07\t\x12\x03\x20\x04\
    6\x1a(\x20Use\x20`bytes::Bytes`\x20for\x20`string`\x20fields\n\n\n\n\x03\
    \x07\t\x02\x12\x03\x16\x07%\n\n\n\x03\x07\t\x04\x12\x03\x20\x04\x0c\n\n\
    \n\x03\x07\t\x05\x12\x03\x20\r\x11\n\n\n\x03\x07\t\x01\x12\x03\x20\x12-\
    \n\n\n\x03\x07\t\x03\x12\x03\x2005\n\t\n\x01\x07\x12\x04#\0,\x01\nI\n\
    \x02\x07\n\x12\x03%\x04.\x1a>\x20When\x20true\x20all\x20fields\x20are\
    \x20public,\x20and\x20not\x20accessors\x20generated\n\n\n\n\x03\x07\n\
    \x02\x12\x03#\x07#\n\n\n\x03\x07\n\x04\x12\x03%\x04\x0c\n\n\n\x03\x07\n\
    \x05\x12\x03%\r\x11\n\n\n\x03\x07\n\x01\x12\x03%\x12%\n\n\n\x03\x07\n\
    \x03\x12\x03%(-\nP\n\x02\x07\x0b\x12\x03'\x043\x1aE\x20When\x20false,\
    \x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20accessors\x20are\x20not\x20g\
    enerated\n\n\n\n\x03\x07\x0b\x02\x12\x03#\x07#\n\n\n\x03\x07\x0b\x04\x12\
    \x03'\x04\x0c\n\n\n\x03\x07\x0b\x05\x12\x03'\r\x11\n\n\n\x03\x07\x0b\x01\
    \x12\x03'\x12*\n\n\n\x03\x07\x0b\x03\x12\x03'-2\n2\n\x02\x07\x0c\x12\x03\
    )\x04;\x1a'\x20Use\x20`bytes::Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\
    \x03\x07\x0c\x02\x12\x03#\x07#\n\n\n\x03\x07\x0c\x04\x12\x03)\x04\x0c\n\
    \n\n\x03\x07\x0c\x05\x12\x03)\r\x11\n\n\n\x03\x07\x0c\x01\x12\x03)\x122\
    \n\n\n\x03\x07\x0c\x03\x12\x03)5:\n3\n\x02\x07\r\x12\x03+\x04<\x1a(\x20U\
    se\x20`bytes::Bytes`\x20for\x20`string`\x20fields\n\n\n\n\x03\x07\r\x02\
    \x12\x03#\x07#\n\n\n\x03\x07\r\x04\x12\x03+\x04\x0c\n\n\n\x03\x07\r\x05\
    \x12\x03+\r\x11\n\n\n\x03\x07\r\x01\x12\x03+\x123\n\n\n\x03\x07\r\x03\
    \x12\x03+6;\
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

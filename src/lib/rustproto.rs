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

    pub const expose_oneof: ::protobuf::ext::ExtFieldRepeated<super::super::descriptor::FileOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldRepeated { field_number: 17001, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0frustproto.proto\x12\trustproto\x1a\x20google/protobuf/descriptor.p\
    roto:A\n\x0cexpose_oneof\x18\xe9\x84\x01\x20\x01(\x08\x12\x1c.google.pro\
    tobuf.FileOptionsR\x0bexposeOneofJy\n\x06\x12\x04\0\0\x08\x01\n\x08\n\
    \x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\x07)\n\x08\n\x01\
    \x02\x12\x03\x04\x08\x11\n\t\n\x01\x07\x12\x04\x06\0\x08\x01\n\t\n\x02\
    \x07\0\x12\x03\x07\x04'\n\n\n\x03\x07\0\x02\x12\x03\x06\x07\"\n\n\n\x03\
    \x07\0\x04\x12\x03\x07\x04\x0c\n\n\n\x03\x07\0\x05\x12\x03\x07\r\x11\n\n\
    \n\x03\x07\0\x01\x12\x03\x07\x12\x1e\n\n\n\x03\x07\0\x03\x12\x03\x07!&\
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

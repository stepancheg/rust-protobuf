// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]


static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x23, 0x73, 0x72, 0x63, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x74, 0x65, 0x73, 0x74,
    0x5f, 0x6e, 0x6f, 0x6e, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x5f, 0x65, 0x6e, 0x75, 0x6d, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x1c, 0x0a, 0x08, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x41, 0x22, 0x10, 0x0a, 0x05, 0x45, 0x6e, 0x75, 0x6d, 0x41, 0x12, 0x07, 0x0a, 0x03, 0x46, 0x4f,
    0x4f, 0x10, 0x00, 0x22, 0x1c, 0x0a, 0x08, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x42, 0x22,
    0x10, 0x0a, 0x05, 0x45, 0x6e, 0x75, 0x6d, 0x42, 0x12, 0x07, 0x0a, 0x03, 0x46, 0x4f, 0x4f, 0x10,
    0x00, 0x4a, 0xcc, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x00, 0x00, 0x02, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x00, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x03, 0x01, 0x04,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x01, 0x09, 0x0e, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x01, 0x11, 0x19, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x01, 0x11, 0x14, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x01, 0x17, 0x18, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x03, 0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x03, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x04, 0x00, 0x12, 0x03,
    0x04, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x09,
    0x0e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x11, 0x19,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x11, 0x14,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x04, 0x17, 0x18,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto };

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data)
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct MessageA {
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> MessageA {
    pub fn new() -> MessageA {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MessageA {
        static mut instance: ::protobuf::lazy::Lazy<MessageA> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const MessageA };
        unsafe {
            instance.get(|| {
                MessageA {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }
}

impl ::protobuf::Message for MessageA {
    fn new() -> MessageA {
        MessageA::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                _ => {
                    let unknown = is.read_unknown(wire_type);
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        *sizes.get_mut(pos) = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variable)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        use protobuf::{Message};
        os.write_unknown_fields(self.get_unknown_fields());
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: Option<MessageA>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: Vec<&'static ::protobuf::reflect::FieldAccessor<MessageA>> = Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<MessageA>(
                    "MessageA",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<MessageA>()
    }
}

impl ::protobuf::Clear for MessageA {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for MessageA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[deriving(Clone,PartialEq,Eq,Show)]
pub enum MessageA_EnumA {
    MessageA_FOO = 0,
}

impl MessageA_EnumA {
    pub fn new(value: i32) -> MessageA_EnumA {
        match value {
            0 => MessageA_FOO,
            _ => fail!()
        }
    }
}

impl ::protobuf::ProtobufEnum for MessageA_EnumA {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn enum_descriptor_static(_: Option<MessageA_EnumA>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::EnumDescriptor };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MessageA_EnumA", file_descriptor_proto())
            })
        }
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct MessageB {
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> MessageB {
    pub fn new() -> MessageB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MessageB {
        static mut instance: ::protobuf::lazy::Lazy<MessageB> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const MessageB };
        unsafe {
            instance.get(|| {
                MessageB {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }
}

impl ::protobuf::Message for MessageB {
    fn new() -> MessageB {
        MessageB::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                _ => {
                    let unknown = is.read_unknown(wire_type);
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        *sizes.get_mut(pos) = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variable)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        use protobuf::{Message};
        os.write_unknown_fields(self.get_unknown_fields());
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: Option<MessageB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: Vec<&'static ::protobuf::reflect::FieldAccessor<MessageB>> = Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<MessageB>(
                    "MessageB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<MessageB>()
    }
}

impl ::protobuf::Clear for MessageB {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for MessageB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[deriving(Clone,PartialEq,Eq,Show)]
pub enum MessageB_EnumB {
    MessageB_FOO = 0,
}

impl MessageB_EnumB {
    pub fn new(value: i32) -> MessageB_EnumB {
        match value {
            0 => MessageB_FOO,
            _ => fail!()
        }
    }
}

impl ::protobuf::ProtobufEnum for MessageB_EnumB {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn enum_descriptor_static(_: Option<MessageB_EnumB>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::EnumDescriptor };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MessageB_EnumB", file_descriptor_proto())
            })
        }
    }
}

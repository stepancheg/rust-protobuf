// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]


#[deriving(Clone,PartialEq,Default)]
pub struct Root {
    nested: ::protobuf::RepeatedField<Root_Nested>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Root {
    pub fn new() -> Root {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Root {
        static mut instance: ::protobuf::lazy::Lazy<Root> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const Root };
        unsafe {
            instance.get(|| {
                Root {
                    nested: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // repeated .Root.Nested nested = 1;

    pub fn clear_nested(&mut self) {
        self.nested.clear();
    }

    // Param is passed by value, moved
    pub fn set_nested(&mut self, v: ::protobuf::RepeatedField<Root_Nested>) {
        self.nested = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nested(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Root_Nested> {
        &mut self.nested
    }

    pub fn get_nested(&'a self) -> &'a [Root_Nested] {
        self.nested.as_slice()
    }
}

impl ::protobuf::Message for Root {
    fn new() -> Root {
        Root::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.nested.push_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.nested.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        for v in self.nested.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Root>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Root>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Root_nested_acc as &'static ::protobuf::reflect::FieldAccessor<Root>) });
                ::protobuf::reflect::MessageDescriptor::new::<Root>(
                    "Root",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Root>()
    }
}

impl ::protobuf::Clear for Root {
    fn clear(&mut self) {
        self.clear_nested();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Root {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Root_nested_acc_type;
static Root_nested_acc: Root_nested_acc_type = Root_nested_acc_type;

impl ::protobuf::reflect::FieldAccessor<Root> for Root_nested_acc_type {
    fn name(&self) -> &'static str {
        "nested"
    }

    fn len_field(&self, m: &Root) -> uint {
        m.get_nested().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a Root, index: uint) -> &'a ::protobuf::Message {
        &m.get_nested()[index] as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Root_Nested {
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Root_Nested {
    pub fn new() -> Root_Nested {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Root_Nested {
        static mut instance: ::protobuf::lazy::Lazy<Root_Nested> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const Root_Nested };
        unsafe {
            instance.get(|| {
                Root_Nested {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }
}

impl ::protobuf::Message for Root_Nested {
    fn new() -> Root_Nested {
        Root_Nested::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Root_Nested>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Root_Nested>> = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Root_Nested>(
                    "Root_Nested",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Root_Nested>()
    }
}

impl ::protobuf::Clear for Root_Nested {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Root_Nested {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x15, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x72, 0x6f, 0x6f,
    0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x2e, 0x0a, 0x04, 0x52, 0x6f, 0x6f, 0x74, 0x12,
    0x1c, 0x0a, 0x06, 0x6e, 0x65, 0x73, 0x74, 0x65, 0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x0c, 0x2e, 0x52, 0x6f, 0x6f, 0x74, 0x2e, 0x4e, 0x65, 0x73, 0x74, 0x65, 0x64, 0x1a, 0x08, 0x0a,
    0x06, 0x4e, 0x65, 0x73, 0x74, 0x65, 0x64, 0x4a, 0xaf, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x02, 0x00,
    0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x08, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00,
    0x03, 0x00, 0x12, 0x04, 0x03, 0x04, 0x04, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00,
    0x01, 0x12, 0x03, 0x03, 0x0c, 0x12, 0x0a, 0x39, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x07, 0x04, 0x25, 0x1a, 0x2c, 0x20, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x69, 0x6e, 0x67, 0x20,
    0x6e, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69,
    0x6e, 0x20, 0x72, 0x6f, 0x6f, 0x74, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x07, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x07, 0x0d, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x1a, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x23, 0x24,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto };

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

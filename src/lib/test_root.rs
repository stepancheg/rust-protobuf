// This file is generated. Do not edit

#![allow(dead_code)]


static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x15, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x72, 0x6f, 0x6f,
    0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x2e, 0x0a, 0x04, 0x52, 0x6f, 0x6f, 0x74, 0x12,
    0x1c, 0x0a, 0x06, 0x6e, 0x65, 0x73, 0x74, 0x65, 0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x0c, 0x2e, 0x52, 0x6f, 0x6f, 0x74, 0x2e, 0x4e, 0x65, 0x73, 0x74, 0x65, 0x64, 0x1a, 0x08, 0x0a,
    0x06, 0x4e, 0x65, 0x73, 0x74, 0x65, 0x64,
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

    pub fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        use protobuf::{Message};
        for v in self.nested.iter() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos);
        };
        os.write_unknown_fields(self.get_unknown_fields());
    }

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

    pub fn add_nested(&mut self, v: Root_Nested) {
        self.nested.push(v);
    }
}

impl ::protobuf::Message for Root {
    fn new() -> Root {
        Root::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(::protobuf::wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = self.nested.push_default();
                    is.merge_message(tmp)
                },
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
        for value in self.nested.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        *sizes.get_mut(pos) = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut ::protobuf::CodedOutputStream) {
        self.check_initialized();
        let mut sizes: Vec<u32> = Vec::new();
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes.as_slice(), &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
        // TODO: assert we've written same number of bytes as computed
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: Option<Root>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: Vec<&'static ::protobuf::reflect::FieldAccessor<Root>> = Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&'static Root_nested_acc as &::protobuf::reflect::FieldAccessor<Root>) });
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
    }
}

impl ::std::fmt::Show for Root {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Root_nested_acc;

impl ::protobuf::reflect::FieldAccessor<Root> for Root_nested_acc {
    fn name(&self) -> &'static str {
        "nested"
    }

    fn len_field(&self, m: &Root) -> uint {
        m.get_nested().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a Root, index: uint) -> &'a ::protobuf::Message {
        &'a m.get_nested()[index] as &'a ::protobuf::Message
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

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        use protobuf::{Message};
        os.write_unknown_fields(self.get_unknown_fields());
    }
}

impl ::protobuf::Message for Root_Nested {
    fn new() -> Root_Nested {
        Root_Nested::new()
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

    fn write_to(&self, os: &mut ::protobuf::CodedOutputStream) {
        self.check_initialized();
        let mut sizes: Vec<u32> = Vec::new();
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes.as_slice(), &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
        // TODO: assert we've written same number of bytes as computed
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: Option<Root_Nested>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: Vec<&'static ::protobuf::reflect::FieldAccessor<Root_Nested>> = Vec::new();
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
    }
}

impl ::std::fmt::Show for Root_Nested {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


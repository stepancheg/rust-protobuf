// This file is generated. Do not edit

use protobuf::*;
use protobuf::rt;
use protobuf::descriptor;
use protobuf::lazy;
use std::default::Default;

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x15, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x72, 0x6f, 0x6f,
    0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x2e, 0x0a, 0x04, 0x52, 0x6f, 0x6f, 0x74, 0x12,
    0x1c, 0x0a, 0x06, 0x6e, 0x65, 0x73, 0x74, 0x65, 0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x0c, 0x2e, 0x52, 0x6f, 0x6f, 0x74, 0x2e, 0x4e, 0x65, 0x73, 0x74, 0x65, 0x64, 0x1a, 0x08, 0x0a,
    0x06, 0x4e, 0x65, 0x73, 0x74, 0x65, 0x64,
];

static mut file_descriptor_proto_lazy: lazy::Lazy<descriptor::FileDescriptorProto> = lazy::Lazy { lock: lazy::ONCE_INIT, ptr: 0 as *descriptor::FileDescriptorProto };

fn parse_descriptor_proto() -> descriptor::FileDescriptorProto {
    parse_from_bytes(file_descriptor_proto_data)
}

pub fn file_descriptor_proto() -> &'static descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}

#[deriving(Clone,Eq,Default)]
pub struct Root {
    nested: ~[Root_Nested],
    unknown_fields: Option<~UnknownFields>,
}

impl<'a> Root {
    pub fn new() -> Root {
        Default::default()
    }

    pub fn default_instance() -> &'static Root {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: Root = Root {
//             nested: ~[],
//             unknown_fields: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for v in self.nested.iter() {
            os.write_tag(1, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        os.write_unknown_fields(self.get_unknown_fields());
    }

    pub fn clear_nested(&mut self) {
        self.nested.clear();
    }

    // Param is passed by value, moved
    pub fn set_nested(&mut self, v: ~[Root_Nested]) {
        self.nested = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nested(&'a mut self) -> &'a mut ~[Root_Nested] {
        &mut self.nested
    }

    pub fn get_nested(&'a self) -> &'a [Root_Nested] {
        self.nested.as_slice()
    }

    pub fn add_nested(&mut self, v: Root_Nested) {
        self.nested.push(v);
    }
}

impl Message for Root {
    fn new() -> Root {
        Root::new()
    }

    fn clear(&mut self) {
        self.clear_nested();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = Root_Nested::new();
                    is.merge_message(&mut tmp);
                    self.nested.push(tmp);
                },
                _ => {
                    let unknown = is.read_unknown(wire_type);
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.nested.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        my_size += rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
        // TODO: assert we've written same number of bytes as computed
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s UnknownFields {
        if self.unknown_fields.is_some() {
            &**self.unknown_fields.get_ref()
        } else {
            UnknownFields::default_instance()
        }
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut UnknownFields {
        if self.unknown_fields.is_none() {
            self.unknown_fields = Some(Default::default())
        }
        &mut **self.unknown_fields.get_mut_ref()
    }
}

#[deriving(Clone,Eq,Default)]
pub struct Root_Nested {
    unknown_fields: Option<~UnknownFields>,
}

impl<'a> Root_Nested {
    pub fn new() -> Root_Nested {
        Default::default()
    }

    pub fn default_instance() -> &'static Root_Nested {
        static instance: Root_Nested = Root_Nested {
            unknown_fields: None,
        };
        &'static instance
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        os.write_unknown_fields(self.get_unknown_fields());
    }
}

impl Message for Root_Nested {
    fn new() -> Root_Nested {
        Root_Nested::new()
    }

    fn clear(&mut self) {
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
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
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32 {
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        my_size += rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
        // TODO: assert we've written same number of bytes as computed
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s UnknownFields {
        if self.unknown_fields.is_some() {
            &**self.unknown_fields.get_ref()
        } else {
            UnknownFields::default_instance()
        }
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut UnknownFields {
        if self.unknown_fields.is_none() {
            self.unknown_fields = Some(Default::default())
        }
        &mut **self.unknown_fields.get_mut_ref()
    }
}

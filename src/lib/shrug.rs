// This file is generated. Do not edit

use protobuf::*;
use protobuf::rt;
use protobuf::descriptor;
use protobuf::lazy;
use std::default::Default;

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x73, 0x68, 0x72, 0x75, 0x67, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x05, 0x73, 0x68, 0x72, 0x75, 0x67, 0x22, 0x12, 0x0a, 0x05, 0x54, 0x65,
    0x73, 0x74, 0x31, 0x12, 0x09, 0x0a, 0x01, 0x61, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05, 0x22, 0x12,
    0x0a, 0x05, 0x54, 0x65, 0x73, 0x74, 0x32, 0x12, 0x09, 0x0a, 0x01, 0x62, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x09, 0x22, 0x20, 0x0a, 0x05, 0x54, 0x65, 0x73, 0x74, 0x33, 0x12, 0x17, 0x0a, 0x01, 0x63,
    0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x73, 0x68, 0x72, 0x75, 0x67, 0x2e, 0x54,
    0x65, 0x73, 0x74, 0x31, 0x22, 0x16, 0x0a, 0x05, 0x54, 0x65, 0x73, 0x74, 0x34, 0x12, 0x0d, 0x0a,
    0x01, 0x64, 0x18, 0x04, 0x20, 0x03, 0x28, 0x05, 0x42, 0x02, 0x10, 0x01, 0x22, 0x3a, 0x0a, 0x12,
    0x54, 0x65, 0x73, 0x74, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x64, 0x55, 0x6e, 0x70, 0x61, 0x63, 0x6b,
    0x65, 0x64, 0x12, 0x10, 0x0a, 0x08, 0x75, 0x6e, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x64, 0x18, 0x04,
    0x20, 0x03, 0x28, 0x05, 0x12, 0x12, 0x0a, 0x06, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x64, 0x18, 0x05,
    0x20, 0x03, 0x28, 0x05, 0x42, 0x02, 0x10, 0x01, 0x22, 0x18, 0x0a, 0x09, 0x54, 0x65, 0x73, 0x74,
    0x45, 0x6d, 0x70, 0x74, 0x79, 0x12, 0x0b, 0x0a, 0x03, 0x66, 0x6f, 0x6f, 0x18, 0x0a, 0x20, 0x01,
    0x28, 0x05, 0x22, 0x19, 0x0a, 0x0c, 0x54, 0x65, 0x73, 0x74, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72,
    0x65, 0x64, 0x12, 0x09, 0x0a, 0x01, 0x62, 0x18, 0x05, 0x20, 0x02, 0x28, 0x08, 0x22, 0x1e, 0x0a,
    0x11, 0x54, 0x65, 0x73, 0x74, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x46, 0x69, 0x65, 0x6c,
    0x64, 0x73, 0x12, 0x09, 0x0a, 0x01, 0x61, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05,
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
pub struct Test1 {
    a: Option<i32>,
    unknown_fields: Option<~UnknownFields>,
}

impl<'a> Test1 {
    pub fn new() -> Test1 {
        Default::default()
    }

    pub fn default_instance() -> &'static Test1 {
        static instance: Test1 = Test1 {
            a: None,
            unknown_fields: None,
        };
        &'static instance
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.a {
            Some(ref v) => {
                os.write_int32(1, *v);
            },
            None => {},
        };
        os.write_unknown_fields(self.get_unknown_fields());
    }

    pub fn clear_a(&mut self) {
        self.a = None;
    }

    pub fn has_a(&self) -> bool {
        self.a.is_some()
    }

    // Param is passed by value, moved
    pub fn set_a(&mut self, v: i32) {
        self.a = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_a(&'a mut self) -> &'a mut i32 {
        if self.a.is_none() {
            self.a = Some(0);
        };
        self.a.get_mut_ref()
    }

    pub fn get_a(&self) -> i32 {
        self.a.unwrap_or_else(|| 0)
    }
}

impl Message for Test1 {
    fn new() -> Test1 {
        Test1::new()
    }

    fn clear(&mut self) {
        self.clear_a();
    }

    fn is_initialized(&self) -> bool {
        if self.a.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.a = Some(tmp);
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
        for value in self.a.iter() {
            my_size += rt::value_size(1, *value, wire_format::WireTypeVarint);
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
pub struct Test2 {
    b: Option<~str>,
    unknown_fields: Option<~UnknownFields>,
}

impl<'a> Test2 {
    pub fn new() -> Test2 {
        Default::default()
    }

    pub fn default_instance() -> &'static Test2 {
        static instance: Test2 = Test2 {
            b: None,
            unknown_fields: None,
        };
        &'static instance
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.b {
            Some(ref v) => {
                os.write_string(2, *v);
            },
            None => {},
        };
        os.write_unknown_fields(self.get_unknown_fields());
    }

    pub fn clear_b(&mut self) {
        self.b = None;
    }

    pub fn has_b(&self) -> bool {
        self.b.is_some()
    }

    // Param is passed by value, moved
    pub fn set_b(&mut self, v: ~str) {
        self.b = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_b(&'a mut self) -> &'a mut ~str {
        if self.b.is_none() {
            self.b = Some(~"");
        };
        self.b.get_mut_ref()
    }

    pub fn get_b(&'a self) -> &'a str {
        match self.b {
            Some(ref v) => v.as_slice(),
            None => &'a "",
        }
    }
}

impl Message for Test2 {
    fn new() -> Test2 {
        Test2::new()
    }

    fn clear(&mut self) {
        self.clear_b();
    }

    fn is_initialized(&self) -> bool {
        if self.b.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.b = Some(tmp);
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
        for value in self.b.iter() {
            my_size += rt::string_size(2, *value);
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
pub struct Test3 {
    c: Option<Test1>,
    unknown_fields: Option<~UnknownFields>,
}

impl<'a> Test3 {
    pub fn new() -> Test3 {
        Default::default()
    }

    pub fn default_instance() -> &'static Test3 {
        static instance: Test3 = Test3 {
            c: None,
            unknown_fields: None,
        };
        &'static instance
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.c {
            Some(ref v) => {
                os.write_tag(3, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
        os.write_unknown_fields(self.get_unknown_fields());
    }

    pub fn clear_c(&mut self) {
        self.c = None;
    }

    pub fn has_c(&self) -> bool {
        self.c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_c(&mut self, v: Test1) {
        self.c = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_c(&'a mut self) -> &'a mut Test1 {
        if self.c.is_none() {
            self.c = Some(Test1::new());
        };
        self.c.get_mut_ref()
    }

    pub fn get_c(&'a self) -> &'a Test1 {
        match self.c {
            Some(ref v) => v,
            None => Test1::default_instance(),
        }
    }
}

impl Message for Test3 {
    fn new() -> Test3 {
        Test3::new()
    }

    fn clear(&mut self) {
        self.clear_c();
    }

    fn is_initialized(&self) -> bool {
        if self.c.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                3 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = Test1::new();
                    is.merge_message(&mut tmp);
                    self.c = Some(tmp);
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
        for value in self.c.iter() {
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
pub struct Test4 {
    d: ~[i32],
    unknown_fields: Option<~UnknownFields>,
}

impl<'a> Test4 {
    pub fn new() -> Test4 {
        Default::default()
    }

    pub fn default_instance() -> &'static Test4 {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: Test4 = Test4 {
//             d: ~[],
//             unknown_fields: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        if !self.d.is_empty() {
            os.write_tag(4, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(rt::vec_packed_data_size(self.d, wire_format::WireTypeVarint));
            for v in self.d.iter() {
                os.write_int32_no_tag(*v);
            };
        };
        os.write_unknown_fields(self.get_unknown_fields());
    }

    pub fn clear_d(&mut self) {
        self.d.clear();
    }

    // Param is passed by value, moved
    pub fn set_d(&mut self, v: ~[i32]) {
        self.d = v;
    }

    // Mutable pointer to the field.
    pub fn mut_d(&'a mut self) -> &'a mut ~[i32] {
        &mut self.d
    }

    pub fn get_d(&'a self) -> &'a [i32] {
        self.d.as_slice()
    }

    pub fn add_d(&mut self, v: i32) {
        self.d.push(v);
    }
}

impl Message for Test4 {
    fn new() -> Test4 {
        Test4::new()
    }

    fn clear(&mut self) {
        self.clear_d();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                4 => {
                    if wire_type == wire_format::WireTypeLengthDelimited {
                        let len = is.read_raw_varint32();
                        let old_limit = is.push_limit(len);
                        while !is.eof() {
                            self.d.push(is.read_int32());
                        }
                        is.pop_limit(old_limit);
                    } else {
                        assert_eq!(wire_format::WireTypeVarint, wire_type);
                        self.d.push(is.read_int32());
                    }
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
        my_size += rt::vec_packed_size(4, self.d, wire_format::WireTypeVarint);
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
pub struct TestPackedUnpacked {
    unpacked: ~[i32],
    packed: ~[i32],
    unknown_fields: Option<~UnknownFields>,
}

impl<'a> TestPackedUnpacked {
    pub fn new() -> TestPackedUnpacked {
        Default::default()
    }

    pub fn default_instance() -> &'static TestPackedUnpacked {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: TestPackedUnpacked = TestPackedUnpacked {
//             unpacked: ~[],
//             packed: ~[],
//             unknown_fields: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for v in self.unpacked.iter() {
            os.write_int32(4, *v);
        };
        if !self.packed.is_empty() {
            os.write_tag(5, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(rt::vec_packed_data_size(self.packed, wire_format::WireTypeVarint));
            for v in self.packed.iter() {
                os.write_int32_no_tag(*v);
            };
        };
        os.write_unknown_fields(self.get_unknown_fields());
    }

    pub fn clear_unpacked(&mut self) {
        self.unpacked.clear();
    }

    // Param is passed by value, moved
    pub fn set_unpacked(&mut self, v: ~[i32]) {
        self.unpacked = v;
    }

    // Mutable pointer to the field.
    pub fn mut_unpacked(&'a mut self) -> &'a mut ~[i32] {
        &mut self.unpacked
    }

    pub fn get_unpacked(&'a self) -> &'a [i32] {
        self.unpacked.as_slice()
    }

    pub fn add_unpacked(&mut self, v: i32) {
        self.unpacked.push(v);
    }

    pub fn clear_packed(&mut self) {
        self.packed.clear();
    }

    // Param is passed by value, moved
    pub fn set_packed(&mut self, v: ~[i32]) {
        self.packed = v;
    }

    // Mutable pointer to the field.
    pub fn mut_packed(&'a mut self) -> &'a mut ~[i32] {
        &mut self.packed
    }

    pub fn get_packed(&'a self) -> &'a [i32] {
        self.packed.as_slice()
    }

    pub fn add_packed(&mut self, v: i32) {
        self.packed.push(v);
    }
}

impl Message for TestPackedUnpacked {
    fn new() -> TestPackedUnpacked {
        TestPackedUnpacked::new()
    }

    fn clear(&mut self) {
        self.clear_unpacked();
        self.clear_packed();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                4 => {
                    if wire_type == wire_format::WireTypeLengthDelimited {
                        let len = is.read_raw_varint32();
                        let old_limit = is.push_limit(len);
                        while !is.eof() {
                            self.unpacked.push(is.read_int32());
                        }
                        is.pop_limit(old_limit);
                    } else {
                        assert_eq!(wire_format::WireTypeVarint, wire_type);
                        self.unpacked.push(is.read_int32());
                    }
                },
                5 => {
                    if wire_type == wire_format::WireTypeLengthDelimited {
                        let len = is.read_raw_varint32();
                        let old_limit = is.push_limit(len);
                        while !is.eof() {
                            self.packed.push(is.read_int32());
                        }
                        is.pop_limit(old_limit);
                    } else {
                        assert_eq!(wire_format::WireTypeVarint, wire_type);
                        self.packed.push(is.read_int32());
                    }
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
        for value in self.unpacked.iter() {
            my_size += rt::value_size(4, *value, wire_format::WireTypeVarint);
        };
        my_size += rt::vec_packed_size(5, self.packed, wire_format::WireTypeVarint);
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
pub struct TestEmpty {
    foo: Option<i32>,
    unknown_fields: Option<~UnknownFields>,
}

impl<'a> TestEmpty {
    pub fn new() -> TestEmpty {
        Default::default()
    }

    pub fn default_instance() -> &'static TestEmpty {
        static instance: TestEmpty = TestEmpty {
            foo: None,
            unknown_fields: None,
        };
        &'static instance
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.foo {
            Some(ref v) => {
                os.write_int32(10, *v);
            },
            None => {},
        };
        os.write_unknown_fields(self.get_unknown_fields());
    }

    pub fn clear_foo(&mut self) {
        self.foo = None;
    }

    pub fn has_foo(&self) -> bool {
        self.foo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_foo(&mut self, v: i32) {
        self.foo = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_foo(&'a mut self) -> &'a mut i32 {
        if self.foo.is_none() {
            self.foo = Some(0);
        };
        self.foo.get_mut_ref()
    }

    pub fn get_foo(&self) -> i32 {
        self.foo.unwrap_or_else(|| 0)
    }
}

impl Message for TestEmpty {
    fn new() -> TestEmpty {
        TestEmpty::new()
    }

    fn clear(&mut self) {
        self.clear_foo();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                10 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.foo = Some(tmp);
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
        for value in self.foo.iter() {
            my_size += rt::value_size(10, *value, wire_format::WireTypeVarint);
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
pub struct TestRequired {
    b: Option<bool>,
    unknown_fields: Option<~UnknownFields>,
}

impl<'a> TestRequired {
    pub fn new() -> TestRequired {
        Default::default()
    }

    pub fn default_instance() -> &'static TestRequired {
        static instance: TestRequired = TestRequired {
            b: None,
            unknown_fields: None,
        };
        &'static instance
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.b {
            Some(ref v) => {
                os.write_bool(5, *v);
            },
            None => {},
        };
        os.write_unknown_fields(self.get_unknown_fields());
    }

    pub fn clear_b(&mut self) {
        self.b = None;
    }

    pub fn has_b(&self) -> bool {
        self.b.is_some()
    }

    // Param is passed by value, moved
    pub fn set_b(&mut self, v: bool) {
        self.b = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_b(&'a mut self) -> &'a mut bool {
        if self.b.is_none() {
            self.b = Some(false);
        };
        self.b.get_mut_ref()
    }

    pub fn get_b(&self) -> bool {
        self.b.unwrap_or_else(|| false)
    }
}

impl Message for TestRequired {
    fn new() -> TestRequired {
        TestRequired::new()
    }

    fn clear(&mut self) {
        self.clear_b();
    }

    fn is_initialized(&self) -> bool {
        if self.b.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                5 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.b = Some(tmp);
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
        if self.b.is_some() {
            my_size += 2;
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
pub struct TestUnknownFields {
    a: Option<i32>,
    unknown_fields: Option<~UnknownFields>,
}

impl<'a> TestUnknownFields {
    pub fn new() -> TestUnknownFields {
        Default::default()
    }

    pub fn default_instance() -> &'static TestUnknownFields {
        static instance: TestUnknownFields = TestUnknownFields {
            a: None,
            unknown_fields: None,
        };
        &'static instance
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.a {
            Some(ref v) => {
                os.write_int32(1, *v);
            },
            None => {},
        };
        os.write_unknown_fields(self.get_unknown_fields());
    }

    pub fn clear_a(&mut self) {
        self.a = None;
    }

    pub fn has_a(&self) -> bool {
        self.a.is_some()
    }

    // Param is passed by value, moved
    pub fn set_a(&mut self, v: i32) {
        self.a = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_a(&'a mut self) -> &'a mut i32 {
        if self.a.is_none() {
            self.a = Some(0);
        };
        self.a.get_mut_ref()
    }

    pub fn get_a(&self) -> i32 {
        self.a.unwrap_or_else(|| 0)
    }
}

impl Message for TestUnknownFields {
    fn new() -> TestUnknownFields {
        TestUnknownFields::new()
    }

    fn clear(&mut self) {
        self.clear_a();
    }

    fn is_initialized(&self) -> bool {
        if self.a.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.a = Some(tmp);
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
        for value in self.a.iter() {
            my_size += rt::value_size(1, *value, wire_format::WireTypeVarint);
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

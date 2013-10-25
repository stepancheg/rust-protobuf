// This file is generated. Do not edit

use protobuf::*;
use protobuf::rt;

#[deriving(Clone,Eq)]
pub struct Test1 {
    a: Option<i32>,
}

impl<'self> Test1 {
    pub fn new() -> Test1 {
        Test1 {
            a: None,
        }
    }

    pub fn default_instance() -> &'static Test1 {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: Test1 = Test1 {
//             a: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.a {
            Some(ref v) => {
                os.write_int32(1, *v);
            },
            None => {},
        };
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
    pub fn mut_a(&'self mut self) -> &'self mut i32 {
        if self.a.is_none() {
            self.a = Some(0);
        };
        self.a.get_mut_ref()
    }

    pub fn get_a(&self) -> i32 {
        self.a.unwrap_or(0)
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
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
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
}

#[deriving(Clone,Eq)]
pub struct Test2 {
    b: Option<~str>,
}

impl<'self> Test2 {
    pub fn new() -> Test2 {
        Test2 {
            b: None,
        }
    }

    pub fn default_instance() -> &'static Test2 {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: Test2 = Test2 {
//             b: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.b {
            Some(ref v) => {
                os.write_string(2, *v);
            },
            None => {},
        };
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
    pub fn mut_b(&'self mut self) -> &'self mut ~str {
        if self.b.is_none() {
            self.b = Some(~"");
        };
        self.b.get_mut_ref()
    }

    pub fn get_b(&'self self) -> &'self str {
        match self.b {
            Some(ref v) => v.as_slice(),
            None => &'self "",
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
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
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
}

#[deriving(Clone,Eq)]
pub struct Test3 {
    c: Option<Test1>,
}

impl<'self> Test3 {
    pub fn new() -> Test3 {
        Test3 {
            c: None,
        }
    }

    pub fn default_instance() -> &'static Test3 {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: Test3 = Test3 {
//             c: None,
//         };
//         &'static instance
        fail!("TODO");
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
    pub fn mut_c(&'self mut self) -> &'self mut Test1 {
        if self.c.is_none() {
            self.c = Some(Test1::new());
        };
        self.c.get_mut_ref()
    }

    pub fn get_c(&'self self) -> &'self Test1 {
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
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
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
}

#[deriving(Clone,Eq)]
pub struct Test4 {
    d: ~[i32],
}

impl<'self> Test4 {
    pub fn new() -> Test4 {
        Test4 {
            d: ~[],
        }
    }

    pub fn default_instance() -> &'static Test4 {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: Test4 = Test4 {
//             d: ~[],
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
    }

    pub fn clear_d(&mut self) {
        self.d.clear();
    }

    // Param is passed by value, moved
    pub fn set_d(&mut self, v: ~[i32]) {
        self.d = v;
    }

    // Mutable pointer to the field.
    pub fn mut_d(&'self mut self) -> &'self mut ~[i32] {
        &mut self.d
    }

    pub fn get_d(&'self self) -> &'self [i32] {
        rt::as_slice_tmp(&self.d)
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
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
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
}

#[deriving(Clone,Eq)]
pub struct TestPackedUnpacked {
    unpacked: ~[i32],
    packed: ~[i32],
}

impl<'self> TestPackedUnpacked {
    pub fn new() -> TestPackedUnpacked {
        TestPackedUnpacked {
            unpacked: ~[],
            packed: ~[],
        }
    }

    pub fn default_instance() -> &'static TestPackedUnpacked {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: TestPackedUnpacked = TestPackedUnpacked {
//             unpacked: ~[],
//             packed: ~[],
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
    }

    pub fn clear_unpacked(&mut self) {
        self.unpacked.clear();
    }

    // Param is passed by value, moved
    pub fn set_unpacked(&mut self, v: ~[i32]) {
        self.unpacked = v;
    }

    // Mutable pointer to the field.
    pub fn mut_unpacked(&'self mut self) -> &'self mut ~[i32] {
        &mut self.unpacked
    }

    pub fn get_unpacked(&'self self) -> &'self [i32] {
        rt::as_slice_tmp(&self.unpacked)
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
    pub fn mut_packed(&'self mut self) -> &'self mut ~[i32] {
        &mut self.packed
    }

    pub fn get_packed(&'self self) -> &'self [i32] {
        rt::as_slice_tmp(&self.packed)
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
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
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
}

#[deriving(Clone,Eq)]
pub struct TestEmpty {
    foo: Option<i32>,
}

impl<'self> TestEmpty {
    pub fn new() -> TestEmpty {
        TestEmpty {
            foo: None,
        }
    }

    pub fn default_instance() -> &'static TestEmpty {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: TestEmpty = TestEmpty {
//             foo: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.foo {
            Some(ref v) => {
                os.write_int32(10, *v);
            },
            None => {},
        };
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
    pub fn mut_foo(&'self mut self) -> &'self mut i32 {
        if self.foo.is_none() {
            self.foo = Some(0);
        };
        self.foo.get_mut_ref()
    }

    pub fn get_foo(&self) -> i32 {
        self.foo.unwrap_or(0)
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
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
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
}

#[deriving(Clone,Eq)]
pub struct TestRequired {
    b: Option<bool>,
}

impl<'self> TestRequired {
    pub fn new() -> TestRequired {
        TestRequired {
            b: None,
        }
    }

    pub fn default_instance() -> &'static TestRequired {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: TestRequired = TestRequired {
//             b: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.b {
            Some(ref v) => {
                os.write_bool(5, *v);
            },
            None => {},
        };
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
    pub fn mut_b(&'self mut self) -> &'self mut bool {
        if self.b.is_none() {
            self.b = Some(false);
        };
        self.b.get_mut_ref()
    }

    pub fn get_b(&self) -> bool {
        self.b.unwrap_or(false)
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
                    // TODO: store in unknown fields
                    is.skip_field(wire_type);
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
}

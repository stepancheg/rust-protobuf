// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]


#[deriving(Clone,PartialEq,Default)]
pub struct Test1 {
    a: ::std::option::Option<i32>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Test1 {
    pub fn new() -> Test1 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Test1 {
        static mut instance: ::protobuf::lazy::Lazy<Test1> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const Test1 };
        unsafe {
            instance.get(|| {
                Test1 {
                    a: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required int32 a = 1;

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

    pub fn get_a(&self) -> i32 {
        self.a.unwrap_or(0)
    }
}

impl ::protobuf::Message for Test1 {
    fn new() -> Test1 {
        Test1::new()
    }

    fn is_initialized(&self) -> bool {
        if self.a.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.a = Some(tmp);
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
        for value in self.a.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.a {
            Some(ref v) => {
                try!(os.write_int32(1, *v));
            },
            None => {},
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
    fn descriptor_static(_: ::std::option::Option<Test1>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Test1>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Test1_a_acc as &'static ::protobuf::reflect::FieldAccessor<Test1>) });
                ::protobuf::reflect::MessageDescriptor::new::<Test1>(
                    "Test1",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Test1>()
    }
}

impl ::protobuf::Clear for Test1 {
    fn clear(&mut self) {
        self.clear_a();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Test1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Test1_a_acc_type;
static Test1_a_acc: Test1_a_acc_type = Test1_a_acc_type;

impl ::protobuf::reflect::FieldAccessor<Test1> for Test1_a_acc_type {
    fn name(&self) -> &'static str {
        "a"
    }

    fn has_field(&self, m: &Test1) -> bool {
        m.has_a()
    }

    fn get_i32(&self, m: &Test1) -> i32 {
        m.get_a()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Test2 {
    b: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Test2 {
    pub fn new() -> Test2 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Test2 {
        static mut instance: ::protobuf::lazy::Lazy<Test2> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const Test2 };
        unsafe {
            instance.get(|| {
                Test2 {
                    b: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string b = 2;

    pub fn clear_b(&mut self) {
        self.b.clear();
    }

    pub fn has_b(&self) -> bool {
        self.b.is_some()
    }

    // Param is passed by value, moved
    pub fn set_b(&mut self, v: ::std::string::String) {
        self.b = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_b(&'a mut self) -> &'a mut ::std::string::String {
        if self.b.is_none() {
            self.b.set_default();
        };
        self.b.as_mut().unwrap()
    }

    pub fn get_b(&'a self) -> &'a str {
        match self.b.as_ref() {
            Some(ref v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for Test2 {
    fn new() -> Test2 {
        Test2::new()
    }

    fn is_initialized(&self) -> bool {
        if self.b.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.b.set_default();
                    try!(is.read_string_into(tmp))
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
        for value in self.b.iter() {
            my_size += ::protobuf::rt::string_size(2, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.b.as_ref() {
            Some(ref v) => {
                try!(os.write_string(2, v.as_slice()));
            },
            None => {},
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
    fn descriptor_static(_: ::std::option::Option<Test2>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Test2>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Test2_b_acc as &'static ::protobuf::reflect::FieldAccessor<Test2>) });
                ::protobuf::reflect::MessageDescriptor::new::<Test2>(
                    "Test2",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Test2>()
    }
}

impl ::protobuf::Clear for Test2 {
    fn clear(&mut self) {
        self.clear_b();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Test2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Test2_b_acc_type;
static Test2_b_acc: Test2_b_acc_type = Test2_b_acc_type;

impl ::protobuf::reflect::FieldAccessor<Test2> for Test2_b_acc_type {
    fn name(&self) -> &'static str {
        "b"
    }

    fn has_field(&self, m: &Test2) -> bool {
        m.has_b()
    }

    fn get_str<'a>(&self, m: &'a Test2) -> &'a str {
        m.get_b()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Test3 {
    c: ::protobuf::SingularPtrField<Test1>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Test3 {
    pub fn new() -> Test3 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Test3 {
        static mut instance: ::protobuf::lazy::Lazy<Test3> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const Test3 };
        unsafe {
            instance.get(|| {
                Test3 {
                    c: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required .shrug.Test1 c = 3;

    pub fn clear_c(&mut self) {
        self.c.clear();
    }

    pub fn has_c(&self) -> bool {
        self.c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_c(&mut self, v: Test1) {
        self.c = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_c(&'a mut self) -> &'a mut Test1 {
        if self.c.is_none() {
            self.c.set_default();
        };
        self.c.as_mut().unwrap()
    }

    pub fn get_c(&'a self) -> &'a Test1 {
        self.c.as_ref().unwrap_or_else(|| Test1::default_instance())
    }
}

impl ::protobuf::Message for Test3 {
    fn new() -> Test3 {
        Test3::new()
    }

    fn is_initialized(&self) -> bool {
        if self.c.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.c.set_default();
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
        for value in self.c.iter() {
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
        match self.c.as_ref() {
            Some(ref v) => {
                try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
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
    fn descriptor_static(_: ::std::option::Option<Test3>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Test3>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Test3_c_acc as &'static ::protobuf::reflect::FieldAccessor<Test3>) });
                ::protobuf::reflect::MessageDescriptor::new::<Test3>(
                    "Test3",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Test3>()
    }
}

impl ::protobuf::Clear for Test3 {
    fn clear(&mut self) {
        self.clear_c();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Test3 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Test3_c_acc_type;
static Test3_c_acc: Test3_c_acc_type = Test3_c_acc_type;

impl ::protobuf::reflect::FieldAccessor<Test3> for Test3_c_acc_type {
    fn name(&self) -> &'static str {
        "c"
    }

    fn has_field(&self, m: &Test3) -> bool {
        m.has_c()
    }

    fn get_message<'a>(&self, m: &'a Test3) -> &'a ::protobuf::Message {
        m.get_c() as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Test4 {
    d: ::std::vec::Vec<i32>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Test4 {
    pub fn new() -> Test4 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Test4 {
        static mut instance: ::protobuf::lazy::Lazy<Test4> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const Test4 };
        unsafe {
            instance.get(|| {
                Test4 {
                    d: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // repeated int32 d = 4;

    pub fn clear_d(&mut self) {
        self.d.clear();
    }

    // Param is passed by value, moved
    pub fn set_d(&mut self, v: ::std::vec::Vec<i32>) {
        self.d = v;
    }

    // Mutable pointer to the field.
    pub fn mut_d(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.d
    }

    pub fn get_d(&'a self) -> &'a [i32] {
        self.d.as_slice()
    }

    pub fn add_d(&mut self, v: i32) {
        self.d.push(v);
    }
}

impl ::protobuf::Message for Test4 {
    fn new() -> Test4 {
        Test4::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                4 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.d.push(try!(is.read_int32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.d.push(try!(is.read_int32()));
                    }
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
        if !self.d.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(4, self.d.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        if !self.d.is_empty() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(self.d.as_slice())));
            for v in self.d.iter() {
                try!(os.write_int32_no_tag(*v));
            };
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
    fn descriptor_static(_: ::std::option::Option<Test4>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Test4>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Test4_d_acc as &'static ::protobuf::reflect::FieldAccessor<Test4>) });
                ::protobuf::reflect::MessageDescriptor::new::<Test4>(
                    "Test4",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Test4>()
    }
}

impl ::protobuf::Clear for Test4 {
    fn clear(&mut self) {
        self.clear_d();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Test4 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Test4_d_acc_type;
static Test4_d_acc: Test4_d_acc_type = Test4_d_acc_type;

impl ::protobuf::reflect::FieldAccessor<Test4> for Test4_d_acc_type {
    fn name(&self) -> &'static str {
        "d"
    }

    fn len_field(&self, m: &Test4) -> uint {
        m.get_d().len()
    }

    fn get_rep_i32<'a>(&self, m: &'a Test4) -> &'a [i32] {
        m.get_d()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct TestPackedUnpacked {
    unpacked: ::std::vec::Vec<i32>,
    packed: ::std::vec::Vec<i32>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestPackedUnpacked {
    pub fn new() -> TestPackedUnpacked {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestPackedUnpacked {
        static mut instance: ::protobuf::lazy::Lazy<TestPackedUnpacked> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const TestPackedUnpacked };
        unsafe {
            instance.get(|| {
                TestPackedUnpacked {
                    unpacked: ::std::vec::Vec::new(),
                    packed: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // repeated int32 unpacked = 4;

    pub fn clear_unpacked(&mut self) {
        self.unpacked.clear();
    }

    // Param is passed by value, moved
    pub fn set_unpacked(&mut self, v: ::std::vec::Vec<i32>) {
        self.unpacked = v;
    }

    // Mutable pointer to the field.
    pub fn mut_unpacked(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.unpacked
    }

    pub fn get_unpacked(&'a self) -> &'a [i32] {
        self.unpacked.as_slice()
    }

    pub fn add_unpacked(&mut self, v: i32) {
        self.unpacked.push(v);
    }

    // repeated int32 packed = 5;

    pub fn clear_packed(&mut self) {
        self.packed.clear();
    }

    // Param is passed by value, moved
    pub fn set_packed(&mut self, v: ::std::vec::Vec<i32>) {
        self.packed = v;
    }

    // Mutable pointer to the field.
    pub fn mut_packed(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.packed
    }

    pub fn get_packed(&'a self) -> &'a [i32] {
        self.packed.as_slice()
    }

    pub fn add_packed(&mut self, v: i32) {
        self.packed.push(v);
    }
}

impl ::protobuf::Message for TestPackedUnpacked {
    fn new() -> TestPackedUnpacked {
        TestPackedUnpacked::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                4 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.unpacked.push(try!(is.read_int32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.unpacked.push(try!(is.read_int32()));
                    }
                },
                5 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.packed.push(try!(is.read_int32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.packed.push(try!(is.read_int32()));
                    }
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
        for value in self.unpacked.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.packed.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(5, self.packed.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        for v in self.unpacked.iter() {
            try!(os.write_int32(4, *v));
        };
        if !self.packed.is_empty() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(self.packed.as_slice())));
            for v in self.packed.iter() {
                try!(os.write_int32_no_tag(*v));
            };
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
    fn descriptor_static(_: ::std::option::Option<TestPackedUnpacked>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestPackedUnpacked>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestPackedUnpacked_unpacked_acc as &'static ::protobuf::reflect::FieldAccessor<TestPackedUnpacked>) });
                fields.push(unsafe { ::std::mem::transmute(&TestPackedUnpacked_packed_acc as &'static ::protobuf::reflect::FieldAccessor<TestPackedUnpacked>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestPackedUnpacked>(
                    "TestPackedUnpacked",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestPackedUnpacked>()
    }
}

impl ::protobuf::Clear for TestPackedUnpacked {
    fn clear(&mut self) {
        self.clear_unpacked();
        self.clear_packed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for TestPackedUnpacked {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestPackedUnpacked_unpacked_acc_type;
static TestPackedUnpacked_unpacked_acc: TestPackedUnpacked_unpacked_acc_type = TestPackedUnpacked_unpacked_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestPackedUnpacked> for TestPackedUnpacked_unpacked_acc_type {
    fn name(&self) -> &'static str {
        "unpacked"
    }

    fn len_field(&self, m: &TestPackedUnpacked) -> uint {
        m.get_unpacked().len()
    }

    fn get_rep_i32<'a>(&self, m: &'a TestPackedUnpacked) -> &'a [i32] {
        m.get_unpacked()
    }
}

#[allow(non_camel_case_types)]
struct TestPackedUnpacked_packed_acc_type;
static TestPackedUnpacked_packed_acc: TestPackedUnpacked_packed_acc_type = TestPackedUnpacked_packed_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestPackedUnpacked> for TestPackedUnpacked_packed_acc_type {
    fn name(&self) -> &'static str {
        "packed"
    }

    fn len_field(&self, m: &TestPackedUnpacked) -> uint {
        m.get_packed().len()
    }

    fn get_rep_i32<'a>(&self, m: &'a TestPackedUnpacked) -> &'a [i32] {
        m.get_packed()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct TestEmpty {
    foo: ::std::option::Option<i32>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestEmpty {
    pub fn new() -> TestEmpty {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestEmpty {
        static mut instance: ::protobuf::lazy::Lazy<TestEmpty> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const TestEmpty };
        unsafe {
            instance.get(|| {
                TestEmpty {
                    foo: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional int32 foo = 10;

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

    pub fn get_foo(&self) -> i32 {
        self.foo.unwrap_or(0)
    }
}

impl ::protobuf::Message for TestEmpty {
    fn new() -> TestEmpty {
        TestEmpty::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.foo = Some(tmp);
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
        for value in self.foo.iter() {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.foo {
            Some(ref v) => {
                try!(os.write_int32(10, *v));
            },
            None => {},
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
    fn descriptor_static(_: ::std::option::Option<TestEmpty>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestEmpty>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestEmpty_foo_acc as &'static ::protobuf::reflect::FieldAccessor<TestEmpty>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestEmpty>(
                    "TestEmpty",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestEmpty>()
    }
}

impl ::protobuf::Clear for TestEmpty {
    fn clear(&mut self) {
        self.clear_foo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for TestEmpty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestEmpty_foo_acc_type;
static TestEmpty_foo_acc: TestEmpty_foo_acc_type = TestEmpty_foo_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestEmpty> for TestEmpty_foo_acc_type {
    fn name(&self) -> &'static str {
        "foo"
    }

    fn has_field(&self, m: &TestEmpty) -> bool {
        m.has_foo()
    }

    fn get_i32(&self, m: &TestEmpty) -> i32 {
        m.get_foo()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct TestRequired {
    b: ::std::option::Option<bool>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestRequired {
    pub fn new() -> TestRequired {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestRequired {
        static mut instance: ::protobuf::lazy::Lazy<TestRequired> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const TestRequired };
        unsafe {
            instance.get(|| {
                TestRequired {
                    b: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required bool b = 5;

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

    pub fn get_b(&self) -> bool {
        self.b.unwrap_or(false)
    }
}

impl ::protobuf::Message for TestRequired {
    fn new() -> TestRequired {
        TestRequired::new()
    }

    fn is_initialized(&self) -> bool {
        if self.b.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.b = Some(tmp);
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
        if self.b.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.b {
            Some(ref v) => {
                try!(os.write_bool(5, *v));
            },
            None => {},
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
    fn descriptor_static(_: ::std::option::Option<TestRequired>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestRequired>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestRequired_b_acc as &'static ::protobuf::reflect::FieldAccessor<TestRequired>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestRequired>(
                    "TestRequired",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestRequired>()
    }
}

impl ::protobuf::Clear for TestRequired {
    fn clear(&mut self) {
        self.clear_b();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for TestRequired {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestRequired_b_acc_type;
static TestRequired_b_acc: TestRequired_b_acc_type = TestRequired_b_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestRequired> for TestRequired_b_acc_type {
    fn name(&self) -> &'static str {
        "b"
    }

    fn has_field(&self, m: &TestRequired) -> bool {
        m.has_b()
    }

    fn get_bool(&self, m: &TestRequired) -> bool {
        m.get_b()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct TestUnknownFields {
    a: ::std::option::Option<i32>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestUnknownFields {
    pub fn new() -> TestUnknownFields {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestUnknownFields {
        static mut instance: ::protobuf::lazy::Lazy<TestUnknownFields> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const TestUnknownFields };
        unsafe {
            instance.get(|| {
                TestUnknownFields {
                    a: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required int32 a = 1;

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

    pub fn get_a(&self) -> i32 {
        self.a.unwrap_or(0)
    }
}

impl ::protobuf::Message for TestUnknownFields {
    fn new() -> TestUnknownFields {
        TestUnknownFields::new()
    }

    fn is_initialized(&self) -> bool {
        if self.a.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.a = Some(tmp);
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
        for value in self.a.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.a {
            Some(ref v) => {
                try!(os.write_int32(1, *v));
            },
            None => {},
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
    fn descriptor_static(_: ::std::option::Option<TestUnknownFields>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestUnknownFields>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestUnknownFields_a_acc as &'static ::protobuf::reflect::FieldAccessor<TestUnknownFields>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestUnknownFields>(
                    "TestUnknownFields",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestUnknownFields>()
    }
}

impl ::protobuf::Clear for TestUnknownFields {
    fn clear(&mut self) {
        self.clear_a();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for TestUnknownFields {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestUnknownFields_a_acc_type;
static TestUnknownFields_a_acc: TestUnknownFields_a_acc_type = TestUnknownFields_a_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestUnknownFields> for TestUnknownFields_a_acc_type {
    fn name(&self) -> &'static str {
        "a"
    }

    fn has_field(&self, m: &TestUnknownFields) -> bool {
        m.has_a()
    }

    fn get_i32(&self, m: &TestUnknownFields) -> i32 {
        m.get_a()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct TestSelfReference {
    r1: ::protobuf::SingularPtrField<TestSelfReference>,
    r2: ::protobuf::SingularPtrField<TestSelfReference>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestSelfReference {
    pub fn new() -> TestSelfReference {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestSelfReference {
        static mut instance: ::protobuf::lazy::Lazy<TestSelfReference> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const TestSelfReference };
        unsafe {
            instance.get(|| {
                TestSelfReference {
                    r1: ::protobuf::SingularPtrField::none(),
                    r2: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required .shrug.TestSelfReference r1 = 1;

    pub fn clear_r1(&mut self) {
        self.r1.clear();
    }

    pub fn has_r1(&self) -> bool {
        self.r1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_r1(&mut self, v: TestSelfReference) {
        self.r1 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_r1(&'a mut self) -> &'a mut TestSelfReference {
        if self.r1.is_none() {
            self.r1.set_default();
        };
        self.r1.as_mut().unwrap()
    }

    pub fn get_r1(&'a self) -> &'a TestSelfReference {
        self.r1.as_ref().unwrap_or_else(|| TestSelfReference::default_instance())
    }

    // optional .shrug.TestSelfReference r2 = 2;

    pub fn clear_r2(&mut self) {
        self.r2.clear();
    }

    pub fn has_r2(&self) -> bool {
        self.r2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_r2(&mut self, v: TestSelfReference) {
        self.r2 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_r2(&'a mut self) -> &'a mut TestSelfReference {
        if self.r2.is_none() {
            self.r2.set_default();
        };
        self.r2.as_mut().unwrap()
    }

    pub fn get_r2(&'a self) -> &'a TestSelfReference {
        self.r2.as_ref().unwrap_or_else(|| TestSelfReference::default_instance())
    }
}

impl ::protobuf::Message for TestSelfReference {
    fn new() -> TestSelfReference {
        TestSelfReference::new()
    }

    fn is_initialized(&self) -> bool {
        if self.r1.is_none() {
            return false;
        };
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
                    let tmp = self.r1.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.r2.set_default();
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
        for value in self.r1.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.r2.iter() {
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
        match self.r1.as_ref() {
            Some(ref v) => {
                try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.r2.as_ref() {
            Some(ref v) => {
                try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
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
    fn descriptor_static(_: ::std::option::Option<TestSelfReference>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestSelfReference>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestSelfReference_r1_acc as &'static ::protobuf::reflect::FieldAccessor<TestSelfReference>) });
                fields.push(unsafe { ::std::mem::transmute(&TestSelfReference_r2_acc as &'static ::protobuf::reflect::FieldAccessor<TestSelfReference>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestSelfReference>(
                    "TestSelfReference",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestSelfReference>()
    }
}

impl ::protobuf::Clear for TestSelfReference {
    fn clear(&mut self) {
        self.clear_r1();
        self.clear_r2();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for TestSelfReference {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestSelfReference_r1_acc_type;
static TestSelfReference_r1_acc: TestSelfReference_r1_acc_type = TestSelfReference_r1_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestSelfReference> for TestSelfReference_r1_acc_type {
    fn name(&self) -> &'static str {
        "r1"
    }

    fn has_field(&self, m: &TestSelfReference) -> bool {
        m.has_r1()
    }

    fn get_message<'a>(&self, m: &'a TestSelfReference) -> &'a ::protobuf::Message {
        m.get_r1() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct TestSelfReference_r2_acc_type;
static TestSelfReference_r2_acc: TestSelfReference_r2_acc_type = TestSelfReference_r2_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestSelfReference> for TestSelfReference_r2_acc_type {
    fn name(&self) -> &'static str {
        "r2"
    }

    fn has_field(&self, m: &TestSelfReference) -> bool {
        m.has_r2()
    }

    fn get_message<'a>(&self, m: &'a TestSelfReference) -> &'a ::protobuf::Message {
        m.get_r2() as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct TestDefaultInstanceField {
    s: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestDefaultInstanceField {
    pub fn new() -> TestDefaultInstanceField {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestDefaultInstanceField {
        static mut instance: ::protobuf::lazy::Lazy<TestDefaultInstanceField> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const TestDefaultInstanceField };
        unsafe {
            instance.get(|| {
                TestDefaultInstanceField {
                    s: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional string s = 1;

    pub fn clear_s(&mut self) {
        self.s.clear();
    }

    pub fn has_s(&self) -> bool {
        self.s.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s(&mut self, v: ::std::string::String) {
        self.s = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_s(&'a mut self) -> &'a mut ::std::string::String {
        if self.s.is_none() {
            self.s.set_default();
        };
        self.s.as_mut().unwrap()
    }

    pub fn get_s(&'a self) -> &'a str {
        match self.s.as_ref() {
            Some(ref v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for TestDefaultInstanceField {
    fn new() -> TestDefaultInstanceField {
        TestDefaultInstanceField::new()
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
                    let tmp = self.s.set_default();
                    try!(is.read_string_into(tmp))
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
        for value in self.s.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.s.as_ref() {
            Some(ref v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
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
    fn descriptor_static(_: ::std::option::Option<TestDefaultInstanceField>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestDefaultInstanceField>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultInstanceField_s_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultInstanceField>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestDefaultInstanceField>(
                    "TestDefaultInstanceField",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestDefaultInstanceField>()
    }
}

impl ::protobuf::Clear for TestDefaultInstanceField {
    fn clear(&mut self) {
        self.clear_s();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for TestDefaultInstanceField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestDefaultInstanceField_s_acc_type;
static TestDefaultInstanceField_s_acc: TestDefaultInstanceField_s_acc_type = TestDefaultInstanceField_s_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultInstanceField> for TestDefaultInstanceField_s_acc_type {
    fn name(&self) -> &'static str {
        "s"
    }

    fn has_field(&self, m: &TestDefaultInstanceField) -> bool {
        m.has_s()
    }

    fn get_str<'a>(&self, m: &'a TestDefaultInstanceField) -> &'a str {
        m.get_s()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct TestDefaultInstance {
    field: ::protobuf::SingularPtrField<TestDefaultInstanceField>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestDefaultInstance {
    pub fn new() -> TestDefaultInstance {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestDefaultInstance {
        static mut instance: ::protobuf::lazy::Lazy<TestDefaultInstance> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const TestDefaultInstance };
        unsafe {
            instance.get(|| {
                TestDefaultInstance {
                    field: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional .shrug.TestDefaultInstanceField field = 1;

    pub fn clear_field(&mut self) {
        self.field.clear();
    }

    pub fn has_field(&self) -> bool {
        self.field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field(&mut self, v: TestDefaultInstanceField) {
        self.field = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field(&'a mut self) -> &'a mut TestDefaultInstanceField {
        if self.field.is_none() {
            self.field.set_default();
        };
        self.field.as_mut().unwrap()
    }

    pub fn get_field(&'a self) -> &'a TestDefaultInstanceField {
        self.field.as_ref().unwrap_or_else(|| TestDefaultInstanceField::default_instance())
    }
}

impl ::protobuf::Message for TestDefaultInstance {
    fn new() -> TestDefaultInstance {
        TestDefaultInstance::new()
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
                    let tmp = self.field.set_default();
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
        for value in self.field.iter() {
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
        match self.field.as_ref() {
            Some(ref v) => {
                try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
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
    fn descriptor_static(_: ::std::option::Option<TestDefaultInstance>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestDefaultInstance>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultInstance_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultInstance>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestDefaultInstance>(
                    "TestDefaultInstance",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestDefaultInstance>()
    }
}

impl ::protobuf::Clear for TestDefaultInstance {
    fn clear(&mut self) {
        self.clear_field();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for TestDefaultInstance {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestDefaultInstance_field_acc_type;
static TestDefaultInstance_field_acc: TestDefaultInstance_field_acc_type = TestDefaultInstance_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultInstance> for TestDefaultInstance_field_acc_type {
    fn name(&self) -> &'static str {
        "field"
    }

    fn has_field(&self, m: &TestDefaultInstance) -> bool {
        m.has_field()
    }

    fn get_message<'a>(&self, m: &'a TestDefaultInstance) -> &'a ::protobuf::Message {
        m.get_field() as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct TestDescriptor {
    stuff: ::std::option::Option<i32>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestDescriptor {
    pub fn new() -> TestDescriptor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestDescriptor {
        static mut instance: ::protobuf::lazy::Lazy<TestDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const TestDescriptor };
        unsafe {
            instance.get(|| {
                TestDescriptor {
                    stuff: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional int32 stuff = 10;

    pub fn clear_stuff(&mut self) {
        self.stuff = None;
    }

    pub fn has_stuff(&self) -> bool {
        self.stuff.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stuff(&mut self, v: i32) {
        self.stuff = Some(v);
    }

    pub fn get_stuff(&self) -> i32 {
        self.stuff.unwrap_or(0)
    }
}

impl ::protobuf::Message for TestDescriptor {
    fn new() -> TestDescriptor {
        TestDescriptor::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.stuff = Some(tmp);
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
        for value in self.stuff.iter() {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.stuff {
            Some(ref v) => {
                try!(os.write_int32(10, *v));
            },
            None => {},
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
    fn descriptor_static(_: ::std::option::Option<TestDescriptor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestDescriptor>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestDescriptor_stuff_acc as &'static ::protobuf::reflect::FieldAccessor<TestDescriptor>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestDescriptor>(
                    "TestDescriptor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestDescriptor>()
    }
}

impl ::protobuf::Clear for TestDescriptor {
    fn clear(&mut self) {
        self.clear_stuff();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for TestDescriptor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestDescriptor_stuff_acc_type;
static TestDescriptor_stuff_acc: TestDescriptor_stuff_acc_type = TestDescriptor_stuff_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDescriptor> for TestDescriptor_stuff_acc_type {
    fn name(&self) -> &'static str {
        "stuff"
    }

    fn has_field(&self, m: &TestDescriptor) -> bool {
        m.has_stuff()
    }

    fn get_i32(&self, m: &TestDescriptor) -> i32 {
        m.get_stuff()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct TestTypesSingular {
    double_field: ::std::option::Option<f64>,
    float_field: ::std::option::Option<f32>,
    int32_field: ::std::option::Option<i32>,
    int64_field: ::std::option::Option<i64>,
    uint32_field: ::std::option::Option<u32>,
    uint64_field: ::std::option::Option<u64>,
    sint32_field: ::std::option::Option<i32>,
    sint64_field: ::std::option::Option<i64>,
    fixed32_field: ::std::option::Option<u32>,
    fixed64_field: ::std::option::Option<u64>,
    sfixed32_field: ::std::option::Option<i32>,
    sfixed64_field: ::std::option::Option<i64>,
    bool_field: ::std::option::Option<bool>,
    string_field: ::protobuf::SingularField<::std::string::String>,
    bytes_field: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestTypesSingular {
    pub fn new() -> TestTypesSingular {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestTypesSingular {
        static mut instance: ::protobuf::lazy::Lazy<TestTypesSingular> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const TestTypesSingular };
        unsafe {
            instance.get(|| {
                TestTypesSingular {
                    double_field: ::std::option::None,
                    float_field: ::std::option::None,
                    int32_field: ::std::option::None,
                    int64_field: ::std::option::None,
                    uint32_field: ::std::option::None,
                    uint64_field: ::std::option::None,
                    sint32_field: ::std::option::None,
                    sint64_field: ::std::option::None,
                    fixed32_field: ::std::option::None,
                    fixed64_field: ::std::option::None,
                    sfixed32_field: ::std::option::None,
                    sfixed64_field: ::std::option::None,
                    bool_field: ::std::option::None,
                    string_field: ::protobuf::SingularField::none(),
                    bytes_field: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional double double_field = 1;

    pub fn clear_double_field(&mut self) {
        self.double_field = None;
    }

    pub fn has_double_field(&self) -> bool {
        self.double_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_double_field(&mut self, v: f64) {
        self.double_field = Some(v);
    }

    pub fn get_double_field(&self) -> f64 {
        self.double_field.unwrap_or(0.)
    }

    // optional float float_field = 2;

    pub fn clear_float_field(&mut self) {
        self.float_field = None;
    }

    pub fn has_float_field(&self) -> bool {
        self.float_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_float_field(&mut self, v: f32) {
        self.float_field = Some(v);
    }

    pub fn get_float_field(&self) -> f32 {
        self.float_field.unwrap_or(0.)
    }

    // optional int32 int32_field = 3;

    pub fn clear_int32_field(&mut self) {
        self.int32_field = None;
    }

    pub fn has_int32_field(&self) -> bool {
        self.int32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_int32_field(&mut self, v: i32) {
        self.int32_field = Some(v);
    }

    pub fn get_int32_field(&self) -> i32 {
        self.int32_field.unwrap_or(0)
    }

    // optional int64 int64_field = 4;

    pub fn clear_int64_field(&mut self) {
        self.int64_field = None;
    }

    pub fn has_int64_field(&self) -> bool {
        self.int64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_int64_field(&mut self, v: i64) {
        self.int64_field = Some(v);
    }

    pub fn get_int64_field(&self) -> i64 {
        self.int64_field.unwrap_or(0)
    }

    // optional uint32 uint32_field = 5;

    pub fn clear_uint32_field(&mut self) {
        self.uint32_field = None;
    }

    pub fn has_uint32_field(&self) -> bool {
        self.uint32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uint32_field(&mut self, v: u32) {
        self.uint32_field = Some(v);
    }

    pub fn get_uint32_field(&self) -> u32 {
        self.uint32_field.unwrap_or(0)
    }

    // optional uint64 uint64_field = 6;

    pub fn clear_uint64_field(&mut self) {
        self.uint64_field = None;
    }

    pub fn has_uint64_field(&self) -> bool {
        self.uint64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uint64_field(&mut self, v: u64) {
        self.uint64_field = Some(v);
    }

    pub fn get_uint64_field(&self) -> u64 {
        self.uint64_field.unwrap_or(0)
    }

    // optional sint32 sint32_field = 7;

    pub fn clear_sint32_field(&mut self) {
        self.sint32_field = None;
    }

    pub fn has_sint32_field(&self) -> bool {
        self.sint32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sint32_field(&mut self, v: i32) {
        self.sint32_field = Some(v);
    }

    pub fn get_sint32_field(&self) -> i32 {
        self.sint32_field.unwrap_or(0)
    }

    // optional sint64 sint64_field = 8;

    pub fn clear_sint64_field(&mut self) {
        self.sint64_field = None;
    }

    pub fn has_sint64_field(&self) -> bool {
        self.sint64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sint64_field(&mut self, v: i64) {
        self.sint64_field = Some(v);
    }

    pub fn get_sint64_field(&self) -> i64 {
        self.sint64_field.unwrap_or(0)
    }

    // optional fixed32 fixed32_field = 9;

    pub fn clear_fixed32_field(&mut self) {
        self.fixed32_field = None;
    }

    pub fn has_fixed32_field(&self) -> bool {
        self.fixed32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fixed32_field(&mut self, v: u32) {
        self.fixed32_field = Some(v);
    }

    pub fn get_fixed32_field(&self) -> u32 {
        self.fixed32_field.unwrap_or(0)
    }

    // optional fixed64 fixed64_field = 10;

    pub fn clear_fixed64_field(&mut self) {
        self.fixed64_field = None;
    }

    pub fn has_fixed64_field(&self) -> bool {
        self.fixed64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fixed64_field(&mut self, v: u64) {
        self.fixed64_field = Some(v);
    }

    pub fn get_fixed64_field(&self) -> u64 {
        self.fixed64_field.unwrap_or(0)
    }

    // optional sfixed32 sfixed32_field = 11;

    pub fn clear_sfixed32_field(&mut self) {
        self.sfixed32_field = None;
    }

    pub fn has_sfixed32_field(&self) -> bool {
        self.sfixed32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sfixed32_field(&mut self, v: i32) {
        self.sfixed32_field = Some(v);
    }

    pub fn get_sfixed32_field(&self) -> i32 {
        self.sfixed32_field.unwrap_or(0)
    }

    // optional sfixed64 sfixed64_field = 12;

    pub fn clear_sfixed64_field(&mut self) {
        self.sfixed64_field = None;
    }

    pub fn has_sfixed64_field(&self) -> bool {
        self.sfixed64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sfixed64_field(&mut self, v: i64) {
        self.sfixed64_field = Some(v);
    }

    pub fn get_sfixed64_field(&self) -> i64 {
        self.sfixed64_field.unwrap_or(0)
    }

    // optional bool bool_field = 13;

    pub fn clear_bool_field(&mut self) {
        self.bool_field = None;
    }

    pub fn has_bool_field(&self) -> bool {
        self.bool_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bool_field(&mut self, v: bool) {
        self.bool_field = Some(v);
    }

    pub fn get_bool_field(&self) -> bool {
        self.bool_field.unwrap_or(false)
    }

    // optional string string_field = 14;

    pub fn clear_string_field(&mut self) {
        self.string_field.clear();
    }

    pub fn has_string_field(&self) -> bool {
        self.string_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_field(&mut self, v: ::std::string::String) {
        self.string_field = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_field(&'a mut self) -> &'a mut ::std::string::String {
        if self.string_field.is_none() {
            self.string_field.set_default();
        };
        self.string_field.as_mut().unwrap()
    }

    pub fn get_string_field(&'a self) -> &'a str {
        match self.string_field.as_ref() {
            Some(ref v) => v.as_slice(),
            None => "",
        }
    }

    // optional bytes bytes_field = 15;

    pub fn clear_bytes_field(&mut self) {
        self.bytes_field.clear();
    }

    pub fn has_bytes_field(&self) -> bool {
        self.bytes_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytes_field(&mut self, v: ::std::vec::Vec<u8>) {
        self.bytes_field = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bytes_field(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.bytes_field.is_none() {
            self.bytes_field.set_default();
        };
        self.bytes_field.as_mut().unwrap()
    }

    pub fn get_bytes_field(&'a self) -> &'a [u8] {
        match self.bytes_field.as_ref() {
            Some(ref v) => v.as_slice(),
            None => [].as_slice(),
        }
    }
}

impl ::protobuf::Message for TestTypesSingular {
    fn new() -> TestTypesSingular {
        TestTypesSingular::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.double_field = Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.float_field = Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.int32_field = Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.int64_field = Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.uint32_field = Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.uint64_field = Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sint32());
                    self.sint32_field = Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sint64());
                    self.sint64_field = Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_fixed32());
                    self.fixed32_field = Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.fixed64_field = Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sfixed32());
                    self.sfixed32_field = Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sfixed64());
                    self.sfixed64_field = Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.bool_field = Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.string_field.set_default();
                    try!(is.read_string_into(tmp))
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.bytes_field.set_default();
                    try!(is.read_bytes_into(tmp))
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
        if self.double_field.is_some() {
            my_size += 9;
        };
        if self.float_field.is_some() {
            my_size += 5;
        };
        for value in self.int32_field.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.int64_field.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.uint32_field.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.uint64_field.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.sint32_field.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.sint64_field.iter() {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.fixed32_field.is_some() {
            my_size += 5;
        };
        if self.fixed64_field.is_some() {
            my_size += 9;
        };
        if self.sfixed32_field.is_some() {
            my_size += 5;
        };
        if self.sfixed64_field.is_some() {
            my_size += 9;
        };
        if self.bool_field.is_some() {
            my_size += 2;
        };
        for value in self.string_field.iter() {
            my_size += ::protobuf::rt::string_size(14, value.as_slice());
        };
        for value in self.bytes_field.iter() {
            my_size += ::protobuf::rt::bytes_size(15, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.double_field {
            Some(ref v) => {
                try!(os.write_double(1, *v));
            },
            None => {},
        };
        match self.float_field {
            Some(ref v) => {
                try!(os.write_float(2, *v));
            },
            None => {},
        };
        match self.int32_field {
            Some(ref v) => {
                try!(os.write_int32(3, *v));
            },
            None => {},
        };
        match self.int64_field {
            Some(ref v) => {
                try!(os.write_int64(4, *v));
            },
            None => {},
        };
        match self.uint32_field {
            Some(ref v) => {
                try!(os.write_uint32(5, *v));
            },
            None => {},
        };
        match self.uint64_field {
            Some(ref v) => {
                try!(os.write_uint64(6, *v));
            },
            None => {},
        };
        match self.sint32_field {
            Some(ref v) => {
                try!(os.write_sint32(7, *v));
            },
            None => {},
        };
        match self.sint64_field {
            Some(ref v) => {
                try!(os.write_sint64(8, *v));
            },
            None => {},
        };
        match self.fixed32_field {
            Some(ref v) => {
                try!(os.write_fixed32(9, *v));
            },
            None => {},
        };
        match self.fixed64_field {
            Some(ref v) => {
                try!(os.write_fixed64(10, *v));
            },
            None => {},
        };
        match self.sfixed32_field {
            Some(ref v) => {
                try!(os.write_sfixed32(11, *v));
            },
            None => {},
        };
        match self.sfixed64_field {
            Some(ref v) => {
                try!(os.write_sfixed64(12, *v));
            },
            None => {},
        };
        match self.bool_field {
            Some(ref v) => {
                try!(os.write_bool(13, *v));
            },
            None => {},
        };
        match self.string_field.as_ref() {
            Some(ref v) => {
                try!(os.write_string(14, v.as_slice()));
            },
            None => {},
        };
        match self.bytes_field.as_ref() {
            Some(ref v) => {
                try!(os.write_bytes(15, v.as_slice()));
            },
            None => {},
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
    fn descriptor_static(_: ::std::option::Option<TestTypesSingular>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestTypesSingular>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestTypesSingular_double_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesSingular>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesSingular_float_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesSingular>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesSingular_int32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesSingular>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesSingular_int64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesSingular>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesSingular_uint32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesSingular>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesSingular_uint64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesSingular>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesSingular_sint32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesSingular>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesSingular_sint64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesSingular>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesSingular_fixed32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesSingular>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesSingular_fixed64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesSingular>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesSingular_sfixed32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesSingular>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesSingular_sfixed64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesSingular>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesSingular_bool_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesSingular>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesSingular_string_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesSingular>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesSingular_bytes_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesSingular>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestTypesSingular>(
                    "TestTypesSingular",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestTypesSingular>()
    }
}

impl ::protobuf::Clear for TestTypesSingular {
    fn clear(&mut self) {
        self.clear_double_field();
        self.clear_float_field();
        self.clear_int32_field();
        self.clear_int64_field();
        self.clear_uint32_field();
        self.clear_uint64_field();
        self.clear_sint32_field();
        self.clear_sint64_field();
        self.clear_fixed32_field();
        self.clear_fixed64_field();
        self.clear_sfixed32_field();
        self.clear_sfixed64_field();
        self.clear_bool_field();
        self.clear_string_field();
        self.clear_bytes_field();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for TestTypesSingular {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestTypesSingular_double_field_acc_type;
static TestTypesSingular_double_field_acc: TestTypesSingular_double_field_acc_type = TestTypesSingular_double_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesSingular> for TestTypesSingular_double_field_acc_type {
    fn name(&self) -> &'static str {
        "double_field"
    }

    fn has_field(&self, m: &TestTypesSingular) -> bool {
        m.has_double_field()
    }

    fn get_f64(&self, m: &TestTypesSingular) -> f64 {
        m.get_double_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesSingular_float_field_acc_type;
static TestTypesSingular_float_field_acc: TestTypesSingular_float_field_acc_type = TestTypesSingular_float_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesSingular> for TestTypesSingular_float_field_acc_type {
    fn name(&self) -> &'static str {
        "float_field"
    }

    fn has_field(&self, m: &TestTypesSingular) -> bool {
        m.has_float_field()
    }

    fn get_f32(&self, m: &TestTypesSingular) -> f32 {
        m.get_float_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesSingular_int32_field_acc_type;
static TestTypesSingular_int32_field_acc: TestTypesSingular_int32_field_acc_type = TestTypesSingular_int32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesSingular> for TestTypesSingular_int32_field_acc_type {
    fn name(&self) -> &'static str {
        "int32_field"
    }

    fn has_field(&self, m: &TestTypesSingular) -> bool {
        m.has_int32_field()
    }

    fn get_i32(&self, m: &TestTypesSingular) -> i32 {
        m.get_int32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesSingular_int64_field_acc_type;
static TestTypesSingular_int64_field_acc: TestTypesSingular_int64_field_acc_type = TestTypesSingular_int64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesSingular> for TestTypesSingular_int64_field_acc_type {
    fn name(&self) -> &'static str {
        "int64_field"
    }

    fn has_field(&self, m: &TestTypesSingular) -> bool {
        m.has_int64_field()
    }

    fn get_i64(&self, m: &TestTypesSingular) -> i64 {
        m.get_int64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesSingular_uint32_field_acc_type;
static TestTypesSingular_uint32_field_acc: TestTypesSingular_uint32_field_acc_type = TestTypesSingular_uint32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesSingular> for TestTypesSingular_uint32_field_acc_type {
    fn name(&self) -> &'static str {
        "uint32_field"
    }

    fn has_field(&self, m: &TestTypesSingular) -> bool {
        m.has_uint32_field()
    }

    fn get_u32(&self, m: &TestTypesSingular) -> u32 {
        m.get_uint32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesSingular_uint64_field_acc_type;
static TestTypesSingular_uint64_field_acc: TestTypesSingular_uint64_field_acc_type = TestTypesSingular_uint64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesSingular> for TestTypesSingular_uint64_field_acc_type {
    fn name(&self) -> &'static str {
        "uint64_field"
    }

    fn has_field(&self, m: &TestTypesSingular) -> bool {
        m.has_uint64_field()
    }

    fn get_u64(&self, m: &TestTypesSingular) -> u64 {
        m.get_uint64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesSingular_sint32_field_acc_type;
static TestTypesSingular_sint32_field_acc: TestTypesSingular_sint32_field_acc_type = TestTypesSingular_sint32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesSingular> for TestTypesSingular_sint32_field_acc_type {
    fn name(&self) -> &'static str {
        "sint32_field"
    }

    fn has_field(&self, m: &TestTypesSingular) -> bool {
        m.has_sint32_field()
    }

    fn get_i32(&self, m: &TestTypesSingular) -> i32 {
        m.get_sint32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesSingular_sint64_field_acc_type;
static TestTypesSingular_sint64_field_acc: TestTypesSingular_sint64_field_acc_type = TestTypesSingular_sint64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesSingular> for TestTypesSingular_sint64_field_acc_type {
    fn name(&self) -> &'static str {
        "sint64_field"
    }

    fn has_field(&self, m: &TestTypesSingular) -> bool {
        m.has_sint64_field()
    }

    fn get_i64(&self, m: &TestTypesSingular) -> i64 {
        m.get_sint64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesSingular_fixed32_field_acc_type;
static TestTypesSingular_fixed32_field_acc: TestTypesSingular_fixed32_field_acc_type = TestTypesSingular_fixed32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesSingular> for TestTypesSingular_fixed32_field_acc_type {
    fn name(&self) -> &'static str {
        "fixed32_field"
    }

    fn has_field(&self, m: &TestTypesSingular) -> bool {
        m.has_fixed32_field()
    }

    fn get_u32(&self, m: &TestTypesSingular) -> u32 {
        m.get_fixed32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesSingular_fixed64_field_acc_type;
static TestTypesSingular_fixed64_field_acc: TestTypesSingular_fixed64_field_acc_type = TestTypesSingular_fixed64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesSingular> for TestTypesSingular_fixed64_field_acc_type {
    fn name(&self) -> &'static str {
        "fixed64_field"
    }

    fn has_field(&self, m: &TestTypesSingular) -> bool {
        m.has_fixed64_field()
    }

    fn get_u64(&self, m: &TestTypesSingular) -> u64 {
        m.get_fixed64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesSingular_sfixed32_field_acc_type;
static TestTypesSingular_sfixed32_field_acc: TestTypesSingular_sfixed32_field_acc_type = TestTypesSingular_sfixed32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesSingular> for TestTypesSingular_sfixed32_field_acc_type {
    fn name(&self) -> &'static str {
        "sfixed32_field"
    }

    fn has_field(&self, m: &TestTypesSingular) -> bool {
        m.has_sfixed32_field()
    }

    fn get_i32(&self, m: &TestTypesSingular) -> i32 {
        m.get_sfixed32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesSingular_sfixed64_field_acc_type;
static TestTypesSingular_sfixed64_field_acc: TestTypesSingular_sfixed64_field_acc_type = TestTypesSingular_sfixed64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesSingular> for TestTypesSingular_sfixed64_field_acc_type {
    fn name(&self) -> &'static str {
        "sfixed64_field"
    }

    fn has_field(&self, m: &TestTypesSingular) -> bool {
        m.has_sfixed64_field()
    }

    fn get_i64(&self, m: &TestTypesSingular) -> i64 {
        m.get_sfixed64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesSingular_bool_field_acc_type;
static TestTypesSingular_bool_field_acc: TestTypesSingular_bool_field_acc_type = TestTypesSingular_bool_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesSingular> for TestTypesSingular_bool_field_acc_type {
    fn name(&self) -> &'static str {
        "bool_field"
    }

    fn has_field(&self, m: &TestTypesSingular) -> bool {
        m.has_bool_field()
    }

    fn get_bool(&self, m: &TestTypesSingular) -> bool {
        m.get_bool_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesSingular_string_field_acc_type;
static TestTypesSingular_string_field_acc: TestTypesSingular_string_field_acc_type = TestTypesSingular_string_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesSingular> for TestTypesSingular_string_field_acc_type {
    fn name(&self) -> &'static str {
        "string_field"
    }

    fn has_field(&self, m: &TestTypesSingular) -> bool {
        m.has_string_field()
    }

    fn get_str<'a>(&self, m: &'a TestTypesSingular) -> &'a str {
        m.get_string_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesSingular_bytes_field_acc_type;
static TestTypesSingular_bytes_field_acc: TestTypesSingular_bytes_field_acc_type = TestTypesSingular_bytes_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesSingular> for TestTypesSingular_bytes_field_acc_type {
    fn name(&self) -> &'static str {
        "bytes_field"
    }

    fn has_field(&self, m: &TestTypesSingular) -> bool {
        m.has_bytes_field()
    }

    fn get_bytes<'a>(&self, m: &'a TestTypesSingular) -> &'a [u8] {
        m.get_bytes_field()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct TestTypesRepeated {
    double_field: ::std::vec::Vec<f64>,
    float_field: ::std::vec::Vec<f32>,
    int32_field: ::std::vec::Vec<i32>,
    int64_field: ::std::vec::Vec<i64>,
    uint32_field: ::std::vec::Vec<u32>,
    uint64_field: ::std::vec::Vec<u64>,
    sint32_field: ::std::vec::Vec<i32>,
    sint64_field: ::std::vec::Vec<i64>,
    fixed32_field: ::std::vec::Vec<u32>,
    fixed64_field: ::std::vec::Vec<u64>,
    sfixed32_field: ::std::vec::Vec<i32>,
    sfixed64_field: ::std::vec::Vec<i64>,
    bool_field: ::std::vec::Vec<bool>,
    string_field: ::protobuf::RepeatedField<::std::string::String>,
    bytes_field: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestTypesRepeated {
    pub fn new() -> TestTypesRepeated {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestTypesRepeated {
        static mut instance: ::protobuf::lazy::Lazy<TestTypesRepeated> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const TestTypesRepeated };
        unsafe {
            instance.get(|| {
                TestTypesRepeated {
                    double_field: ::std::vec::Vec::new(),
                    float_field: ::std::vec::Vec::new(),
                    int32_field: ::std::vec::Vec::new(),
                    int64_field: ::std::vec::Vec::new(),
                    uint32_field: ::std::vec::Vec::new(),
                    uint64_field: ::std::vec::Vec::new(),
                    sint32_field: ::std::vec::Vec::new(),
                    sint64_field: ::std::vec::Vec::new(),
                    fixed32_field: ::std::vec::Vec::new(),
                    fixed64_field: ::std::vec::Vec::new(),
                    sfixed32_field: ::std::vec::Vec::new(),
                    sfixed64_field: ::std::vec::Vec::new(),
                    bool_field: ::std::vec::Vec::new(),
                    string_field: ::protobuf::RepeatedField::new(),
                    bytes_field: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // repeated double double_field = 1;

    pub fn clear_double_field(&mut self) {
        self.double_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_double_field(&mut self, v: ::std::vec::Vec<f64>) {
        self.double_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_double_field(&'a mut self) -> &'a mut ::std::vec::Vec<f64> {
        &mut self.double_field
    }

    pub fn get_double_field(&'a self) -> &'a [f64] {
        self.double_field.as_slice()
    }

    pub fn add_double_field(&mut self, v: f64) {
        self.double_field.push(v);
    }

    // repeated float float_field = 2;

    pub fn clear_float_field(&mut self) {
        self.float_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_float_field(&mut self, v: ::std::vec::Vec<f32>) {
        self.float_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_float_field(&'a mut self) -> &'a mut ::std::vec::Vec<f32> {
        &mut self.float_field
    }

    pub fn get_float_field(&'a self) -> &'a [f32] {
        self.float_field.as_slice()
    }

    pub fn add_float_field(&mut self, v: f32) {
        self.float_field.push(v);
    }

    // repeated int32 int32_field = 3;

    pub fn clear_int32_field(&mut self) {
        self.int32_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_int32_field(&mut self, v: ::std::vec::Vec<i32>) {
        self.int32_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_int32_field(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.int32_field
    }

    pub fn get_int32_field(&'a self) -> &'a [i32] {
        self.int32_field.as_slice()
    }

    pub fn add_int32_field(&mut self, v: i32) {
        self.int32_field.push(v);
    }

    // repeated int64 int64_field = 4;

    pub fn clear_int64_field(&mut self) {
        self.int64_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_int64_field(&mut self, v: ::std::vec::Vec<i64>) {
        self.int64_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_int64_field(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.int64_field
    }

    pub fn get_int64_field(&'a self) -> &'a [i64] {
        self.int64_field.as_slice()
    }

    pub fn add_int64_field(&mut self, v: i64) {
        self.int64_field.push(v);
    }

    // repeated uint32 uint32_field = 5;

    pub fn clear_uint32_field(&mut self) {
        self.uint32_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_uint32_field(&mut self, v: ::std::vec::Vec<u32>) {
        self.uint32_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uint32_field(&'a mut self) -> &'a mut ::std::vec::Vec<u32> {
        &mut self.uint32_field
    }

    pub fn get_uint32_field(&'a self) -> &'a [u32] {
        self.uint32_field.as_slice()
    }

    pub fn add_uint32_field(&mut self, v: u32) {
        self.uint32_field.push(v);
    }

    // repeated uint64 uint64_field = 6;

    pub fn clear_uint64_field(&mut self) {
        self.uint64_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_uint64_field(&mut self, v: ::std::vec::Vec<u64>) {
        self.uint64_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uint64_field(&'a mut self) -> &'a mut ::std::vec::Vec<u64> {
        &mut self.uint64_field
    }

    pub fn get_uint64_field(&'a self) -> &'a [u64] {
        self.uint64_field.as_slice()
    }

    pub fn add_uint64_field(&mut self, v: u64) {
        self.uint64_field.push(v);
    }

    // repeated sint32 sint32_field = 7;

    pub fn clear_sint32_field(&mut self) {
        self.sint32_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_sint32_field(&mut self, v: ::std::vec::Vec<i32>) {
        self.sint32_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sint32_field(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.sint32_field
    }

    pub fn get_sint32_field(&'a self) -> &'a [i32] {
        self.sint32_field.as_slice()
    }

    pub fn add_sint32_field(&mut self, v: i32) {
        self.sint32_field.push(v);
    }

    // repeated sint64 sint64_field = 8;

    pub fn clear_sint64_field(&mut self) {
        self.sint64_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_sint64_field(&mut self, v: ::std::vec::Vec<i64>) {
        self.sint64_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sint64_field(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.sint64_field
    }

    pub fn get_sint64_field(&'a self) -> &'a [i64] {
        self.sint64_field.as_slice()
    }

    pub fn add_sint64_field(&mut self, v: i64) {
        self.sint64_field.push(v);
    }

    // repeated fixed32 fixed32_field = 9;

    pub fn clear_fixed32_field(&mut self) {
        self.fixed32_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_fixed32_field(&mut self, v: ::std::vec::Vec<u32>) {
        self.fixed32_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fixed32_field(&'a mut self) -> &'a mut ::std::vec::Vec<u32> {
        &mut self.fixed32_field
    }

    pub fn get_fixed32_field(&'a self) -> &'a [u32] {
        self.fixed32_field.as_slice()
    }

    pub fn add_fixed32_field(&mut self, v: u32) {
        self.fixed32_field.push(v);
    }

    // repeated fixed64 fixed64_field = 10;

    pub fn clear_fixed64_field(&mut self) {
        self.fixed64_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_fixed64_field(&mut self, v: ::std::vec::Vec<u64>) {
        self.fixed64_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fixed64_field(&'a mut self) -> &'a mut ::std::vec::Vec<u64> {
        &mut self.fixed64_field
    }

    pub fn get_fixed64_field(&'a self) -> &'a [u64] {
        self.fixed64_field.as_slice()
    }

    pub fn add_fixed64_field(&mut self, v: u64) {
        self.fixed64_field.push(v);
    }

    // repeated sfixed32 sfixed32_field = 11;

    pub fn clear_sfixed32_field(&mut self) {
        self.sfixed32_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_sfixed32_field(&mut self, v: ::std::vec::Vec<i32>) {
        self.sfixed32_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sfixed32_field(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.sfixed32_field
    }

    pub fn get_sfixed32_field(&'a self) -> &'a [i32] {
        self.sfixed32_field.as_slice()
    }

    pub fn add_sfixed32_field(&mut self, v: i32) {
        self.sfixed32_field.push(v);
    }

    // repeated sfixed64 sfixed64_field = 12;

    pub fn clear_sfixed64_field(&mut self) {
        self.sfixed64_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_sfixed64_field(&mut self, v: ::std::vec::Vec<i64>) {
        self.sfixed64_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sfixed64_field(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.sfixed64_field
    }

    pub fn get_sfixed64_field(&'a self) -> &'a [i64] {
        self.sfixed64_field.as_slice()
    }

    pub fn add_sfixed64_field(&mut self, v: i64) {
        self.sfixed64_field.push(v);
    }

    // repeated bool bool_field = 13;

    pub fn clear_bool_field(&mut self) {
        self.bool_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_bool_field(&mut self, v: ::std::vec::Vec<bool>) {
        self.bool_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bool_field(&'a mut self) -> &'a mut ::std::vec::Vec<bool> {
        &mut self.bool_field
    }

    pub fn get_bool_field(&'a self) -> &'a [bool] {
        self.bool_field.as_slice()
    }

    pub fn add_bool_field(&mut self, v: bool) {
        self.bool_field.push(v);
    }

    // repeated string string_field = 14;

    pub fn clear_string_field(&mut self) {
        self.string_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_string_field(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.string_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_string_field(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.string_field
    }

    pub fn get_string_field(&'a self) -> &'a [::std::string::String] {
        self.string_field.as_slice()
    }

    pub fn add_string_field(&mut self, v: ::std::string::String) {
        self.string_field.push(v);
    }

    // repeated bytes bytes_field = 15;

    pub fn clear_bytes_field(&mut self) {
        self.bytes_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_bytes_field(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.bytes_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bytes_field(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.bytes_field
    }

    pub fn get_bytes_field(&'a self) -> &'a [::std::vec::Vec<u8>] {
        self.bytes_field.as_slice()
    }

    pub fn add_bytes_field(&mut self, v: ::std::vec::Vec<u8>) {
        self.bytes_field.push(v);
    }
}

impl ::protobuf::Message for TestTypesRepeated {
    fn new() -> TestTypesRepeated {
        TestTypesRepeated::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.double_field.push(try!(is.read_double()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.double_field.push(try!(is.read_double()));
                    }
                },
                2 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.float_field.push(try!(is.read_float()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.float_field.push(try!(is.read_float()));
                    }
                },
                3 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.int32_field.push(try!(is.read_int32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.int32_field.push(try!(is.read_int32()));
                    }
                },
                4 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.int64_field.push(try!(is.read_int64()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.int64_field.push(try!(is.read_int64()));
                    }
                },
                5 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.uint32_field.push(try!(is.read_uint32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.uint32_field.push(try!(is.read_uint32()));
                    }
                },
                6 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.uint64_field.push(try!(is.read_uint64()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.uint64_field.push(try!(is.read_uint64()));
                    }
                },
                7 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.sint32_field.push(try!(is.read_sint32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.sint32_field.push(try!(is.read_sint32()));
                    }
                },
                8 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.sint64_field.push(try!(is.read_sint64()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.sint64_field.push(try!(is.read_sint64()));
                    }
                },
                9 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.fixed32_field.push(try!(is.read_fixed32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.fixed32_field.push(try!(is.read_fixed32()));
                    }
                },
                10 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.fixed64_field.push(try!(is.read_fixed64()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.fixed64_field.push(try!(is.read_fixed64()));
                    }
                },
                11 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.sfixed32_field.push(try!(is.read_sfixed32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.sfixed32_field.push(try!(is.read_sfixed32()));
                    }
                },
                12 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.sfixed64_field.push(try!(is.read_sfixed64()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.sfixed64_field.push(try!(is.read_sfixed64()));
                    }
                },
                13 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.bool_field.push(try!(is.read_bool()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.bool_field.push(try!(is.read_bool()));
                    }
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.string_field.push_default();
                    try!(is.read_string_into(tmp))
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.bytes_field.push_default();
                    try!(is.read_bytes_into(tmp))
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
        my_size += 9 * self.double_field.len() as u32;
        my_size += 5 * self.float_field.len() as u32;
        for value in self.int32_field.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.int64_field.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.uint32_field.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.uint64_field.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.sint32_field.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.sint64_field.iter() {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += 5 * self.fixed32_field.len() as u32;
        my_size += 9 * self.fixed64_field.len() as u32;
        my_size += 5 * self.sfixed32_field.len() as u32;
        my_size += 9 * self.sfixed64_field.len() as u32;
        my_size += 2 * self.bool_field.len() as u32;
        for value in self.string_field.iter() {
            my_size += ::protobuf::rt::string_size(14, value.as_slice());
        };
        for value in self.bytes_field.iter() {
            my_size += ::protobuf::rt::bytes_size(15, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        for v in self.double_field.iter() {
            try!(os.write_double(1, *v));
        };
        for v in self.float_field.iter() {
            try!(os.write_float(2, *v));
        };
        for v in self.int32_field.iter() {
            try!(os.write_int32(3, *v));
        };
        for v in self.int64_field.iter() {
            try!(os.write_int64(4, *v));
        };
        for v in self.uint32_field.iter() {
            try!(os.write_uint32(5, *v));
        };
        for v in self.uint64_field.iter() {
            try!(os.write_uint64(6, *v));
        };
        for v in self.sint32_field.iter() {
            try!(os.write_sint32(7, *v));
        };
        for v in self.sint64_field.iter() {
            try!(os.write_sint64(8, *v));
        };
        for v in self.fixed32_field.iter() {
            try!(os.write_fixed32(9, *v));
        };
        for v in self.fixed64_field.iter() {
            try!(os.write_fixed64(10, *v));
        };
        for v in self.sfixed32_field.iter() {
            try!(os.write_sfixed32(11, *v));
        };
        for v in self.sfixed64_field.iter() {
            try!(os.write_sfixed64(12, *v));
        };
        for v in self.bool_field.iter() {
            try!(os.write_bool(13, *v));
        };
        for v in self.string_field.iter() {
            try!(os.write_string(14, v.as_slice()));
        };
        for v in self.bytes_field.iter() {
            try!(os.write_bytes(15, v.as_slice()));
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
    fn descriptor_static(_: ::std::option::Option<TestTypesRepeated>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestTypesRepeated>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeated_double_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeated>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeated_float_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeated>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeated_int32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeated>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeated_int64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeated>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeated_uint32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeated>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeated_uint64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeated>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeated_sint32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeated>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeated_sint64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeated>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeated_fixed32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeated>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeated_fixed64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeated>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeated_sfixed32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeated>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeated_sfixed64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeated>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeated_bool_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeated>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeated_string_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeated>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeated_bytes_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeated>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestTypesRepeated>(
                    "TestTypesRepeated",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestTypesRepeated>()
    }
}

impl ::protobuf::Clear for TestTypesRepeated {
    fn clear(&mut self) {
        self.clear_double_field();
        self.clear_float_field();
        self.clear_int32_field();
        self.clear_int64_field();
        self.clear_uint32_field();
        self.clear_uint64_field();
        self.clear_sint32_field();
        self.clear_sint64_field();
        self.clear_fixed32_field();
        self.clear_fixed64_field();
        self.clear_sfixed32_field();
        self.clear_sfixed64_field();
        self.clear_bool_field();
        self.clear_string_field();
        self.clear_bytes_field();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for TestTypesRepeated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestTypesRepeated_double_field_acc_type;
static TestTypesRepeated_double_field_acc: TestTypesRepeated_double_field_acc_type = TestTypesRepeated_double_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeated> for TestTypesRepeated_double_field_acc_type {
    fn name(&self) -> &'static str {
        "double_field"
    }

    fn len_field(&self, m: &TestTypesRepeated) -> uint {
        m.get_double_field().len()
    }

    fn get_rep_f64<'a>(&self, m: &'a TestTypesRepeated) -> &'a [f64] {
        m.get_double_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeated_float_field_acc_type;
static TestTypesRepeated_float_field_acc: TestTypesRepeated_float_field_acc_type = TestTypesRepeated_float_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeated> for TestTypesRepeated_float_field_acc_type {
    fn name(&self) -> &'static str {
        "float_field"
    }

    fn len_field(&self, m: &TestTypesRepeated) -> uint {
        m.get_float_field().len()
    }

    fn get_rep_f32<'a>(&self, m: &'a TestTypesRepeated) -> &'a [f32] {
        m.get_float_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeated_int32_field_acc_type;
static TestTypesRepeated_int32_field_acc: TestTypesRepeated_int32_field_acc_type = TestTypesRepeated_int32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeated> for TestTypesRepeated_int32_field_acc_type {
    fn name(&self) -> &'static str {
        "int32_field"
    }

    fn len_field(&self, m: &TestTypesRepeated) -> uint {
        m.get_int32_field().len()
    }

    fn get_rep_i32<'a>(&self, m: &'a TestTypesRepeated) -> &'a [i32] {
        m.get_int32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeated_int64_field_acc_type;
static TestTypesRepeated_int64_field_acc: TestTypesRepeated_int64_field_acc_type = TestTypesRepeated_int64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeated> for TestTypesRepeated_int64_field_acc_type {
    fn name(&self) -> &'static str {
        "int64_field"
    }

    fn len_field(&self, m: &TestTypesRepeated) -> uint {
        m.get_int64_field().len()
    }

    fn get_rep_i64<'a>(&self, m: &'a TestTypesRepeated) -> &'a [i64] {
        m.get_int64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeated_uint32_field_acc_type;
static TestTypesRepeated_uint32_field_acc: TestTypesRepeated_uint32_field_acc_type = TestTypesRepeated_uint32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeated> for TestTypesRepeated_uint32_field_acc_type {
    fn name(&self) -> &'static str {
        "uint32_field"
    }

    fn len_field(&self, m: &TestTypesRepeated) -> uint {
        m.get_uint32_field().len()
    }

    fn get_rep_u32<'a>(&self, m: &'a TestTypesRepeated) -> &'a [u32] {
        m.get_uint32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeated_uint64_field_acc_type;
static TestTypesRepeated_uint64_field_acc: TestTypesRepeated_uint64_field_acc_type = TestTypesRepeated_uint64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeated> for TestTypesRepeated_uint64_field_acc_type {
    fn name(&self) -> &'static str {
        "uint64_field"
    }

    fn len_field(&self, m: &TestTypesRepeated) -> uint {
        m.get_uint64_field().len()
    }

    fn get_rep_u64<'a>(&self, m: &'a TestTypesRepeated) -> &'a [u64] {
        m.get_uint64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeated_sint32_field_acc_type;
static TestTypesRepeated_sint32_field_acc: TestTypesRepeated_sint32_field_acc_type = TestTypesRepeated_sint32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeated> for TestTypesRepeated_sint32_field_acc_type {
    fn name(&self) -> &'static str {
        "sint32_field"
    }

    fn len_field(&self, m: &TestTypesRepeated) -> uint {
        m.get_sint32_field().len()
    }

    fn get_rep_i32<'a>(&self, m: &'a TestTypesRepeated) -> &'a [i32] {
        m.get_sint32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeated_sint64_field_acc_type;
static TestTypesRepeated_sint64_field_acc: TestTypesRepeated_sint64_field_acc_type = TestTypesRepeated_sint64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeated> for TestTypesRepeated_sint64_field_acc_type {
    fn name(&self) -> &'static str {
        "sint64_field"
    }

    fn len_field(&self, m: &TestTypesRepeated) -> uint {
        m.get_sint64_field().len()
    }

    fn get_rep_i64<'a>(&self, m: &'a TestTypesRepeated) -> &'a [i64] {
        m.get_sint64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeated_fixed32_field_acc_type;
static TestTypesRepeated_fixed32_field_acc: TestTypesRepeated_fixed32_field_acc_type = TestTypesRepeated_fixed32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeated> for TestTypesRepeated_fixed32_field_acc_type {
    fn name(&self) -> &'static str {
        "fixed32_field"
    }

    fn len_field(&self, m: &TestTypesRepeated) -> uint {
        m.get_fixed32_field().len()
    }

    fn get_rep_u32<'a>(&self, m: &'a TestTypesRepeated) -> &'a [u32] {
        m.get_fixed32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeated_fixed64_field_acc_type;
static TestTypesRepeated_fixed64_field_acc: TestTypesRepeated_fixed64_field_acc_type = TestTypesRepeated_fixed64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeated> for TestTypesRepeated_fixed64_field_acc_type {
    fn name(&self) -> &'static str {
        "fixed64_field"
    }

    fn len_field(&self, m: &TestTypesRepeated) -> uint {
        m.get_fixed64_field().len()
    }

    fn get_rep_u64<'a>(&self, m: &'a TestTypesRepeated) -> &'a [u64] {
        m.get_fixed64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeated_sfixed32_field_acc_type;
static TestTypesRepeated_sfixed32_field_acc: TestTypesRepeated_sfixed32_field_acc_type = TestTypesRepeated_sfixed32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeated> for TestTypesRepeated_sfixed32_field_acc_type {
    fn name(&self) -> &'static str {
        "sfixed32_field"
    }

    fn len_field(&self, m: &TestTypesRepeated) -> uint {
        m.get_sfixed32_field().len()
    }

    fn get_rep_i32<'a>(&self, m: &'a TestTypesRepeated) -> &'a [i32] {
        m.get_sfixed32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeated_sfixed64_field_acc_type;
static TestTypesRepeated_sfixed64_field_acc: TestTypesRepeated_sfixed64_field_acc_type = TestTypesRepeated_sfixed64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeated> for TestTypesRepeated_sfixed64_field_acc_type {
    fn name(&self) -> &'static str {
        "sfixed64_field"
    }

    fn len_field(&self, m: &TestTypesRepeated) -> uint {
        m.get_sfixed64_field().len()
    }

    fn get_rep_i64<'a>(&self, m: &'a TestTypesRepeated) -> &'a [i64] {
        m.get_sfixed64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeated_bool_field_acc_type;
static TestTypesRepeated_bool_field_acc: TestTypesRepeated_bool_field_acc_type = TestTypesRepeated_bool_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeated> for TestTypesRepeated_bool_field_acc_type {
    fn name(&self) -> &'static str {
        "bool_field"
    }

    fn len_field(&self, m: &TestTypesRepeated) -> uint {
        m.get_bool_field().len()
    }

    fn get_rep_bool<'a>(&self, m: &'a TestTypesRepeated) -> &'a [bool] {
        m.get_bool_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeated_string_field_acc_type;
static TestTypesRepeated_string_field_acc: TestTypesRepeated_string_field_acc_type = TestTypesRepeated_string_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeated> for TestTypesRepeated_string_field_acc_type {
    fn name(&self) -> &'static str {
        "string_field"
    }

    fn len_field(&self, m: &TestTypesRepeated) -> uint {
        m.get_string_field().len()
    }

    fn get_rep_str<'a>(&self, m: &'a TestTypesRepeated) -> &'a [::std::string::String] {
        m.get_string_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeated_bytes_field_acc_type;
static TestTypesRepeated_bytes_field_acc: TestTypesRepeated_bytes_field_acc_type = TestTypesRepeated_bytes_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeated> for TestTypesRepeated_bytes_field_acc_type {
    fn name(&self) -> &'static str {
        "bytes_field"
    }

    fn len_field(&self, m: &TestTypesRepeated) -> uint {
        m.get_bytes_field().len()
    }

    fn get_rep_bytes<'a>(&self, m: &'a TestTypesRepeated) -> &'a [::std::vec::Vec<u8>] {
        m.get_bytes_field()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct TestTypesRepeatedPacked {
    double_field: ::std::vec::Vec<f64>,
    float_field: ::std::vec::Vec<f32>,
    int32_field: ::std::vec::Vec<i32>,
    int64_field: ::std::vec::Vec<i64>,
    uint32_field: ::std::vec::Vec<u32>,
    uint64_field: ::std::vec::Vec<u64>,
    sint32_field: ::std::vec::Vec<i32>,
    sint64_field: ::std::vec::Vec<i64>,
    fixed32_field: ::std::vec::Vec<u32>,
    fixed64_field: ::std::vec::Vec<u64>,
    sfixed32_field: ::std::vec::Vec<i32>,
    sfixed64_field: ::std::vec::Vec<i64>,
    bool_field: ::std::vec::Vec<bool>,
    string_field: ::protobuf::RepeatedField<::std::string::String>,
    bytes_field: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestTypesRepeatedPacked {
    pub fn new() -> TestTypesRepeatedPacked {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestTypesRepeatedPacked {
        static mut instance: ::protobuf::lazy::Lazy<TestTypesRepeatedPacked> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const TestTypesRepeatedPacked };
        unsafe {
            instance.get(|| {
                TestTypesRepeatedPacked {
                    double_field: ::std::vec::Vec::new(),
                    float_field: ::std::vec::Vec::new(),
                    int32_field: ::std::vec::Vec::new(),
                    int64_field: ::std::vec::Vec::new(),
                    uint32_field: ::std::vec::Vec::new(),
                    uint64_field: ::std::vec::Vec::new(),
                    sint32_field: ::std::vec::Vec::new(),
                    sint64_field: ::std::vec::Vec::new(),
                    fixed32_field: ::std::vec::Vec::new(),
                    fixed64_field: ::std::vec::Vec::new(),
                    sfixed32_field: ::std::vec::Vec::new(),
                    sfixed64_field: ::std::vec::Vec::new(),
                    bool_field: ::std::vec::Vec::new(),
                    string_field: ::protobuf::RepeatedField::new(),
                    bytes_field: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // repeated double double_field = 1;

    pub fn clear_double_field(&mut self) {
        self.double_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_double_field(&mut self, v: ::std::vec::Vec<f64>) {
        self.double_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_double_field(&'a mut self) -> &'a mut ::std::vec::Vec<f64> {
        &mut self.double_field
    }

    pub fn get_double_field(&'a self) -> &'a [f64] {
        self.double_field.as_slice()
    }

    pub fn add_double_field(&mut self, v: f64) {
        self.double_field.push(v);
    }

    // repeated float float_field = 2;

    pub fn clear_float_field(&mut self) {
        self.float_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_float_field(&mut self, v: ::std::vec::Vec<f32>) {
        self.float_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_float_field(&'a mut self) -> &'a mut ::std::vec::Vec<f32> {
        &mut self.float_field
    }

    pub fn get_float_field(&'a self) -> &'a [f32] {
        self.float_field.as_slice()
    }

    pub fn add_float_field(&mut self, v: f32) {
        self.float_field.push(v);
    }

    // repeated int32 int32_field = 3;

    pub fn clear_int32_field(&mut self) {
        self.int32_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_int32_field(&mut self, v: ::std::vec::Vec<i32>) {
        self.int32_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_int32_field(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.int32_field
    }

    pub fn get_int32_field(&'a self) -> &'a [i32] {
        self.int32_field.as_slice()
    }

    pub fn add_int32_field(&mut self, v: i32) {
        self.int32_field.push(v);
    }

    // repeated int64 int64_field = 4;

    pub fn clear_int64_field(&mut self) {
        self.int64_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_int64_field(&mut self, v: ::std::vec::Vec<i64>) {
        self.int64_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_int64_field(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.int64_field
    }

    pub fn get_int64_field(&'a self) -> &'a [i64] {
        self.int64_field.as_slice()
    }

    pub fn add_int64_field(&mut self, v: i64) {
        self.int64_field.push(v);
    }

    // repeated uint32 uint32_field = 5;

    pub fn clear_uint32_field(&mut self) {
        self.uint32_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_uint32_field(&mut self, v: ::std::vec::Vec<u32>) {
        self.uint32_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uint32_field(&'a mut self) -> &'a mut ::std::vec::Vec<u32> {
        &mut self.uint32_field
    }

    pub fn get_uint32_field(&'a self) -> &'a [u32] {
        self.uint32_field.as_slice()
    }

    pub fn add_uint32_field(&mut self, v: u32) {
        self.uint32_field.push(v);
    }

    // repeated uint64 uint64_field = 6;

    pub fn clear_uint64_field(&mut self) {
        self.uint64_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_uint64_field(&mut self, v: ::std::vec::Vec<u64>) {
        self.uint64_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uint64_field(&'a mut self) -> &'a mut ::std::vec::Vec<u64> {
        &mut self.uint64_field
    }

    pub fn get_uint64_field(&'a self) -> &'a [u64] {
        self.uint64_field.as_slice()
    }

    pub fn add_uint64_field(&mut self, v: u64) {
        self.uint64_field.push(v);
    }

    // repeated sint32 sint32_field = 7;

    pub fn clear_sint32_field(&mut self) {
        self.sint32_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_sint32_field(&mut self, v: ::std::vec::Vec<i32>) {
        self.sint32_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sint32_field(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.sint32_field
    }

    pub fn get_sint32_field(&'a self) -> &'a [i32] {
        self.sint32_field.as_slice()
    }

    pub fn add_sint32_field(&mut self, v: i32) {
        self.sint32_field.push(v);
    }

    // repeated sint64 sint64_field = 8;

    pub fn clear_sint64_field(&mut self) {
        self.sint64_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_sint64_field(&mut self, v: ::std::vec::Vec<i64>) {
        self.sint64_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sint64_field(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.sint64_field
    }

    pub fn get_sint64_field(&'a self) -> &'a [i64] {
        self.sint64_field.as_slice()
    }

    pub fn add_sint64_field(&mut self, v: i64) {
        self.sint64_field.push(v);
    }

    // repeated fixed32 fixed32_field = 9;

    pub fn clear_fixed32_field(&mut self) {
        self.fixed32_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_fixed32_field(&mut self, v: ::std::vec::Vec<u32>) {
        self.fixed32_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fixed32_field(&'a mut self) -> &'a mut ::std::vec::Vec<u32> {
        &mut self.fixed32_field
    }

    pub fn get_fixed32_field(&'a self) -> &'a [u32] {
        self.fixed32_field.as_slice()
    }

    pub fn add_fixed32_field(&mut self, v: u32) {
        self.fixed32_field.push(v);
    }

    // repeated fixed64 fixed64_field = 10;

    pub fn clear_fixed64_field(&mut self) {
        self.fixed64_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_fixed64_field(&mut self, v: ::std::vec::Vec<u64>) {
        self.fixed64_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fixed64_field(&'a mut self) -> &'a mut ::std::vec::Vec<u64> {
        &mut self.fixed64_field
    }

    pub fn get_fixed64_field(&'a self) -> &'a [u64] {
        self.fixed64_field.as_slice()
    }

    pub fn add_fixed64_field(&mut self, v: u64) {
        self.fixed64_field.push(v);
    }

    // repeated sfixed32 sfixed32_field = 11;

    pub fn clear_sfixed32_field(&mut self) {
        self.sfixed32_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_sfixed32_field(&mut self, v: ::std::vec::Vec<i32>) {
        self.sfixed32_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sfixed32_field(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.sfixed32_field
    }

    pub fn get_sfixed32_field(&'a self) -> &'a [i32] {
        self.sfixed32_field.as_slice()
    }

    pub fn add_sfixed32_field(&mut self, v: i32) {
        self.sfixed32_field.push(v);
    }

    // repeated sfixed64 sfixed64_field = 12;

    pub fn clear_sfixed64_field(&mut self) {
        self.sfixed64_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_sfixed64_field(&mut self, v: ::std::vec::Vec<i64>) {
        self.sfixed64_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sfixed64_field(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.sfixed64_field
    }

    pub fn get_sfixed64_field(&'a self) -> &'a [i64] {
        self.sfixed64_field.as_slice()
    }

    pub fn add_sfixed64_field(&mut self, v: i64) {
        self.sfixed64_field.push(v);
    }

    // repeated bool bool_field = 13;

    pub fn clear_bool_field(&mut self) {
        self.bool_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_bool_field(&mut self, v: ::std::vec::Vec<bool>) {
        self.bool_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bool_field(&'a mut self) -> &'a mut ::std::vec::Vec<bool> {
        &mut self.bool_field
    }

    pub fn get_bool_field(&'a self) -> &'a [bool] {
        self.bool_field.as_slice()
    }

    pub fn add_bool_field(&mut self, v: bool) {
        self.bool_field.push(v);
    }

    // repeated string string_field = 14;

    pub fn clear_string_field(&mut self) {
        self.string_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_string_field(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.string_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_string_field(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.string_field
    }

    pub fn get_string_field(&'a self) -> &'a [::std::string::String] {
        self.string_field.as_slice()
    }

    pub fn add_string_field(&mut self, v: ::std::string::String) {
        self.string_field.push(v);
    }

    // repeated bytes bytes_field = 15;

    pub fn clear_bytes_field(&mut self) {
        self.bytes_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_bytes_field(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.bytes_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bytes_field(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.bytes_field
    }

    pub fn get_bytes_field(&'a self) -> &'a [::std::vec::Vec<u8>] {
        self.bytes_field.as_slice()
    }

    pub fn add_bytes_field(&mut self, v: ::std::vec::Vec<u8>) {
        self.bytes_field.push(v);
    }
}

impl ::protobuf::Message for TestTypesRepeatedPacked {
    fn new() -> TestTypesRepeatedPacked {
        TestTypesRepeatedPacked::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.double_field.push(try!(is.read_double()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.double_field.push(try!(is.read_double()));
                    }
                },
                2 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.float_field.push(try!(is.read_float()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.float_field.push(try!(is.read_float()));
                    }
                },
                3 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.int32_field.push(try!(is.read_int32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.int32_field.push(try!(is.read_int32()));
                    }
                },
                4 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.int64_field.push(try!(is.read_int64()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.int64_field.push(try!(is.read_int64()));
                    }
                },
                5 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.uint32_field.push(try!(is.read_uint32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.uint32_field.push(try!(is.read_uint32()));
                    }
                },
                6 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.uint64_field.push(try!(is.read_uint64()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.uint64_field.push(try!(is.read_uint64()));
                    }
                },
                7 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.sint32_field.push(try!(is.read_sint32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.sint32_field.push(try!(is.read_sint32()));
                    }
                },
                8 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.sint64_field.push(try!(is.read_sint64()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.sint64_field.push(try!(is.read_sint64()));
                    }
                },
                9 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.fixed32_field.push(try!(is.read_fixed32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.fixed32_field.push(try!(is.read_fixed32()));
                    }
                },
                10 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.fixed64_field.push(try!(is.read_fixed64()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.fixed64_field.push(try!(is.read_fixed64()));
                    }
                },
                11 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.sfixed32_field.push(try!(is.read_sfixed32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.sfixed32_field.push(try!(is.read_sfixed32()));
                    }
                },
                12 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.sfixed64_field.push(try!(is.read_sfixed64()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.sfixed64_field.push(try!(is.read_sfixed64()));
                    }
                },
                13 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.bool_field.push(try!(is.read_bool()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.bool_field.push(try!(is.read_bool()));
                    }
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.string_field.push_default();
                    try!(is.read_string_into(tmp))
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.bytes_field.push_default();
                    try!(is.read_bytes_into(tmp))
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
        if !self.double_field.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.double_field.len() as u32) + (self.double_field.len() * 8) as u32;
        };
        if !self.float_field.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.float_field.len() as u32) + (self.float_field.len() * 4) as u32;
        };
        if !self.int32_field.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(3, self.int32_field.as_slice());
        };
        if !self.int64_field.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(4, self.int64_field.as_slice());
        };
        if !self.uint32_field.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(5, self.uint32_field.as_slice());
        };
        if !self.uint64_field.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(6, self.uint64_field.as_slice());
        };
        if !self.sint32_field.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(7, self.sint32_field.as_slice());
        };
        if !self.sint64_field.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(8, self.sint64_field.as_slice());
        };
        if !self.fixed32_field.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.fixed32_field.len() as u32) + (self.fixed32_field.len() * 4) as u32;
        };
        if !self.fixed64_field.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.fixed64_field.len() as u32) + (self.fixed64_field.len() * 8) as u32;
        };
        if !self.sfixed32_field.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.sfixed32_field.len() as u32) + (self.sfixed32_field.len() * 4) as u32;
        };
        if !self.sfixed64_field.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.sfixed64_field.len() as u32) + (self.sfixed64_field.len() * 8) as u32;
        };
        if !self.bool_field.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.bool_field.len() as u32) + (self.bool_field.len() * 1) as u32;
        };
        for value in self.string_field.iter() {
            my_size += ::protobuf::rt::string_size(14, value.as_slice());
        };
        for value in self.bytes_field.iter() {
            my_size += ::protobuf::rt::bytes_size(15, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        if !self.double_field.is_empty() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32((self.double_field.len() * 8) as u32));
            for v in self.double_field.iter() {
                try!(os.write_double_no_tag(*v));
            };
        };
        if !self.float_field.is_empty() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32((self.float_field.len() * 4) as u32));
            for v in self.float_field.iter() {
                try!(os.write_float_no_tag(*v));
            };
        };
        if !self.int32_field.is_empty() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(self.int32_field.as_slice())));
            for v in self.int32_field.iter() {
                try!(os.write_int32_no_tag(*v));
            };
        };
        if !self.int64_field.is_empty() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(self.int64_field.as_slice())));
            for v in self.int64_field.iter() {
                try!(os.write_int64_no_tag(*v));
            };
        };
        if !self.uint32_field.is_empty() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(self.uint32_field.as_slice())));
            for v in self.uint32_field.iter() {
                try!(os.write_uint32_no_tag(*v));
            };
        };
        if !self.uint64_field.is_empty() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(self.uint64_field.as_slice())));
            for v in self.uint64_field.iter() {
                try!(os.write_uint64_no_tag(*v));
            };
        };
        if !self.sint32_field.is_empty() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(self.sint32_field.as_slice())));
            for v in self.sint32_field.iter() {
                try!(os.write_sint32_no_tag(*v));
            };
        };
        if !self.sint64_field.is_empty() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(self.sint64_field.as_slice())));
            for v in self.sint64_field.iter() {
                try!(os.write_sint64_no_tag(*v));
            };
        };
        if !self.fixed32_field.is_empty() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32((self.fixed32_field.len() * 4) as u32));
            for v in self.fixed32_field.iter() {
                try!(os.write_fixed32_no_tag(*v));
            };
        };
        if !self.fixed64_field.is_empty() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32((self.fixed64_field.len() * 8) as u32));
            for v in self.fixed64_field.iter() {
                try!(os.write_fixed64_no_tag(*v));
            };
        };
        if !self.sfixed32_field.is_empty() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32((self.sfixed32_field.len() * 4) as u32));
            for v in self.sfixed32_field.iter() {
                try!(os.write_sfixed32_no_tag(*v));
            };
        };
        if !self.sfixed64_field.is_empty() {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32((self.sfixed64_field.len() * 8) as u32));
            for v in self.sfixed64_field.iter() {
                try!(os.write_sfixed64_no_tag(*v));
            };
        };
        if !self.bool_field.is_empty() {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32((self.bool_field.len() * 1) as u32));
            for v in self.bool_field.iter() {
                try!(os.write_bool_no_tag(*v));
            };
        };
        for v in self.string_field.iter() {
            try!(os.write_string(14, v.as_slice()));
        };
        for v in self.bytes_field.iter() {
            try!(os.write_bytes(15, v.as_slice()));
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
    fn descriptor_static(_: ::std::option::Option<TestTypesRepeatedPacked>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeatedPacked_double_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeatedPacked_float_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeatedPacked_int32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeatedPacked_int64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeatedPacked_uint32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeatedPacked_uint64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeatedPacked_sint32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeatedPacked_sint64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeatedPacked_fixed32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeatedPacked_fixed64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeatedPacked_sfixed32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeatedPacked_sfixed64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeatedPacked_bool_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeatedPacked_string_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypesRepeatedPacked_bytes_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestTypesRepeatedPacked>(
                    "TestTypesRepeatedPacked",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestTypesRepeatedPacked>()
    }
}

impl ::protobuf::Clear for TestTypesRepeatedPacked {
    fn clear(&mut self) {
        self.clear_double_field();
        self.clear_float_field();
        self.clear_int32_field();
        self.clear_int64_field();
        self.clear_uint32_field();
        self.clear_uint64_field();
        self.clear_sint32_field();
        self.clear_sint64_field();
        self.clear_fixed32_field();
        self.clear_fixed64_field();
        self.clear_sfixed32_field();
        self.clear_sfixed64_field();
        self.clear_bool_field();
        self.clear_string_field();
        self.clear_bytes_field();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for TestTypesRepeatedPacked {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestTypesRepeatedPacked_double_field_acc_type;
static TestTypesRepeatedPacked_double_field_acc: TestTypesRepeatedPacked_double_field_acc_type = TestTypesRepeatedPacked_double_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked> for TestTypesRepeatedPacked_double_field_acc_type {
    fn name(&self) -> &'static str {
        "double_field"
    }

    fn len_field(&self, m: &TestTypesRepeatedPacked) -> uint {
        m.get_double_field().len()
    }

    fn get_rep_f64<'a>(&self, m: &'a TestTypesRepeatedPacked) -> &'a [f64] {
        m.get_double_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeatedPacked_float_field_acc_type;
static TestTypesRepeatedPacked_float_field_acc: TestTypesRepeatedPacked_float_field_acc_type = TestTypesRepeatedPacked_float_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked> for TestTypesRepeatedPacked_float_field_acc_type {
    fn name(&self) -> &'static str {
        "float_field"
    }

    fn len_field(&self, m: &TestTypesRepeatedPacked) -> uint {
        m.get_float_field().len()
    }

    fn get_rep_f32<'a>(&self, m: &'a TestTypesRepeatedPacked) -> &'a [f32] {
        m.get_float_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeatedPacked_int32_field_acc_type;
static TestTypesRepeatedPacked_int32_field_acc: TestTypesRepeatedPacked_int32_field_acc_type = TestTypesRepeatedPacked_int32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked> for TestTypesRepeatedPacked_int32_field_acc_type {
    fn name(&self) -> &'static str {
        "int32_field"
    }

    fn len_field(&self, m: &TestTypesRepeatedPacked) -> uint {
        m.get_int32_field().len()
    }

    fn get_rep_i32<'a>(&self, m: &'a TestTypesRepeatedPacked) -> &'a [i32] {
        m.get_int32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeatedPacked_int64_field_acc_type;
static TestTypesRepeatedPacked_int64_field_acc: TestTypesRepeatedPacked_int64_field_acc_type = TestTypesRepeatedPacked_int64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked> for TestTypesRepeatedPacked_int64_field_acc_type {
    fn name(&self) -> &'static str {
        "int64_field"
    }

    fn len_field(&self, m: &TestTypesRepeatedPacked) -> uint {
        m.get_int64_field().len()
    }

    fn get_rep_i64<'a>(&self, m: &'a TestTypesRepeatedPacked) -> &'a [i64] {
        m.get_int64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeatedPacked_uint32_field_acc_type;
static TestTypesRepeatedPacked_uint32_field_acc: TestTypesRepeatedPacked_uint32_field_acc_type = TestTypesRepeatedPacked_uint32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked> for TestTypesRepeatedPacked_uint32_field_acc_type {
    fn name(&self) -> &'static str {
        "uint32_field"
    }

    fn len_field(&self, m: &TestTypesRepeatedPacked) -> uint {
        m.get_uint32_field().len()
    }

    fn get_rep_u32<'a>(&self, m: &'a TestTypesRepeatedPacked) -> &'a [u32] {
        m.get_uint32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeatedPacked_uint64_field_acc_type;
static TestTypesRepeatedPacked_uint64_field_acc: TestTypesRepeatedPacked_uint64_field_acc_type = TestTypesRepeatedPacked_uint64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked> for TestTypesRepeatedPacked_uint64_field_acc_type {
    fn name(&self) -> &'static str {
        "uint64_field"
    }

    fn len_field(&self, m: &TestTypesRepeatedPacked) -> uint {
        m.get_uint64_field().len()
    }

    fn get_rep_u64<'a>(&self, m: &'a TestTypesRepeatedPacked) -> &'a [u64] {
        m.get_uint64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeatedPacked_sint32_field_acc_type;
static TestTypesRepeatedPacked_sint32_field_acc: TestTypesRepeatedPacked_sint32_field_acc_type = TestTypesRepeatedPacked_sint32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked> for TestTypesRepeatedPacked_sint32_field_acc_type {
    fn name(&self) -> &'static str {
        "sint32_field"
    }

    fn len_field(&self, m: &TestTypesRepeatedPacked) -> uint {
        m.get_sint32_field().len()
    }

    fn get_rep_i32<'a>(&self, m: &'a TestTypesRepeatedPacked) -> &'a [i32] {
        m.get_sint32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeatedPacked_sint64_field_acc_type;
static TestTypesRepeatedPacked_sint64_field_acc: TestTypesRepeatedPacked_sint64_field_acc_type = TestTypesRepeatedPacked_sint64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked> for TestTypesRepeatedPacked_sint64_field_acc_type {
    fn name(&self) -> &'static str {
        "sint64_field"
    }

    fn len_field(&self, m: &TestTypesRepeatedPacked) -> uint {
        m.get_sint64_field().len()
    }

    fn get_rep_i64<'a>(&self, m: &'a TestTypesRepeatedPacked) -> &'a [i64] {
        m.get_sint64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeatedPacked_fixed32_field_acc_type;
static TestTypesRepeatedPacked_fixed32_field_acc: TestTypesRepeatedPacked_fixed32_field_acc_type = TestTypesRepeatedPacked_fixed32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked> for TestTypesRepeatedPacked_fixed32_field_acc_type {
    fn name(&self) -> &'static str {
        "fixed32_field"
    }

    fn len_field(&self, m: &TestTypesRepeatedPacked) -> uint {
        m.get_fixed32_field().len()
    }

    fn get_rep_u32<'a>(&self, m: &'a TestTypesRepeatedPacked) -> &'a [u32] {
        m.get_fixed32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeatedPacked_fixed64_field_acc_type;
static TestTypesRepeatedPacked_fixed64_field_acc: TestTypesRepeatedPacked_fixed64_field_acc_type = TestTypesRepeatedPacked_fixed64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked> for TestTypesRepeatedPacked_fixed64_field_acc_type {
    fn name(&self) -> &'static str {
        "fixed64_field"
    }

    fn len_field(&self, m: &TestTypesRepeatedPacked) -> uint {
        m.get_fixed64_field().len()
    }

    fn get_rep_u64<'a>(&self, m: &'a TestTypesRepeatedPacked) -> &'a [u64] {
        m.get_fixed64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeatedPacked_sfixed32_field_acc_type;
static TestTypesRepeatedPacked_sfixed32_field_acc: TestTypesRepeatedPacked_sfixed32_field_acc_type = TestTypesRepeatedPacked_sfixed32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked> for TestTypesRepeatedPacked_sfixed32_field_acc_type {
    fn name(&self) -> &'static str {
        "sfixed32_field"
    }

    fn len_field(&self, m: &TestTypesRepeatedPacked) -> uint {
        m.get_sfixed32_field().len()
    }

    fn get_rep_i32<'a>(&self, m: &'a TestTypesRepeatedPacked) -> &'a [i32] {
        m.get_sfixed32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeatedPacked_sfixed64_field_acc_type;
static TestTypesRepeatedPacked_sfixed64_field_acc: TestTypesRepeatedPacked_sfixed64_field_acc_type = TestTypesRepeatedPacked_sfixed64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked> for TestTypesRepeatedPacked_sfixed64_field_acc_type {
    fn name(&self) -> &'static str {
        "sfixed64_field"
    }

    fn len_field(&self, m: &TestTypesRepeatedPacked) -> uint {
        m.get_sfixed64_field().len()
    }

    fn get_rep_i64<'a>(&self, m: &'a TestTypesRepeatedPacked) -> &'a [i64] {
        m.get_sfixed64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeatedPacked_bool_field_acc_type;
static TestTypesRepeatedPacked_bool_field_acc: TestTypesRepeatedPacked_bool_field_acc_type = TestTypesRepeatedPacked_bool_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked> for TestTypesRepeatedPacked_bool_field_acc_type {
    fn name(&self) -> &'static str {
        "bool_field"
    }

    fn len_field(&self, m: &TestTypesRepeatedPacked) -> uint {
        m.get_bool_field().len()
    }

    fn get_rep_bool<'a>(&self, m: &'a TestTypesRepeatedPacked) -> &'a [bool] {
        m.get_bool_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeatedPacked_string_field_acc_type;
static TestTypesRepeatedPacked_string_field_acc: TestTypesRepeatedPacked_string_field_acc_type = TestTypesRepeatedPacked_string_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked> for TestTypesRepeatedPacked_string_field_acc_type {
    fn name(&self) -> &'static str {
        "string_field"
    }

    fn len_field(&self, m: &TestTypesRepeatedPacked) -> uint {
        m.get_string_field().len()
    }

    fn get_rep_str<'a>(&self, m: &'a TestTypesRepeatedPacked) -> &'a [::std::string::String] {
        m.get_string_field()
    }
}

#[allow(non_camel_case_types)]
struct TestTypesRepeatedPacked_bytes_field_acc_type;
static TestTypesRepeatedPacked_bytes_field_acc: TestTypesRepeatedPacked_bytes_field_acc_type = TestTypesRepeatedPacked_bytes_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypesRepeatedPacked> for TestTypesRepeatedPacked_bytes_field_acc_type {
    fn name(&self) -> &'static str {
        "bytes_field"
    }

    fn len_field(&self, m: &TestTypesRepeatedPacked) -> uint {
        m.get_bytes_field().len()
    }

    fn get_rep_bytes<'a>(&self, m: &'a TestTypesRepeatedPacked) -> &'a [::std::vec::Vec<u8>] {
        m.get_bytes_field()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct TestDefaultValues {
    double_field: ::std::option::Option<f64>,
    float_field: ::std::option::Option<f32>,
    int32_field: ::std::option::Option<i32>,
    int64_field: ::std::option::Option<i64>,
    uint32_field: ::std::option::Option<u32>,
    uint64_field: ::std::option::Option<u64>,
    sint32_field: ::std::option::Option<i32>,
    sint64_field: ::std::option::Option<i64>,
    fixed32_field: ::std::option::Option<u32>,
    fixed64_field: ::std::option::Option<u64>,
    sfixed32_field: ::std::option::Option<i32>,
    sfixed64_field: ::std::option::Option<i64>,
    bool_field: ::std::option::Option<bool>,
    string_field: ::protobuf::SingularField<::std::string::String>,
    bytes_field: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    enum_field: ::std::option::Option<EnumForDefaultValue>,
    enum_field_without_default: ::std::option::Option<EnumForDefaultValue>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestDefaultValues {
    pub fn new() -> TestDefaultValues {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestDefaultValues {
        static mut instance: ::protobuf::lazy::Lazy<TestDefaultValues> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const TestDefaultValues };
        unsafe {
            instance.get(|| {
                TestDefaultValues {
                    double_field: ::std::option::None,
                    float_field: ::std::option::None,
                    int32_field: ::std::option::None,
                    int64_field: ::std::option::None,
                    uint32_field: ::std::option::None,
                    uint64_field: ::std::option::None,
                    sint32_field: ::std::option::None,
                    sint64_field: ::std::option::None,
                    fixed32_field: ::std::option::None,
                    fixed64_field: ::std::option::None,
                    sfixed32_field: ::std::option::None,
                    sfixed64_field: ::std::option::None,
                    bool_field: ::std::option::None,
                    string_field: ::protobuf::SingularField::none(),
                    bytes_field: ::protobuf::SingularField::none(),
                    enum_field: ::std::option::None,
                    enum_field_without_default: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional double double_field = 1;

    pub fn clear_double_field(&mut self) {
        self.double_field = None;
    }

    pub fn has_double_field(&self) -> bool {
        self.double_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_double_field(&mut self, v: f64) {
        self.double_field = Some(v);
    }

    pub fn get_double_field(&self) -> f64 {
        self.double_field.unwrap_or(1f64)
    }

    // optional float float_field = 2;

    pub fn clear_float_field(&mut self) {
        self.float_field = None;
    }

    pub fn has_float_field(&self) -> bool {
        self.float_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_float_field(&mut self, v: f32) {
        self.float_field = Some(v);
    }

    pub fn get_float_field(&self) -> f32 {
        self.float_field.unwrap_or(2f32)
    }

    // optional int32 int32_field = 3;

    pub fn clear_int32_field(&mut self) {
        self.int32_field = None;
    }

    pub fn has_int32_field(&self) -> bool {
        self.int32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_int32_field(&mut self, v: i32) {
        self.int32_field = Some(v);
    }

    pub fn get_int32_field(&self) -> i32 {
        self.int32_field.unwrap_or(3i32)
    }

    // optional int64 int64_field = 4;

    pub fn clear_int64_field(&mut self) {
        self.int64_field = None;
    }

    pub fn has_int64_field(&self) -> bool {
        self.int64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_int64_field(&mut self, v: i64) {
        self.int64_field = Some(v);
    }

    pub fn get_int64_field(&self) -> i64 {
        self.int64_field.unwrap_or(4i64)
    }

    // optional uint32 uint32_field = 5;

    pub fn clear_uint32_field(&mut self) {
        self.uint32_field = None;
    }

    pub fn has_uint32_field(&self) -> bool {
        self.uint32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uint32_field(&mut self, v: u32) {
        self.uint32_field = Some(v);
    }

    pub fn get_uint32_field(&self) -> u32 {
        self.uint32_field.unwrap_or(5u32)
    }

    // optional uint64 uint64_field = 6;

    pub fn clear_uint64_field(&mut self) {
        self.uint64_field = None;
    }

    pub fn has_uint64_field(&self) -> bool {
        self.uint64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uint64_field(&mut self, v: u64) {
        self.uint64_field = Some(v);
    }

    pub fn get_uint64_field(&self) -> u64 {
        self.uint64_field.unwrap_or(6u64)
    }

    // optional sint32 sint32_field = 7;

    pub fn clear_sint32_field(&mut self) {
        self.sint32_field = None;
    }

    pub fn has_sint32_field(&self) -> bool {
        self.sint32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sint32_field(&mut self, v: i32) {
        self.sint32_field = Some(v);
    }

    pub fn get_sint32_field(&self) -> i32 {
        self.sint32_field.unwrap_or(7i32)
    }

    // optional sint64 sint64_field = 8;

    pub fn clear_sint64_field(&mut self) {
        self.sint64_field = None;
    }

    pub fn has_sint64_field(&self) -> bool {
        self.sint64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sint64_field(&mut self, v: i64) {
        self.sint64_field = Some(v);
    }

    pub fn get_sint64_field(&self) -> i64 {
        self.sint64_field.unwrap_or(8i64)
    }

    // optional fixed32 fixed32_field = 9;

    pub fn clear_fixed32_field(&mut self) {
        self.fixed32_field = None;
    }

    pub fn has_fixed32_field(&self) -> bool {
        self.fixed32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fixed32_field(&mut self, v: u32) {
        self.fixed32_field = Some(v);
    }

    pub fn get_fixed32_field(&self) -> u32 {
        self.fixed32_field.unwrap_or(9u32)
    }

    // optional fixed64 fixed64_field = 10;

    pub fn clear_fixed64_field(&mut self) {
        self.fixed64_field = None;
    }

    pub fn has_fixed64_field(&self) -> bool {
        self.fixed64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fixed64_field(&mut self, v: u64) {
        self.fixed64_field = Some(v);
    }

    pub fn get_fixed64_field(&self) -> u64 {
        self.fixed64_field.unwrap_or(10u64)
    }

    // optional sfixed32 sfixed32_field = 11;

    pub fn clear_sfixed32_field(&mut self) {
        self.sfixed32_field = None;
    }

    pub fn has_sfixed32_field(&self) -> bool {
        self.sfixed32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sfixed32_field(&mut self, v: i32) {
        self.sfixed32_field = Some(v);
    }

    pub fn get_sfixed32_field(&self) -> i32 {
        self.sfixed32_field.unwrap_or(11i32)
    }

    // optional sfixed64 sfixed64_field = 12;

    pub fn clear_sfixed64_field(&mut self) {
        self.sfixed64_field = None;
    }

    pub fn has_sfixed64_field(&self) -> bool {
        self.sfixed64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sfixed64_field(&mut self, v: i64) {
        self.sfixed64_field = Some(v);
    }

    pub fn get_sfixed64_field(&self) -> i64 {
        self.sfixed64_field.unwrap_or(12i64)
    }

    // optional bool bool_field = 13;

    pub fn clear_bool_field(&mut self) {
        self.bool_field = None;
    }

    pub fn has_bool_field(&self) -> bool {
        self.bool_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bool_field(&mut self, v: bool) {
        self.bool_field = Some(v);
    }

    pub fn get_bool_field(&self) -> bool {
        self.bool_field.unwrap_or(true)
    }

    // optional string string_field = 14;

    pub fn clear_string_field(&mut self) {
        self.string_field.clear();
    }

    pub fn has_string_field(&self) -> bool {
        self.string_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_field(&mut self, v: ::std::string::String) {
        self.string_field = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_field(&'a mut self) -> &'a mut ::std::string::String {
        if self.string_field.is_none() {
            self.string_field.set_default();
        };
        self.string_field.as_mut().unwrap()
    }

    pub fn get_string_field(&'a self) -> &'a str {
        match self.string_field.as_ref() {
            Some(ref v) => v.as_slice(),
            None => "abc\n22",
        }
    }

    // optional bytes bytes_field = 15;

    pub fn clear_bytes_field(&mut self) {
        self.bytes_field.clear();
    }

    pub fn has_bytes_field(&self) -> bool {
        self.bytes_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytes_field(&mut self, v: ::std::vec::Vec<u8>) {
        self.bytes_field = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bytes_field(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.bytes_field.is_none() {
            self.bytes_field.set_default();
        };
        self.bytes_field.as_mut().unwrap()
    }

    pub fn get_bytes_field(&'a self) -> &'a [u8] {
        match self.bytes_field.as_ref() {
            Some(ref v) => v.as_slice(),
            None => b"cde\n33",
        }
    }

    // optional .shrug.EnumForDefaultValue enum_field = 16;

    pub fn clear_enum_field(&mut self) {
        self.enum_field = None;
    }

    pub fn has_enum_field(&self) -> bool {
        self.enum_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enum_field(&mut self, v: EnumForDefaultValue) {
        self.enum_field = Some(v);
    }

    pub fn get_enum_field(&self) -> EnumForDefaultValue {
        self.enum_field.unwrap_or(TWO)
    }

    // optional .shrug.EnumForDefaultValue enum_field_without_default = 17;

    pub fn clear_enum_field_without_default(&mut self) {
        self.enum_field_without_default = None;
    }

    pub fn has_enum_field_without_default(&self) -> bool {
        self.enum_field_without_default.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enum_field_without_default(&mut self, v: EnumForDefaultValue) {
        self.enum_field_without_default = Some(v);
    }

    pub fn get_enum_field_without_default(&self) -> EnumForDefaultValue {
        self.enum_field_without_default.unwrap_or(ONE)
    }
}

impl ::protobuf::Message for TestDefaultValues {
    fn new() -> TestDefaultValues {
        TestDefaultValues::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.double_field = Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.float_field = Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.int32_field = Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.int64_field = Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.uint32_field = Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.uint64_field = Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sint32());
                    self.sint32_field = Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sint64());
                    self.sint64_field = Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_fixed32());
                    self.fixed32_field = Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.fixed64_field = Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sfixed32());
                    self.sfixed32_field = Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sfixed64());
                    self.sfixed64_field = Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.bool_field = Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.string_field.set_default();
                    try!(is.read_string_into(tmp))
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.bytes_field.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = EnumForDefaultValue::new(try!(is.read_int32()));
                    self.enum_field = Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = EnumForDefaultValue::new(try!(is.read_int32()));
                    self.enum_field_without_default = Some(tmp);
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
        if self.double_field.is_some() {
            my_size += 9;
        };
        if self.float_field.is_some() {
            my_size += 5;
        };
        for value in self.int32_field.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.int64_field.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.uint32_field.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.uint64_field.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.sint32_field.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.sint64_field.iter() {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.fixed32_field.is_some() {
            my_size += 5;
        };
        if self.fixed64_field.is_some() {
            my_size += 9;
        };
        if self.sfixed32_field.is_some() {
            my_size += 5;
        };
        if self.sfixed64_field.is_some() {
            my_size += 9;
        };
        if self.bool_field.is_some() {
            my_size += 2;
        };
        for value in self.string_field.iter() {
            my_size += ::protobuf::rt::string_size(14, value.as_slice());
        };
        for value in self.bytes_field.iter() {
            my_size += ::protobuf::rt::bytes_size(15, value.as_slice());
        };
        for value in self.enum_field.iter() {
            my_size += ::protobuf::rt::enum_size(16, *value);
        };
        for value in self.enum_field_without_default.iter() {
            my_size += ::protobuf::rt::enum_size(17, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.double_field {
            Some(ref v) => {
                try!(os.write_double(1, *v));
            },
            None => {},
        };
        match self.float_field {
            Some(ref v) => {
                try!(os.write_float(2, *v));
            },
            None => {},
        };
        match self.int32_field {
            Some(ref v) => {
                try!(os.write_int32(3, *v));
            },
            None => {},
        };
        match self.int64_field {
            Some(ref v) => {
                try!(os.write_int64(4, *v));
            },
            None => {},
        };
        match self.uint32_field {
            Some(ref v) => {
                try!(os.write_uint32(5, *v));
            },
            None => {},
        };
        match self.uint64_field {
            Some(ref v) => {
                try!(os.write_uint64(6, *v));
            },
            None => {},
        };
        match self.sint32_field {
            Some(ref v) => {
                try!(os.write_sint32(7, *v));
            },
            None => {},
        };
        match self.sint64_field {
            Some(ref v) => {
                try!(os.write_sint64(8, *v));
            },
            None => {},
        };
        match self.fixed32_field {
            Some(ref v) => {
                try!(os.write_fixed32(9, *v));
            },
            None => {},
        };
        match self.fixed64_field {
            Some(ref v) => {
                try!(os.write_fixed64(10, *v));
            },
            None => {},
        };
        match self.sfixed32_field {
            Some(ref v) => {
                try!(os.write_sfixed32(11, *v));
            },
            None => {},
        };
        match self.sfixed64_field {
            Some(ref v) => {
                try!(os.write_sfixed64(12, *v));
            },
            None => {},
        };
        match self.bool_field {
            Some(ref v) => {
                try!(os.write_bool(13, *v));
            },
            None => {},
        };
        match self.string_field.as_ref() {
            Some(ref v) => {
                try!(os.write_string(14, v.as_slice()));
            },
            None => {},
        };
        match self.bytes_field.as_ref() {
            Some(ref v) => {
                try!(os.write_bytes(15, v.as_slice()));
            },
            None => {},
        };
        match self.enum_field {
            Some(ref v) => {
                try!(os.write_enum(16, *v as i32));
            },
            None => {},
        };
        match self.enum_field_without_default {
            Some(ref v) => {
                try!(os.write_enum(17, *v as i32));
            },
            None => {},
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
    fn descriptor_static(_: ::std::option::Option<TestDefaultValues>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultValues_double_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>) });
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultValues_float_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>) });
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultValues_int32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>) });
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultValues_int64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>) });
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultValues_uint32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>) });
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultValues_uint64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>) });
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultValues_sint32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>) });
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultValues_sint64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>) });
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultValues_fixed32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>) });
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultValues_fixed64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>) });
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultValues_sfixed32_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>) });
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultValues_sfixed64_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>) });
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultValues_bool_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>) });
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultValues_string_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>) });
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultValues_bytes_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>) });
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultValues_enum_field_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>) });
                fields.push(unsafe { ::std::mem::transmute(&TestDefaultValues_enum_field_without_default_acc as &'static ::protobuf::reflect::FieldAccessor<TestDefaultValues>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestDefaultValues>(
                    "TestDefaultValues",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestDefaultValues>()
    }
}

impl ::protobuf::Clear for TestDefaultValues {
    fn clear(&mut self) {
        self.clear_double_field();
        self.clear_float_field();
        self.clear_int32_field();
        self.clear_int64_field();
        self.clear_uint32_field();
        self.clear_uint64_field();
        self.clear_sint32_field();
        self.clear_sint64_field();
        self.clear_fixed32_field();
        self.clear_fixed64_field();
        self.clear_sfixed32_field();
        self.clear_sfixed64_field();
        self.clear_bool_field();
        self.clear_string_field();
        self.clear_bytes_field();
        self.clear_enum_field();
        self.clear_enum_field_without_default();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for TestDefaultValues {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestDefaultValues_double_field_acc_type;
static TestDefaultValues_double_field_acc: TestDefaultValues_double_field_acc_type = TestDefaultValues_double_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultValues> for TestDefaultValues_double_field_acc_type {
    fn name(&self) -> &'static str {
        "double_field"
    }

    fn has_field(&self, m: &TestDefaultValues) -> bool {
        m.has_double_field()
    }

    fn get_f64(&self, m: &TestDefaultValues) -> f64 {
        m.get_double_field()
    }
}

#[allow(non_camel_case_types)]
struct TestDefaultValues_float_field_acc_type;
static TestDefaultValues_float_field_acc: TestDefaultValues_float_field_acc_type = TestDefaultValues_float_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultValues> for TestDefaultValues_float_field_acc_type {
    fn name(&self) -> &'static str {
        "float_field"
    }

    fn has_field(&self, m: &TestDefaultValues) -> bool {
        m.has_float_field()
    }

    fn get_f32(&self, m: &TestDefaultValues) -> f32 {
        m.get_float_field()
    }
}

#[allow(non_camel_case_types)]
struct TestDefaultValues_int32_field_acc_type;
static TestDefaultValues_int32_field_acc: TestDefaultValues_int32_field_acc_type = TestDefaultValues_int32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultValues> for TestDefaultValues_int32_field_acc_type {
    fn name(&self) -> &'static str {
        "int32_field"
    }

    fn has_field(&self, m: &TestDefaultValues) -> bool {
        m.has_int32_field()
    }

    fn get_i32(&self, m: &TestDefaultValues) -> i32 {
        m.get_int32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestDefaultValues_int64_field_acc_type;
static TestDefaultValues_int64_field_acc: TestDefaultValues_int64_field_acc_type = TestDefaultValues_int64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultValues> for TestDefaultValues_int64_field_acc_type {
    fn name(&self) -> &'static str {
        "int64_field"
    }

    fn has_field(&self, m: &TestDefaultValues) -> bool {
        m.has_int64_field()
    }

    fn get_i64(&self, m: &TestDefaultValues) -> i64 {
        m.get_int64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestDefaultValues_uint32_field_acc_type;
static TestDefaultValues_uint32_field_acc: TestDefaultValues_uint32_field_acc_type = TestDefaultValues_uint32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultValues> for TestDefaultValues_uint32_field_acc_type {
    fn name(&self) -> &'static str {
        "uint32_field"
    }

    fn has_field(&self, m: &TestDefaultValues) -> bool {
        m.has_uint32_field()
    }

    fn get_u32(&self, m: &TestDefaultValues) -> u32 {
        m.get_uint32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestDefaultValues_uint64_field_acc_type;
static TestDefaultValues_uint64_field_acc: TestDefaultValues_uint64_field_acc_type = TestDefaultValues_uint64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultValues> for TestDefaultValues_uint64_field_acc_type {
    fn name(&self) -> &'static str {
        "uint64_field"
    }

    fn has_field(&self, m: &TestDefaultValues) -> bool {
        m.has_uint64_field()
    }

    fn get_u64(&self, m: &TestDefaultValues) -> u64 {
        m.get_uint64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestDefaultValues_sint32_field_acc_type;
static TestDefaultValues_sint32_field_acc: TestDefaultValues_sint32_field_acc_type = TestDefaultValues_sint32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultValues> for TestDefaultValues_sint32_field_acc_type {
    fn name(&self) -> &'static str {
        "sint32_field"
    }

    fn has_field(&self, m: &TestDefaultValues) -> bool {
        m.has_sint32_field()
    }

    fn get_i32(&self, m: &TestDefaultValues) -> i32 {
        m.get_sint32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestDefaultValues_sint64_field_acc_type;
static TestDefaultValues_sint64_field_acc: TestDefaultValues_sint64_field_acc_type = TestDefaultValues_sint64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultValues> for TestDefaultValues_sint64_field_acc_type {
    fn name(&self) -> &'static str {
        "sint64_field"
    }

    fn has_field(&self, m: &TestDefaultValues) -> bool {
        m.has_sint64_field()
    }

    fn get_i64(&self, m: &TestDefaultValues) -> i64 {
        m.get_sint64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestDefaultValues_fixed32_field_acc_type;
static TestDefaultValues_fixed32_field_acc: TestDefaultValues_fixed32_field_acc_type = TestDefaultValues_fixed32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultValues> for TestDefaultValues_fixed32_field_acc_type {
    fn name(&self) -> &'static str {
        "fixed32_field"
    }

    fn has_field(&self, m: &TestDefaultValues) -> bool {
        m.has_fixed32_field()
    }

    fn get_u32(&self, m: &TestDefaultValues) -> u32 {
        m.get_fixed32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestDefaultValues_fixed64_field_acc_type;
static TestDefaultValues_fixed64_field_acc: TestDefaultValues_fixed64_field_acc_type = TestDefaultValues_fixed64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultValues> for TestDefaultValues_fixed64_field_acc_type {
    fn name(&self) -> &'static str {
        "fixed64_field"
    }

    fn has_field(&self, m: &TestDefaultValues) -> bool {
        m.has_fixed64_field()
    }

    fn get_u64(&self, m: &TestDefaultValues) -> u64 {
        m.get_fixed64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestDefaultValues_sfixed32_field_acc_type;
static TestDefaultValues_sfixed32_field_acc: TestDefaultValues_sfixed32_field_acc_type = TestDefaultValues_sfixed32_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultValues> for TestDefaultValues_sfixed32_field_acc_type {
    fn name(&self) -> &'static str {
        "sfixed32_field"
    }

    fn has_field(&self, m: &TestDefaultValues) -> bool {
        m.has_sfixed32_field()
    }

    fn get_i32(&self, m: &TestDefaultValues) -> i32 {
        m.get_sfixed32_field()
    }
}

#[allow(non_camel_case_types)]
struct TestDefaultValues_sfixed64_field_acc_type;
static TestDefaultValues_sfixed64_field_acc: TestDefaultValues_sfixed64_field_acc_type = TestDefaultValues_sfixed64_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultValues> for TestDefaultValues_sfixed64_field_acc_type {
    fn name(&self) -> &'static str {
        "sfixed64_field"
    }

    fn has_field(&self, m: &TestDefaultValues) -> bool {
        m.has_sfixed64_field()
    }

    fn get_i64(&self, m: &TestDefaultValues) -> i64 {
        m.get_sfixed64_field()
    }
}

#[allow(non_camel_case_types)]
struct TestDefaultValues_bool_field_acc_type;
static TestDefaultValues_bool_field_acc: TestDefaultValues_bool_field_acc_type = TestDefaultValues_bool_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultValues> for TestDefaultValues_bool_field_acc_type {
    fn name(&self) -> &'static str {
        "bool_field"
    }

    fn has_field(&self, m: &TestDefaultValues) -> bool {
        m.has_bool_field()
    }

    fn get_bool(&self, m: &TestDefaultValues) -> bool {
        m.get_bool_field()
    }
}

#[allow(non_camel_case_types)]
struct TestDefaultValues_string_field_acc_type;
static TestDefaultValues_string_field_acc: TestDefaultValues_string_field_acc_type = TestDefaultValues_string_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultValues> for TestDefaultValues_string_field_acc_type {
    fn name(&self) -> &'static str {
        "string_field"
    }

    fn has_field(&self, m: &TestDefaultValues) -> bool {
        m.has_string_field()
    }

    fn get_str<'a>(&self, m: &'a TestDefaultValues) -> &'a str {
        m.get_string_field()
    }
}

#[allow(non_camel_case_types)]
struct TestDefaultValues_bytes_field_acc_type;
static TestDefaultValues_bytes_field_acc: TestDefaultValues_bytes_field_acc_type = TestDefaultValues_bytes_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultValues> for TestDefaultValues_bytes_field_acc_type {
    fn name(&self) -> &'static str {
        "bytes_field"
    }

    fn has_field(&self, m: &TestDefaultValues) -> bool {
        m.has_bytes_field()
    }

    fn get_bytes<'a>(&self, m: &'a TestDefaultValues) -> &'a [u8] {
        m.get_bytes_field()
    }
}

#[allow(non_camel_case_types)]
struct TestDefaultValues_enum_field_acc_type;
static TestDefaultValues_enum_field_acc: TestDefaultValues_enum_field_acc_type = TestDefaultValues_enum_field_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultValues> for TestDefaultValues_enum_field_acc_type {
    fn name(&self) -> &'static str {
        "enum_field"
    }

    fn has_field(&self, m: &TestDefaultValues) -> bool {
        m.has_enum_field()
    }

    fn get_enum<'a>(&self, m: &TestDefaultValues) -> &'static ::protobuf::reflect::EnumValueDescriptor {
        use protobuf::{ProtobufEnum};
        m.get_enum_field().descriptor()
    }
}

#[allow(non_camel_case_types)]
struct TestDefaultValues_enum_field_without_default_acc_type;
static TestDefaultValues_enum_field_without_default_acc: TestDefaultValues_enum_field_without_default_acc_type = TestDefaultValues_enum_field_without_default_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestDefaultValues> for TestDefaultValues_enum_field_without_default_acc_type {
    fn name(&self) -> &'static str {
        "enum_field_without_default"
    }

    fn has_field(&self, m: &TestDefaultValues) -> bool {
        m.has_enum_field_without_default()
    }

    fn get_enum<'a>(&self, m: &TestDefaultValues) -> &'static ::protobuf::reflect::EnumValueDescriptor {
        use protobuf::{ProtobufEnum};
        m.get_enum_field_without_default().descriptor()
    }
}

#[deriving(Clone,PartialEq,Eq,Show)]
pub enum TestEnumDescriptor {
    RED = 1,
    BLUE = 2,
    GREEN = 3,
}

impl TestEnumDescriptor {
    pub fn new(value: i32) -> TestEnumDescriptor {
        match value {
            1 => RED,
            2 => BLUE,
            3 => GREEN,
            _ => panic!()
        }
    }
}

impl ::protobuf::ProtobufEnum for TestEnumDescriptor {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn enum_descriptor_static(_: Option<TestEnumDescriptor>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::EnumDescriptor };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("TestEnumDescriptor", file_descriptor_proto())
            })
        }
    }
}

#[deriving(Clone,PartialEq,Eq,Show)]
pub enum EnumForDefaultValue {
    ONE = 1,
    TWO = 2,
    THREE = 3,
}

impl EnumForDefaultValue {
    pub fn new(value: i32) -> EnumForDefaultValue {
        match value {
            1 => ONE,
            2 => TWO,
            3 => THREE,
            _ => panic!()
        }
    }
}

impl ::protobuf::ProtobufEnum for EnumForDefaultValue {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn enum_descriptor_static(_: Option<EnumForDefaultValue>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::EnumDescriptor };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EnumForDefaultValue", file_descriptor_proto())
            })
        }
    }
}

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
    0x64, 0x73, 0x12, 0x09, 0x0a, 0x01, 0x61, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05, 0x22, 0x5f, 0x0a,
    0x11, 0x54, 0x65, 0x73, 0x74, 0x53, 0x65, 0x6c, 0x66, 0x52, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e,
    0x63, 0x65, 0x12, 0x24, 0x0a, 0x02, 0x72, 0x31, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x18,
    0x2e, 0x73, 0x68, 0x72, 0x75, 0x67, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x53, 0x65, 0x6c, 0x66, 0x52,
    0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x12, 0x24, 0x0a, 0x02, 0x72, 0x32, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x73, 0x68, 0x72, 0x75, 0x67, 0x2e, 0x54, 0x65, 0x73,
    0x74, 0x53, 0x65, 0x6c, 0x66, 0x52, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x22, 0x25,
    0x0a, 0x18, 0x54, 0x65, 0x73, 0x74, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x49, 0x6e, 0x73,
    0x74, 0x61, 0x6e, 0x63, 0x65, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x09, 0x0a, 0x01, 0x73, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x22, 0x45, 0x0a, 0x13, 0x54, 0x65, 0x73, 0x74, 0x44, 0x65, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x12, 0x2e, 0x0a, 0x05,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x73, 0x68,
    0x72, 0x75, 0x67, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x49,
    0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x22, 0x1f, 0x0a, 0x0e,
    0x54, 0x65, 0x73, 0x74, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x12, 0x0d,
    0x0a, 0x05, 0x73, 0x74, 0x75, 0x66, 0x66, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x05, 0x22, 0xdd, 0x02,
    0x0a, 0x11, 0x54, 0x65, 0x73, 0x74, 0x54, 0x79, 0x70, 0x65, 0x73, 0x53, 0x69, 0x6e, 0x67, 0x75,
    0x6c, 0x61, 0x72, 0x12, 0x14, 0x0a, 0x0c, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x5f, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x12, 0x13, 0x0a, 0x0b, 0x66, 0x6c, 0x6f,
    0x61, 0x74, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x12, 0x13,
    0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x12, 0x14, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74,
    0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x14,
    0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x04, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x11, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x69,
    0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x12,
    0x12, 0x15, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x09, 0x20, 0x01, 0x28, 0x07, 0x12, 0x15, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64,
    0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x06, 0x12, 0x16,
    0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x18, 0x0b, 0x20, 0x01, 0x28, 0x0f, 0x12, 0x16, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64,
    0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x10, 0x12, 0x12,
    0x0a, 0x0a, 0x62, 0x6f, 0x6f, 0x6c, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0d, 0x20, 0x01,
    0x28, 0x08, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x09, 0x12, 0x13, 0x0a, 0x0b, 0x62, 0x79, 0x74, 0x65,
    0x73, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x91, 0x03,
    0x0a, 0x11, 0x54, 0x65, 0x73, 0x74, 0x54, 0x79, 0x70, 0x65, 0x73, 0x52, 0x65, 0x70, 0x65, 0x61,
    0x74, 0x65, 0x64, 0x12, 0x18, 0x0a, 0x0c, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x5f, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x01, 0x42, 0x02, 0x10, 0x00, 0x12, 0x17, 0x0a,
    0x0b, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x02, 0x20, 0x03,
    0x28, 0x02, 0x42, 0x02, 0x10, 0x00, 0x12, 0x17, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x03, 0x20, 0x03, 0x28, 0x05, 0x42, 0x02, 0x10, 0x00, 0x12,
    0x17, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x04,
    0x20, 0x03, 0x28, 0x03, 0x42, 0x02, 0x10, 0x00, 0x12, 0x18, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74,
    0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0d, 0x42, 0x02,
    0x10, 0x00, 0x12, 0x18, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x18, 0x06, 0x20, 0x03, 0x28, 0x04, 0x42, 0x02, 0x10, 0x00, 0x12, 0x18, 0x0a, 0x0c,
    0x73, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x07, 0x20, 0x03,
    0x28, 0x11, 0x42, 0x02, 0x10, 0x00, 0x12, 0x18, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34,
    0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x08, 0x20, 0x03, 0x28, 0x12, 0x42, 0x02, 0x10, 0x00,
    0x12, 0x19, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x09, 0x20, 0x03, 0x28, 0x07, 0x42, 0x02, 0x10, 0x00, 0x12, 0x19, 0x0a, 0x0d, 0x66,
    0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0a, 0x20, 0x03,
    0x28, 0x06, 0x42, 0x02, 0x10, 0x00, 0x12, 0x1a, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64,
    0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0b, 0x20, 0x03, 0x28, 0x0f, 0x42, 0x02,
    0x10, 0x00, 0x12, 0x1a, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x18, 0x0c, 0x20, 0x03, 0x28, 0x10, 0x42, 0x02, 0x10, 0x00, 0x12, 0x16,
    0x0a, 0x0a, 0x62, 0x6f, 0x6f, 0x6c, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0d, 0x20, 0x03,
    0x28, 0x08, 0x42, 0x02, 0x10, 0x00, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67,
    0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0e, 0x20, 0x03, 0x28, 0x09, 0x12, 0x13, 0x0a, 0x0b,
    0x62, 0x79, 0x74, 0x65, 0x73, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0f, 0x20, 0x03, 0x28,
    0x0c, 0x22, 0x97, 0x03, 0x0a, 0x17, 0x54, 0x65, 0x73, 0x74, 0x54, 0x79, 0x70, 0x65, 0x73, 0x52,
    0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x64, 0x12, 0x18, 0x0a,
    0x0c, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x01, 0x42, 0x02, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x0b, 0x66, 0x6c, 0x6f, 0x61, 0x74,
    0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x02, 0x20, 0x03, 0x28, 0x02, 0x42, 0x02, 0x10, 0x01,
    0x12, 0x17, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18,
    0x03, 0x20, 0x03, 0x28, 0x05, 0x42, 0x02, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x0b, 0x69, 0x6e, 0x74,
    0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x04, 0x20, 0x03, 0x28, 0x03, 0x42, 0x02,
    0x10, 0x01, 0x12, 0x18, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0d, 0x42, 0x02, 0x10, 0x01, 0x12, 0x18, 0x0a, 0x0c,
    0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x06, 0x20, 0x03,
    0x28, 0x04, 0x42, 0x02, 0x10, 0x01, 0x12, 0x18, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74, 0x33, 0x32,
    0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x07, 0x20, 0x03, 0x28, 0x11, 0x42, 0x02, 0x10, 0x01,
    0x12, 0x18, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x18, 0x08, 0x20, 0x03, 0x28, 0x12, 0x42, 0x02, 0x10, 0x01, 0x12, 0x19, 0x0a, 0x0d, 0x66, 0x69,
    0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x09, 0x20, 0x03, 0x28,
    0x07, 0x42, 0x02, 0x10, 0x01, 0x12, 0x19, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34,
    0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0a, 0x20, 0x03, 0x28, 0x06, 0x42, 0x02, 0x10, 0x01,
    0x12, 0x1a, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x18, 0x0b, 0x20, 0x03, 0x28, 0x0f, 0x42, 0x02, 0x10, 0x01, 0x12, 0x1a, 0x0a, 0x0e,
    0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0c,
    0x20, 0x03, 0x28, 0x10, 0x42, 0x02, 0x10, 0x01, 0x12, 0x16, 0x0a, 0x0a, 0x62, 0x6f, 0x6f, 0x6c,
    0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0d, 0x20, 0x03, 0x28, 0x08, 0x42, 0x02, 0x10, 0x01,
    0x12, 0x14, 0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x18, 0x0e, 0x20, 0x03, 0x28, 0x09, 0x12, 0x13, 0x0a, 0x0b, 0x62, 0x79, 0x74, 0x65, 0x73, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0f, 0x20, 0x03, 0x28, 0x0c, 0x22, 0x90, 0x04, 0x0a, 0x11,
    0x54, 0x65, 0x73, 0x74, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x56, 0x61, 0x6c, 0x75, 0x65,
    0x73, 0x12, 0x17, 0x0a, 0x0c, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x3a, 0x01, 0x31, 0x12, 0x16, 0x0a, 0x0b, 0x66, 0x6c,
    0x6f, 0x61, 0x74, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x3a,
    0x01, 0x32, 0x12, 0x16, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x3a, 0x01, 0x33, 0x12, 0x16, 0x0a, 0x0b, 0x69, 0x6e,
    0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x3a,
    0x01, 0x34, 0x12, 0x17, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x3a, 0x01, 0x35, 0x12, 0x17, 0x0a, 0x0c, 0x75,
    0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x04, 0x3a, 0x01, 0x36, 0x12, 0x17, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x11, 0x3a, 0x01, 0x37, 0x12, 0x17, 0x0a,
    0x0c, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x12, 0x3a, 0x01, 0x38, 0x12, 0x18, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33,
    0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x09, 0x20, 0x01, 0x28, 0x07, 0x3a, 0x01, 0x39,
    0x12, 0x19, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x06, 0x3a, 0x02, 0x31, 0x30, 0x12, 0x1a, 0x0a, 0x0e, 0x73,
    0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0b, 0x20,
    0x01, 0x28, 0x0f, 0x3a, 0x02, 0x31, 0x31, 0x12, 0x1a, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65,
    0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x10, 0x3a,
    0x02, 0x31, 0x32, 0x12, 0x18, 0x0a, 0x0a, 0x62, 0x6f, 0x6f, 0x6c, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x04, 0x74, 0x72, 0x75, 0x65, 0x12, 0x1c, 0x0a,
    0x0c, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0e, 0x20,
    0x01, 0x28, 0x09, 0x3a, 0x06, 0x61, 0x62, 0x63, 0x0a, 0x32, 0x32, 0x12, 0x1c, 0x0a, 0x0b, 0x62,
    0x79, 0x74, 0x65, 0x73, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0c,
    0x3a, 0x07, 0x63, 0x64, 0x65, 0x5c, 0x6e, 0x33, 0x33, 0x12, 0x33, 0x0a, 0x0a, 0x65, 0x6e, 0x75,
    0x6d, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1a, 0x2e,
    0x73, 0x68, 0x72, 0x75, 0x67, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x46, 0x6f, 0x72, 0x44, 0x65, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x03, 0x54, 0x57, 0x4f, 0x12, 0x3e,
    0x0a, 0x1a, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x77, 0x69, 0x74,
    0x68, 0x6f, 0x75, 0x74, 0x5f, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x18, 0x11, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x1a, 0x2e, 0x73, 0x68, 0x72, 0x75, 0x67, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x46,
    0x6f, 0x72, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x2a, 0x32,
    0x0a, 0x12, 0x54, 0x65, 0x73, 0x74, 0x45, 0x6e, 0x75, 0x6d, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x6f, 0x72, 0x12, 0x07, 0x0a, 0x03, 0x52, 0x45, 0x44, 0x10, 0x01, 0x12, 0x08, 0x0a,
    0x04, 0x42, 0x4c, 0x55, 0x45, 0x10, 0x02, 0x12, 0x09, 0x0a, 0x05, 0x47, 0x52, 0x45, 0x45, 0x4e,
    0x10, 0x03, 0x2a, 0x32, 0x0a, 0x13, 0x45, 0x6e, 0x75, 0x6d, 0x46, 0x6f, 0x72, 0x44, 0x65, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x07, 0x0a, 0x03, 0x4f, 0x4e, 0x45,
    0x10, 0x01, 0x12, 0x07, 0x0a, 0x03, 0x54, 0x57, 0x4f, 0x10, 0x02, 0x12, 0x09, 0x0a, 0x05, 0x54,
    0x48, 0x52, 0x45, 0x45, 0x10, 0x03, 0x4a, 0xec, 0x49, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0x8a,
    0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x0d, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x04, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x02, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x04,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x13, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x03, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x06, 0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x06, 0x08,
    0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x07, 0x04, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x07, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x07, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x14, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x07, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0a, 0x00,
    0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x0d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x0b, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0b, 0x13, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0b, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x0e, 0x00, 0x10, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x0f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x0f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f,
    0x13, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x17, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x08, 0x12, 0x03, 0x0f, 0x19, 0x26, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x0f, 0x1a, 0x25, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x0f, 0x1a,
    0x20, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x0f, 0x1a, 0x20, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x1a, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x03, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x21, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x13, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x13,
    0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x14, 0x04, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x14, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x14, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x14, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x15, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x15,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x15, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x13, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x15, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x08, 0x12, 0x03, 0x15, 0x1e, 0x2b, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x04,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x15, 0x1f, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x04, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x15, 0x1f, 0x25, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x04, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x15, 0x1f, 0x25,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x04, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x15, 0x1f, 0x25, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x04, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x15, 0x26, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x18, 0x00,
    0x1a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x18, 0x08, 0x11, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x19, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x19, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x19, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x19, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x19, 0x19, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x1c, 0x00, 0x1e, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x1d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x1d, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d,
    0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1d, 0x16, 0x17,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x20, 0x00, 0x22, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x07, 0x01, 0x12, 0x03, 0x20, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00,
    0x12, 0x03, 0x21, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x21, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x21, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x13, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x17, 0x18, 0x0a, 0x24, 0x0a,
    0x02, 0x04, 0x08, 0x12, 0x04, 0x25, 0x00, 0x28, 0x01, 0x1a, 0x18, 0x20, 0x6a, 0x75, 0x73, 0x74,
    0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x69, 0x74, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x69, 0x6c,
    0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x25, 0x08, 0x19, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x26, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x26, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x26, 0x0d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x26, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x26, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x27, 0x04,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x27, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x06, 0x12, 0x03, 0x27, 0x0d, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x27, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x27, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12,
    0x04, 0x2a, 0x00, 0x2c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x2a, 0x08,
    0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x04, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x14, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x2b, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x2e, 0x00,
    0x30, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x2f, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x2f, 0x0d, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x2f, 0x26, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x2f, 0x2e, 0x2f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x32, 0x00, 0x34, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x32, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x00, 0x12, 0x03, 0x33, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x33, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x33, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x33,
    0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x33, 0x1b, 0x1d,
    0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x36, 0x00, 0x3a, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x05, 0x00, 0x01, 0x12, 0x03, 0x36, 0x05, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x37, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x37, 0x04, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x37, 0x0a,
    0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x38, 0x04, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x38, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x38, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x39, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x39, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x39, 0x0c, 0x0d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x3c, 0x00, 0x4c, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x3c, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0c, 0x02, 0x00, 0x12, 0x03, 0x3d, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x3d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x3d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3d,
    0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3d, 0x23, 0x24,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x3e, 0x04, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x3e, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x3e, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x03, 0x3f,
    0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3f, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3f, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3f, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c,
    0x02, 0x03, 0x12, 0x03, 0x40, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x40, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x40, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x01, 0x12, 0x03, 0x40, 0x13,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x03, 0x12, 0x03, 0x40, 0x21, 0x22, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x04, 0x12, 0x03, 0x41, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x04, 0x04, 0x12, 0x03, 0x41, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x41, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x41, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x41, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x05, 0x12, 0x03, 0x42, 0x04,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x04, 0x12, 0x03, 0x42, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x05, 0x12, 0x03, 0x42, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x05, 0x01, 0x12, 0x03, 0x42, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x05, 0x03, 0x12, 0x03, 0x42, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02,
    0x06, 0x12, 0x03, 0x43, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x04, 0x12,
    0x03, 0x43, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x05, 0x12, 0x03, 0x43,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x01, 0x12, 0x03, 0x43, 0x14, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x03, 0x12, 0x03, 0x43, 0x23, 0x24, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0c, 0x02, 0x07, 0x12, 0x03, 0x44, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x07, 0x04, 0x12, 0x03, 0x44, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x07, 0x05, 0x12, 0x03, 0x44, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x44, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x07, 0x03, 0x12, 0x03,
    0x44, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x08, 0x12, 0x03, 0x45, 0x04, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x08, 0x04, 0x12, 0x03, 0x45, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x08, 0x05, 0x12, 0x03, 0x45, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x08, 0x01, 0x12, 0x03, 0x45, 0x15, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x08, 0x03, 0x12, 0x03, 0x45, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x09,
    0x12, 0x03, 0x46, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x09, 0x04, 0x12, 0x03,
    0x46, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x09, 0x05, 0x12, 0x03, 0x46, 0x0d,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x09, 0x01, 0x12, 0x03, 0x46, 0x15, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x09, 0x03, 0x12, 0x03, 0x46, 0x25, 0x27, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0c, 0x02, 0x0a, 0x12, 0x03, 0x47, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x0a, 0x04, 0x12, 0x03, 0x47, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0a,
    0x05, 0x12, 0x03, 0x47, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0a, 0x01, 0x12,
    0x03, 0x47, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x47,
    0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x0b, 0x12, 0x03, 0x48, 0x04, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x48, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x48, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x48, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x0b, 0x03, 0x12, 0x03, 0x48, 0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x0c, 0x12,
    0x03, 0x49, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x49,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x49, 0x0d, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x49, 0x12, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x49, 0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0c, 0x02, 0x0d, 0x12, 0x03, 0x4a, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x0d, 0x04, 0x12, 0x03, 0x4a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0d, 0x05,
    0x12, 0x03, 0x4a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0d, 0x01, 0x12, 0x03,
    0x4a, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x4a, 0x23,
    0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x0e, 0x12, 0x03, 0x4b, 0x04, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x4b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x4b, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x0e, 0x01, 0x12, 0x03, 0x4b, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0e,
    0x03, 0x12, 0x03, 0x4b, 0x21, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x04, 0x4e, 0x00,
    0x5e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x03, 0x4e, 0x08, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x4f, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x4f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x4f, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x4f, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x08, 0x12, 0x03, 0x4f, 0x25,
    0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x4f,
    0x26, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x03, 0x4f, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x4f, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4f, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x0d, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x4f, 0x2d, 0x32, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x03, 0x50, 0x04, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x50, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x50, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x50, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x03, 0x50,
    0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x08, 0x12, 0x03, 0x50, 0x23, 0x31,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x50, 0x24,
    0x30, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x50, 0x24, 0x2a, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x50, 0x24, 0x2a, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x01, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x50, 0x24, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x50, 0x2b, 0x30, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0d, 0x02, 0x02, 0x12, 0x03, 0x51, 0x04, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x51, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x51, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x51, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x03, 0x12, 0x03, 0x51, 0x21,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x08, 0x12, 0x03, 0x51, 0x23, 0x31, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x51, 0x24, 0x30,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x51,
    0x24, 0x2a, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x51, 0x24, 0x2a, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x02, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x51, 0x24, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02,
    0x02, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x51, 0x2b, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0d, 0x02, 0x03, 0x12, 0x03, 0x52, 0x04, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x52, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x52, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x01, 0x12, 0x03, 0x52,
    0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x03, 0x12, 0x03, 0x52, 0x21, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x08, 0x12, 0x03, 0x52, 0x23, 0x31, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x0d, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x52, 0x24, 0x30, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x52, 0x24,
    0x2a, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x52, 0x24, 0x2a, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x52, 0x24, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x03,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x52, 0x2b, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d,
    0x02, 0x04, 0x12, 0x03, 0x53, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x53, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x53, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x01, 0x12, 0x03, 0x53, 0x14,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x03, 0x12, 0x03, 0x53, 0x23, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x08, 0x12, 0x03, 0x53, 0x25, 0x33, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x0d, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x53, 0x26, 0x32, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x0d, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x53, 0x26, 0x2c,
    0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x53, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x53, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x04, 0x08,
    0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x53, 0x2d, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02,
    0x05, 0x12, 0x03, 0x54, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x04, 0x12,
    0x03, 0x54, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x05, 0x12, 0x03, 0x54,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x01, 0x12, 0x03, 0x54, 0x14, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x03, 0x12, 0x03, 0x54, 0x23, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x08, 0x12, 0x03, 0x54, 0x25, 0x33, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x0d, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x54, 0x26, 0x32, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x0d, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x54, 0x26, 0x2c, 0x0a,
    0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x54,
    0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x54, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x05, 0x08, 0xe7,
    0x07, 0x00, 0x03, 0x12, 0x03, 0x54, 0x2d, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x06,
    0x12, 0x03, 0x55, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x04, 0x12, 0x03,
    0x55, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x05, 0x12, 0x03, 0x55, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x01, 0x12, 0x03, 0x55, 0x14, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x03, 0x12, 0x03, 0x55, 0x23, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x06, 0x08, 0x12, 0x03, 0x55, 0x25, 0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x0d, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x55, 0x26, 0x32, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x0d, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x55, 0x26, 0x2c, 0x0a, 0x11,
    0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x55, 0x26,
    0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x55, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x06, 0x08, 0xe7, 0x07,
    0x00, 0x03, 0x12, 0x03, 0x55, 0x2d, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x07, 0x12,
    0x03, 0x56, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x04, 0x12, 0x03, 0x56,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x05, 0x12, 0x03, 0x56, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x01, 0x12, 0x03, 0x56, 0x14, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x03, 0x12, 0x03, 0x56, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x07, 0x08, 0x12, 0x03, 0x56, 0x25, 0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d,
    0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x56, 0x26, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x0d, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x56, 0x26, 0x2c, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x0d, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x56, 0x26, 0x2c,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x56, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x56, 0x2d, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x08, 0x12, 0x03,
    0x57, 0x04, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x04, 0x12, 0x03, 0x57, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x05, 0x12, 0x03, 0x57, 0x0d, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x01, 0x12, 0x03, 0x57, 0x15, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x08, 0x03, 0x12, 0x03, 0x57, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x08, 0x08, 0x12, 0x03, 0x57, 0x27, 0x35, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02,
    0x08, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x57, 0x28, 0x34, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d,
    0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x57, 0x28, 0x2e, 0x0a, 0x11, 0x0a, 0x0a,
    0x04, 0x0d, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x57, 0x28, 0x2e, 0x0a,
    0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x57, 0x28, 0x2e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x03,
    0x12, 0x03, 0x57, 0x2f, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x09, 0x12, 0x03, 0x58,
    0x04, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x04, 0x12, 0x03, 0x58, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x05, 0x12, 0x03, 0x58, 0x0d, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x01, 0x12, 0x03, 0x58, 0x15, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x09, 0x03, 0x12, 0x03, 0x58, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x09, 0x08, 0x12, 0x03, 0x58, 0x28, 0x36, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x09,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x58, 0x29, 0x35, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02,
    0x09, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x58, 0x29, 0x2f, 0x0a, 0x11, 0x0a, 0x0a, 0x04,
    0x0d, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x58, 0x29, 0x2f, 0x0a, 0x12,
    0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x58,
    0x29, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12,
    0x03, 0x58, 0x30, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0a, 0x12, 0x03, 0x59, 0x04,
    0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x59, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x59, 0x0d, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x59, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x59, 0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x0a, 0x08, 0x12, 0x03, 0x59, 0x2a, 0x38, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x0a, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x59, 0x2b, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x0a,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x59, 0x2b, 0x31, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d,
    0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x59, 0x2b, 0x31, 0x0a, 0x12, 0x0a,
    0x0b, 0x04, 0x0d, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x59, 0x2b,
    0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03,
    0x59, 0x32, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0b, 0x12, 0x03, 0x5a, 0x04, 0x39,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x5a, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x5a, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x5a, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x0b, 0x03, 0x12, 0x03, 0x5a, 0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b,
    0x08, 0x12, 0x03, 0x5a, 0x2a, 0x38, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x0b, 0x08, 0xe7,
    0x07, 0x00, 0x12, 0x03, 0x5a, 0x2b, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x0b, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x5a, 0x2b, 0x31, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02,
    0x0b, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x5a, 0x2b, 0x31, 0x0a, 0x12, 0x0a, 0x0b,
    0x04, 0x0d, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5a, 0x2b, 0x31,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x5a,
    0x32, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0c, 0x12, 0x03, 0x5b, 0x04, 0x31, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x5b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x5b, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x5b, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x0c, 0x03, 0x12, 0x03, 0x5b, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x08,
    0x12, 0x03, 0x5b, 0x22, 0x30, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x0c, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x5b, 0x23, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x0c, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x5b, 0x23, 0x29, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x0c,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x5b, 0x23, 0x29, 0x0a, 0x12, 0x0a, 0x0b, 0x04,
    0x0d, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5b, 0x23, 0x29, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x5b, 0x2a,
    0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0d, 0x12, 0x03, 0x5c, 0x04, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x5c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x5c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x0d, 0x01, 0x12, 0x03, 0x5c, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d,
    0x03, 0x12, 0x03, 0x5c, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0e, 0x12, 0x03,
    0x5d, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x5d, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x5d, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x5d, 0x13, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x5d, 0x21, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x0e, 0x12, 0x04, 0x60, 0x00, 0x70, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x03,
    0x60, 0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x61, 0x04, 0x33,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03, 0x61, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x03, 0x61, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x61, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x61, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00,
    0x08, 0x12, 0x03, 0x61, 0x25, 0x32, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x12, 0x03, 0x61, 0x26, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x61, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x61, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b,
    0x04, 0x0e, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x61, 0x26, 0x2c,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x61,
    0x2d, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x03, 0x62, 0x04, 0x31, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x03, 0x62, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x01, 0x05, 0x12, 0x03, 0x62, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x01, 0x01, 0x12, 0x03, 0x62, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x62, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x08,
    0x12, 0x03, 0x62, 0x23, 0x30, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x62, 0x24, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x01, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x62, 0x24, 0x2a, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x01,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x62, 0x24, 0x2a, 0x0a, 0x12, 0x0a, 0x0b, 0x04,
    0x0e, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x62, 0x24, 0x2a, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x62, 0x2b,
    0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x02, 0x12, 0x03, 0x63, 0x04, 0x31, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x04, 0x12, 0x03, 0x63, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x02, 0x05, 0x12, 0x03, 0x63, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x63, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x63, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x08, 0x12,
    0x03, 0x63, 0x23, 0x30, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x63, 0x24, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x02, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x63, 0x24, 0x2a, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x02, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x63, 0x24, 0x2a, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e,
    0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x63, 0x24, 0x2a, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x0e, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x63, 0x2b, 0x2f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x03, 0x12, 0x03, 0x64, 0x04, 0x31, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x03, 0x04, 0x12, 0x03, 0x64, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x03, 0x05, 0x12, 0x03, 0x64, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x64, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x64, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x08, 0x12, 0x03,
    0x64, 0x23, 0x30, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x12,
    0x03, 0x64, 0x24, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x64, 0x24, 0x2a, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x03, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x64, 0x24, 0x2a, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02,
    0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x64, 0x24, 0x2a, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x0e, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x64, 0x2b, 0x2f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x04, 0x12, 0x03, 0x65, 0x04, 0x33, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x04, 0x04, 0x12, 0x03, 0x65, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x65, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x65, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x65, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x08, 0x12, 0x03, 0x65,
    0x25, 0x32, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03,
    0x65, 0x26, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x12, 0x03, 0x65, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x04, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x65, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x04,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x65, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x0e, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x65, 0x2d, 0x31, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0e, 0x02, 0x05, 0x12, 0x03, 0x66, 0x04, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x05, 0x04, 0x12, 0x03, 0x66, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x05, 0x05, 0x12, 0x03, 0x66, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x66, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x03, 0x12, 0x03,
    0x66, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x08, 0x12, 0x03, 0x66, 0x25,
    0x32, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x66,
    0x26, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x03, 0x66, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x66, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x05, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x66, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x0e, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x66, 0x2d, 0x31, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0e, 0x02, 0x06, 0x12, 0x03, 0x67, 0x04, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x06, 0x04, 0x12, 0x03, 0x67, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x67, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x67, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x06, 0x03, 0x12, 0x03, 0x67,
    0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x06, 0x08, 0x12, 0x03, 0x67, 0x25, 0x32,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x67, 0x26,
    0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x67, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x67, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x06, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x67, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e,
    0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x67, 0x2d, 0x31, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0e, 0x02, 0x07, 0x12, 0x03, 0x68, 0x04, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x07, 0x04, 0x12, 0x03, 0x68, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x07, 0x05,
    0x12, 0x03, 0x68, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x68, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x07, 0x03, 0x12, 0x03, 0x68, 0x23,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x07, 0x08, 0x12, 0x03, 0x68, 0x25, 0x32, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x68, 0x26, 0x31,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x68,
    0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x68, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x07, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x68, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02,
    0x07, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x68, 0x2d, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0e, 0x02, 0x08, 0x12, 0x03, 0x69, 0x04, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x08,
    0x04, 0x12, 0x03, 0x69, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x08, 0x05, 0x12,
    0x03, 0x69, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x08, 0x01, 0x12, 0x03, 0x69,
    0x15, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x08, 0x03, 0x12, 0x03, 0x69, 0x25, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x08, 0x08, 0x12, 0x03, 0x69, 0x27, 0x34, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x0e, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x69, 0x28, 0x33, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x69, 0x28,
    0x2e, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x69, 0x28, 0x2e, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x69, 0x28, 0x2e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x08,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x69, 0x2f, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x09, 0x12, 0x03, 0x6a, 0x04, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x09, 0x04,
    0x12, 0x03, 0x6a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x09, 0x05, 0x12, 0x03,
    0x6a, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x09, 0x01, 0x12, 0x03, 0x6a, 0x15,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x09, 0x03, 0x12, 0x03, 0x6a, 0x25, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x09, 0x08, 0x12, 0x03, 0x6a, 0x28, 0x35, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x0e, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x6a, 0x29, 0x34, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x0e, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x6a, 0x29, 0x2f,
    0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x6a, 0x29, 0x2f, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x6a, 0x29, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x09, 0x08,
    0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x6a, 0x30, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02,
    0x0a, 0x12, 0x03, 0x6b, 0x04, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0a, 0x04, 0x12,
    0x03, 0x6b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x6b,
    0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x6b, 0x16, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x6b, 0x27, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0a, 0x08, 0x12, 0x03, 0x6b, 0x2a, 0x37, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x0e, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x6b, 0x2b, 0x36, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x0e, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x6b, 0x2b, 0x31, 0x0a,
    0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x6b,
    0x2b, 0x31, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x6b, 0x2b, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x0a, 0x08, 0xe7,
    0x07, 0x00, 0x03, 0x12, 0x03, 0x6b, 0x32, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x0b,
    0x12, 0x03, 0x6c, 0x04, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0b, 0x04, 0x12, 0x03,
    0x6c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x6c, 0x0d,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x6c, 0x16, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x6c, 0x27, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x0b, 0x08, 0x12, 0x03, 0x6c, 0x2a, 0x37, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x0e, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x6c, 0x2b, 0x36, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x0e, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x6c, 0x2b, 0x31, 0x0a, 0x11,
    0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x6c, 0x2b,
    0x31, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x6c, 0x2b, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x0b, 0x08, 0xe7, 0x07,
    0x00, 0x03, 0x12, 0x03, 0x6c, 0x32, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x0c, 0x12,
    0x03, 0x6d, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x6d,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x6d, 0x0d, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x6d, 0x12, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x6d, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x0c, 0x08, 0x12, 0x03, 0x6d, 0x22, 0x2f, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e,
    0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x6d, 0x23, 0x2e, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x0e, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x6d, 0x23, 0x29, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x0e, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x6d, 0x23, 0x29,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x6d, 0x23, 0x29, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x6d, 0x2a, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x0d, 0x12, 0x03,
    0x6e, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x6e, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x6e, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x6e, 0x14, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x6e, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0e, 0x02, 0x0e, 0x12, 0x03, 0x6f, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0e,
    0x04, 0x12, 0x03, 0x6f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0e, 0x05, 0x12,
    0x03, 0x6f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x6f,
    0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x6f, 0x21, 0x23,
    0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x72, 0x00, 0x76, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x05, 0x01, 0x01, 0x12, 0x03, 0x72, 0x05, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x73, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x73, 0x04, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x73, 0x0a,
    0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x74, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x74, 0x04, 0x07, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x74, 0x0a, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x75, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x75, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x75, 0x0c, 0x0d, 0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x05, 0x78, 0x00, 0x8a, 0x01, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x03, 0x78, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0f, 0x02, 0x00, 0x12, 0x03, 0x79, 0x04, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x79, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x79, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x79, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x03, 0x79, 0x23,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x08, 0x12, 0x03, 0x79, 0x31, 0x3f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x07, 0x12, 0x03, 0x79, 0x3d, 0x3e, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x03, 0x7a, 0x04, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x7a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x7a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x7a, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7a,
    0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x08, 0x12, 0x03, 0x7a, 0x31, 0x3f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x07, 0x12, 0x03, 0x7a, 0x3d, 0x3e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0f, 0x02, 0x02, 0x12, 0x03, 0x7b, 0x04, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x02, 0x04, 0x12, 0x03, 0x7b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x7b, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x7b, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x7b, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x08, 0x12, 0x03, 0x7b, 0x31,
    0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x07, 0x12, 0x03, 0x7b, 0x3d, 0x3e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x03, 0x12, 0x03, 0x7c, 0x04, 0x40, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x03, 0x04, 0x12, 0x03, 0x7c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x7c, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x7c, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x7c, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x08, 0x12, 0x03, 0x7c,
    0x31, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x07, 0x12, 0x03, 0x7c, 0x3d, 0x3e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x04, 0x12, 0x03, 0x7d, 0x04, 0x40, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x04, 0x04, 0x12, 0x03, 0x7d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x04, 0x05, 0x12, 0x03, 0x7d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x7d, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x7d, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x08, 0x12, 0x03,
    0x7d, 0x31, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x07, 0x12, 0x03, 0x7d, 0x3d,
    0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x05, 0x12, 0x03, 0x7e, 0x04, 0x40, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x04, 0x12, 0x03, 0x7e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x05, 0x05, 0x12, 0x03, 0x7e, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x7e, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x7e, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x08, 0x12,
    0x03, 0x7e, 0x31, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x07, 0x12, 0x03, 0x7e,
    0x3d, 0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x06, 0x12, 0x03, 0x7f, 0x04, 0x40, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x04, 0x12, 0x03, 0x7f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x06, 0x05, 0x12, 0x03, 0x7f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x06, 0x01, 0x12, 0x03, 0x7f, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x7f, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x08,
    0x12, 0x03, 0x7f, 0x31, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x07, 0x12, 0x03,
    0x7f, 0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x07, 0x12, 0x04, 0x80, 0x01, 0x04,
    0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x04, 0x12, 0x04, 0x80, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x05, 0x12, 0x04, 0x80, 0x01, 0x0d, 0x13, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x01, 0x12, 0x04, 0x80, 0x01, 0x14, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x03, 0x12, 0x04, 0x80, 0x01, 0x23, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x07, 0x08, 0x12, 0x04, 0x80, 0x01, 0x31, 0x3f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x07, 0x07, 0x12, 0x04, 0x80, 0x01, 0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x08, 0x12, 0x04, 0x81, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x08, 0x04, 0x12, 0x04, 0x81, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08,
    0x05, 0x12, 0x04, 0x81, 0x01, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x01,
    0x12, 0x04, 0x81, 0x01, 0x15, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x03, 0x12,
    0x04, 0x81, 0x01, 0x25, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x08, 0x12, 0x04,
    0x81, 0x01, 0x31, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x07, 0x12, 0x04, 0x81,
    0x01, 0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x09, 0x12, 0x04, 0x82, 0x01, 0x04,
    0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x04, 0x12, 0x04, 0x82, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x05, 0x12, 0x04, 0x82, 0x01, 0x0d, 0x14, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x01, 0x12, 0x04, 0x82, 0x01, 0x15, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x03, 0x12, 0x04, 0x82, 0x01, 0x25, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x09, 0x08, 0x12, 0x04, 0x82, 0x01, 0x31, 0x3f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x09, 0x07, 0x12, 0x04, 0x82, 0x01, 0x3c, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x0a, 0x12, 0x04, 0x83, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x0a, 0x04, 0x12, 0x04, 0x83, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a,
    0x05, 0x12, 0x04, 0x83, 0x01, 0x0d, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a, 0x01,
    0x12, 0x04, 0x83, 0x01, 0x16, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a, 0x03, 0x12,
    0x04, 0x83, 0x01, 0x27, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a, 0x08, 0x12, 0x04,
    0x83, 0x01, 0x31, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a, 0x07, 0x12, 0x04, 0x83,
    0x01, 0x3c, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x0b, 0x12, 0x04, 0x84, 0x01, 0x04,
    0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0b, 0x04, 0x12, 0x04, 0x84, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0b, 0x05, 0x12, 0x04, 0x84, 0x01, 0x0d, 0x15, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0b, 0x01, 0x12, 0x04, 0x84, 0x01, 0x16, 0x24, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0b, 0x03, 0x12, 0x04, 0x84, 0x01, 0x27, 0x29, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x0b, 0x08, 0x12, 0x04, 0x84, 0x01, 0x31, 0x3f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x0b, 0x07, 0x12, 0x04, 0x84, 0x01, 0x3c, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x0c, 0x12, 0x04, 0x85, 0x01, 0x04, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x0c, 0x04, 0x12, 0x04, 0x85, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0c,
    0x05, 0x12, 0x04, 0x85, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0c, 0x01,
    0x12, 0x04, 0x85, 0x01, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0c, 0x03, 0x12,
    0x04, 0x85, 0x01, 0x1f, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0c, 0x08, 0x12, 0x04,
    0x85, 0x01, 0x31, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0c, 0x07, 0x12, 0x04, 0x85,
    0x01, 0x3c, 0x40, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x0d, 0x12, 0x04, 0x86, 0x01, 0x04,
    0x47, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0d, 0x04, 0x12, 0x04, 0x86, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0d, 0x05, 0x12, 0x04, 0x86, 0x01, 0x0d, 0x13, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0d, 0x01, 0x12, 0x04, 0x86, 0x01, 0x14, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0d, 0x03, 0x12, 0x04, 0x86, 0x01, 0x23, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x0d, 0x08, 0x12, 0x04, 0x86, 0x01, 0x31, 0x46, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x0d, 0x07, 0x12, 0x04, 0x86, 0x01, 0x3c, 0x45, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x0e, 0x12, 0x04, 0x87, 0x01, 0x04, 0x47, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x0e, 0x04, 0x12, 0x04, 0x87, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0e,
    0x05, 0x12, 0x04, 0x87, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0e, 0x01,
    0x12, 0x04, 0x87, 0x01, 0x13, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0e, 0x03, 0x12,
    0x04, 0x87, 0x01, 0x21, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0e, 0x08, 0x12, 0x04,
    0x87, 0x01, 0x31, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0e, 0x07, 0x12, 0x04, 0x87,
    0x01, 0x3c, 0x45, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x0f, 0x12, 0x04, 0x88, 0x01, 0x04,
    0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0f, 0x04, 0x12, 0x04, 0x88, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0f, 0x06, 0x12, 0x04, 0x88, 0x01, 0x0d, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0f, 0x01, 0x12, 0x04, 0x88, 0x01, 0x21, 0x2b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0f, 0x03, 0x12, 0x04, 0x88, 0x01, 0x2e, 0x30, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x0f, 0x08, 0x12, 0x04, 0x88, 0x01, 0x31, 0x40, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x0f, 0x07, 0x12, 0x04, 0x88, 0x01, 0x3c, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x10, 0x12, 0x04, 0x89, 0x01, 0x04, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x10, 0x04, 0x12, 0x04, 0x89, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x10,
    0x06, 0x12, 0x04, 0x89, 0x01, 0x0d, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x10, 0x01,
    0x12, 0x04, 0x89, 0x01, 0x21, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x10, 0x03, 0x12,
    0x04, 0x89, 0x01, 0x3e, 0x40,
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

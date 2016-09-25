// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Test1 {
    // message fields
    a: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Test1 {}

impl Test1 {
    pub fn new() -> Test1 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Test1 {
        static mut instance: ::protobuf::lazy::Lazy<Test1> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Test1,
        };
        unsafe {
            instance.get(|| {
                Test1 {
                    a: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 a = 1;

    pub fn clear_a(&mut self) {
        self.a = ::std::option::Option::None;
    }

    pub fn has_a(&self) -> bool {
        self.a.is_some()
    }

    // Param is passed by value, moved
    pub fn set_a(&mut self, v: i32) {
        self.a = ::std::option::Option::Some(v);
    }

    pub fn get_a(&self) -> i32 {
        self.a.unwrap_or(0)
    }
}

impl ::protobuf::Message for Test1 {
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
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.a = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.a {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.a {
            try!(os.write_int32(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Test1>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Test1 {
    fn new() -> Test1 {
        Test1::new()
    }

    fn descriptor_static(_: ::std::option::Option<Test1>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "a",
                    Test1::has_a,
                    Test1::get_a,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Test1>(
                    "Test1",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Test1 {
    fn clear(&mut self) {
        self.clear_a();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Test1 {
    fn eq(&self, other: &Test1) -> bool {
        self.a == other.a &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Test1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Test2 {
    // message fields
    b: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Test2 {}

impl Test2 {
    pub fn new() -> Test2 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Test2 {
        static mut instance: ::protobuf::lazy::Lazy<Test2> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Test2,
        };
        unsafe {
            instance.get(|| {
                Test2 {
                    b: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
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
    pub fn mut_b(&mut self) -> &mut ::std::string::String {
        if self.b.is_none() {
            self.b.set_default();
        };
        self.b.as_mut().unwrap()
    }

    // Take field
    pub fn take_b(&mut self) -> ::std::string::String {
        self.b.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_b(&self) -> &str {
        match self.b.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Test2 {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.b));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.b {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.b.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Test2>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Test2 {
    fn new() -> Test2 {
        Test2::new()
    }

    fn descriptor_static(_: ::std::option::Option<Test2>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "b",
                    Test2::has_b,
                    Test2::get_b,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Test2>(
                    "Test2",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Test2 {
    fn clear(&mut self) {
        self.clear_b();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Test2 {
    fn eq(&self, other: &Test2) -> bool {
        self.b == other.b &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Test2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Test3 {
    // message fields
    c: ::protobuf::SingularPtrField<Test1>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Test3 {}

impl Test3 {
    pub fn new() -> Test3 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Test3 {
        static mut instance: ::protobuf::lazy::Lazy<Test3> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Test3,
        };
        unsafe {
            instance.get(|| {
                Test3 {
                    c: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .basic.Test1 c = 3;

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
    pub fn mut_c(&mut self) -> &mut Test1 {
        if self.c.is_none() {
            self.c.set_default();
        };
        self.c.as_mut().unwrap()
    }

    // Take field
    pub fn take_c(&mut self) -> Test1 {
        self.c.take().unwrap_or_else(|| Test1::new())
    }

    pub fn get_c(&self) -> &Test1 {
        self.c.as_ref().unwrap_or_else(|| Test1::default_instance())
    }
}

impl ::protobuf::Message for Test3 {
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.c));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.c {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.c.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Test3>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Test3 {
    fn new() -> Test3 {
        Test3::new()
    }

    fn descriptor_static(_: ::std::option::Option<Test3>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "c",
                    Test3::has_c,
                    Test3::get_c,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Test3>(
                    "Test3",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Test3 {
    fn clear(&mut self) {
        self.clear_c();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Test3 {
    fn eq(&self, other: &Test3) -> bool {
        self.c == other.c &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Test3 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Test4 {
    // message fields
    d: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Test4 {}

impl Test4 {
    pub fn new() -> Test4 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Test4 {
        static mut instance: ::protobuf::lazy::Lazy<Test4> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Test4,
        };
        unsafe {
            instance.get(|| {
                Test4 {
                    d: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
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
    pub fn mut_d(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.d
    }

    // Take field
    pub fn take_d(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.d, ::std::vec::Vec::new())
    }

    pub fn get_d(&self) -> &[i32] {
        &self.d
    }
}

impl ::protobuf::Message for Test4 {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                4 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.d));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.d.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(4, &self.d);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.d.is_empty() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.d)));
            for v in &self.d {
                try!(os.write_int32_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Test4>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Test4 {
    fn new() -> Test4 {
        Test4::new()
    }

    fn descriptor_static(_: ::std::option::Option<Test4>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "d",
                    Test4::get_d,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Test4>(
                    "Test4",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Test4 {
    fn clear(&mut self) {
        self.clear_d();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Test4 {
    fn eq(&self, other: &Test4) -> bool {
        self.d == other.d &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Test4 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestPackedUnpacked {
    // message fields
    unpacked: ::std::vec::Vec<i32>,
    packed: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestPackedUnpacked {}

impl TestPackedUnpacked {
    pub fn new() -> TestPackedUnpacked {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestPackedUnpacked {
        static mut instance: ::protobuf::lazy::Lazy<TestPackedUnpacked> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestPackedUnpacked,
        };
        unsafe {
            instance.get(|| {
                TestPackedUnpacked {
                    unpacked: ::std::vec::Vec::new(),
                    packed: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
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
    pub fn mut_unpacked(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.unpacked
    }

    // Take field
    pub fn take_unpacked(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.unpacked, ::std::vec::Vec::new())
    }

    pub fn get_unpacked(&self) -> &[i32] {
        &self.unpacked
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
    pub fn mut_packed(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.packed
    }

    // Take field
    pub fn take_packed(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.packed, ::std::vec::Vec::new())
    }

    pub fn get_packed(&self) -> &[i32] {
        &self.packed
    }
}

impl ::protobuf::Message for TestPackedUnpacked {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                4 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.unpacked));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.packed));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.unpacked {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.packed.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(5, &self.packed);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.unpacked {
            try!(os.write_int32(4, *v));
        };
        if !self.packed.is_empty() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.packed)));
            for v in &self.packed {
                try!(os.write_int32_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestPackedUnpacked>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestPackedUnpacked {
    fn new() -> TestPackedUnpacked {
        TestPackedUnpacked::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestPackedUnpacked>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "unpacked",
                    TestPackedUnpacked::get_unpacked,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "packed",
                    TestPackedUnpacked::get_packed,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestPackedUnpacked>(
                    "TestPackedUnpacked",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestPackedUnpacked {
    fn clear(&mut self) {
        self.clear_unpacked();
        self.clear_packed();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestPackedUnpacked {
    fn eq(&self, other: &TestPackedUnpacked) -> bool {
        self.unpacked == other.unpacked &&
        self.packed == other.packed &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestPackedUnpacked {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestEmpty {
    // message fields
    foo: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestEmpty {}

impl TestEmpty {
    pub fn new() -> TestEmpty {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestEmpty {
        static mut instance: ::protobuf::lazy::Lazy<TestEmpty> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestEmpty,
        };
        unsafe {
            instance.get(|| {
                TestEmpty {
                    foo: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 foo = 10;

    pub fn clear_foo(&mut self) {
        self.foo = ::std::option::Option::None;
    }

    pub fn has_foo(&self) -> bool {
        self.foo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_foo(&mut self, v: i32) {
        self.foo = ::std::option::Option::Some(v);
    }

    pub fn get_foo(&self) -> i32 {
        self.foo.unwrap_or(0)
    }
}

impl ::protobuf::Message for TestEmpty {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.foo = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.foo {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.foo {
            try!(os.write_int32(10, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestEmpty>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestEmpty {
    fn new() -> TestEmpty {
        TestEmpty::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestEmpty>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "foo",
                    TestEmpty::has_foo,
                    TestEmpty::get_foo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestEmpty>(
                    "TestEmpty",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestEmpty {
    fn clear(&mut self) {
        self.clear_foo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestEmpty {
    fn eq(&self, other: &TestEmpty) -> bool {
        self.foo == other.foo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestEmpty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestRequired {
    // message fields
    b: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestRequired {}

impl TestRequired {
    pub fn new() -> TestRequired {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestRequired {
        static mut instance: ::protobuf::lazy::Lazy<TestRequired> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestRequired,
        };
        unsafe {
            instance.get(|| {
                TestRequired {
                    b: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool b = 5;

    pub fn clear_b(&mut self) {
        self.b = ::std::option::Option::None;
    }

    pub fn has_b(&self) -> bool {
        self.b.is_some()
    }

    // Param is passed by value, moved
    pub fn set_b(&mut self, v: bool) {
        self.b = ::std::option::Option::Some(v);
    }

    pub fn get_b(&self) -> bool {
        self.b.unwrap_or(false)
    }
}

impl ::protobuf::Message for TestRequired {
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
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.b = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.b.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.b {
            try!(os.write_bool(5, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestRequired>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestRequired {
    fn new() -> TestRequired {
        TestRequired::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestRequired>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "b",
                    TestRequired::has_b,
                    TestRequired::get_b,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestRequired>(
                    "TestRequired",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestRequired {
    fn clear(&mut self) {
        self.clear_b();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestRequired {
    fn eq(&self, other: &TestRequired) -> bool {
        self.b == other.b &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestRequired {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestUnknownFields {
    // message fields
    a: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestUnknownFields {}

impl TestUnknownFields {
    pub fn new() -> TestUnknownFields {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestUnknownFields {
        static mut instance: ::protobuf::lazy::Lazy<TestUnknownFields> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestUnknownFields,
        };
        unsafe {
            instance.get(|| {
                TestUnknownFields {
                    a: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 a = 1;

    pub fn clear_a(&mut self) {
        self.a = ::std::option::Option::None;
    }

    pub fn has_a(&self) -> bool {
        self.a.is_some()
    }

    // Param is passed by value, moved
    pub fn set_a(&mut self, v: i32) {
        self.a = ::std::option::Option::Some(v);
    }

    pub fn get_a(&self) -> i32 {
        self.a.unwrap_or(0)
    }
}

impl ::protobuf::Message for TestUnknownFields {
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
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.a = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.a {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.a {
            try!(os.write_int32(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestUnknownFields>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestUnknownFields {
    fn new() -> TestUnknownFields {
        TestUnknownFields::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestUnknownFields>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "a",
                    TestUnknownFields::has_a,
                    TestUnknownFields::get_a,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestUnknownFields>(
                    "TestUnknownFields",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestUnknownFields {
    fn clear(&mut self) {
        self.clear_a();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestUnknownFields {
    fn eq(&self, other: &TestUnknownFields) -> bool {
        self.a == other.a &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestUnknownFields {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestSelfReference {
    // message fields
    r1: ::protobuf::SingularPtrField<TestSelfReference>,
    r2: ::protobuf::SingularPtrField<TestSelfReference>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestSelfReference {}

impl TestSelfReference {
    pub fn new() -> TestSelfReference {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestSelfReference {
        static mut instance: ::protobuf::lazy::Lazy<TestSelfReference> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestSelfReference,
        };
        unsafe {
            instance.get(|| {
                TestSelfReference {
                    r1: ::protobuf::SingularPtrField::none(),
                    r2: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .basic.TestSelfReference r1 = 1;

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
    pub fn mut_r1(&mut self) -> &mut TestSelfReference {
        if self.r1.is_none() {
            self.r1.set_default();
        };
        self.r1.as_mut().unwrap()
    }

    // Take field
    pub fn take_r1(&mut self) -> TestSelfReference {
        self.r1.take().unwrap_or_else(|| TestSelfReference::new())
    }

    pub fn get_r1(&self) -> &TestSelfReference {
        self.r1.as_ref().unwrap_or_else(|| TestSelfReference::default_instance())
    }

    // optional .basic.TestSelfReference r2 = 2;

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
    pub fn mut_r2(&mut self) -> &mut TestSelfReference {
        if self.r2.is_none() {
            self.r2.set_default();
        };
        self.r2.as_mut().unwrap()
    }

    // Take field
    pub fn take_r2(&mut self) -> TestSelfReference {
        self.r2.take().unwrap_or_else(|| TestSelfReference::new())
    }

    pub fn get_r2(&self) -> &TestSelfReference {
        self.r2.as_ref().unwrap_or_else(|| TestSelfReference::default_instance())
    }
}

impl ::protobuf::Message for TestSelfReference {
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.r1));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.r2));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.r1 {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.r2 {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.r1.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.r2.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestSelfReference>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestSelfReference {
    fn new() -> TestSelfReference {
        TestSelfReference::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestSelfReference>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "r1",
                    TestSelfReference::has_r1,
                    TestSelfReference::get_r1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "r2",
                    TestSelfReference::has_r2,
                    TestSelfReference::get_r2,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestSelfReference>(
                    "TestSelfReference",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestSelfReference {
    fn clear(&mut self) {
        self.clear_r1();
        self.clear_r2();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestSelfReference {
    fn eq(&self, other: &TestSelfReference) -> bool {
        self.r1 == other.r1 &&
        self.r2 == other.r2 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestSelfReference {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestDefaultInstanceField {
    // message fields
    s: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestDefaultInstanceField {}

impl TestDefaultInstanceField {
    pub fn new() -> TestDefaultInstanceField {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestDefaultInstanceField {
        static mut instance: ::protobuf::lazy::Lazy<TestDefaultInstanceField> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestDefaultInstanceField,
        };
        unsafe {
            instance.get(|| {
                TestDefaultInstanceField {
                    s: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
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
    pub fn mut_s(&mut self) -> &mut ::std::string::String {
        if self.s.is_none() {
            self.s.set_default();
        };
        self.s.as_mut().unwrap()
    }

    // Take field
    pub fn take_s(&mut self) -> ::std::string::String {
        self.s.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_s(&self) -> &str {
        match self.s.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for TestDefaultInstanceField {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.s));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.s {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.s.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestDefaultInstanceField>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestDefaultInstanceField {
    fn new() -> TestDefaultInstanceField {
        TestDefaultInstanceField::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestDefaultInstanceField>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "s",
                    TestDefaultInstanceField::has_s,
                    TestDefaultInstanceField::get_s,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestDefaultInstanceField>(
                    "TestDefaultInstanceField",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestDefaultInstanceField {
    fn clear(&mut self) {
        self.clear_s();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestDefaultInstanceField {
    fn eq(&self, other: &TestDefaultInstanceField) -> bool {
        self.s == other.s &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestDefaultInstanceField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestDefaultInstance {
    // message fields
    field: ::protobuf::SingularPtrField<TestDefaultInstanceField>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestDefaultInstance {}

impl TestDefaultInstance {
    pub fn new() -> TestDefaultInstance {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestDefaultInstance {
        static mut instance: ::protobuf::lazy::Lazy<TestDefaultInstance> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestDefaultInstance,
        };
        unsafe {
            instance.get(|| {
                TestDefaultInstance {
                    field: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .basic.TestDefaultInstanceField field = 1;

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
    pub fn mut_field(&mut self) -> &mut TestDefaultInstanceField {
        if self.field.is_none() {
            self.field.set_default();
        };
        self.field.as_mut().unwrap()
    }

    // Take field
    pub fn take_field(&mut self) -> TestDefaultInstanceField {
        self.field.take().unwrap_or_else(|| TestDefaultInstanceField::new())
    }

    pub fn get_field(&self) -> &TestDefaultInstanceField {
        self.field.as_ref().unwrap_or_else(|| TestDefaultInstanceField::default_instance())
    }
}

impl ::protobuf::Message for TestDefaultInstance {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.field));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.field {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestDefaultInstance>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestDefaultInstance {
    fn new() -> TestDefaultInstance {
        TestDefaultInstance::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestDefaultInstance>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "field",
                    TestDefaultInstance::has_field,
                    TestDefaultInstance::get_field,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestDefaultInstance>(
                    "TestDefaultInstance",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestDefaultInstance {
    fn clear(&mut self) {
        self.clear_field();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestDefaultInstance {
    fn eq(&self, other: &TestDefaultInstance) -> bool {
        self.field == other.field &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestDefaultInstance {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestDescriptor {
    // message fields
    stuff: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestDescriptor {}

impl TestDescriptor {
    pub fn new() -> TestDescriptor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestDescriptor {
        static mut instance: ::protobuf::lazy::Lazy<TestDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestDescriptor,
        };
        unsafe {
            instance.get(|| {
                TestDescriptor {
                    stuff: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 stuff = 10;

    pub fn clear_stuff(&mut self) {
        self.stuff = ::std::option::Option::None;
    }

    pub fn has_stuff(&self) -> bool {
        self.stuff.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stuff(&mut self, v: i32) {
        self.stuff = ::std::option::Option::Some(v);
    }

    pub fn get_stuff(&self) -> i32 {
        self.stuff.unwrap_or(0)
    }
}

impl ::protobuf::Message for TestDescriptor {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.stuff = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.stuff {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.stuff {
            try!(os.write_int32(10, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestDescriptor>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestDescriptor {
    fn new() -> TestDescriptor {
        TestDescriptor::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestDescriptor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "stuff",
                    TestDescriptor::has_stuff,
                    TestDescriptor::get_stuff,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestDescriptor>(
                    "TestDescriptor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestDescriptor {
    fn clear(&mut self) {
        self.clear_stuff();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestDescriptor {
    fn eq(&self, other: &TestDescriptor) -> bool {
        self.stuff == other.stuff &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestDescriptor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestTypesSingular {
    // message fields
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
    enum_field: ::std::option::Option<TestEnumDescriptor>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestTypesSingular {}

impl TestTypesSingular {
    pub fn new() -> TestTypesSingular {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestTypesSingular {
        static mut instance: ::protobuf::lazy::Lazy<TestTypesSingular> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestTypesSingular,
        };
        unsafe {
            instance.get(|| {
                TestTypesSingular {
                    double_field: ::std::option::Option::None,
                    float_field: ::std::option::Option::None,
                    int32_field: ::std::option::Option::None,
                    int64_field: ::std::option::Option::None,
                    uint32_field: ::std::option::Option::None,
                    uint64_field: ::std::option::Option::None,
                    sint32_field: ::std::option::Option::None,
                    sint64_field: ::std::option::Option::None,
                    fixed32_field: ::std::option::Option::None,
                    fixed64_field: ::std::option::Option::None,
                    sfixed32_field: ::std::option::Option::None,
                    sfixed64_field: ::std::option::Option::None,
                    bool_field: ::std::option::Option::None,
                    string_field: ::protobuf::SingularField::none(),
                    bytes_field: ::protobuf::SingularField::none(),
                    enum_field: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional double double_field = 1;

    pub fn clear_double_field(&mut self) {
        self.double_field = ::std::option::Option::None;
    }

    pub fn has_double_field(&self) -> bool {
        self.double_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_double_field(&mut self, v: f64) {
        self.double_field = ::std::option::Option::Some(v);
    }

    pub fn get_double_field(&self) -> f64 {
        self.double_field.unwrap_or(0.)
    }

    // optional float float_field = 2;

    pub fn clear_float_field(&mut self) {
        self.float_field = ::std::option::Option::None;
    }

    pub fn has_float_field(&self) -> bool {
        self.float_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_float_field(&mut self, v: f32) {
        self.float_field = ::std::option::Option::Some(v);
    }

    pub fn get_float_field(&self) -> f32 {
        self.float_field.unwrap_or(0.)
    }

    // optional int32 int32_field = 3;

    pub fn clear_int32_field(&mut self) {
        self.int32_field = ::std::option::Option::None;
    }

    pub fn has_int32_field(&self) -> bool {
        self.int32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_int32_field(&mut self, v: i32) {
        self.int32_field = ::std::option::Option::Some(v);
    }

    pub fn get_int32_field(&self) -> i32 {
        self.int32_field.unwrap_or(0)
    }

    // optional int64 int64_field = 4;

    pub fn clear_int64_field(&mut self) {
        self.int64_field = ::std::option::Option::None;
    }

    pub fn has_int64_field(&self) -> bool {
        self.int64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_int64_field(&mut self, v: i64) {
        self.int64_field = ::std::option::Option::Some(v);
    }

    pub fn get_int64_field(&self) -> i64 {
        self.int64_field.unwrap_or(0)
    }

    // optional uint32 uint32_field = 5;

    pub fn clear_uint32_field(&mut self) {
        self.uint32_field = ::std::option::Option::None;
    }

    pub fn has_uint32_field(&self) -> bool {
        self.uint32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uint32_field(&mut self, v: u32) {
        self.uint32_field = ::std::option::Option::Some(v);
    }

    pub fn get_uint32_field(&self) -> u32 {
        self.uint32_field.unwrap_or(0)
    }

    // optional uint64 uint64_field = 6;

    pub fn clear_uint64_field(&mut self) {
        self.uint64_field = ::std::option::Option::None;
    }

    pub fn has_uint64_field(&self) -> bool {
        self.uint64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uint64_field(&mut self, v: u64) {
        self.uint64_field = ::std::option::Option::Some(v);
    }

    pub fn get_uint64_field(&self) -> u64 {
        self.uint64_field.unwrap_or(0)
    }

    // optional sint32 sint32_field = 7;

    pub fn clear_sint32_field(&mut self) {
        self.sint32_field = ::std::option::Option::None;
    }

    pub fn has_sint32_field(&self) -> bool {
        self.sint32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sint32_field(&mut self, v: i32) {
        self.sint32_field = ::std::option::Option::Some(v);
    }

    pub fn get_sint32_field(&self) -> i32 {
        self.sint32_field.unwrap_or(0)
    }

    // optional sint64 sint64_field = 8;

    pub fn clear_sint64_field(&mut self) {
        self.sint64_field = ::std::option::Option::None;
    }

    pub fn has_sint64_field(&self) -> bool {
        self.sint64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sint64_field(&mut self, v: i64) {
        self.sint64_field = ::std::option::Option::Some(v);
    }

    pub fn get_sint64_field(&self) -> i64 {
        self.sint64_field.unwrap_or(0)
    }

    // optional fixed32 fixed32_field = 9;

    pub fn clear_fixed32_field(&mut self) {
        self.fixed32_field = ::std::option::Option::None;
    }

    pub fn has_fixed32_field(&self) -> bool {
        self.fixed32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fixed32_field(&mut self, v: u32) {
        self.fixed32_field = ::std::option::Option::Some(v);
    }

    pub fn get_fixed32_field(&self) -> u32 {
        self.fixed32_field.unwrap_or(0)
    }

    // optional fixed64 fixed64_field = 10;

    pub fn clear_fixed64_field(&mut self) {
        self.fixed64_field = ::std::option::Option::None;
    }

    pub fn has_fixed64_field(&self) -> bool {
        self.fixed64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fixed64_field(&mut self, v: u64) {
        self.fixed64_field = ::std::option::Option::Some(v);
    }

    pub fn get_fixed64_field(&self) -> u64 {
        self.fixed64_field.unwrap_or(0)
    }

    // optional sfixed32 sfixed32_field = 11;

    pub fn clear_sfixed32_field(&mut self) {
        self.sfixed32_field = ::std::option::Option::None;
    }

    pub fn has_sfixed32_field(&self) -> bool {
        self.sfixed32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sfixed32_field(&mut self, v: i32) {
        self.sfixed32_field = ::std::option::Option::Some(v);
    }

    pub fn get_sfixed32_field(&self) -> i32 {
        self.sfixed32_field.unwrap_or(0)
    }

    // optional sfixed64 sfixed64_field = 12;

    pub fn clear_sfixed64_field(&mut self) {
        self.sfixed64_field = ::std::option::Option::None;
    }

    pub fn has_sfixed64_field(&self) -> bool {
        self.sfixed64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sfixed64_field(&mut self, v: i64) {
        self.sfixed64_field = ::std::option::Option::Some(v);
    }

    pub fn get_sfixed64_field(&self) -> i64 {
        self.sfixed64_field.unwrap_or(0)
    }

    // optional bool bool_field = 13;

    pub fn clear_bool_field(&mut self) {
        self.bool_field = ::std::option::Option::None;
    }

    pub fn has_bool_field(&self) -> bool {
        self.bool_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bool_field(&mut self, v: bool) {
        self.bool_field = ::std::option::Option::Some(v);
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
    pub fn mut_string_field(&mut self) -> &mut ::std::string::String {
        if self.string_field.is_none() {
            self.string_field.set_default();
        };
        self.string_field.as_mut().unwrap()
    }

    // Take field
    pub fn take_string_field(&mut self) -> ::std::string::String {
        self.string_field.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_string_field(&self) -> &str {
        match self.string_field.as_ref() {
            Some(v) => &v,
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
    pub fn mut_bytes_field(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bytes_field.is_none() {
            self.bytes_field.set_default();
        };
        self.bytes_field.as_mut().unwrap()
    }

    // Take field
    pub fn take_bytes_field(&mut self) -> ::std::vec::Vec<u8> {
        self.bytes_field.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bytes_field(&self) -> &[u8] {
        match self.bytes_field.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .basic.TestEnumDescriptor enum_field = 16;

    pub fn clear_enum_field(&mut self) {
        self.enum_field = ::std::option::Option::None;
    }

    pub fn has_enum_field(&self) -> bool {
        self.enum_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enum_field(&mut self, v: TestEnumDescriptor) {
        self.enum_field = ::std::option::Option::Some(v);
    }

    pub fn get_enum_field(&self) -> TestEnumDescriptor {
        self.enum_field.unwrap_or(TestEnumDescriptor::RED)
    }
}

impl ::protobuf::Message for TestTypesSingular {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.double_field = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.float_field = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.int32_field = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.int64_field = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.uint32_field = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.uint64_field = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.sint32_field = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint64());
                    self.sint64_field = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed32());
                    self.fixed32_field = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.fixed64_field = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sfixed32());
                    self.sfixed32_field = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sfixed64());
                    self.sfixed64_field = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.bool_field = ::std::option::Option::Some(tmp);
                },
                14 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.string_field));
                },
                15 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bytes_field));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.enum_field = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.double_field.is_some() {
            my_size += 9;
        };
        if self.float_field.is_some() {
            my_size += 5;
        };
        for value in &self.int32_field {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.int64_field {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.uint32_field {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.uint64_field {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.sint32_field {
            my_size += ::protobuf::rt::value_varint_zigzag_size(7, *value);
        };
        for value in &self.sint64_field {
            my_size += ::protobuf::rt::value_varint_zigzag_size(8, *value);
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
        for value in &self.string_field {
            my_size += ::protobuf::rt::string_size(14, &value);
        };
        for value in &self.bytes_field {
            my_size += ::protobuf::rt::bytes_size(15, &value);
        };
        for value in &self.enum_field {
            my_size += ::protobuf::rt::enum_size(16, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.double_field {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.float_field {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.int32_field {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.int64_field {
            try!(os.write_int64(4, v));
        };
        if let Some(v) = self.uint32_field {
            try!(os.write_uint32(5, v));
        };
        if let Some(v) = self.uint64_field {
            try!(os.write_uint64(6, v));
        };
        if let Some(v) = self.sint32_field {
            try!(os.write_sint32(7, v));
        };
        if let Some(v) = self.sint64_field {
            try!(os.write_sint64(8, v));
        };
        if let Some(v) = self.fixed32_field {
            try!(os.write_fixed32(9, v));
        };
        if let Some(v) = self.fixed64_field {
            try!(os.write_fixed64(10, v));
        };
        if let Some(v) = self.sfixed32_field {
            try!(os.write_sfixed32(11, v));
        };
        if let Some(v) = self.sfixed64_field {
            try!(os.write_sfixed64(12, v));
        };
        if let Some(v) = self.bool_field {
            try!(os.write_bool(13, v));
        };
        if let Some(v) = self.string_field.as_ref() {
            try!(os.write_string(14, &v));
        };
        if let Some(v) = self.bytes_field.as_ref() {
            try!(os.write_bytes(15, &v));
        };
        if let Some(v) = self.enum_field {
            try!(os.write_enum(16, v.value()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestTypesSingular>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestTypesSingular {
    fn new() -> TestTypesSingular {
        TestTypesSingular::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestTypesSingular>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "double_field",
                    TestTypesSingular::has_double_field,
                    TestTypesSingular::get_double_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "float_field",
                    TestTypesSingular::has_float_field,
                    TestTypesSingular::get_float_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "int32_field",
                    TestTypesSingular::has_int32_field,
                    TestTypesSingular::get_int32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "int64_field",
                    TestTypesSingular::has_int64_field,
                    TestTypesSingular::get_int64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "uint32_field",
                    TestTypesSingular::has_uint32_field,
                    TestTypesSingular::get_uint32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "uint64_field",
                    TestTypesSingular::has_uint64_field,
                    TestTypesSingular::get_uint64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sint32_field",
                    TestTypesSingular::has_sint32_field,
                    TestTypesSingular::get_sint32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "sint64_field",
                    TestTypesSingular::has_sint64_field,
                    TestTypesSingular::get_sint64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "fixed32_field",
                    TestTypesSingular::has_fixed32_field,
                    TestTypesSingular::get_fixed32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "fixed64_field",
                    TestTypesSingular::has_fixed64_field,
                    TestTypesSingular::get_fixed64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sfixed32_field",
                    TestTypesSingular::has_sfixed32_field,
                    TestTypesSingular::get_sfixed32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "sfixed64_field",
                    TestTypesSingular::has_sfixed64_field,
                    TestTypesSingular::get_sfixed64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "bool_field",
                    TestTypesSingular::has_bool_field,
                    TestTypesSingular::get_bool_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "string_field",
                    TestTypesSingular::has_string_field,
                    TestTypesSingular::get_string_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bytes_field",
                    TestTypesSingular::has_bytes_field,
                    TestTypesSingular::get_bytes_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "enum_field",
                    TestTypesSingular::has_enum_field,
                    TestTypesSingular::get_enum_field,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestTypesSingular>(
                    "TestTypesSingular",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
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
        self.clear_enum_field();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestTypesSingular {
    fn eq(&self, other: &TestTypesSingular) -> bool {
        self.double_field == other.double_field &&
        self.float_field == other.float_field &&
        self.int32_field == other.int32_field &&
        self.int64_field == other.int64_field &&
        self.uint32_field == other.uint32_field &&
        self.uint64_field == other.uint64_field &&
        self.sint32_field == other.sint32_field &&
        self.sint64_field == other.sint64_field &&
        self.fixed32_field == other.fixed32_field &&
        self.fixed64_field == other.fixed64_field &&
        self.sfixed32_field == other.sfixed32_field &&
        self.sfixed64_field == other.sfixed64_field &&
        self.bool_field == other.bool_field &&
        self.string_field == other.string_field &&
        self.bytes_field == other.bytes_field &&
        self.enum_field == other.enum_field &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestTypesSingular {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestTypesRepeated {
    // message fields
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
    enum_field: ::std::vec::Vec<TestEnumDescriptor>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestTypesRepeated {}

impl TestTypesRepeated {
    pub fn new() -> TestTypesRepeated {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestTypesRepeated {
        static mut instance: ::protobuf::lazy::Lazy<TestTypesRepeated> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestTypesRepeated,
        };
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
                    enum_field: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
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
    pub fn mut_double_field(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.double_field
    }

    // Take field
    pub fn take_double_field(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.double_field, ::std::vec::Vec::new())
    }

    pub fn get_double_field(&self) -> &[f64] {
        &self.double_field
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
    pub fn mut_float_field(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.float_field
    }

    // Take field
    pub fn take_float_field(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.float_field, ::std::vec::Vec::new())
    }

    pub fn get_float_field(&self) -> &[f32] {
        &self.float_field
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
    pub fn mut_int32_field(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.int32_field
    }

    // Take field
    pub fn take_int32_field(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.int32_field, ::std::vec::Vec::new())
    }

    pub fn get_int32_field(&self) -> &[i32] {
        &self.int32_field
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
    pub fn mut_int64_field(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.int64_field
    }

    // Take field
    pub fn take_int64_field(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.int64_field, ::std::vec::Vec::new())
    }

    pub fn get_int64_field(&self) -> &[i64] {
        &self.int64_field
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
    pub fn mut_uint32_field(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.uint32_field
    }

    // Take field
    pub fn take_uint32_field(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.uint32_field, ::std::vec::Vec::new())
    }

    pub fn get_uint32_field(&self) -> &[u32] {
        &self.uint32_field
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
    pub fn mut_uint64_field(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.uint64_field
    }

    // Take field
    pub fn take_uint64_field(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.uint64_field, ::std::vec::Vec::new())
    }

    pub fn get_uint64_field(&self) -> &[u64] {
        &self.uint64_field
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
    pub fn mut_sint32_field(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.sint32_field
    }

    // Take field
    pub fn take_sint32_field(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.sint32_field, ::std::vec::Vec::new())
    }

    pub fn get_sint32_field(&self) -> &[i32] {
        &self.sint32_field
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
    pub fn mut_sint64_field(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.sint64_field
    }

    // Take field
    pub fn take_sint64_field(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.sint64_field, ::std::vec::Vec::new())
    }

    pub fn get_sint64_field(&self) -> &[i64] {
        &self.sint64_field
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
    pub fn mut_fixed32_field(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.fixed32_field
    }

    // Take field
    pub fn take_fixed32_field(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.fixed32_field, ::std::vec::Vec::new())
    }

    pub fn get_fixed32_field(&self) -> &[u32] {
        &self.fixed32_field
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
    pub fn mut_fixed64_field(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.fixed64_field
    }

    // Take field
    pub fn take_fixed64_field(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.fixed64_field, ::std::vec::Vec::new())
    }

    pub fn get_fixed64_field(&self) -> &[u64] {
        &self.fixed64_field
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
    pub fn mut_sfixed32_field(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.sfixed32_field
    }

    // Take field
    pub fn take_sfixed32_field(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.sfixed32_field, ::std::vec::Vec::new())
    }

    pub fn get_sfixed32_field(&self) -> &[i32] {
        &self.sfixed32_field
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
    pub fn mut_sfixed64_field(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.sfixed64_field
    }

    // Take field
    pub fn take_sfixed64_field(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.sfixed64_field, ::std::vec::Vec::new())
    }

    pub fn get_sfixed64_field(&self) -> &[i64] {
        &self.sfixed64_field
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
    pub fn mut_bool_field(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.bool_field
    }

    // Take field
    pub fn take_bool_field(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.bool_field, ::std::vec::Vec::new())
    }

    pub fn get_bool_field(&self) -> &[bool] {
        &self.bool_field
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
    pub fn mut_string_field(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.string_field
    }

    // Take field
    pub fn take_string_field(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.string_field, ::protobuf::RepeatedField::new())
    }

    pub fn get_string_field(&self) -> &[::std::string::String] {
        &self.string_field
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
    pub fn mut_bytes_field(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.bytes_field
    }

    // Take field
    pub fn take_bytes_field(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.bytes_field, ::protobuf::RepeatedField::new())
    }

    pub fn get_bytes_field(&self) -> &[::std::vec::Vec<u8>] {
        &self.bytes_field
    }

    // repeated .basic.TestEnumDescriptor enum_field = 16;

    pub fn clear_enum_field(&mut self) {
        self.enum_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_enum_field(&mut self, v: ::std::vec::Vec<TestEnumDescriptor>) {
        self.enum_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_enum_field(&mut self) -> &mut ::std::vec::Vec<TestEnumDescriptor> {
        &mut self.enum_field
    }

    // Take field
    pub fn take_enum_field(&mut self) -> ::std::vec::Vec<TestEnumDescriptor> {
        ::std::mem::replace(&mut self.enum_field, ::std::vec::Vec::new())
    }

    pub fn get_enum_field(&self) -> &[TestEnumDescriptor] {
        &self.enum_field
    }
}

impl ::protobuf::Message for TestTypesRepeated {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.double_field));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.float_field));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.int32_field));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.int64_field));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.uint32_field));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.uint64_field));
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_sint32_into(wire_type, is, &mut self.sint32_field));
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_sint64_into(wire_type, is, &mut self.sint64_field));
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.fixed32_field));
                },
                10 => {
                    try!(::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.fixed64_field));
                },
                11 => {
                    try!(::protobuf::rt::read_repeated_sfixed32_into(wire_type, is, &mut self.sfixed32_field));
                },
                12 => {
                    try!(::protobuf::rt::read_repeated_sfixed64_into(wire_type, is, &mut self.sfixed64_field));
                },
                13 => {
                    try!(::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.bool_field));
                },
                14 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.string_field));
                },
                15 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.bytes_field));
                },
                16 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.enum_field));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += 9 * self.double_field.len() as u32;
        my_size += 5 * self.float_field.len() as u32;
        for value in &self.int32_field {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.int64_field {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.uint32_field {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.uint64_field {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.sint32_field {
            my_size += ::protobuf::rt::value_varint_zigzag_size(7, *value);
        };
        for value in &self.sint64_field {
            my_size += ::protobuf::rt::value_varint_zigzag_size(8, *value);
        };
        my_size += 5 * self.fixed32_field.len() as u32;
        my_size += 9 * self.fixed64_field.len() as u32;
        my_size += 5 * self.sfixed32_field.len() as u32;
        my_size += 9 * self.sfixed64_field.len() as u32;
        my_size += 2 * self.bool_field.len() as u32;
        for value in &self.string_field {
            my_size += ::protobuf::rt::string_size(14, &value);
        };
        for value in &self.bytes_field {
            my_size += ::protobuf::rt::bytes_size(15, &value);
        };
        for value in &self.enum_field {
            my_size += ::protobuf::rt::enum_size(16, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.double_field {
            try!(os.write_double(1, *v));
        };
        for v in &self.float_field {
            try!(os.write_float(2, *v));
        };
        for v in &self.int32_field {
            try!(os.write_int32(3, *v));
        };
        for v in &self.int64_field {
            try!(os.write_int64(4, *v));
        };
        for v in &self.uint32_field {
            try!(os.write_uint32(5, *v));
        };
        for v in &self.uint64_field {
            try!(os.write_uint64(6, *v));
        };
        for v in &self.sint32_field {
            try!(os.write_sint32(7, *v));
        };
        for v in &self.sint64_field {
            try!(os.write_sint64(8, *v));
        };
        for v in &self.fixed32_field {
            try!(os.write_fixed32(9, *v));
        };
        for v in &self.fixed64_field {
            try!(os.write_fixed64(10, *v));
        };
        for v in &self.sfixed32_field {
            try!(os.write_sfixed32(11, *v));
        };
        for v in &self.sfixed64_field {
            try!(os.write_sfixed64(12, *v));
        };
        for v in &self.bool_field {
            try!(os.write_bool(13, *v));
        };
        for v in &self.string_field {
            try!(os.write_string(14, &v));
        };
        for v in &self.bytes_field {
            try!(os.write_bytes(15, &v));
        };
        for v in &self.enum_field {
            try!(os.write_enum(16, v.value()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestTypesRepeated>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestTypesRepeated {
    fn new() -> TestTypesRepeated {
        TestTypesRepeated::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestTypesRepeated>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "double_field",
                    TestTypesRepeated::get_double_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "float_field",
                    TestTypesRepeated::get_float_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "int32_field",
                    TestTypesRepeated::get_int32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "int64_field",
                    TestTypesRepeated::get_int64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "uint32_field",
                    TestTypesRepeated::get_uint32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u64_accessor(
                    "uint64_field",
                    TestTypesRepeated::get_uint64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "sint32_field",
                    TestTypesRepeated::get_sint32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "sint64_field",
                    TestTypesRepeated::get_sint64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "fixed32_field",
                    TestTypesRepeated::get_fixed32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u64_accessor(
                    "fixed64_field",
                    TestTypesRepeated::get_fixed64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "sfixed32_field",
                    TestTypesRepeated::get_sfixed32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "sfixed64_field",
                    TestTypesRepeated::get_sfixed64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bool_accessor(
                    "bool_field",
                    TestTypesRepeated::get_bool_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "string_field",
                    TestTypesRepeated::get_string_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "bytes_field",
                    TestTypesRepeated::get_bytes_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "enum_field",
                    TestTypesRepeated::get_enum_field,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestTypesRepeated>(
                    "TestTypesRepeated",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
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
        self.clear_enum_field();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestTypesRepeated {
    fn eq(&self, other: &TestTypesRepeated) -> bool {
        self.double_field == other.double_field &&
        self.float_field == other.float_field &&
        self.int32_field == other.int32_field &&
        self.int64_field == other.int64_field &&
        self.uint32_field == other.uint32_field &&
        self.uint64_field == other.uint64_field &&
        self.sint32_field == other.sint32_field &&
        self.sint64_field == other.sint64_field &&
        self.fixed32_field == other.fixed32_field &&
        self.fixed64_field == other.fixed64_field &&
        self.sfixed32_field == other.sfixed32_field &&
        self.sfixed64_field == other.sfixed64_field &&
        self.bool_field == other.bool_field &&
        self.string_field == other.string_field &&
        self.bytes_field == other.bytes_field &&
        self.enum_field == other.enum_field &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestTypesRepeated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestTypesRepeatedPacked {
    // message fields
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
    enum_field: ::std::vec::Vec<TestEnumDescriptor>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestTypesRepeatedPacked {}

impl TestTypesRepeatedPacked {
    pub fn new() -> TestTypesRepeatedPacked {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestTypesRepeatedPacked {
        static mut instance: ::protobuf::lazy::Lazy<TestTypesRepeatedPacked> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestTypesRepeatedPacked,
        };
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
                    enum_field: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
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
    pub fn mut_double_field(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.double_field
    }

    // Take field
    pub fn take_double_field(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.double_field, ::std::vec::Vec::new())
    }

    pub fn get_double_field(&self) -> &[f64] {
        &self.double_field
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
    pub fn mut_float_field(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.float_field
    }

    // Take field
    pub fn take_float_field(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.float_field, ::std::vec::Vec::new())
    }

    pub fn get_float_field(&self) -> &[f32] {
        &self.float_field
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
    pub fn mut_int32_field(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.int32_field
    }

    // Take field
    pub fn take_int32_field(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.int32_field, ::std::vec::Vec::new())
    }

    pub fn get_int32_field(&self) -> &[i32] {
        &self.int32_field
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
    pub fn mut_int64_field(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.int64_field
    }

    // Take field
    pub fn take_int64_field(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.int64_field, ::std::vec::Vec::new())
    }

    pub fn get_int64_field(&self) -> &[i64] {
        &self.int64_field
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
    pub fn mut_uint32_field(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.uint32_field
    }

    // Take field
    pub fn take_uint32_field(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.uint32_field, ::std::vec::Vec::new())
    }

    pub fn get_uint32_field(&self) -> &[u32] {
        &self.uint32_field
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
    pub fn mut_uint64_field(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.uint64_field
    }

    // Take field
    pub fn take_uint64_field(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.uint64_field, ::std::vec::Vec::new())
    }

    pub fn get_uint64_field(&self) -> &[u64] {
        &self.uint64_field
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
    pub fn mut_sint32_field(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.sint32_field
    }

    // Take field
    pub fn take_sint32_field(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.sint32_field, ::std::vec::Vec::new())
    }

    pub fn get_sint32_field(&self) -> &[i32] {
        &self.sint32_field
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
    pub fn mut_sint64_field(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.sint64_field
    }

    // Take field
    pub fn take_sint64_field(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.sint64_field, ::std::vec::Vec::new())
    }

    pub fn get_sint64_field(&self) -> &[i64] {
        &self.sint64_field
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
    pub fn mut_fixed32_field(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.fixed32_field
    }

    // Take field
    pub fn take_fixed32_field(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.fixed32_field, ::std::vec::Vec::new())
    }

    pub fn get_fixed32_field(&self) -> &[u32] {
        &self.fixed32_field
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
    pub fn mut_fixed64_field(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.fixed64_field
    }

    // Take field
    pub fn take_fixed64_field(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.fixed64_field, ::std::vec::Vec::new())
    }

    pub fn get_fixed64_field(&self) -> &[u64] {
        &self.fixed64_field
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
    pub fn mut_sfixed32_field(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.sfixed32_field
    }

    // Take field
    pub fn take_sfixed32_field(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.sfixed32_field, ::std::vec::Vec::new())
    }

    pub fn get_sfixed32_field(&self) -> &[i32] {
        &self.sfixed32_field
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
    pub fn mut_sfixed64_field(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.sfixed64_field
    }

    // Take field
    pub fn take_sfixed64_field(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.sfixed64_field, ::std::vec::Vec::new())
    }

    pub fn get_sfixed64_field(&self) -> &[i64] {
        &self.sfixed64_field
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
    pub fn mut_bool_field(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.bool_field
    }

    // Take field
    pub fn take_bool_field(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.bool_field, ::std::vec::Vec::new())
    }

    pub fn get_bool_field(&self) -> &[bool] {
        &self.bool_field
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
    pub fn mut_string_field(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.string_field
    }

    // Take field
    pub fn take_string_field(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.string_field, ::protobuf::RepeatedField::new())
    }

    pub fn get_string_field(&self) -> &[::std::string::String] {
        &self.string_field
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
    pub fn mut_bytes_field(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.bytes_field
    }

    // Take field
    pub fn take_bytes_field(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.bytes_field, ::protobuf::RepeatedField::new())
    }

    pub fn get_bytes_field(&self) -> &[::std::vec::Vec<u8>] {
        &self.bytes_field
    }

    // repeated .basic.TestEnumDescriptor enum_field = 16;

    pub fn clear_enum_field(&mut self) {
        self.enum_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_enum_field(&mut self, v: ::std::vec::Vec<TestEnumDescriptor>) {
        self.enum_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_enum_field(&mut self) -> &mut ::std::vec::Vec<TestEnumDescriptor> {
        &mut self.enum_field
    }

    // Take field
    pub fn take_enum_field(&mut self) -> ::std::vec::Vec<TestEnumDescriptor> {
        ::std::mem::replace(&mut self.enum_field, ::std::vec::Vec::new())
    }

    pub fn get_enum_field(&self) -> &[TestEnumDescriptor] {
        &self.enum_field
    }
}

impl ::protobuf::Message for TestTypesRepeatedPacked {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.double_field));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.float_field));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.int32_field));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.int64_field));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.uint32_field));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.uint64_field));
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_sint32_into(wire_type, is, &mut self.sint32_field));
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_sint64_into(wire_type, is, &mut self.sint64_field));
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.fixed32_field));
                },
                10 => {
                    try!(::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.fixed64_field));
                },
                11 => {
                    try!(::protobuf::rt::read_repeated_sfixed32_into(wire_type, is, &mut self.sfixed32_field));
                },
                12 => {
                    try!(::protobuf::rt::read_repeated_sfixed64_into(wire_type, is, &mut self.sfixed64_field));
                },
                13 => {
                    try!(::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.bool_field));
                },
                14 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.string_field));
                },
                15 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.bytes_field));
                },
                16 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.enum_field));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.double_field.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.double_field.len() as u32) + (self.double_field.len() * 8) as u32;
        };
        if !self.float_field.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.float_field.len() as u32) + (self.float_field.len() * 4) as u32;
        };
        if !self.int32_field.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(3, &self.int32_field);
        };
        if !self.int64_field.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(4, &self.int64_field);
        };
        if !self.uint32_field.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(5, &self.uint32_field);
        };
        if !self.uint64_field.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(6, &self.uint64_field);
        };
        if !self.sint32_field.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(7, &self.sint32_field);
        };
        if !self.sint64_field.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_zigzag_size(8, &self.sint64_field);
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
        for value in &self.string_field {
            my_size += ::protobuf::rt::string_size(14, &value);
        };
        for value in &self.bytes_field {
            my_size += ::protobuf::rt::bytes_size(15, &value);
        };
        if !self.enum_field.is_empty() {
            my_size += ::protobuf::rt::vec_packed_enum_size(16, &self.enum_field);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.double_field.is_empty() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.double_field.len() * 8) as u32));
            for v in &self.double_field {
                try!(os.write_double_no_tag(*v));
            };
        };
        if !self.float_field.is_empty() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.float_field.len() * 4) as u32));
            for v in &self.float_field {
                try!(os.write_float_no_tag(*v));
            };
        };
        if !self.int32_field.is_empty() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.int32_field)));
            for v in &self.int32_field {
                try!(os.write_int32_no_tag(*v));
            };
        };
        if !self.int64_field.is_empty() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.int64_field)));
            for v in &self.int64_field {
                try!(os.write_int64_no_tag(*v));
            };
        };
        if !self.uint32_field.is_empty() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.uint32_field)));
            for v in &self.uint32_field {
                try!(os.write_uint32_no_tag(*v));
            };
        };
        if !self.uint64_field.is_empty() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.uint64_field)));
            for v in &self.uint64_field {
                try!(os.write_uint64_no_tag(*v));
            };
        };
        if !self.sint32_field.is_empty() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.sint32_field)));
            for v in &self.sint32_field {
                try!(os.write_sint32_no_tag(*v));
            };
        };
        if !self.sint64_field.is_empty() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_zigzag_data_size(&self.sint64_field)));
            for v in &self.sint64_field {
                try!(os.write_sint64_no_tag(*v));
            };
        };
        if !self.fixed32_field.is_empty() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.fixed32_field.len() * 4) as u32));
            for v in &self.fixed32_field {
                try!(os.write_fixed32_no_tag(*v));
            };
        };
        if !self.fixed64_field.is_empty() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.fixed64_field.len() * 8) as u32));
            for v in &self.fixed64_field {
                try!(os.write_fixed64_no_tag(*v));
            };
        };
        if !self.sfixed32_field.is_empty() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.sfixed32_field.len() * 4) as u32));
            for v in &self.sfixed32_field {
                try!(os.write_sfixed32_no_tag(*v));
            };
        };
        if !self.sfixed64_field.is_empty() {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.sfixed64_field.len() * 8) as u32));
            for v in &self.sfixed64_field {
                try!(os.write_sfixed64_no_tag(*v));
            };
        };
        if !self.bool_field.is_empty() {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.bool_field.len() * 1) as u32));
            for v in &self.bool_field {
                try!(os.write_bool_no_tag(*v));
            };
        };
        for v in &self.string_field {
            try!(os.write_string(14, &v));
        };
        for v in &self.bytes_field {
            try!(os.write_bytes(15, &v));
        };
        if !self.enum_field.is_empty() {
            try!(os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_enum_data_size(&self.enum_field)));
            for v in &self.enum_field {
                try!(os.write_enum_no_tag(v.value()));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestTypesRepeatedPacked>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestTypesRepeatedPacked {
    fn new() -> TestTypesRepeatedPacked {
        TestTypesRepeatedPacked::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestTypesRepeatedPacked>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "double_field",
                    TestTypesRepeatedPacked::get_double_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f32_accessor(
                    "float_field",
                    TestTypesRepeatedPacked::get_float_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "int32_field",
                    TestTypesRepeatedPacked::get_int32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "int64_field",
                    TestTypesRepeatedPacked::get_int64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "uint32_field",
                    TestTypesRepeatedPacked::get_uint32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u64_accessor(
                    "uint64_field",
                    TestTypesRepeatedPacked::get_uint64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "sint32_field",
                    TestTypesRepeatedPacked::get_sint32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "sint64_field",
                    TestTypesRepeatedPacked::get_sint64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "fixed32_field",
                    TestTypesRepeatedPacked::get_fixed32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u64_accessor(
                    "fixed64_field",
                    TestTypesRepeatedPacked::get_fixed64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "sfixed32_field",
                    TestTypesRepeatedPacked::get_sfixed32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "sfixed64_field",
                    TestTypesRepeatedPacked::get_sfixed64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bool_accessor(
                    "bool_field",
                    TestTypesRepeatedPacked::get_bool_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "string_field",
                    TestTypesRepeatedPacked::get_string_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "bytes_field",
                    TestTypesRepeatedPacked::get_bytes_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "enum_field",
                    TestTypesRepeatedPacked::get_enum_field,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestTypesRepeatedPacked>(
                    "TestTypesRepeatedPacked",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
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
        self.clear_enum_field();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestTypesRepeatedPacked {
    fn eq(&self, other: &TestTypesRepeatedPacked) -> bool {
        self.double_field == other.double_field &&
        self.float_field == other.float_field &&
        self.int32_field == other.int32_field &&
        self.int64_field == other.int64_field &&
        self.uint32_field == other.uint32_field &&
        self.uint64_field == other.uint64_field &&
        self.sint32_field == other.sint32_field &&
        self.sint64_field == other.sint64_field &&
        self.fixed32_field == other.fixed32_field &&
        self.fixed64_field == other.fixed64_field &&
        self.sfixed32_field == other.sfixed32_field &&
        self.sfixed64_field == other.sfixed64_field &&
        self.bool_field == other.bool_field &&
        self.string_field == other.string_field &&
        self.bytes_field == other.bytes_field &&
        self.enum_field == other.enum_field &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestTypesRepeatedPacked {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestDefaultValues {
    // message fields
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
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestDefaultValues {}

impl TestDefaultValues {
    pub fn new() -> TestDefaultValues {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestDefaultValues {
        static mut instance: ::protobuf::lazy::Lazy<TestDefaultValues> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestDefaultValues,
        };
        unsafe {
            instance.get(|| {
                TestDefaultValues {
                    double_field: ::std::option::Option::None,
                    float_field: ::std::option::Option::None,
                    int32_field: ::std::option::Option::None,
                    int64_field: ::std::option::Option::None,
                    uint32_field: ::std::option::Option::None,
                    uint64_field: ::std::option::Option::None,
                    sint32_field: ::std::option::Option::None,
                    sint64_field: ::std::option::Option::None,
                    fixed32_field: ::std::option::Option::None,
                    fixed64_field: ::std::option::Option::None,
                    sfixed32_field: ::std::option::Option::None,
                    sfixed64_field: ::std::option::Option::None,
                    bool_field: ::std::option::Option::None,
                    string_field: ::protobuf::SingularField::none(),
                    bytes_field: ::protobuf::SingularField::none(),
                    enum_field: ::std::option::Option::None,
                    enum_field_without_default: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional double double_field = 1;

    pub fn clear_double_field(&mut self) {
        self.double_field = ::std::option::Option::None;
    }

    pub fn has_double_field(&self) -> bool {
        self.double_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_double_field(&mut self, v: f64) {
        self.double_field = ::std::option::Option::Some(v);
    }

    pub fn get_double_field(&self) -> f64 {
        self.double_field.unwrap_or(1f64)
    }

    // optional float float_field = 2;

    pub fn clear_float_field(&mut self) {
        self.float_field = ::std::option::Option::None;
    }

    pub fn has_float_field(&self) -> bool {
        self.float_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_float_field(&mut self, v: f32) {
        self.float_field = ::std::option::Option::Some(v);
    }

    pub fn get_float_field(&self) -> f32 {
        self.float_field.unwrap_or(2f32)
    }

    // optional int32 int32_field = 3;

    pub fn clear_int32_field(&mut self) {
        self.int32_field = ::std::option::Option::None;
    }

    pub fn has_int32_field(&self) -> bool {
        self.int32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_int32_field(&mut self, v: i32) {
        self.int32_field = ::std::option::Option::Some(v);
    }

    pub fn get_int32_field(&self) -> i32 {
        self.int32_field.unwrap_or(3i32)
    }

    // optional int64 int64_field = 4;

    pub fn clear_int64_field(&mut self) {
        self.int64_field = ::std::option::Option::None;
    }

    pub fn has_int64_field(&self) -> bool {
        self.int64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_int64_field(&mut self, v: i64) {
        self.int64_field = ::std::option::Option::Some(v);
    }

    pub fn get_int64_field(&self) -> i64 {
        self.int64_field.unwrap_or(4i64)
    }

    // optional uint32 uint32_field = 5;

    pub fn clear_uint32_field(&mut self) {
        self.uint32_field = ::std::option::Option::None;
    }

    pub fn has_uint32_field(&self) -> bool {
        self.uint32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uint32_field(&mut self, v: u32) {
        self.uint32_field = ::std::option::Option::Some(v);
    }

    pub fn get_uint32_field(&self) -> u32 {
        self.uint32_field.unwrap_or(5u32)
    }

    // optional uint64 uint64_field = 6;

    pub fn clear_uint64_field(&mut self) {
        self.uint64_field = ::std::option::Option::None;
    }

    pub fn has_uint64_field(&self) -> bool {
        self.uint64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uint64_field(&mut self, v: u64) {
        self.uint64_field = ::std::option::Option::Some(v);
    }

    pub fn get_uint64_field(&self) -> u64 {
        self.uint64_field.unwrap_or(6u64)
    }

    // optional sint32 sint32_field = 7;

    pub fn clear_sint32_field(&mut self) {
        self.sint32_field = ::std::option::Option::None;
    }

    pub fn has_sint32_field(&self) -> bool {
        self.sint32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sint32_field(&mut self, v: i32) {
        self.sint32_field = ::std::option::Option::Some(v);
    }

    pub fn get_sint32_field(&self) -> i32 {
        self.sint32_field.unwrap_or(7i32)
    }

    // optional sint64 sint64_field = 8;

    pub fn clear_sint64_field(&mut self) {
        self.sint64_field = ::std::option::Option::None;
    }

    pub fn has_sint64_field(&self) -> bool {
        self.sint64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sint64_field(&mut self, v: i64) {
        self.sint64_field = ::std::option::Option::Some(v);
    }

    pub fn get_sint64_field(&self) -> i64 {
        self.sint64_field.unwrap_or(8i64)
    }

    // optional fixed32 fixed32_field = 9;

    pub fn clear_fixed32_field(&mut self) {
        self.fixed32_field = ::std::option::Option::None;
    }

    pub fn has_fixed32_field(&self) -> bool {
        self.fixed32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fixed32_field(&mut self, v: u32) {
        self.fixed32_field = ::std::option::Option::Some(v);
    }

    pub fn get_fixed32_field(&self) -> u32 {
        self.fixed32_field.unwrap_or(9u32)
    }

    // optional fixed64 fixed64_field = 10;

    pub fn clear_fixed64_field(&mut self) {
        self.fixed64_field = ::std::option::Option::None;
    }

    pub fn has_fixed64_field(&self) -> bool {
        self.fixed64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fixed64_field(&mut self, v: u64) {
        self.fixed64_field = ::std::option::Option::Some(v);
    }

    pub fn get_fixed64_field(&self) -> u64 {
        self.fixed64_field.unwrap_or(10u64)
    }

    // optional sfixed32 sfixed32_field = 11;

    pub fn clear_sfixed32_field(&mut self) {
        self.sfixed32_field = ::std::option::Option::None;
    }

    pub fn has_sfixed32_field(&self) -> bool {
        self.sfixed32_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sfixed32_field(&mut self, v: i32) {
        self.sfixed32_field = ::std::option::Option::Some(v);
    }

    pub fn get_sfixed32_field(&self) -> i32 {
        self.sfixed32_field.unwrap_or(11i32)
    }

    // optional sfixed64 sfixed64_field = 12;

    pub fn clear_sfixed64_field(&mut self) {
        self.sfixed64_field = ::std::option::Option::None;
    }

    pub fn has_sfixed64_field(&self) -> bool {
        self.sfixed64_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sfixed64_field(&mut self, v: i64) {
        self.sfixed64_field = ::std::option::Option::Some(v);
    }

    pub fn get_sfixed64_field(&self) -> i64 {
        self.sfixed64_field.unwrap_or(12i64)
    }

    // optional bool bool_field = 13;

    pub fn clear_bool_field(&mut self) {
        self.bool_field = ::std::option::Option::None;
    }

    pub fn has_bool_field(&self) -> bool {
        self.bool_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bool_field(&mut self, v: bool) {
        self.bool_field = ::std::option::Option::Some(v);
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
    pub fn mut_string_field(&mut self) -> &mut ::std::string::String {
        if self.string_field.is_none() {
            self.string_field.set_default();
        };
        self.string_field.as_mut().unwrap()
    }

    // Take field
    pub fn take_string_field(&mut self) -> ::std::string::String {
        self.string_field.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_string_field(&self) -> &str {
        match self.string_field.as_ref() {
            Some(v) => &v,
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
    pub fn mut_bytes_field(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.bytes_field.is_none() {
            self.bytes_field.set_default();
        };
        self.bytes_field.as_mut().unwrap()
    }

    // Take field
    pub fn take_bytes_field(&mut self) -> ::std::vec::Vec<u8> {
        self.bytes_field.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bytes_field(&self) -> &[u8] {
        match self.bytes_field.as_ref() {
            Some(v) => &v,
            None => b"cde\n33",
        }
    }

    // optional .basic.EnumForDefaultValue enum_field = 16;

    pub fn clear_enum_field(&mut self) {
        self.enum_field = ::std::option::Option::None;
    }

    pub fn has_enum_field(&self) -> bool {
        self.enum_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enum_field(&mut self, v: EnumForDefaultValue) {
        self.enum_field = ::std::option::Option::Some(v);
    }

    pub fn get_enum_field(&self) -> EnumForDefaultValue {
        self.enum_field.unwrap_or(EnumForDefaultValue::TWO)
    }

    // optional .basic.EnumForDefaultValue enum_field_without_default = 17;

    pub fn clear_enum_field_without_default(&mut self) {
        self.enum_field_without_default = ::std::option::Option::None;
    }

    pub fn has_enum_field_without_default(&self) -> bool {
        self.enum_field_without_default.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enum_field_without_default(&mut self, v: EnumForDefaultValue) {
        self.enum_field_without_default = ::std::option::Option::Some(v);
    }

    pub fn get_enum_field_without_default(&self) -> EnumForDefaultValue {
        self.enum_field_without_default.unwrap_or(EnumForDefaultValue::ONE)
    }
}

impl ::protobuf::Message for TestDefaultValues {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.double_field = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.float_field = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.int32_field = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.int64_field = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.uint32_field = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.uint64_field = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.sint32_field = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint64());
                    self.sint64_field = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed32());
                    self.fixed32_field = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.fixed64_field = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sfixed32());
                    self.sfixed32_field = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sfixed64());
                    self.sfixed64_field = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.bool_field = ::std::option::Option::Some(tmp);
                },
                14 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.string_field));
                },
                15 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bytes_field));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.enum_field = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.enum_field_without_default = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.double_field.is_some() {
            my_size += 9;
        };
        if self.float_field.is_some() {
            my_size += 5;
        };
        for value in &self.int32_field {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.int64_field {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.uint32_field {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.uint64_field {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.sint32_field {
            my_size += ::protobuf::rt::value_varint_zigzag_size(7, *value);
        };
        for value in &self.sint64_field {
            my_size += ::protobuf::rt::value_varint_zigzag_size(8, *value);
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
        for value in &self.string_field {
            my_size += ::protobuf::rt::string_size(14, &value);
        };
        for value in &self.bytes_field {
            my_size += ::protobuf::rt::bytes_size(15, &value);
        };
        for value in &self.enum_field {
            my_size += ::protobuf::rt::enum_size(16, *value);
        };
        for value in &self.enum_field_without_default {
            my_size += ::protobuf::rt::enum_size(17, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.double_field {
            try!(os.write_double(1, v));
        };
        if let Some(v) = self.float_field {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.int32_field {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.int64_field {
            try!(os.write_int64(4, v));
        };
        if let Some(v) = self.uint32_field {
            try!(os.write_uint32(5, v));
        };
        if let Some(v) = self.uint64_field {
            try!(os.write_uint64(6, v));
        };
        if let Some(v) = self.sint32_field {
            try!(os.write_sint32(7, v));
        };
        if let Some(v) = self.sint64_field {
            try!(os.write_sint64(8, v));
        };
        if let Some(v) = self.fixed32_field {
            try!(os.write_fixed32(9, v));
        };
        if let Some(v) = self.fixed64_field {
            try!(os.write_fixed64(10, v));
        };
        if let Some(v) = self.sfixed32_field {
            try!(os.write_sfixed32(11, v));
        };
        if let Some(v) = self.sfixed64_field {
            try!(os.write_sfixed64(12, v));
        };
        if let Some(v) = self.bool_field {
            try!(os.write_bool(13, v));
        };
        if let Some(v) = self.string_field.as_ref() {
            try!(os.write_string(14, &v));
        };
        if let Some(v) = self.bytes_field.as_ref() {
            try!(os.write_bytes(15, &v));
        };
        if let Some(v) = self.enum_field {
            try!(os.write_enum(16, v.value()));
        };
        if let Some(v) = self.enum_field_without_default {
            try!(os.write_enum(17, v.value()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestDefaultValues>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestDefaultValues {
    fn new() -> TestDefaultValues {
        TestDefaultValues::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestDefaultValues>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "double_field",
                    TestDefaultValues::has_double_field,
                    TestDefaultValues::get_double_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "float_field",
                    TestDefaultValues::has_float_field,
                    TestDefaultValues::get_float_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "int32_field",
                    TestDefaultValues::has_int32_field,
                    TestDefaultValues::get_int32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "int64_field",
                    TestDefaultValues::has_int64_field,
                    TestDefaultValues::get_int64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "uint32_field",
                    TestDefaultValues::has_uint32_field,
                    TestDefaultValues::get_uint32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "uint64_field",
                    TestDefaultValues::has_uint64_field,
                    TestDefaultValues::get_uint64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sint32_field",
                    TestDefaultValues::has_sint32_field,
                    TestDefaultValues::get_sint32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "sint64_field",
                    TestDefaultValues::has_sint64_field,
                    TestDefaultValues::get_sint64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "fixed32_field",
                    TestDefaultValues::has_fixed32_field,
                    TestDefaultValues::get_fixed32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "fixed64_field",
                    TestDefaultValues::has_fixed64_field,
                    TestDefaultValues::get_fixed64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sfixed32_field",
                    TestDefaultValues::has_sfixed32_field,
                    TestDefaultValues::get_sfixed32_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "sfixed64_field",
                    TestDefaultValues::has_sfixed64_field,
                    TestDefaultValues::get_sfixed64_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "bool_field",
                    TestDefaultValues::has_bool_field,
                    TestDefaultValues::get_bool_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "string_field",
                    TestDefaultValues::has_string_field,
                    TestDefaultValues::get_string_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bytes_field",
                    TestDefaultValues::has_bytes_field,
                    TestDefaultValues::get_bytes_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "enum_field",
                    TestDefaultValues::has_enum_field,
                    TestDefaultValues::get_enum_field,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "enum_field_without_default",
                    TestDefaultValues::has_enum_field_without_default,
                    TestDefaultValues::get_enum_field_without_default,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestDefaultValues>(
                    "TestDefaultValues",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
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

impl ::std::cmp::PartialEq for TestDefaultValues {
    fn eq(&self, other: &TestDefaultValues) -> bool {
        self.double_field == other.double_field &&
        self.float_field == other.float_field &&
        self.int32_field == other.int32_field &&
        self.int64_field == other.int64_field &&
        self.uint32_field == other.uint32_field &&
        self.uint64_field == other.uint64_field &&
        self.sint32_field == other.sint32_field &&
        self.sint64_field == other.sint64_field &&
        self.fixed32_field == other.fixed32_field &&
        self.fixed64_field == other.fixed64_field &&
        self.sfixed32_field == other.sfixed32_field &&
        self.sfixed64_field == other.sfixed64_field &&
        self.bool_field == other.bool_field &&
        self.string_field == other.string_field &&
        self.bytes_field == other.bytes_field &&
        self.enum_field == other.enum_field &&
        self.enum_field_without_default == other.enum_field_without_default &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestDefaultValues {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestExtremeDefaultValues {
    // message fields
    inf_double: ::std::option::Option<f64>,
    neg_inf_double: ::std::option::Option<f64>,
    nan_double: ::std::option::Option<f64>,
    inf_float: ::std::option::Option<f32>,
    neg_inf_float: ::std::option::Option<f32>,
    nan_float: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestExtremeDefaultValues {}

impl TestExtremeDefaultValues {
    pub fn new() -> TestExtremeDefaultValues {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestExtremeDefaultValues {
        static mut instance: ::protobuf::lazy::Lazy<TestExtremeDefaultValues> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestExtremeDefaultValues,
        };
        unsafe {
            instance.get(|| {
                TestExtremeDefaultValues {
                    inf_double: ::std::option::Option::None,
                    neg_inf_double: ::std::option::Option::None,
                    nan_double: ::std::option::Option::None,
                    inf_float: ::std::option::Option::None,
                    neg_inf_float: ::std::option::Option::None,
                    nan_float: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional double inf_double = 14;

    pub fn clear_inf_double(&mut self) {
        self.inf_double = ::std::option::Option::None;
    }

    pub fn has_inf_double(&self) -> bool {
        self.inf_double.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inf_double(&mut self, v: f64) {
        self.inf_double = ::std::option::Option::Some(v);
    }

    pub fn get_inf_double(&self) -> f64 {
        self.inf_double.unwrap_or(::std::f64::INFINITY)
    }

    // optional double neg_inf_double = 15;

    pub fn clear_neg_inf_double(&mut self) {
        self.neg_inf_double = ::std::option::Option::None;
    }

    pub fn has_neg_inf_double(&self) -> bool {
        self.neg_inf_double.is_some()
    }

    // Param is passed by value, moved
    pub fn set_neg_inf_double(&mut self, v: f64) {
        self.neg_inf_double = ::std::option::Option::Some(v);
    }

    pub fn get_neg_inf_double(&self) -> f64 {
        self.neg_inf_double.unwrap_or(::std::f64::NEG_INFINITY)
    }

    // optional double nan_double = 16;

    pub fn clear_nan_double(&mut self) {
        self.nan_double = ::std::option::Option::None;
    }

    pub fn has_nan_double(&self) -> bool {
        self.nan_double.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nan_double(&mut self, v: f64) {
        self.nan_double = ::std::option::Option::Some(v);
    }

    pub fn get_nan_double(&self) -> f64 {
        self.nan_double.unwrap_or(::std::f64::NAN)
    }

    // optional float inf_float = 17;

    pub fn clear_inf_float(&mut self) {
        self.inf_float = ::std::option::Option::None;
    }

    pub fn has_inf_float(&self) -> bool {
        self.inf_float.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inf_float(&mut self, v: f32) {
        self.inf_float = ::std::option::Option::Some(v);
    }

    pub fn get_inf_float(&self) -> f32 {
        self.inf_float.unwrap_or(::std::f32::INFINITY)
    }

    // optional float neg_inf_float = 18;

    pub fn clear_neg_inf_float(&mut self) {
        self.neg_inf_float = ::std::option::Option::None;
    }

    pub fn has_neg_inf_float(&self) -> bool {
        self.neg_inf_float.is_some()
    }

    // Param is passed by value, moved
    pub fn set_neg_inf_float(&mut self, v: f32) {
        self.neg_inf_float = ::std::option::Option::Some(v);
    }

    pub fn get_neg_inf_float(&self) -> f32 {
        self.neg_inf_float.unwrap_or(::std::f32::NEG_INFINITY)
    }

    // optional float nan_float = 19;

    pub fn clear_nan_float(&mut self) {
        self.nan_float = ::std::option::Option::None;
    }

    pub fn has_nan_float(&self) -> bool {
        self.nan_float.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nan_float(&mut self, v: f32) {
        self.nan_float = ::std::option::Option::Some(v);
    }

    pub fn get_nan_float(&self) -> f32 {
        self.nan_float.unwrap_or(::std::f32::NAN)
    }
}

impl ::protobuf::Message for TestExtremeDefaultValues {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.inf_double = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.neg_inf_double = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_double());
                    self.nan_double = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.inf_float = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.neg_inf_float = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.nan_float = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.inf_double.is_some() {
            my_size += 9;
        };
        if self.neg_inf_double.is_some() {
            my_size += 9;
        };
        if self.nan_double.is_some() {
            my_size += 10;
        };
        if self.inf_float.is_some() {
            my_size += 6;
        };
        if self.neg_inf_float.is_some() {
            my_size += 6;
        };
        if self.nan_float.is_some() {
            my_size += 6;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.inf_double {
            try!(os.write_double(14, v));
        };
        if let Some(v) = self.neg_inf_double {
            try!(os.write_double(15, v));
        };
        if let Some(v) = self.nan_double {
            try!(os.write_double(16, v));
        };
        if let Some(v) = self.inf_float {
            try!(os.write_float(17, v));
        };
        if let Some(v) = self.neg_inf_float {
            try!(os.write_float(18, v));
        };
        if let Some(v) = self.nan_float {
            try!(os.write_float(19, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestExtremeDefaultValues>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestExtremeDefaultValues {
    fn new() -> TestExtremeDefaultValues {
        TestExtremeDefaultValues::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestExtremeDefaultValues>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "inf_double",
                    TestExtremeDefaultValues::has_inf_double,
                    TestExtremeDefaultValues::get_inf_double,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "neg_inf_double",
                    TestExtremeDefaultValues::has_neg_inf_double,
                    TestExtremeDefaultValues::get_neg_inf_double,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "nan_double",
                    TestExtremeDefaultValues::has_nan_double,
                    TestExtremeDefaultValues::get_nan_double,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "inf_float",
                    TestExtremeDefaultValues::has_inf_float,
                    TestExtremeDefaultValues::get_inf_float,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "neg_inf_float",
                    TestExtremeDefaultValues::has_neg_inf_float,
                    TestExtremeDefaultValues::get_neg_inf_float,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "nan_float",
                    TestExtremeDefaultValues::has_nan_float,
                    TestExtremeDefaultValues::get_nan_float,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestExtremeDefaultValues>(
                    "TestExtremeDefaultValues",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestExtremeDefaultValues {
    fn clear(&mut self) {
        self.clear_inf_double();
        self.clear_neg_inf_double();
        self.clear_nan_double();
        self.clear_inf_float();
        self.clear_neg_inf_float();
        self.clear_nan_float();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestExtremeDefaultValues {
    fn eq(&self, other: &TestExtremeDefaultValues) -> bool {
        self.inf_double == other.inf_double &&
        self.neg_inf_double == other.neg_inf_double &&
        self.nan_double == other.nan_double &&
        self.inf_float == other.inf_float &&
        self.neg_inf_float == other.neg_inf_float &&
        self.nan_float == other.nan_float &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestExtremeDefaultValues {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestInvalidTag {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestInvalidTag {}

impl TestInvalidTag {
    pub fn new() -> TestInvalidTag {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestInvalidTag {
        static mut instance: ::protobuf::lazy::Lazy<TestInvalidTag> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestInvalidTag,
        };
        unsafe {
            instance.get(|| {
                TestInvalidTag {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for TestInvalidTag {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestInvalidTag>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestInvalidTag {
    fn new() -> TestInvalidTag {
        TestInvalidTag::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestInvalidTag>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<TestInvalidTag>(
                    "TestInvalidTag",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestInvalidTag {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestInvalidTag {
    fn eq(&self, other: &TestInvalidTag) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestInvalidTag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestTruncated {
    // message fields
    ints: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestTruncated {}

impl TestTruncated {
    pub fn new() -> TestTruncated {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestTruncated {
        static mut instance: ::protobuf::lazy::Lazy<TestTruncated> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestTruncated,
        };
        unsafe {
            instance.get(|| {
                TestTruncated {
                    ints: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated fixed32 ints = 2;

    pub fn clear_ints(&mut self) {
        self.ints.clear();
    }

    // Param is passed by value, moved
    pub fn set_ints(&mut self, v: ::std::vec::Vec<u32>) {
        self.ints = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ints(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ints
    }

    // Take field
    pub fn take_ints(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.ints, ::std::vec::Vec::new())
    }

    pub fn get_ints(&self) -> &[u32] {
        &self.ints
    }
}

impl ::protobuf::Message for TestTruncated {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    try!(::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.ints));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.ints.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.ints.len() as u32) + (self.ints.len() * 4) as u32;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.ints.is_empty() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.ints.len() * 4) as u32));
            for v in &self.ints {
                try!(os.write_fixed32_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestTruncated>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestTruncated {
    fn new() -> TestTruncated {
        TestTruncated::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestTruncated>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "ints",
                    TestTruncated::get_ints,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestTruncated>(
                    "TestTruncated",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestTruncated {
    fn clear(&mut self) {
        self.clear_ints();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestTruncated {
    fn eq(&self, other: &TestTruncated) -> bool {
        self.ints == other.ints &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestTruncated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestBugSint {
    // message fields
    s32: ::std::option::Option<i32>,
    s64: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TestBugSint {}

impl TestBugSint {
    pub fn new() -> TestBugSint {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestBugSint {
        static mut instance: ::protobuf::lazy::Lazy<TestBugSint> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestBugSint,
        };
        unsafe {
            instance.get(|| {
                TestBugSint {
                    s32: ::std::option::Option::None,
                    s64: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional sint32 s32 = 1;

    pub fn clear_s32(&mut self) {
        self.s32 = ::std::option::Option::None;
    }

    pub fn has_s32(&self) -> bool {
        self.s32.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s32(&mut self, v: i32) {
        self.s32 = ::std::option::Option::Some(v);
    }

    pub fn get_s32(&self) -> i32 {
        self.s32.unwrap_or(0)
    }

    // optional sint64 s64 = 2;

    pub fn clear_s64(&mut self) {
        self.s64 = ::std::option::Option::None;
    }

    pub fn has_s64(&self) -> bool {
        self.s64.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s64(&mut self, v: i64) {
        self.s64 = ::std::option::Option::Some(v);
    }

    pub fn get_s64(&self) -> i64 {
        self.s64.unwrap_or(0)
    }
}

impl ::protobuf::Message for TestBugSint {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.s32 = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint64());
                    self.s64 = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.s32 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, *value);
        };
        for value in &self.s64 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.s32 {
            try!(os.write_sint32(1, v));
        };
        if let Some(v) = self.s64 {
            try!(os.write_sint64(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestBugSint>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestBugSint {
    fn new() -> TestBugSint {
        TestBugSint::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestBugSint>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "s32",
                    TestBugSint::has_s32,
                    TestBugSint::get_s32,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "s64",
                    TestBugSint::has_s64,
                    TestBugSint::get_s64,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestBugSint>(
                    "TestBugSint",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestBugSint {
    fn clear(&mut self) {
        self.clear_s32();
        self.clear_s64();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestBugSint {
    fn eq(&self, other: &TestBugSint) -> bool {
        self.s32 == other.s32 &&
        self.s64 == other.s64 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestBugSint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum TestEnumDescriptor {
    RED = 1,
    BLUE = 2,
    GREEN = 3,
}

impl ::protobuf::ProtobufEnum for TestEnumDescriptor {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<TestEnumDescriptor> {
        match value {
            1 => ::std::option::Option::Some(TestEnumDescriptor::RED),
            2 => ::std::option::Option::Some(TestEnumDescriptor::BLUE),
            3 => ::std::option::Option::Some(TestEnumDescriptor::GREEN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [TestEnumDescriptor] = &[
            TestEnumDescriptor::RED,
            TestEnumDescriptor::BLUE,
            TestEnumDescriptor::GREEN,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<TestEnumDescriptor>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("TestEnumDescriptor", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for TestEnumDescriptor {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EnumForDefaultValue {
    ONE = 1,
    TWO = 2,
    THREE = 3,
}

impl ::protobuf::ProtobufEnum for EnumForDefaultValue {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EnumForDefaultValue> {
        match value {
            1 => ::std::option::Option::Some(EnumForDefaultValue::ONE),
            2 => ::std::option::Option::Some(EnumForDefaultValue::TWO),
            3 => ::std::option::Option::Some(EnumForDefaultValue::THREE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EnumForDefaultValue] = &[
            EnumForDefaultValue::ONE,
            EnumForDefaultValue::TWO,
            EnumForDefaultValue::THREE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EnumForDefaultValue>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EnumForDefaultValue", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EnumForDefaultValue {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x14, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x70, 0x62, 0x5f, 0x62, 0x61, 0x73, 0x69, 0x63,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x62, 0x61, 0x73, 0x69, 0x63, 0x22, 0x12, 0x0a,
    0x05, 0x54, 0x65, 0x73, 0x74, 0x31, 0x12, 0x09, 0x0a, 0x01, 0x61, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x05, 0x22, 0x12, 0x0a, 0x05, 0x54, 0x65, 0x73, 0x74, 0x32, 0x12, 0x09, 0x0a, 0x01, 0x62, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x09, 0x22, 0x20, 0x0a, 0x05, 0x54, 0x65, 0x73, 0x74, 0x33, 0x12, 0x17,
    0x0a, 0x01, 0x63, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x62, 0x61, 0x73, 0x69,
    0x63, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x31, 0x22, 0x16, 0x0a, 0x05, 0x54, 0x65, 0x73, 0x74, 0x34,
    0x12, 0x0d, 0x0a, 0x01, 0x64, 0x18, 0x04, 0x20, 0x03, 0x28, 0x05, 0x42, 0x02, 0x10, 0x01, 0x22,
    0x3a, 0x0a, 0x12, 0x54, 0x65, 0x73, 0x74, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x64, 0x55, 0x6e, 0x70,
    0x61, 0x63, 0x6b, 0x65, 0x64, 0x12, 0x10, 0x0a, 0x08, 0x75, 0x6e, 0x70, 0x61, 0x63, 0x6b, 0x65,
    0x64, 0x18, 0x04, 0x20, 0x03, 0x28, 0x05, 0x12, 0x12, 0x0a, 0x06, 0x70, 0x61, 0x63, 0x6b, 0x65,
    0x64, 0x18, 0x05, 0x20, 0x03, 0x28, 0x05, 0x42, 0x02, 0x10, 0x01, 0x22, 0x18, 0x0a, 0x09, 0x54,
    0x65, 0x73, 0x74, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x12, 0x0b, 0x0a, 0x03, 0x66, 0x6f, 0x6f, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x05, 0x22, 0x19, 0x0a, 0x0c, 0x54, 0x65, 0x73, 0x74, 0x52, 0x65, 0x71,
    0x75, 0x69, 0x72, 0x65, 0x64, 0x12, 0x09, 0x0a, 0x01, 0x62, 0x18, 0x05, 0x20, 0x02, 0x28, 0x08,
    0x22, 0x1e, 0x0a, 0x11, 0x54, 0x65, 0x73, 0x74, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x46,
    0x69, 0x65, 0x6c, 0x64, 0x73, 0x12, 0x09, 0x0a, 0x01, 0x61, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05,
    0x22, 0x5f, 0x0a, 0x11, 0x54, 0x65, 0x73, 0x74, 0x53, 0x65, 0x6c, 0x66, 0x52, 0x65, 0x66, 0x65,
    0x72, 0x65, 0x6e, 0x63, 0x65, 0x12, 0x24, 0x0a, 0x02, 0x72, 0x31, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0b, 0x32, 0x18, 0x2e, 0x62, 0x61, 0x73, 0x69, 0x63, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x53, 0x65,
    0x6c, 0x66, 0x52, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x12, 0x24, 0x0a, 0x02, 0x72,
    0x32, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x62, 0x61, 0x73, 0x69, 0x63, 0x2e,
    0x54, 0x65, 0x73, 0x74, 0x53, 0x65, 0x6c, 0x66, 0x52, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63,
    0x65, 0x22, 0x25, 0x0a, 0x18, 0x54, 0x65, 0x73, 0x74, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74,
    0x49, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x09, 0x0a,
    0x01, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x22, 0x45, 0x0a, 0x13, 0x54, 0x65, 0x73, 0x74,
    0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x12,
    0x2e, 0x0a, 0x05, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f,
    0x2e, 0x62, 0x61, 0x73, 0x69, 0x63, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x44, 0x65, 0x66, 0x61, 0x75,
    0x6c, 0x74, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x22,
    0x1f, 0x0a, 0x0e, 0x54, 0x65, 0x73, 0x74, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f,
    0x72, 0x12, 0x0d, 0x0a, 0x05, 0x73, 0x74, 0x75, 0x66, 0x66, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x05,
    0x22, 0x8c, 0x03, 0x0a, 0x11, 0x54, 0x65, 0x73, 0x74, 0x54, 0x79, 0x70, 0x65, 0x73, 0x53, 0x69,
    0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x12, 0x14, 0x0a, 0x0c, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65,
    0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x12, 0x13, 0x0a, 0x0b,
    0x66, 0x6c, 0x6f, 0x61, 0x74, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x02, 0x12, 0x13, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x12, 0x14, 0x0a, 0x0c, 0x75,
    0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x0d, 0x12, 0x14, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74, 0x33,
    0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x11, 0x12, 0x14, 0x0a,
    0x0c, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x12, 0x12, 0x15, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x18, 0x09, 0x20, 0x01, 0x28, 0x07, 0x12, 0x15, 0x0a, 0x0d, 0x66, 0x69,
    0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0a, 0x20, 0x01, 0x28,
    0x06, 0x12, 0x16, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0f, 0x12, 0x16, 0x0a, 0x0e, 0x73, 0x66, 0x69,
    0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0c, 0x20, 0x01, 0x28,
    0x10, 0x12, 0x12, 0x0a, 0x0a, 0x62, 0x6f, 0x6f, 0x6c, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18,
    0x0d, 0x20, 0x01, 0x28, 0x08, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x09, 0x12, 0x13, 0x0a, 0x0b, 0x62,
    0x79, 0x74, 0x65, 0x73, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0c,
    0x12, 0x2d, 0x0a, 0x0a, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x10,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x62, 0x61, 0x73, 0x69, 0x63, 0x2e, 0x54, 0x65, 0x73,
    0x74, 0x45, 0x6e, 0x75, 0x6d, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x22,
    0xc4, 0x03, 0x0a, 0x11, 0x54, 0x65, 0x73, 0x74, 0x54, 0x79, 0x70, 0x65, 0x73, 0x52, 0x65, 0x70,
    0x65, 0x61, 0x74, 0x65, 0x64, 0x12, 0x18, 0x0a, 0x0c, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x01, 0x42, 0x02, 0x10, 0x00, 0x12,
    0x17, 0x0a, 0x0b, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x02, 0x42, 0x02, 0x10, 0x00, 0x12, 0x17, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x33,
    0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x03, 0x20, 0x03, 0x28, 0x05, 0x42, 0x02, 0x10,
    0x00, 0x12, 0x17, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x18, 0x04, 0x20, 0x03, 0x28, 0x03, 0x42, 0x02, 0x10, 0x00, 0x12, 0x18, 0x0a, 0x0c, 0x75, 0x69,
    0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0d,
    0x42, 0x02, 0x10, 0x00, 0x12, 0x18, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x18, 0x06, 0x20, 0x03, 0x28, 0x04, 0x42, 0x02, 0x10, 0x00, 0x12, 0x18,
    0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x07,
    0x20, 0x03, 0x28, 0x11, 0x42, 0x02, 0x10, 0x00, 0x12, 0x18, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74,
    0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x08, 0x20, 0x03, 0x28, 0x12, 0x42, 0x02,
    0x10, 0x00, 0x12, 0x19, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x18, 0x09, 0x20, 0x03, 0x28, 0x07, 0x42, 0x02, 0x10, 0x00, 0x12, 0x19, 0x0a,
    0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0a,
    0x20, 0x03, 0x28, 0x06, 0x42, 0x02, 0x10, 0x00, 0x12, 0x1a, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78,
    0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0b, 0x20, 0x03, 0x28, 0x0f,
    0x42, 0x02, 0x10, 0x00, 0x12, 0x1a, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34,
    0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0c, 0x20, 0x03, 0x28, 0x10, 0x42, 0x02, 0x10, 0x00,
    0x12, 0x16, 0x0a, 0x0a, 0x62, 0x6f, 0x6f, 0x6c, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0d,
    0x20, 0x03, 0x28, 0x08, 0x42, 0x02, 0x10, 0x00, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69,
    0x6e, 0x67, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0e, 0x20, 0x03, 0x28, 0x09, 0x12, 0x13,
    0x0a, 0x0b, 0x62, 0x79, 0x74, 0x65, 0x73, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0f, 0x20,
    0x03, 0x28, 0x0c, 0x12, 0x31, 0x0a, 0x0a, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x10, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x62, 0x61, 0x73, 0x69, 0x63, 0x2e,
    0x54, 0x65, 0x73, 0x74, 0x45, 0x6e, 0x75, 0x6d, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x6f, 0x72, 0x42, 0x02, 0x10, 0x00, 0x22, 0xca, 0x03, 0x0a, 0x17, 0x54, 0x65, 0x73, 0x74, 0x54,
    0x79, 0x70, 0x65, 0x73, 0x52, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x50, 0x61, 0x63, 0x6b,
    0x65, 0x64, 0x12, 0x18, 0x0a, 0x0c, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x5f, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x01, 0x42, 0x02, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x0b,
    0x66, 0x6c, 0x6f, 0x61, 0x74, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x02, 0x42, 0x02, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x18, 0x03, 0x20, 0x03, 0x28, 0x05, 0x42, 0x02, 0x10, 0x01, 0x12, 0x17,
    0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x04, 0x20,
    0x03, 0x28, 0x03, 0x42, 0x02, 0x10, 0x01, 0x12, 0x18, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x33,
    0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0d, 0x42, 0x02, 0x10,
    0x01, 0x12, 0x18, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x06, 0x20, 0x03, 0x28, 0x04, 0x42, 0x02, 0x10, 0x01, 0x12, 0x18, 0x0a, 0x0c, 0x73,
    0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x07, 0x20, 0x03, 0x28,
    0x11, 0x42, 0x02, 0x10, 0x01, 0x12, 0x18, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x08, 0x20, 0x03, 0x28, 0x12, 0x42, 0x02, 0x10, 0x01, 0x12,
    0x19, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x18, 0x09, 0x20, 0x03, 0x28, 0x07, 0x42, 0x02, 0x10, 0x01, 0x12, 0x19, 0x0a, 0x0d, 0x66, 0x69,
    0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0a, 0x20, 0x03, 0x28,
    0x06, 0x42, 0x02, 0x10, 0x01, 0x12, 0x1a, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33,
    0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0b, 0x20, 0x03, 0x28, 0x0f, 0x42, 0x02, 0x10,
    0x01, 0x12, 0x1a, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x18, 0x0c, 0x20, 0x03, 0x28, 0x10, 0x42, 0x02, 0x10, 0x01, 0x12, 0x16, 0x0a,
    0x0a, 0x62, 0x6f, 0x6f, 0x6c, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0d, 0x20, 0x03, 0x28,
    0x08, 0x42, 0x02, 0x10, 0x01, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0e, 0x20, 0x03, 0x28, 0x09, 0x12, 0x13, 0x0a, 0x0b, 0x62,
    0x79, 0x74, 0x65, 0x73, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0f, 0x20, 0x03, 0x28, 0x0c,
    0x12, 0x31, 0x0a, 0x0a, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x10,
    0x20, 0x03, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x62, 0x61, 0x73, 0x69, 0x63, 0x2e, 0x54, 0x65, 0x73,
    0x74, 0x45, 0x6e, 0x75, 0x6d, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x42,
    0x02, 0x10, 0x01, 0x22, 0x90, 0x04, 0x0a, 0x11, 0x54, 0x65, 0x73, 0x74, 0x44, 0x65, 0x66, 0x61,
    0x75, 0x6c, 0x74, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x12, 0x17, 0x0a, 0x0c, 0x64, 0x6f, 0x75,
    0x62, 0x6c, 0x65, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x3a,
    0x01, 0x31, 0x12, 0x16, 0x0a, 0x0b, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x3a, 0x01, 0x32, 0x12, 0x16, 0x0a, 0x0b, 0x69, 0x6e,
    0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x3a,
    0x01, 0x33, 0x12, 0x16, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x3a, 0x01, 0x34, 0x12, 0x17, 0x0a, 0x0c, 0x75, 0x69,
    0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d,
    0x3a, 0x01, 0x35, 0x12, 0x17, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x3a, 0x01, 0x36, 0x12, 0x17, 0x0a, 0x0c,
    0x73, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x11, 0x3a, 0x01, 0x37, 0x12, 0x17, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x12, 0x3a, 0x01, 0x38, 0x12, 0x18,
    0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18,
    0x09, 0x20, 0x01, 0x28, 0x07, 0x3a, 0x01, 0x39, 0x12, 0x19, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65,
    0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x06, 0x3a,
    0x02, 0x31, 0x30, 0x12, 0x1a, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0f, 0x3a, 0x02, 0x31, 0x31, 0x12,
    0x1a, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x10, 0x3a, 0x02, 0x31, 0x32, 0x12, 0x18, 0x0a, 0x0a, 0x62,
    0x6f, 0x6f, 0x6c, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x08, 0x3a,
    0x04, 0x74, 0x72, 0x75, 0x65, 0x12, 0x1c, 0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x09, 0x3a, 0x06, 0x61, 0x62, 0x63,
    0x0a, 0x32, 0x32, 0x12, 0x1c, 0x0a, 0x0b, 0x62, 0x79, 0x74, 0x65, 0x73, 0x5f, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0c, 0x3a, 0x07, 0x63, 0x64, 0x65, 0x5c, 0x6e, 0x33,
    0x33, 0x12, 0x33, 0x0a, 0x0a, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18,
    0x10, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1a, 0x2e, 0x62, 0x61, 0x73, 0x69, 0x63, 0x2e, 0x45, 0x6e,
    0x75, 0x6d, 0x46, 0x6f, 0x72, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x56, 0x61, 0x6c, 0x75,
    0x65, 0x3a, 0x03, 0x54, 0x57, 0x4f, 0x12, 0x3e, 0x0a, 0x1a, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x5f, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75, 0x74, 0x5f, 0x64, 0x65, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1a, 0x2e, 0x62, 0x61, 0x73,
    0x69, 0x63, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x46, 0x6f, 0x72, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c,
    0x74, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22, 0xb7, 0x01, 0x0a, 0x18, 0x54, 0x65, 0x73, 0x74, 0x45,
    0x78, 0x74, 0x72, 0x65, 0x6d, 0x65, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x56, 0x61, 0x6c,
    0x75, 0x65, 0x73, 0x12, 0x17, 0x0a, 0x0a, 0x69, 0x6e, 0x66, 0x5f, 0x64, 0x6f, 0x75, 0x62, 0x6c,
    0x65, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x01, 0x3a, 0x03, 0x69, 0x6e, 0x66, 0x12, 0x1c, 0x0a, 0x0e,
    0x6e, 0x65, 0x67, 0x5f, 0x69, 0x6e, 0x66, 0x5f, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x18, 0x0f,
    0x20, 0x01, 0x28, 0x01, 0x3a, 0x04, 0x2d, 0x69, 0x6e, 0x66, 0x12, 0x17, 0x0a, 0x0a, 0x6e, 0x61,
    0x6e, 0x5f, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x18, 0x10, 0x20, 0x01, 0x28, 0x01, 0x3a, 0x03,
    0x6e, 0x61, 0x6e, 0x12, 0x16, 0x0a, 0x09, 0x69, 0x6e, 0x66, 0x5f, 0x66, 0x6c, 0x6f, 0x61, 0x74,
    0x18, 0x11, 0x20, 0x01, 0x28, 0x02, 0x3a, 0x03, 0x69, 0x6e, 0x66, 0x12, 0x1b, 0x0a, 0x0d, 0x6e,
    0x65, 0x67, 0x5f, 0x69, 0x6e, 0x66, 0x5f, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x18, 0x12, 0x20, 0x01,
    0x28, 0x02, 0x3a, 0x04, 0x2d, 0x69, 0x6e, 0x66, 0x12, 0x16, 0x0a, 0x09, 0x6e, 0x61, 0x6e, 0x5f,
    0x66, 0x6c, 0x6f, 0x61, 0x74, 0x18, 0x13, 0x20, 0x01, 0x28, 0x02, 0x3a, 0x03, 0x6e, 0x61, 0x6e,
    0x22, 0x10, 0x0a, 0x0e, 0x54, 0x65, 0x73, 0x74, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x54,
    0x61, 0x67, 0x22, 0x21, 0x0a, 0x0d, 0x54, 0x65, 0x73, 0x74, 0x54, 0x72, 0x75, 0x6e, 0x63, 0x61,
    0x74, 0x65, 0x64, 0x12, 0x10, 0x0a, 0x04, 0x69, 0x6e, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x07, 0x42, 0x02, 0x10, 0x01, 0x22, 0x27, 0x0a, 0x0b, 0x54, 0x65, 0x73, 0x74, 0x42, 0x75, 0x67,
    0x53, 0x69, 0x6e, 0x74, 0x12, 0x0b, 0x0a, 0x03, 0x73, 0x33, 0x32, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x11, 0x12, 0x0b, 0x0a, 0x03, 0x73, 0x36, 0x34, 0x18, 0x02, 0x20, 0x01, 0x28, 0x12, 0x2a, 0x32,
    0x0a, 0x12, 0x54, 0x65, 0x73, 0x74, 0x45, 0x6e, 0x75, 0x6d, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x6f, 0x72, 0x12, 0x07, 0x0a, 0x03, 0x52, 0x45, 0x44, 0x10, 0x01, 0x12, 0x08, 0x0a,
    0x04, 0x42, 0x4c, 0x55, 0x45, 0x10, 0x02, 0x12, 0x09, 0x0a, 0x05, 0x47, 0x52, 0x45, 0x45, 0x4e,
    0x10, 0x03, 0x2a, 0x32, 0x0a, 0x13, 0x45, 0x6e, 0x75, 0x6d, 0x46, 0x6f, 0x72, 0x44, 0x65, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x07, 0x0a, 0x03, 0x4f, 0x4e, 0x45,
    0x10, 0x01, 0x12, 0x07, 0x0a, 0x03, 0x54, 0x57, 0x4f, 0x10, 0x02, 0x12, 0x09, 0x0a, 0x05, 0x54,
    0x48, 0x52, 0x45, 0x45, 0x10, 0x03, 0x4a, 0x82, 0x56, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xa4,
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
    0x39, 0x0c, 0x0d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x3c, 0x00, 0x4d, 0x01, 0x0a,
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
    0x03, 0x12, 0x03, 0x4b, 0x21, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x0f, 0x12, 0x03,
    0x4c, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0f, 0x04, 0x12, 0x03, 0x4c, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0f, 0x06, 0x12, 0x03, 0x4c, 0x0d, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x4c, 0x20, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x4c, 0x2d, 0x2f, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x0d, 0x12, 0x04, 0x4f, 0x00, 0x60, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x03,
    0x4f, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x50, 0x04, 0x34,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x50, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x03, 0x50, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x50, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x50, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00,
    0x08, 0x12, 0x03, 0x50, 0x25, 0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x12, 0x03, 0x50, 0x26, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x50, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x50, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b,
    0x04, 0x0d, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x50, 0x26, 0x2c,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x50,
    0x2d, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x03, 0x51, 0x04, 0x32, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x03, 0x51, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x01, 0x05, 0x12, 0x03, 0x51, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x01, 0x01, 0x12, 0x03, 0x51, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x51, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x08,
    0x12, 0x03, 0x51, 0x23, 0x31, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x51, 0x24, 0x30, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x01, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x51, 0x24, 0x2a, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x01,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x51, 0x24, 0x2a, 0x0a, 0x12, 0x0a, 0x0b, 0x04,
    0x0d, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x51, 0x24, 0x2a, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x51, 0x2b,
    0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x02, 0x12, 0x03, 0x52, 0x04, 0x32, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x04, 0x12, 0x03, 0x52, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x02, 0x05, 0x12, 0x03, 0x52, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x52, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x52, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x08, 0x12,
    0x03, 0x52, 0x23, 0x31, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x52, 0x24, 0x30, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x02, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x52, 0x24, 0x2a, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x02, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x52, 0x24, 0x2a, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d,
    0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x52, 0x24, 0x2a, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x0d, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x52, 0x2b, 0x30,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x03, 0x12, 0x03, 0x53, 0x04, 0x32, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x03, 0x04, 0x12, 0x03, 0x53, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x03, 0x05, 0x12, 0x03, 0x53, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x53, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x53, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x08, 0x12, 0x03,
    0x53, 0x23, 0x31, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x12,
    0x03, 0x53, 0x24, 0x30, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x53, 0x24, 0x2a, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x03, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x53, 0x24, 0x2a, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02,
    0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x53, 0x24, 0x2a, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x0d, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x53, 0x2b, 0x30, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x04, 0x12, 0x03, 0x54, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x04, 0x04, 0x12, 0x03, 0x54, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x54, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x54, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x54, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x08, 0x12, 0x03, 0x54,
    0x25, 0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03,
    0x54, 0x26, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x12, 0x03, 0x54, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x04, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x54, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x04,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x54, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x0d, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x54, 0x2d, 0x32, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0d, 0x02, 0x05, 0x12, 0x03, 0x55, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x05, 0x04, 0x12, 0x03, 0x55, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x05, 0x05, 0x12, 0x03, 0x55, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x55, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x03, 0x12, 0x03,
    0x55, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x08, 0x12, 0x03, 0x55, 0x25,
    0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x55,
    0x26, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x03, 0x55, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x55, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x05, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x55, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x0d, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x55, 0x2d, 0x32, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0d, 0x02, 0x06, 0x12, 0x03, 0x56, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x06, 0x04, 0x12, 0x03, 0x56, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x56, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x56, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x03, 0x12, 0x03, 0x56,
    0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x08, 0x12, 0x03, 0x56, 0x25, 0x33,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x56, 0x26,
    0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x56, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x56, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x06, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x56, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d,
    0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x56, 0x2d, 0x32, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0d, 0x02, 0x07, 0x12, 0x03, 0x57, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x07, 0x04, 0x12, 0x03, 0x57, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x05,
    0x12, 0x03, 0x57, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x57, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x03, 0x12, 0x03, 0x57, 0x23,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x08, 0x12, 0x03, 0x57, 0x25, 0x33, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x57, 0x26, 0x32,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x57,
    0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x57, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x07, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x57, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02,
    0x07, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x57, 0x2d, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0d, 0x02, 0x08, 0x12, 0x03, 0x58, 0x04, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08,
    0x04, 0x12, 0x03, 0x58, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x05, 0x12,
    0x03, 0x58, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x01, 0x12, 0x03, 0x58,
    0x15, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x03, 0x12, 0x03, 0x58, 0x25, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x08, 0x12, 0x03, 0x58, 0x27, 0x35, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x0d, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x58, 0x28, 0x34, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x58, 0x28,
    0x2e, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x58, 0x28, 0x2e, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x58, 0x28, 0x2e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x08,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x58, 0x2f, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d,
    0x02, 0x09, 0x12, 0x03, 0x59, 0x04, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x04,
    0x12, 0x03, 0x59, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x05, 0x12, 0x03,
    0x59, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x01, 0x12, 0x03, 0x59, 0x15,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x03, 0x12, 0x03, 0x59, 0x25, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x08, 0x12, 0x03, 0x59, 0x28, 0x36, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x0d, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x59, 0x29, 0x35, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x0d, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x59, 0x29, 0x2f,
    0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x59, 0x29, 0x2f, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x59, 0x29, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x09, 0x08,
    0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x59, 0x30, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02,
    0x0a, 0x12, 0x03, 0x5a, 0x04, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x04, 0x12,
    0x03, 0x5a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x5a,
    0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x5a, 0x16, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x5a, 0x27, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x08, 0x12, 0x03, 0x5a, 0x2a, 0x38, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x0d, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x5a, 0x2b, 0x37, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x0d, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x5a, 0x2b, 0x31, 0x0a,
    0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x5a,
    0x2b, 0x31, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x5a, 0x2b, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x0a, 0x08, 0xe7,
    0x07, 0x00, 0x03, 0x12, 0x03, 0x5a, 0x32, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0b,
    0x12, 0x03, 0x5b, 0x04, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b, 0x04, 0x12, 0x03,
    0x5b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x5b, 0x0d,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x5b, 0x16, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x5b, 0x27, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x0b, 0x08, 0x12, 0x03, 0x5b, 0x2a, 0x38, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x0d, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x5b, 0x2b, 0x37, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x0d, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x5b, 0x2b, 0x31, 0x0a, 0x11,
    0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x5b, 0x2b,
    0x31, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x5b, 0x2b, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x0b, 0x08, 0xe7, 0x07,
    0x00, 0x03, 0x12, 0x03, 0x5b, 0x32, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0c, 0x12,
    0x03, 0x5c, 0x04, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x5c,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x5c, 0x0d, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x5c, 0x12, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x5c, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x0c, 0x08, 0x12, 0x03, 0x5c, 0x22, 0x30, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d,
    0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x5c, 0x23, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x0d, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x5c, 0x23, 0x29, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x0d, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x5c, 0x23, 0x29,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x5c, 0x23, 0x29, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x5c, 0x2a, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0d, 0x12, 0x03,
    0x5d, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x5d, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x5d, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x5d, 0x14, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x5d, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0d, 0x02, 0x0e, 0x12, 0x03, 0x5e, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e,
    0x04, 0x12, 0x03, 0x5e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x05, 0x12,
    0x03, 0x5e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x5e,
    0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x5e, 0x21, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0f, 0x12, 0x03, 0x5f, 0x04, 0x3f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x0f, 0x04, 0x12, 0x03, 0x5f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x0f, 0x06, 0x12, 0x03, 0x5f, 0x0d, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x0f, 0x01, 0x12, 0x03, 0x5f, 0x20, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0f, 0x03,
    0x12, 0x03, 0x5f, 0x2d, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0f, 0x08, 0x12, 0x03,
    0x5f, 0x30, 0x3e, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x0f, 0x08, 0xe7, 0x07, 0x00, 0x12,
    0x03, 0x5f, 0x31, 0x3d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x0f, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x5f, 0x31, 0x37, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x0f, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x5f, 0x31, 0x37, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02,
    0x0f, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5f, 0x31, 0x37, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x0d, 0x02, 0x0f, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x5f, 0x38, 0x3d, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x04, 0x62, 0x00, 0x73, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x0e, 0x01, 0x12, 0x03, 0x62, 0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12,
    0x03, 0x63, 0x04, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03, 0x63,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x03, 0x63, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x63, 0x14, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x03, 0x63, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x08, 0x12, 0x03, 0x63, 0x25, 0x32, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e,
    0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x63, 0x26, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x0e, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x63, 0x26, 0x2c, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x0e, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x63, 0x26, 0x2c,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x63, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x63, 0x2d, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x03,
    0x64, 0x04, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x03, 0x64, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x05, 0x12, 0x03, 0x64, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x03, 0x64, 0x13, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x03, 0x64, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x01, 0x08, 0x12, 0x03, 0x64, 0x23, 0x30, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02,
    0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x64, 0x24, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x64, 0x24, 0x2a, 0x0a, 0x11, 0x0a, 0x0a,
    0x04, 0x0e, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x64, 0x24, 0x2a, 0x0a,
    0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x64, 0x24, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03,
    0x12, 0x03, 0x64, 0x2b, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x02, 0x12, 0x03, 0x65,
    0x04, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x04, 0x12, 0x03, 0x65, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x05, 0x12, 0x03, 0x65, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x01, 0x12, 0x03, 0x65, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x02, 0x03, 0x12, 0x03, 0x65, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x02, 0x08, 0x12, 0x03, 0x65, 0x23, 0x30, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x02,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x65, 0x24, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02,
    0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x65, 0x24, 0x2a, 0x0a, 0x11, 0x0a, 0x0a, 0x04,
    0x0e, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x65, 0x24, 0x2a, 0x0a, 0x12,
    0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x65,
    0x24, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12,
    0x03, 0x65, 0x2b, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x03, 0x12, 0x03, 0x66, 0x04,
    0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x04, 0x12, 0x03, 0x66, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x05, 0x12, 0x03, 0x66, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x03, 0x01, 0x12, 0x03, 0x66, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x03, 0x03, 0x12, 0x03, 0x66, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x03, 0x08, 0x12, 0x03, 0x66, 0x23, 0x30, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x03, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x66, 0x24, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x03,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x66, 0x24, 0x2a, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e,
    0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x66, 0x24, 0x2a, 0x0a, 0x12, 0x0a,
    0x0b, 0x04, 0x0e, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x66, 0x24,
    0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03,
    0x66, 0x2b, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x04, 0x12, 0x03, 0x67, 0x04, 0x33,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x04, 0x12, 0x03, 0x67, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x05, 0x12, 0x03, 0x67, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x04, 0x01, 0x12, 0x03, 0x67, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x67, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04,
    0x08, 0x12, 0x03, 0x67, 0x25, 0x32, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x04, 0x08, 0xe7,
    0x07, 0x00, 0x12, 0x03, 0x67, 0x26, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x04, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x67, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02,
    0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x67, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b,
    0x04, 0x0e, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x67, 0x26, 0x2c,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x67,
    0x2d, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x05, 0x12, 0x03, 0x68, 0x04, 0x33, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x04, 0x12, 0x03, 0x68, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x05, 0x05, 0x12, 0x03, 0x68, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x05, 0x01, 0x12, 0x03, 0x68, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x68, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x08,
    0x12, 0x03, 0x68, 0x25, 0x32, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x05, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x68, 0x26, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x05, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x68, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x05,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x68, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04,
    0x0e, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x68, 0x26, 0x2c, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x68, 0x2d,
    0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x06, 0x12, 0x03, 0x69, 0x04, 0x33, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x06, 0x04, 0x12, 0x03, 0x69, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x06, 0x05, 0x12, 0x03, 0x69, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x06, 0x01, 0x12, 0x03, 0x69, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x06,
    0x03, 0x12, 0x03, 0x69, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x06, 0x08, 0x12,
    0x03, 0x69, 0x25, 0x32, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x69, 0x26, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x06, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x69, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x06, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x69, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e,
    0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x69, 0x26, 0x2c, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x0e, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x69, 0x2d, 0x31,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x07, 0x12, 0x03, 0x6a, 0x04, 0x33, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x07, 0x04, 0x12, 0x03, 0x6a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x07, 0x05, 0x12, 0x03, 0x6a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x07, 0x01, 0x12, 0x03, 0x6a, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x07, 0x03,
    0x12, 0x03, 0x6a, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x07, 0x08, 0x12, 0x03,
    0x6a, 0x25, 0x32, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x12,
    0x03, 0x6a, 0x26, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x6a, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x07, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x6a, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02,
    0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6a, 0x26, 0x2c, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x0e, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x6a, 0x2d, 0x31, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x08, 0x12, 0x03, 0x6b, 0x04, 0x35, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x08, 0x04, 0x12, 0x03, 0x6b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x08, 0x05, 0x12, 0x03, 0x6b, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x08,
    0x01, 0x12, 0x03, 0x6b, 0x15, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x08, 0x03, 0x12,
    0x03, 0x6b, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x08, 0x08, 0x12, 0x03, 0x6b,
    0x27, 0x34, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03,
    0x6b, 0x28, 0x33, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x12, 0x03, 0x6b, 0x28, 0x2e, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x08, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x6b, 0x28, 0x2e, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x08,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6b, 0x28, 0x2e, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x0e, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x6b, 0x2f, 0x33, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0e, 0x02, 0x09, 0x12, 0x03, 0x6c, 0x04, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x09, 0x04, 0x12, 0x03, 0x6c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x09, 0x05, 0x12, 0x03, 0x6c, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x09, 0x01,
    0x12, 0x03, 0x6c, 0x15, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x09, 0x03, 0x12, 0x03,
    0x6c, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x09, 0x08, 0x12, 0x03, 0x6c, 0x28,
    0x35, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x6c,
    0x29, 0x34, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x03, 0x6c, 0x29, 0x2f, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x6c, 0x29, 0x2f, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x09, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6c, 0x29, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x0e, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x6c, 0x30, 0x34, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0e, 0x02, 0x0a, 0x12, 0x03, 0x6d, 0x04, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x0a, 0x04, 0x12, 0x03, 0x6d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0a,
    0x05, 0x12, 0x03, 0x6d, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0a, 0x01, 0x12,
    0x03, 0x6d, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x6d,
    0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0a, 0x08, 0x12, 0x03, 0x6d, 0x2a, 0x37,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x6d, 0x2b,
    0x36, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x6d, 0x2b, 0x31, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x6d, 0x2b, 0x31, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x0a, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6d, 0x2b, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e,
    0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x6d, 0x32, 0x36, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0e, 0x02, 0x0b, 0x12, 0x03, 0x6e, 0x04, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x0b, 0x04, 0x12, 0x03, 0x6e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0b, 0x05,
    0x12, 0x03, 0x6e, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0b, 0x01, 0x12, 0x03,
    0x6e, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x6e, 0x27,
    0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0b, 0x08, 0x12, 0x03, 0x6e, 0x2a, 0x37, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x6e, 0x2b, 0x36,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x6e,
    0x2b, 0x31, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x6e, 0x2b, 0x31, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x0b, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6e, 0x2b, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02,
    0x0b, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x6e, 0x32, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0e, 0x02, 0x0c, 0x12, 0x03, 0x6f, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0c,
    0x04, 0x12, 0x03, 0x6f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0c, 0x05, 0x12,
    0x03, 0x6f, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x6f,
    0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x6f, 0x1f, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0c, 0x08, 0x12, 0x03, 0x6f, 0x22, 0x2f, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x0e, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x6f, 0x23, 0x2e, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x6f, 0x23,
    0x29, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x6f, 0x23, 0x29, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x6f, 0x23, 0x29, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x0c,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x6f, 0x2a, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x0d, 0x12, 0x03, 0x70, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0d, 0x04,
    0x12, 0x03, 0x70, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0d, 0x05, 0x12, 0x03,
    0x70, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x70, 0x14,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x70, 0x23, 0x25, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x0e, 0x12, 0x03, 0x71, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x71, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x0e, 0x05, 0x12, 0x03, 0x71, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0e,
    0x01, 0x12, 0x03, 0x71, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0e, 0x03, 0x12,
    0x03, 0x71, 0x21, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x0f, 0x12, 0x03, 0x72, 0x04,
    0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0f, 0x04, 0x12, 0x03, 0x72, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x0f, 0x06, 0x12, 0x03, 0x72, 0x0d, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x72, 0x20, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x72, 0x2d, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x0f, 0x08, 0x12, 0x03, 0x72, 0x30, 0x3d, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0e, 0x02, 0x0f, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x72, 0x31, 0x3c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x0f,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x72, 0x31, 0x37, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0e,
    0x02, 0x0f, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x72, 0x31, 0x37, 0x0a, 0x12, 0x0a,
    0x0b, 0x04, 0x0e, 0x02, 0x0f, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x72, 0x31,
    0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x0f, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03,
    0x72, 0x38, 0x3c, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x75, 0x00, 0x79, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x75, 0x05, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x76, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x76, 0x04, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x76, 0x0a, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x77, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x77, 0x04, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x77, 0x0a, 0x0b, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03, 0x78, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x78, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x78, 0x0c, 0x0d, 0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x05, 0x7b, 0x00,
    0x8e, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x03, 0x7b, 0x08, 0x19, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x03, 0x7c, 0x04, 0x40, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x7c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x7c, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x7c, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x08, 0x12, 0x03, 0x7c,
    0x31, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x07, 0x12, 0x03, 0x7c, 0x3d, 0x3e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x03, 0x7d, 0x04, 0x40, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x01, 0x05, 0x12, 0x03, 0x7d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x7d, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x7d, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x08, 0x12, 0x03,
    0x7d, 0x31, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x07, 0x12, 0x03, 0x7d, 0x3d,
    0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x02, 0x12, 0x03, 0x7f, 0x04, 0x40, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x04, 0x12, 0x03, 0x7f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x02, 0x05, 0x12, 0x03, 0x7f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x7f, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x7f, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x08, 0x12,
    0x03, 0x7f, 0x31, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x07, 0x12, 0x03, 0x7f,
    0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x03, 0x12, 0x04, 0x80, 0x01, 0x04, 0x40,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x04, 0x12, 0x04, 0x80, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x05, 0x12, 0x04, 0x80, 0x01, 0x0d, 0x12, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x01, 0x12, 0x04, 0x80, 0x01, 0x13, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x03, 0x03, 0x12, 0x04, 0x80, 0x01, 0x21, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x03, 0x08, 0x12, 0x04, 0x80, 0x01, 0x31, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x03, 0x07, 0x12, 0x04, 0x80, 0x01, 0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f,
    0x02, 0x04, 0x12, 0x04, 0x81, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04,
    0x04, 0x12, 0x04, 0x81, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x05,
    0x12, 0x04, 0x81, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x81, 0x01, 0x14, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x81, 0x01, 0x23, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x08, 0x12, 0x04, 0x81,
    0x01, 0x31, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x07, 0x12, 0x04, 0x81, 0x01,
    0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x05, 0x12, 0x04, 0x82, 0x01, 0x04, 0x40,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x04, 0x12, 0x04, 0x82, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x05, 0x12, 0x04, 0x82, 0x01, 0x0d, 0x13, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x01, 0x12, 0x04, 0x82, 0x01, 0x14, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x05, 0x03, 0x12, 0x04, 0x82, 0x01, 0x23, 0x24, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x05, 0x08, 0x12, 0x04, 0x82, 0x01, 0x31, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x05, 0x07, 0x12, 0x04, 0x82, 0x01, 0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f,
    0x02, 0x06, 0x12, 0x04, 0x83, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06,
    0x04, 0x12, 0x04, 0x83, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x05,
    0x12, 0x04, 0x83, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x01, 0x12,
    0x04, 0x83, 0x01, 0x14, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x03, 0x12, 0x04,
    0x83, 0x01, 0x23, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x08, 0x12, 0x04, 0x83,
    0x01, 0x31, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x07, 0x12, 0x04, 0x83, 0x01,
    0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x07, 0x12, 0x04, 0x84, 0x01, 0x04, 0x40,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x04, 0x12, 0x04, 0x84, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x05, 0x12, 0x04, 0x84, 0x01, 0x0d, 0x13, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x01, 0x12, 0x04, 0x84, 0x01, 0x14, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x07, 0x03, 0x12, 0x04, 0x84, 0x01, 0x23, 0x24, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x07, 0x08, 0x12, 0x04, 0x84, 0x01, 0x31, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x07, 0x07, 0x12, 0x04, 0x84, 0x01, 0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f,
    0x02, 0x08, 0x12, 0x04, 0x85, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08,
    0x04, 0x12, 0x04, 0x85, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x05,
    0x12, 0x04, 0x85, 0x01, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x01, 0x12,
    0x04, 0x85, 0x01, 0x15, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x03, 0x12, 0x04,
    0x85, 0x01, 0x25, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x08, 0x12, 0x04, 0x85,
    0x01, 0x31, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x07, 0x12, 0x04, 0x85, 0x01,
    0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x09, 0x12, 0x04, 0x86, 0x01, 0x04, 0x40,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x04, 0x12, 0x04, 0x86, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x05, 0x12, 0x04, 0x86, 0x01, 0x0d, 0x14, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x01, 0x12, 0x04, 0x86, 0x01, 0x15, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x09, 0x03, 0x12, 0x04, 0x86, 0x01, 0x25, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x09, 0x08, 0x12, 0x04, 0x86, 0x01, 0x31, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x09, 0x07, 0x12, 0x04, 0x86, 0x01, 0x3c, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f,
    0x02, 0x0a, 0x12, 0x04, 0x87, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a,
    0x04, 0x12, 0x04, 0x87, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a, 0x05,
    0x12, 0x04, 0x87, 0x01, 0x0d, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a, 0x01, 0x12,
    0x04, 0x87, 0x01, 0x16, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a, 0x03, 0x12, 0x04,
    0x87, 0x01, 0x27, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a, 0x08, 0x12, 0x04, 0x87,
    0x01, 0x31, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a, 0x07, 0x12, 0x04, 0x87, 0x01,
    0x3c, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x0b, 0x12, 0x04, 0x88, 0x01, 0x04, 0x40,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0b, 0x04, 0x12, 0x04, 0x88, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0b, 0x05, 0x12, 0x04, 0x88, 0x01, 0x0d, 0x15, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0b, 0x01, 0x12, 0x04, 0x88, 0x01, 0x16, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x0b, 0x03, 0x12, 0x04, 0x88, 0x01, 0x27, 0x29, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x0b, 0x08, 0x12, 0x04, 0x88, 0x01, 0x31, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x0b, 0x07, 0x12, 0x04, 0x88, 0x01, 0x3c, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f,
    0x02, 0x0c, 0x12, 0x04, 0x89, 0x01, 0x04, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0c,
    0x04, 0x12, 0x04, 0x89, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0c, 0x05,
    0x12, 0x04, 0x89, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0c, 0x01, 0x12,
    0x04, 0x89, 0x01, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0c, 0x03, 0x12, 0x04,
    0x89, 0x01, 0x1f, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0c, 0x08, 0x12, 0x04, 0x89,
    0x01, 0x31, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0c, 0x07, 0x12, 0x04, 0x89, 0x01,
    0x3c, 0x40, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x0d, 0x12, 0x04, 0x8a, 0x01, 0x04, 0x47,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0d, 0x04, 0x12, 0x04, 0x8a, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0d, 0x05, 0x12, 0x04, 0x8a, 0x01, 0x0d, 0x13, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0d, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x14, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x0d, 0x03, 0x12, 0x04, 0x8a, 0x01, 0x23, 0x25, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x0d, 0x08, 0x12, 0x04, 0x8a, 0x01, 0x31, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x0d, 0x07, 0x12, 0x04, 0x8a, 0x01, 0x3c, 0x45, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f,
    0x02, 0x0e, 0x12, 0x04, 0x8b, 0x01, 0x04, 0x47, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0e,
    0x04, 0x12, 0x04, 0x8b, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0e, 0x05,
    0x12, 0x04, 0x8b, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0e, 0x01, 0x12,
    0x04, 0x8b, 0x01, 0x13, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0e, 0x03, 0x12, 0x04,
    0x8b, 0x01, 0x21, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0e, 0x08, 0x12, 0x04, 0x8b,
    0x01, 0x31, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0e, 0x07, 0x12, 0x04, 0x8b, 0x01,
    0x3c, 0x45, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x0f, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x41,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0f, 0x04, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0f, 0x06, 0x12, 0x04, 0x8c, 0x01, 0x0d, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0f, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x21, 0x2b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x0f, 0x03, 0x12, 0x04, 0x8c, 0x01, 0x2e, 0x30, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x0f, 0x08, 0x12, 0x04, 0x8c, 0x01, 0x31, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x0f, 0x07, 0x12, 0x04, 0x8c, 0x01, 0x3c, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f,
    0x02, 0x10, 0x12, 0x04, 0x8d, 0x01, 0x04, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x10,
    0x04, 0x12, 0x04, 0x8d, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x10, 0x06,
    0x12, 0x04, 0x8d, 0x01, 0x0d, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x10, 0x01, 0x12,
    0x04, 0x8d, 0x01, 0x21, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x10, 0x03, 0x12, 0x04,
    0x8d, 0x01, 0x3e, 0x40, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0x90, 0x01, 0x00, 0x98,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0x90, 0x01, 0x08, 0x20, 0x0a,
    0x39, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0x92, 0x01, 0x04, 0x34, 0x1a, 0x2b, 0x20,
    0x54, 0x65, 0x78, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x6e, 0x66, 0x69, 0x6e, 0x69,
    0x74, 0x65, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x2d, 0x70, 0x6f, 0x69, 0x6e,
    0x74, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x92, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x92, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x92, 0x01, 0x14, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x92, 0x01, 0x21, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x08, 0x12,
    0x04, 0x92, 0x01, 0x24, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x07, 0x12, 0x04,
    0x92, 0x01, 0x2f, 0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x04, 0x93, 0x01,
    0x04, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x04, 0x93, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x04, 0x93, 0x01, 0x0d, 0x13,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0x93, 0x01, 0x14, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0x93, 0x01, 0x25, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x08, 0x12, 0x04, 0x93, 0x01, 0x28, 0x38, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x01, 0x07, 0x12, 0x04, 0x93, 0x01, 0x33, 0x37, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x10, 0x02, 0x02, 0x12, 0x04, 0x94, 0x01, 0x04, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x94, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x02, 0x05, 0x12, 0x04, 0x94, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x94, 0x01, 0x14, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x03,
    0x12, 0x04, 0x94, 0x01, 0x21, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x08, 0x12,
    0x04, 0x94, 0x01, 0x24, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x07, 0x12, 0x04,
    0x94, 0x01, 0x2f, 0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x03, 0x12, 0x04, 0x95, 0x01,
    0x04, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x04, 0x12, 0x04, 0x95, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x05, 0x12, 0x04, 0x95, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x01, 0x12, 0x04, 0x95, 0x01, 0x13, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x03, 0x12, 0x04, 0x95, 0x01, 0x1f, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x08, 0x12, 0x04, 0x95, 0x01, 0x22, 0x31, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x03, 0x07, 0x12, 0x04, 0x95, 0x01, 0x2d, 0x30, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x10, 0x02, 0x04, 0x12, 0x04, 0x96, 0x01, 0x04, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x96, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x04, 0x05, 0x12, 0x04, 0x96, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x96, 0x01, 0x13, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x96, 0x01, 0x23, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x08, 0x12,
    0x04, 0x96, 0x01, 0x26, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x07, 0x12, 0x04,
    0x96, 0x01, 0x31, 0x35, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x05, 0x12, 0x04, 0x97, 0x01,
    0x04, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x04, 0x12, 0x04, 0x97, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x05, 0x12, 0x04, 0x97, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x01, 0x12, 0x04, 0x97, 0x01, 0x13, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x03, 0x12, 0x04, 0x97, 0x01, 0x1f, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x08, 0x12, 0x04, 0x97, 0x01, 0x22, 0x31, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x05, 0x07, 0x12, 0x04, 0x97, 0x01, 0x2d, 0x30, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x11, 0x12, 0x06, 0x9a, 0x01, 0x00, 0x9b, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11,
    0x01, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0x9d,
    0x01, 0x00, 0x9f, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0x9d, 0x01,
    0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0x9e, 0x01, 0x04, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9e, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9e, 0x01, 0x0d, 0x14, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x15, 0x19, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9e, 0x01, 0x1c, 0x1d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x00, 0x08, 0x12, 0x04, 0x9e, 0x01, 0x1e, 0x2b, 0x0a, 0x10, 0x0a, 0x08, 0x04,
    0x12, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0x9e, 0x01, 0x1f, 0x2a, 0x0a, 0x11, 0x0a,
    0x09, 0x04, 0x12, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0x9e, 0x01, 0x1f, 0x25,
    0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x12, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04,
    0x9e, 0x01, 0x1f, 0x25, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x12, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x1f, 0x25, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x12, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0x9e, 0x01, 0x26, 0x2a, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x13, 0x12, 0x06, 0xa1, 0x01, 0x00, 0xa4, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13,
    0x01, 0x12, 0x04, 0xa1, 0x01, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12,
    0x04, 0xa2, 0x01, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xa2, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa2,
    0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa2, 0x01,
    0x14, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa2, 0x01, 0x1a,
    0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x04, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa3, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa3, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x14, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa3, 0x01, 0x1a, 0x1b,
];

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

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

#[derive(PartialEq,Clone,Default)]
pub struct Test1 {
    // message fields
    a: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(Test1::new)
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

    fn get_a_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.a
    }

    fn mut_a_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.a
    }
}

impl ::protobuf::Message for Test1 {
    fn is_initialized(&self) -> bool {
        if self.a.is_none() {
            return false;
        };
        true
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.a {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::CodedMessage for Test1 {
    fn merge_from<I: ::protobuf::InputSource>(&mut self, is: &mut ::protobuf::CodedInputStream<I>) -> ::protobuf::ProtobufResult<()> {
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
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "a",
                    Test1::get_a_for_reflect,
                    Test1::mut_a_for_reflect,
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

impl ::std::fmt::Debug for Test1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Test1 {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Test2 {
    // message fields
    b: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(Test2::new)
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

    fn get_b_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.b
    }

    fn mut_b_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.b
    }
}

impl ::protobuf::Message for Test2 {
    fn is_initialized(&self) -> bool {
        if self.b.is_none() {
            return false;
        };
        true
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.b.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::CodedMessage for Test2 {
    fn merge_from<I: ::protobuf::InputSource>(&mut self, is: &mut ::protobuf::CodedInputStream<I>) -> ::protobuf::ProtobufResult<()> {
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "b",
                    Test2::get_b_for_reflect,
                    Test2::mut_b_for_reflect,
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

impl ::std::fmt::Debug for Test2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Test2 {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Test3 {
    // message fields
    c: ::protobuf::SingularPtrField<Test1>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(Test3::new)
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

    fn get_c_for_reflect(&self) -> &::protobuf::SingularPtrField<Test1> {
        &self.c
    }

    fn mut_c_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Test1> {
        &mut self.c
    }
}

impl ::protobuf::Message for Test3 {
    fn is_initialized(&self) -> bool {
        if self.c.is_none() {
            return false;
        };
        true
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.c.as_ref() {
            let len = v.compute_size();
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::CodedMessage for Test3 {
    fn merge_from<I: ::protobuf::InputSource>(&mut self, is: &mut ::protobuf::CodedInputStream<I>) -> ::protobuf::ProtobufResult<()> {
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Test1>>(
                    "c",
                    Test3::get_c_for_reflect,
                    Test3::mut_c_for_reflect,
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

impl ::std::fmt::Debug for Test3 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Test3 {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Test4 {
    // message fields
    d: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(Test4::new)
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

    fn get_d_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.d
    }

    fn mut_d_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.d
    }
}

impl ::protobuf::Message for Test4 {
    fn is_initialized(&self) -> bool {
        true
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::CodedMessage for Test4 {
    fn merge_from<I: ::protobuf::InputSource>(&mut self, is: &mut ::protobuf::CodedInputStream<I>) -> ::protobuf::ProtobufResult<()> {
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
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "d",
                    Test4::get_d_for_reflect,
                    Test4::mut_d_for_reflect,
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

impl ::std::fmt::Debug for Test4 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Test4 {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestPackedUnpacked {
    // message fields
    unpacked: ::std::vec::Vec<i32>,
    packed: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TestPackedUnpacked::new)
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

    fn get_unpacked_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.unpacked
    }

    fn mut_unpacked_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.unpacked
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

    fn get_packed_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.packed
    }

    fn mut_packed_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.packed
    }
}

impl ::protobuf::Message for TestPackedUnpacked {
    fn is_initialized(&self) -> bool {
        true
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::CodedMessage for TestPackedUnpacked {
    fn merge_from<I: ::protobuf::InputSource>(&mut self, is: &mut ::protobuf::CodedInputStream<I>) -> ::protobuf::ProtobufResult<()> {
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
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "unpacked",
                    TestPackedUnpacked::get_unpacked_for_reflect,
                    TestPackedUnpacked::mut_unpacked_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "packed",
                    TestPackedUnpacked::get_packed_for_reflect,
                    TestPackedUnpacked::mut_packed_for_reflect,
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

impl ::std::fmt::Debug for TestPackedUnpacked {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestPackedUnpacked {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestEmpty {
    // message fields
    foo: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TestEmpty::new)
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

    fn get_foo_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.foo
    }

    fn mut_foo_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.foo
    }
}

impl ::protobuf::Message for TestEmpty {
    fn is_initialized(&self) -> bool {
        true
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.foo {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::CodedMessage for TestEmpty {
    fn merge_from<I: ::protobuf::InputSource>(&mut self, is: &mut ::protobuf::CodedInputStream<I>) -> ::protobuf::ProtobufResult<()> {
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
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "foo",
                    TestEmpty::get_foo_for_reflect,
                    TestEmpty::mut_foo_for_reflect,
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

impl ::std::fmt::Debug for TestEmpty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestEmpty {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestUnknownFields {
    // message fields
    a: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TestUnknownFields::new)
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

    fn get_a_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.a
    }

    fn mut_a_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.a
    }
}

impl ::protobuf::Message for TestUnknownFields {
    fn is_initialized(&self) -> bool {
        if self.a.is_none() {
            return false;
        };
        true
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.a {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::CodedMessage for TestUnknownFields {
    fn merge_from<I: ::protobuf::InputSource>(&mut self, is: &mut ::protobuf::CodedInputStream<I>) -> ::protobuf::ProtobufResult<()> {
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
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "a",
                    TestUnknownFields::get_a_for_reflect,
                    TestUnknownFields::mut_a_for_reflect,
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

impl ::std::fmt::Debug for TestUnknownFields {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestUnknownFields {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestSelfReference {
    // message fields
    r1: ::protobuf::SingularPtrField<TestSelfReference>,
    r2: ::protobuf::SingularPtrField<TestSelfReference>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TestSelfReference::new)
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

    fn get_r1_for_reflect(&self) -> &::protobuf::SingularPtrField<TestSelfReference> {
        &self.r1
    }

    fn mut_r1_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TestSelfReference> {
        &mut self.r1
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

    fn get_r2_for_reflect(&self) -> &::protobuf::SingularPtrField<TestSelfReference> {
        &self.r2
    }

    fn mut_r2_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TestSelfReference> {
        &mut self.r2
    }
}

impl ::protobuf::Message for TestSelfReference {
    fn is_initialized(&self) -> bool {
        if self.r1.is_none() {
            return false;
        };
        true
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.r1.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.r2.as_ref() {
            let len = v.compute_size();
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::CodedMessage for TestSelfReference {
    fn merge_from<I: ::protobuf::InputSource>(&mut self, is: &mut ::protobuf::CodedInputStream<I>) -> ::protobuf::ProtobufResult<()> {
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TestSelfReference>>(
                    "r1",
                    TestSelfReference::get_r1_for_reflect,
                    TestSelfReference::mut_r1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TestSelfReference>>(
                    "r2",
                    TestSelfReference::get_r2_for_reflect,
                    TestSelfReference::mut_r2_for_reflect,
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

impl ::std::fmt::Debug for TestSelfReference {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestSelfReference {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestDefaultInstanceField {
    // message fields
    s: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TestDefaultInstanceField::new)
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

    fn get_s_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.s
    }

    fn mut_s_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.s
    }
}

impl ::protobuf::Message for TestDefaultInstanceField {
    fn is_initialized(&self) -> bool {
        true
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.s.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::CodedMessage for TestDefaultInstanceField {
    fn merge_from<I: ::protobuf::InputSource>(&mut self, is: &mut ::protobuf::CodedInputStream<I>) -> ::protobuf::ProtobufResult<()> {
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "s",
                    TestDefaultInstanceField::get_s_for_reflect,
                    TestDefaultInstanceField::mut_s_for_reflect,
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

impl ::std::fmt::Debug for TestDefaultInstanceField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestDefaultInstanceField {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestDefaultInstance {
    // message fields
    field: ::protobuf::SingularPtrField<TestDefaultInstanceField>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TestDefaultInstance::new)
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

    fn get_field_for_reflect(&self) -> &::protobuf::SingularPtrField<TestDefaultInstanceField> {
        &self.field
    }

    fn mut_field_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TestDefaultInstanceField> {
        &mut self.field
    }
}

impl ::protobuf::Message for TestDefaultInstance {
    fn is_initialized(&self) -> bool {
        true
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.field.as_ref() {
            let len = v.compute_size();
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::CodedMessage for TestDefaultInstance {
    fn merge_from<I: ::protobuf::InputSource>(&mut self, is: &mut ::protobuf::CodedInputStream<I>) -> ::protobuf::ProtobufResult<()> {
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TestDefaultInstanceField>>(
                    "field",
                    TestDefaultInstance::get_field_for_reflect,
                    TestDefaultInstance::mut_field_for_reflect,
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

impl ::std::fmt::Debug for TestDefaultInstance {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestDefaultInstance {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestDescriptor {
    // message fields
    stuff: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TestDescriptor::new)
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

    fn get_stuff_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.stuff
    }

    fn mut_stuff_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.stuff
    }
}

impl ::protobuf::Message for TestDescriptor {
    fn is_initialized(&self) -> bool {
        true
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.stuff {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::CodedMessage for TestDescriptor {
    fn merge_from<I: ::protobuf::InputSource>(&mut self, is: &mut ::protobuf::CodedInputStream<I>) -> ::protobuf::ProtobufResult<()> {
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
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "stuff",
                    TestDescriptor::get_stuff_for_reflect,
                    TestDescriptor::mut_stuff_for_reflect,
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

impl ::std::fmt::Debug for TestDescriptor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestDescriptor {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
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
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TestTypesSingular::new)
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

    fn get_double_field_for_reflect(&self) -> &::std::option::Option<f64> {
        &self.double_field
    }

    fn mut_double_field_for_reflect(&mut self) -> &mut ::std::option::Option<f64> {
        &mut self.double_field
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

    fn get_float_field_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.float_field
    }

    fn mut_float_field_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.float_field
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

    fn get_int32_field_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.int32_field
    }

    fn mut_int32_field_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.int32_field
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

    fn get_int64_field_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.int64_field
    }

    fn mut_int64_field_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.int64_field
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

    fn get_uint32_field_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.uint32_field
    }

    fn mut_uint32_field_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.uint32_field
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

    fn get_uint64_field_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.uint64_field
    }

    fn mut_uint64_field_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.uint64_field
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

    fn get_sint32_field_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.sint32_field
    }

    fn mut_sint32_field_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.sint32_field
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

    fn get_sint64_field_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.sint64_field
    }

    fn mut_sint64_field_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.sint64_field
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

    fn get_fixed32_field_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.fixed32_field
    }

    fn mut_fixed32_field_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.fixed32_field
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

    fn get_fixed64_field_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.fixed64_field
    }

    fn mut_fixed64_field_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.fixed64_field
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

    fn get_sfixed32_field_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.sfixed32_field
    }

    fn mut_sfixed32_field_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.sfixed32_field
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

    fn get_sfixed64_field_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.sfixed64_field
    }

    fn mut_sfixed64_field_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.sfixed64_field
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

    fn get_bool_field_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.bool_field
    }

    fn mut_bool_field_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.bool_field
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

    fn get_string_field_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.string_field
    }

    fn mut_string_field_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.string_field
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

    fn get_bytes_field_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.bytes_field
    }

    fn mut_bytes_field_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.bytes_field
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

    fn get_enum_field_for_reflect(&self) -> &::std::option::Option<TestEnumDescriptor> {
        &self.enum_field
    }

    fn mut_enum_field_for_reflect(&mut self) -> &mut ::std::option::Option<TestEnumDescriptor> {
        &mut self.enum_field
    }
}

impl ::protobuf::Message for TestTypesSingular {
    fn is_initialized(&self) -> bool {
        true
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.double_field {
            my_size += 9;
        };
        if let Some(v) = self.float_field {
            my_size += 5;
        };
        if let Some(v) = self.int32_field {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.int64_field {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.uint32_field {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.uint64_field {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sint32_field {
            my_size += ::protobuf::rt::value_varint_zigzag_size(7, v);
        };
        if let Some(v) = self.sint64_field {
            my_size += ::protobuf::rt::value_varint_zigzag_size(8, v);
        };
        if let Some(v) = self.fixed32_field {
            my_size += 5;
        };
        if let Some(v) = self.fixed64_field {
            my_size += 9;
        };
        if let Some(v) = self.sfixed32_field {
            my_size += 5;
        };
        if let Some(v) = self.sfixed64_field {
            my_size += 9;
        };
        if let Some(v) = self.bool_field {
            my_size += 2;
        };
        if let Some(v) = self.string_field.as_ref() {
            my_size += ::protobuf::rt::string_size(14, &v);
        };
        if let Some(v) = self.bytes_field.as_ref() {
            my_size += ::protobuf::rt::bytes_size(15, &v);
        };
        if let Some(v) = self.enum_field {
            my_size += ::protobuf::rt::enum_size(16, v);
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::CodedMessage for TestTypesSingular {
    fn merge_from<I: ::protobuf::InputSource>(&mut self, is: &mut ::protobuf::CodedInputStream<I>) -> ::protobuf::ProtobufResult<()> {
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
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "double_field",
                    TestTypesSingular::get_double_field_for_reflect,
                    TestTypesSingular::mut_double_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "float_field",
                    TestTypesSingular::get_float_field_for_reflect,
                    TestTypesSingular::mut_float_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "int32_field",
                    TestTypesSingular::get_int32_field_for_reflect,
                    TestTypesSingular::mut_int32_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "int64_field",
                    TestTypesSingular::get_int64_field_for_reflect,
                    TestTypesSingular::mut_int64_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "uint32_field",
                    TestTypesSingular::get_uint32_field_for_reflect,
                    TestTypesSingular::mut_uint32_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "uint64_field",
                    TestTypesSingular::get_uint64_field_for_reflect,
                    TestTypesSingular::mut_uint64_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "sint32_field",
                    TestTypesSingular::get_sint32_field_for_reflect,
                    TestTypesSingular::mut_sint32_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "sint64_field",
                    TestTypesSingular::get_sint64_field_for_reflect,
                    TestTypesSingular::mut_sint64_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "fixed32_field",
                    TestTypesSingular::get_fixed32_field_for_reflect,
                    TestTypesSingular::mut_fixed32_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "fixed64_field",
                    TestTypesSingular::get_fixed64_field_for_reflect,
                    TestTypesSingular::mut_fixed64_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSfixed32>(
                    "sfixed32_field",
                    TestTypesSingular::get_sfixed32_field_for_reflect,
                    TestTypesSingular::mut_sfixed32_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSfixed64>(
                    "sfixed64_field",
                    TestTypesSingular::get_sfixed64_field_for_reflect,
                    TestTypesSingular::mut_sfixed64_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "bool_field",
                    TestTypesSingular::get_bool_field_for_reflect,
                    TestTypesSingular::mut_bool_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "string_field",
                    TestTypesSingular::get_string_field_for_reflect,
                    TestTypesSingular::mut_string_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bytes_field",
                    TestTypesSingular::get_bytes_field_for_reflect,
                    TestTypesSingular::mut_bytes_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<TestEnumDescriptor>>(
                    "enum_field",
                    TestTypesSingular::get_enum_field_for_reflect,
                    TestTypesSingular::mut_enum_field_for_reflect,
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

impl ::std::fmt::Debug for TestTypesSingular {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestTypesSingular {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
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
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TestTypesRepeated::new)
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

    fn get_double_field_for_reflect(&self) -> &::std::vec::Vec<f64> {
        &self.double_field
    }

    fn mut_double_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.double_field
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

    fn get_float_field_for_reflect(&self) -> &::std::vec::Vec<f32> {
        &self.float_field
    }

    fn mut_float_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.float_field
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

    fn get_int32_field_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.int32_field
    }

    fn mut_int32_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.int32_field
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

    fn get_int64_field_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.int64_field
    }

    fn mut_int64_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.int64_field
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

    fn get_uint32_field_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.uint32_field
    }

    fn mut_uint32_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.uint32_field
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

    fn get_uint64_field_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.uint64_field
    }

    fn mut_uint64_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.uint64_field
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

    fn get_sint32_field_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.sint32_field
    }

    fn mut_sint32_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.sint32_field
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

    fn get_sint64_field_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.sint64_field
    }

    fn mut_sint64_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.sint64_field
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

    fn get_fixed32_field_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.fixed32_field
    }

    fn mut_fixed32_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.fixed32_field
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

    fn get_fixed64_field_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.fixed64_field
    }

    fn mut_fixed64_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.fixed64_field
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

    fn get_sfixed32_field_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.sfixed32_field
    }

    fn mut_sfixed32_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.sfixed32_field
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

    fn get_sfixed64_field_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.sfixed64_field
    }

    fn mut_sfixed64_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.sfixed64_field
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

    fn get_bool_field_for_reflect(&self) -> &::std::vec::Vec<bool> {
        &self.bool_field
    }

    fn mut_bool_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.bool_field
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

    fn get_string_field_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.string_field
    }

    fn mut_string_field_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.string_field
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

    fn get_bytes_field_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.bytes_field
    }

    fn mut_bytes_field_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.bytes_field
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

    fn get_enum_field_for_reflect(&self) -> &::std::vec::Vec<TestEnumDescriptor> {
        &self.enum_field
    }

    fn mut_enum_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<TestEnumDescriptor> {
        &mut self.enum_field
    }
}

impl ::protobuf::Message for TestTypesRepeated {
    fn is_initialized(&self) -> bool {
        true
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::CodedMessage for TestTypesRepeated {
    fn merge_from<I: ::protobuf::InputSource>(&mut self, is: &mut ::protobuf::CodedInputStream<I>) -> ::protobuf::ProtobufResult<()> {
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
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "double_field",
                    TestTypesRepeated::get_double_field_for_reflect,
                    TestTypesRepeated::mut_double_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "float_field",
                    TestTypesRepeated::get_float_field_for_reflect,
                    TestTypesRepeated::mut_float_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "int32_field",
                    TestTypesRepeated::get_int32_field_for_reflect,
                    TestTypesRepeated::mut_int32_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "int64_field",
                    TestTypesRepeated::get_int64_field_for_reflect,
                    TestTypesRepeated::mut_int64_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "uint32_field",
                    TestTypesRepeated::get_uint32_field_for_reflect,
                    TestTypesRepeated::mut_uint32_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "uint64_field",
                    TestTypesRepeated::get_uint64_field_for_reflect,
                    TestTypesRepeated::mut_uint64_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "sint32_field",
                    TestTypesRepeated::get_sint32_field_for_reflect,
                    TestTypesRepeated::mut_sint32_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "sint64_field",
                    TestTypesRepeated::get_sint64_field_for_reflect,
                    TestTypesRepeated::mut_sint64_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "fixed32_field",
                    TestTypesRepeated::get_fixed32_field_for_reflect,
                    TestTypesRepeated::mut_fixed32_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "fixed64_field",
                    TestTypesRepeated::get_fixed64_field_for_reflect,
                    TestTypesRepeated::mut_fixed64_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeSfixed32>(
                    "sfixed32_field",
                    TestTypesRepeated::get_sfixed32_field_for_reflect,
                    TestTypesRepeated::mut_sfixed32_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeSfixed64>(
                    "sfixed64_field",
                    TestTypesRepeated::get_sfixed64_field_for_reflect,
                    TestTypesRepeated::mut_sfixed64_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "bool_field",
                    TestTypesRepeated::get_bool_field_for_reflect,
                    TestTypesRepeated::mut_bool_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "string_field",
                    TestTypesRepeated::get_string_field_for_reflect,
                    TestTypesRepeated::mut_string_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bytes_field",
                    TestTypesRepeated::get_bytes_field_for_reflect,
                    TestTypesRepeated::mut_bytes_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<TestEnumDescriptor>>(
                    "enum_field",
                    TestTypesRepeated::get_enum_field_for_reflect,
                    TestTypesRepeated::mut_enum_field_for_reflect,
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

impl ::std::fmt::Debug for TestTypesRepeated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestTypesRepeated {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
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
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TestTypesRepeatedPacked::new)
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

    fn get_double_field_for_reflect(&self) -> &::std::vec::Vec<f64> {
        &self.double_field
    }

    fn mut_double_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.double_field
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

    fn get_float_field_for_reflect(&self) -> &::std::vec::Vec<f32> {
        &self.float_field
    }

    fn mut_float_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.float_field
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

    fn get_int32_field_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.int32_field
    }

    fn mut_int32_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.int32_field
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

    fn get_int64_field_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.int64_field
    }

    fn mut_int64_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.int64_field
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

    fn get_uint32_field_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.uint32_field
    }

    fn mut_uint32_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.uint32_field
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

    fn get_uint64_field_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.uint64_field
    }

    fn mut_uint64_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.uint64_field
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

    fn get_sint32_field_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.sint32_field
    }

    fn mut_sint32_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.sint32_field
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

    fn get_sint64_field_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.sint64_field
    }

    fn mut_sint64_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.sint64_field
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

    fn get_fixed32_field_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.fixed32_field
    }

    fn mut_fixed32_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.fixed32_field
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

    fn get_fixed64_field_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.fixed64_field
    }

    fn mut_fixed64_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.fixed64_field
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

    fn get_sfixed32_field_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.sfixed32_field
    }

    fn mut_sfixed32_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.sfixed32_field
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

    fn get_sfixed64_field_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.sfixed64_field
    }

    fn mut_sfixed64_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.sfixed64_field
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

    fn get_bool_field_for_reflect(&self) -> &::std::vec::Vec<bool> {
        &self.bool_field
    }

    fn mut_bool_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.bool_field
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

    fn get_string_field_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.string_field
    }

    fn mut_string_field_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.string_field
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

    fn get_bytes_field_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.bytes_field
    }

    fn mut_bytes_field_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.bytes_field
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

    fn get_enum_field_for_reflect(&self) -> &::std::vec::Vec<TestEnumDescriptor> {
        &self.enum_field
    }

    fn mut_enum_field_for_reflect(&mut self) -> &mut ::std::vec::Vec<TestEnumDescriptor> {
        &mut self.enum_field
    }
}

impl ::protobuf::Message for TestTypesRepeatedPacked {
    fn is_initialized(&self) -> bool {
        true
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::CodedMessage for TestTypesRepeatedPacked {
    fn merge_from<I: ::protobuf::InputSource>(&mut self, is: &mut ::protobuf::CodedInputStream<I>) -> ::protobuf::ProtobufResult<()> {
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
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "double_field",
                    TestTypesRepeatedPacked::get_double_field_for_reflect,
                    TestTypesRepeatedPacked::mut_double_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "float_field",
                    TestTypesRepeatedPacked::get_float_field_for_reflect,
                    TestTypesRepeatedPacked::mut_float_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "int32_field",
                    TestTypesRepeatedPacked::get_int32_field_for_reflect,
                    TestTypesRepeatedPacked::mut_int32_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "int64_field",
                    TestTypesRepeatedPacked::get_int64_field_for_reflect,
                    TestTypesRepeatedPacked::mut_int64_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "uint32_field",
                    TestTypesRepeatedPacked::get_uint32_field_for_reflect,
                    TestTypesRepeatedPacked::mut_uint32_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "uint64_field",
                    TestTypesRepeatedPacked::get_uint64_field_for_reflect,
                    TestTypesRepeatedPacked::mut_uint64_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "sint32_field",
                    TestTypesRepeatedPacked::get_sint32_field_for_reflect,
                    TestTypesRepeatedPacked::mut_sint32_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "sint64_field",
                    TestTypesRepeatedPacked::get_sint64_field_for_reflect,
                    TestTypesRepeatedPacked::mut_sint64_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "fixed32_field",
                    TestTypesRepeatedPacked::get_fixed32_field_for_reflect,
                    TestTypesRepeatedPacked::mut_fixed32_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "fixed64_field",
                    TestTypesRepeatedPacked::get_fixed64_field_for_reflect,
                    TestTypesRepeatedPacked::mut_fixed64_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeSfixed32>(
                    "sfixed32_field",
                    TestTypesRepeatedPacked::get_sfixed32_field_for_reflect,
                    TestTypesRepeatedPacked::mut_sfixed32_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeSfixed64>(
                    "sfixed64_field",
                    TestTypesRepeatedPacked::get_sfixed64_field_for_reflect,
                    TestTypesRepeatedPacked::mut_sfixed64_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "bool_field",
                    TestTypesRepeatedPacked::get_bool_field_for_reflect,
                    TestTypesRepeatedPacked::mut_bool_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "string_field",
                    TestTypesRepeatedPacked::get_string_field_for_reflect,
                    TestTypesRepeatedPacked::mut_string_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bytes_field",
                    TestTypesRepeatedPacked::get_bytes_field_for_reflect,
                    TestTypesRepeatedPacked::mut_bytes_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<TestEnumDescriptor>>(
                    "enum_field",
                    TestTypesRepeatedPacked::get_enum_field_for_reflect,
                    TestTypesRepeatedPacked::mut_enum_field_for_reflect,
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

impl ::std::fmt::Debug for TestTypesRepeatedPacked {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestTypesRepeatedPacked {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestInvalidTag {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TestInvalidTag::new)
        }
    }
}

impl ::protobuf::Message for TestInvalidTag {
    fn is_initialized(&self) -> bool {
        true
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::CodedMessage for TestInvalidTag {
    fn merge_from<I: ::protobuf::InputSource>(&mut self, is: &mut ::protobuf::CodedInputStream<I>) -> ::protobuf::ProtobufResult<()> {
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

impl ::std::fmt::Debug for TestInvalidTag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestInvalidTag {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestTruncated {
    // message fields
    ints: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TestTruncated::new)
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

    fn get_ints_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.ints
    }

    fn mut_ints_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ints
    }
}

impl ::protobuf::Message for TestTruncated {
    fn is_initialized(&self) -> bool {
        true
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::CodedMessage for TestTruncated {
    fn merge_from<I: ::protobuf::InputSource>(&mut self, is: &mut ::protobuf::CodedInputStream<I>) -> ::protobuf::ProtobufResult<()> {
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
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "ints",
                    TestTruncated::get_ints_for_reflect,
                    TestTruncated::mut_ints_for_reflect,
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

impl ::std::fmt::Debug for TestTruncated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestTruncated {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestBugSint {
    // message fields
    s32: ::std::option::Option<i32>,
    s64: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(TestBugSint::new)
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

    fn get_s32_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.s32
    }

    fn mut_s32_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.s32
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

    fn get_s64_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.s64
    }

    fn mut_s64_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.s64
    }
}

impl ::protobuf::Message for TestBugSint {
    fn is_initialized(&self) -> bool {
        true
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.s32 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        };
        if let Some(v) = self.s64 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, v);
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::CodedMessage for TestBugSint {
    fn merge_from<I: ::protobuf::InputSource>(&mut self, is: &mut ::protobuf::CodedInputStream<I>) -> ::protobuf::ProtobufResult<()> {
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
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "s32",
                    TestBugSint::get_s32_for_reflect,
                    TestBugSint::mut_s32_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "s64",
                    TestBugSint::get_s64_for_reflect,
                    TestBugSint::mut_s64_for_reflect,
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

impl ::std::fmt::Debug for TestBugSint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestBugSint {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
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

impl ::protobuf::reflect::ProtobufValue for TestEnumDescriptor {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x62, 0x61, 0x73, 0x69, 0x63, 0x5f, 0x70, 0x62, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x62, 0x61, 0x73, 0x69, 0x63, 0x22, 0x15, 0x0a, 0x05,
    0x54, 0x65, 0x73, 0x74, 0x31, 0x12, 0x0c, 0x0a, 0x01, 0x61, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05,
    0x52, 0x01, 0x61, 0x22, 0x15, 0x0a, 0x05, 0x54, 0x65, 0x73, 0x74, 0x32, 0x12, 0x0c, 0x0a, 0x01,
    0x62, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x52, 0x01, 0x62, 0x22, 0x23, 0x0a, 0x05, 0x54, 0x65,
    0x73, 0x74, 0x33, 0x12, 0x1a, 0x0a, 0x01, 0x63, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0c,
    0x2e, 0x62, 0x61, 0x73, 0x69, 0x63, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x31, 0x52, 0x01, 0x63, 0x22,
    0x19, 0x0a, 0x05, 0x54, 0x65, 0x73, 0x74, 0x34, 0x12, 0x10, 0x0a, 0x01, 0x64, 0x18, 0x04, 0x20,
    0x03, 0x28, 0x05, 0x52, 0x01, 0x64, 0x42, 0x02, 0x10, 0x01, 0x22, 0x4c, 0x0a, 0x12, 0x54, 0x65,
    0x73, 0x74, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x64, 0x55, 0x6e, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x64,
    0x12, 0x1a, 0x0a, 0x08, 0x75, 0x6e, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x64, 0x18, 0x04, 0x20, 0x03,
    0x28, 0x05, 0x52, 0x08, 0x75, 0x6e, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x64, 0x12, 0x1a, 0x0a, 0x06,
    0x70, 0x61, 0x63, 0x6b, 0x65, 0x64, 0x18, 0x05, 0x20, 0x03, 0x28, 0x05, 0x52, 0x06, 0x70, 0x61,
    0x63, 0x6b, 0x65, 0x64, 0x42, 0x02, 0x10, 0x01, 0x22, 0x1d, 0x0a, 0x09, 0x54, 0x65, 0x73, 0x74,
    0x45, 0x6d, 0x70, 0x74, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x66, 0x6f, 0x6f, 0x18, 0x0a, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x03, 0x66, 0x6f, 0x6f, 0x22, 0x21, 0x0a, 0x11, 0x54, 0x65, 0x73, 0x74, 0x55,
    0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x12, 0x0c, 0x0a, 0x01,
    0x61, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05, 0x52, 0x01, 0x61, 0x22, 0x67, 0x0a, 0x11, 0x54, 0x65,
    0x73, 0x74, 0x53, 0x65, 0x6c, 0x66, 0x52, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x12,
    0x28, 0x0a, 0x02, 0x72, 0x31, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x62, 0x61,
    0x73, 0x69, 0x63, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x53, 0x65, 0x6c, 0x66, 0x52, 0x65, 0x66, 0x65,
    0x72, 0x65, 0x6e, 0x63, 0x65, 0x52, 0x02, 0x72, 0x31, 0x12, 0x28, 0x0a, 0x02, 0x72, 0x32, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x62, 0x61, 0x73, 0x69, 0x63, 0x2e, 0x54, 0x65,
    0x73, 0x74, 0x53, 0x65, 0x6c, 0x66, 0x52, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x52,
    0x02, 0x72, 0x32, 0x22, 0x28, 0x0a, 0x18, 0x54, 0x65, 0x73, 0x74, 0x44, 0x65, 0x66, 0x61, 0x75,
    0x6c, 0x74, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12,
    0x0c, 0x0a, 0x01, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x01, 0x73, 0x22, 0x4c, 0x0a,
    0x13, 0x54, 0x65, 0x73, 0x74, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x49, 0x6e, 0x73, 0x74,
    0x61, 0x6e, 0x63, 0x65, 0x12, 0x35, 0x0a, 0x05, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x62, 0x61, 0x73, 0x69, 0x63, 0x2e, 0x54, 0x65, 0x73, 0x74,
    0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x46,
    0x69, 0x65, 0x6c, 0x64, 0x52, 0x05, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x22, 0x26, 0x0a, 0x0e, 0x54,
    0x65, 0x73, 0x74, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x12, 0x14, 0x0a,
    0x05, 0x73, 0x74, 0x75, 0x66, 0x66, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x05, 0x52, 0x05, 0x73, 0x74,
    0x75, 0x66, 0x66, 0x22, 0xda, 0x04, 0x0a, 0x11, 0x54, 0x65, 0x73, 0x74, 0x54, 0x79, 0x70, 0x65,
    0x73, 0x53, 0x69, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x12, 0x21, 0x0a, 0x0c, 0x64, 0x6f, 0x75,
    0x62, 0x6c, 0x65, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x52,
    0x0b, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x1f, 0x0a, 0x0b,
    0x66, 0x6c, 0x6f, 0x61, 0x74, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x02, 0x52, 0x0a, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x1f, 0x0a,
    0x0b, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x0a, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x1f,
    0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x03, 0x52, 0x0a, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12,
    0x21, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x46, 0x69, 0x65,
    0x6c, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34,
    0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x11, 0x52, 0x0b, 0x73, 0x69, 0x6e,
    0x74, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74,
    0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x12, 0x52, 0x0b,
    0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x66,
    0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x09, 0x20, 0x01,
    0x28, 0x07, 0x52, 0x0c, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64,
    0x12, 0x23, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0c, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34,
    0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x25, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33,
    0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0f, 0x52, 0x0d, 0x73,
    0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x25, 0x0a, 0x0e,
    0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0c,
    0x20, 0x01, 0x28, 0x10, 0x52, 0x0d, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x46, 0x69,
    0x65, 0x6c, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x62, 0x6f, 0x6f, 0x6c, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x08, 0x52, 0x09, 0x62, 0x6f, 0x6f, 0x6c, 0x46, 0x69, 0x65,
    0x6c, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67,
    0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x62, 0x79, 0x74, 0x65, 0x73, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0a, 0x62, 0x79, 0x74, 0x65,
    0x73, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x38, 0x0a, 0x0a, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x62, 0x61, 0x73,
    0x69, 0x63, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x45, 0x6e, 0x75, 0x6d, 0x44, 0x65, 0x73, 0x63, 0x72,
    0x69, 0x70, 0x74, 0x6f, 0x72, 0x52, 0x09, 0x65, 0x6e, 0x75, 0x6d, 0x46, 0x69, 0x65, 0x6c, 0x64,
    0x22, 0x92, 0x05, 0x0a, 0x11, 0x54, 0x65, 0x73, 0x74, 0x54, 0x79, 0x70, 0x65, 0x73, 0x52, 0x65,
    0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x12, 0x25, 0x0a, 0x0c, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65,
    0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x01, 0x52, 0x0b, 0x64, 0x6f,
    0x75, 0x62, 0x6c, 0x65, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x02, 0x10, 0x00, 0x12, 0x23, 0x0a,
    0x0b, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x02, 0x20, 0x03,
    0x28, 0x02, 0x52, 0x0a, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x02,
    0x10, 0x00, 0x12, 0x23, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x03, 0x20, 0x03, 0x28, 0x05, 0x52, 0x0a, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x46, 0x69,
    0x65, 0x6c, 0x64, 0x42, 0x02, 0x10, 0x00, 0x12, 0x23, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x36, 0x34,
    0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x04, 0x20, 0x03, 0x28, 0x03, 0x52, 0x0a, 0x69, 0x6e,
    0x74, 0x36, 0x34, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x02, 0x10, 0x00, 0x12, 0x25, 0x0a, 0x0c,
    0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x05, 0x20, 0x03,
    0x28, 0x0d, 0x52, 0x0b, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42,
    0x02, 0x10, 0x00, 0x12, 0x25, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x18, 0x06, 0x20, 0x03, 0x28, 0x04, 0x52, 0x0b, 0x75, 0x69, 0x6e, 0x74, 0x36,
    0x34, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x02, 0x10, 0x00, 0x12, 0x25, 0x0a, 0x0c, 0x73, 0x69,
    0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x07, 0x20, 0x03, 0x28, 0x11,
    0x52, 0x0b, 0x73, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x02, 0x10,
    0x00, 0x12, 0x25, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x08, 0x20, 0x03, 0x28, 0x12, 0x52, 0x0b, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x46,
    0x69, 0x65, 0x6c, 0x64, 0x42, 0x02, 0x10, 0x00, 0x12, 0x27, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65,
    0x64, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x09, 0x20, 0x03, 0x28, 0x07, 0x52,
    0x0c, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x02, 0x10,
    0x00, 0x12, 0x27, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x18, 0x0a, 0x20, 0x03, 0x28, 0x06, 0x52, 0x0c, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36,
    0x34, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x02, 0x10, 0x00, 0x12, 0x29, 0x0a, 0x0e, 0x73, 0x66,
    0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0b, 0x20, 0x03,
    0x28, 0x0f, 0x52, 0x0d, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c,
    0x64, 0x42, 0x02, 0x10, 0x00, 0x12, 0x29, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36,
    0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0c, 0x20, 0x03, 0x28, 0x10, 0x52, 0x0d, 0x73,
    0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x02, 0x10, 0x00,
    0x12, 0x21, 0x0a, 0x0a, 0x62, 0x6f, 0x6f, 0x6c, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0d,
    0x20, 0x03, 0x28, 0x08, 0x52, 0x09, 0x62, 0x6f, 0x6f, 0x6c, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42,
    0x02, 0x10, 0x00, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x18, 0x0e, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0b, 0x73, 0x74, 0x72, 0x69, 0x6e,
    0x67, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x62, 0x79, 0x74, 0x65, 0x73, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0f, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x0a, 0x62, 0x79, 0x74,
    0x65, 0x73, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x3c, 0x0a, 0x0a, 0x65, 0x6e, 0x75, 0x6d, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x10, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x62, 0x61,
    0x73, 0x69, 0x63, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x45, 0x6e, 0x75, 0x6d, 0x44, 0x65, 0x73, 0x63,
    0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x52, 0x09, 0x65, 0x6e, 0x75, 0x6d, 0x46, 0x69, 0x65, 0x6c,
    0x64, 0x42, 0x02, 0x10, 0x00, 0x22, 0x98, 0x05, 0x0a, 0x17, 0x54, 0x65, 0x73, 0x74, 0x54, 0x79,
    0x70, 0x65, 0x73, 0x52, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x50, 0x61, 0x63, 0x6b, 0x65,
    0x64, 0x12, 0x25, 0x0a, 0x0c, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x01, 0x52, 0x0b, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x46,
    0x69, 0x65, 0x6c, 0x64, 0x42, 0x02, 0x10, 0x01, 0x12, 0x23, 0x0a, 0x0b, 0x66, 0x6c, 0x6f, 0x61,
    0x74, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x02, 0x20, 0x03, 0x28, 0x02, 0x52, 0x0a, 0x66,
    0x6c, 0x6f, 0x61, 0x74, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x02, 0x10, 0x01, 0x12, 0x23, 0x0a,
    0x0b, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x03, 0x20, 0x03,
    0x28, 0x05, 0x52, 0x0a, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x02,
    0x10, 0x01, 0x12, 0x23, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x18, 0x04, 0x20, 0x03, 0x28, 0x03, 0x52, 0x0a, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x46, 0x69,
    0x65, 0x6c, 0x64, 0x42, 0x02, 0x10, 0x01, 0x12, 0x25, 0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x33,
    0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x0b, 0x75,
    0x69, 0x6e, 0x74, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x02, 0x10, 0x01, 0x12, 0x25,
    0x0a, 0x0c, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x06,
    0x20, 0x03, 0x28, 0x04, 0x52, 0x0b, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x46, 0x69, 0x65, 0x6c,
    0x64, 0x42, 0x02, 0x10, 0x01, 0x12, 0x25, 0x0a, 0x0c, 0x73, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x07, 0x20, 0x03, 0x28, 0x11, 0x52, 0x0b, 0x73, 0x69, 0x6e,
    0x74, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x02, 0x10, 0x01, 0x12, 0x25, 0x0a, 0x0c,
    0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x08, 0x20, 0x03,
    0x28, 0x12, 0x52, 0x0b, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42,
    0x02, 0x10, 0x01, 0x12, 0x27, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x18, 0x09, 0x20, 0x03, 0x28, 0x07, 0x52, 0x0c, 0x66, 0x69, 0x78, 0x65,
    0x64, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x02, 0x10, 0x01, 0x12, 0x27, 0x0a, 0x0d,
    0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0a, 0x20,
    0x03, 0x28, 0x06, 0x52, 0x0c, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x46, 0x69, 0x65, 0x6c,
    0x64, 0x42, 0x02, 0x10, 0x01, 0x12, 0x29, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33,
    0x32, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0b, 0x20, 0x03, 0x28, 0x0f, 0x52, 0x0d, 0x73,
    0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x02, 0x10, 0x01,
    0x12, 0x29, 0x0a, 0x0e, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x18, 0x0c, 0x20, 0x03, 0x28, 0x10, 0x52, 0x0d, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64,
    0x36, 0x34, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x02, 0x10, 0x01, 0x12, 0x21, 0x0a, 0x0a, 0x62,
    0x6f, 0x6f, 0x6c, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0d, 0x20, 0x03, 0x28, 0x08, 0x52,
    0x09, 0x62, 0x6f, 0x6f, 0x6c, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x02, 0x10, 0x01, 0x12, 0x21,
    0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x0e,
    0x20, 0x03, 0x28, 0x09, 0x52, 0x0b, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x46, 0x69, 0x65, 0x6c,
    0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x62, 0x79, 0x74, 0x65, 0x73, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x18, 0x0f, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x0a, 0x62, 0x79, 0x74, 0x65, 0x73, 0x46, 0x69, 0x65,
    0x6c, 0x64, 0x12, 0x3c, 0x0a, 0x0a, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x18, 0x10, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x62, 0x61, 0x73, 0x69, 0x63, 0x2e, 0x54,
    0x65, 0x73, 0x74, 0x45, 0x6e, 0x75, 0x6d, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f,
    0x72, 0x52, 0x09, 0x65, 0x6e, 0x75, 0x6d, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x02, 0x10, 0x01,
    0x22, 0x10, 0x0a, 0x0e, 0x54, 0x65, 0x73, 0x74, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x54,
    0x61, 0x67, 0x22, 0x27, 0x0a, 0x0d, 0x54, 0x65, 0x73, 0x74, 0x54, 0x72, 0x75, 0x6e, 0x63, 0x61,
    0x74, 0x65, 0x64, 0x12, 0x16, 0x0a, 0x04, 0x69, 0x6e, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x07, 0x52, 0x04, 0x69, 0x6e, 0x74, 0x73, 0x42, 0x02, 0x10, 0x01, 0x22, 0x31, 0x0a, 0x0b, 0x54,
    0x65, 0x73, 0x74, 0x42, 0x75, 0x67, 0x53, 0x69, 0x6e, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x73, 0x33,
    0x32, 0x18, 0x01, 0x20, 0x01, 0x28, 0x11, 0x52, 0x03, 0x73, 0x33, 0x32, 0x12, 0x10, 0x0a, 0x03,
    0x73, 0x36, 0x34, 0x18, 0x02, 0x20, 0x01, 0x28, 0x12, 0x52, 0x03, 0x73, 0x36, 0x34, 0x2a, 0x32,
    0x0a, 0x12, 0x54, 0x65, 0x73, 0x74, 0x45, 0x6e, 0x75, 0x6d, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x6f, 0x72, 0x12, 0x07, 0x0a, 0x03, 0x52, 0x45, 0x44, 0x10, 0x01, 0x12, 0x08, 0x0a,
    0x04, 0x42, 0x4c, 0x55, 0x45, 0x10, 0x02, 0x12, 0x09, 0x0a, 0x05, 0x47, 0x52, 0x45, 0x45, 0x4e,
    0x10, 0x03, 0x4a, 0xf7, 0x40, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x7d, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08,
    0x0d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x05, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x05, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x05,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x13, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x17, 0x18, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x08, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x08, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x09, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x14, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x02, 0x12, 0x04, 0x0c, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x0c, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x04, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0d, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x13, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x10, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x10, 0x08, 0x0d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x11, 0x04, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x11, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x11, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x11, 0x13, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x11, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x08, 0x12, 0x03,
    0x11, 0x19, 0x26, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12,
    0x03, 0x11, 0x1a, 0x25, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x11, 0x1a, 0x20, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x11, 0x1a, 0x20, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x03, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x1a, 0x20, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x11, 0x21, 0x25, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x15, 0x00, 0x18, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x04, 0x01, 0x12, 0x03, 0x15, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12,
    0x03, 0x16, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x16,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x16, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x13, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x17, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x17, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x17, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x17, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x17, 0x1c,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x08, 0x12, 0x03, 0x17, 0x1e, 0x2b, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x04, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x17, 0x1f, 0x2a,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x04, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x17,
    0x1f, 0x25, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x04, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x17, 0x1f, 0x25, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x04, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x17, 0x1f, 0x25, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x04, 0x02,
    0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x17, 0x26, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x05, 0x12, 0x04, 0x1a, 0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03,
    0x1a, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x04, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1b, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1b, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x19, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04,
    0x1e, 0x00, 0x20, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x19,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x1f, 0x04, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x1f, 0x13, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x1f, 0x17, 0x18, 0x0a, 0x24, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x23, 0x00, 0x26,
    0x01, 0x1a, 0x18, 0x20, 0x6a, 0x75, 0x73, 0x74, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x69,
    0x74, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x69, 0x6c, 0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x07, 0x01, 0x12, 0x03, 0x23, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12,
    0x03, 0x24, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x24,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x24, 0x0d, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x1f, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x25, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x25, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x25, 0x0d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x25, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x25, 0x24,
    0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x28, 0x00, 0x2a, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x28, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02,
    0x00, 0x12, 0x03, 0x29, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x29, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x29,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x14, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x18, 0x19, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x2c, 0x00, 0x2e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09,
    0x01, 0x12, 0x03, 0x2c, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03,
    0x2d, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2d, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2d, 0x0d, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x26, 0x2b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x2e, 0x2f, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x0a, 0x12, 0x04, 0x30, 0x00, 0x32, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03,
    0x30, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x31, 0x04, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x31, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x31, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x31, 0x1b, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04,
    0x34, 0x00, 0x38, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x34, 0x05, 0x17,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x35, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x35, 0x04, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x35, 0x0a, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x36, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x36, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x36,
    0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x37, 0x04, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x37, 0x04, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x37, 0x0c, 0x0d, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x0b, 0x12, 0x04, 0x3a, 0x00, 0x4b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03,
    0x3a, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x3b, 0x04, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3b, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3b, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x3b, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01,
    0x12, 0x03, 0x3c, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x3c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3c, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3c, 0x13, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3c, 0x21, 0x22, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x03, 0x3d, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x3d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x3d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x3d, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3d,
    0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x03, 0x12, 0x03, 0x3e, 0x04, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x04, 0x12, 0x03, 0x3e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x03, 0x05, 0x12, 0x03, 0x3e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3e, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x3e, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x04, 0x12,
    0x03, 0x3f, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x04, 0x12, 0x03, 0x3f,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x05, 0x12, 0x03, 0x3f, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x01, 0x12, 0x03, 0x3f, 0x14, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x03, 0x12, 0x03, 0x3f, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x05, 0x12, 0x03, 0x40, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x05, 0x04, 0x12, 0x03, 0x40, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x40, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x40, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x03, 0x12, 0x03, 0x40, 0x23,
    0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x06, 0x12, 0x03, 0x41, 0x04, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06, 0x04, 0x12, 0x03, 0x41, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x06, 0x05, 0x12, 0x03, 0x41, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x06, 0x01, 0x12, 0x03, 0x41, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06,
    0x03, 0x12, 0x03, 0x41, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x07, 0x12, 0x03,
    0x42, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x07, 0x04, 0x12, 0x03, 0x42, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x07, 0x05, 0x12, 0x03, 0x42, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x07, 0x01, 0x12, 0x03, 0x42, 0x14, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x07, 0x03, 0x12, 0x03, 0x42, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x08, 0x12, 0x03, 0x43, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08,
    0x04, 0x12, 0x03, 0x43, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x05, 0x12,
    0x03, 0x43, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x01, 0x12, 0x03, 0x43,
    0x15, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x03, 0x12, 0x03, 0x43, 0x25, 0x26,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x09, 0x12, 0x03, 0x44, 0x04, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x09, 0x04, 0x12, 0x03, 0x44, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x09, 0x05, 0x12, 0x03, 0x44, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x09, 0x01, 0x12, 0x03, 0x44, 0x15, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x09, 0x03,
    0x12, 0x03, 0x44, 0x25, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x0a, 0x12, 0x03, 0x45,
    0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x45, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x45, 0x0d, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x45, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x45, 0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x0b, 0x12, 0x03, 0x46, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0b, 0x04,
    0x12, 0x03, 0x46, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0b, 0x05, 0x12, 0x03,
    0x46, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x46, 0x16,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x46, 0x27, 0x29, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x0c, 0x12, 0x03, 0x47, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x47, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x0c, 0x05, 0x12, 0x03, 0x47, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0c,
    0x01, 0x12, 0x03, 0x47, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0c, 0x03, 0x12,
    0x03, 0x47, 0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x0d, 0x12, 0x03, 0x48, 0x04,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x48, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x48, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x48, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x48, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02,
    0x0e, 0x12, 0x03, 0x49, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0e, 0x04, 0x12,
    0x03, 0x49, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x49,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x49, 0x13, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x49, 0x21, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0b, 0x02, 0x0f, 0x12, 0x03, 0x4a, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x0f, 0x04, 0x12, 0x03, 0x4a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x0f, 0x06, 0x12, 0x03, 0x4a, 0x0d, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0f, 0x01,
    0x12, 0x03, 0x4a, 0x20, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0f, 0x03, 0x12, 0x03,
    0x4a, 0x2d, 0x2f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x4d, 0x00, 0x5e, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x4d, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0c, 0x02, 0x00, 0x12, 0x03, 0x4e, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x4e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x4e, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4e,
    0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4e, 0x23, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x08, 0x12, 0x03, 0x4e, 0x25, 0x33, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x0c, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x4e, 0x26, 0x32, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x4e, 0x26,
    0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0c, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x4e, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0c, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x4e, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x4e, 0x2d, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c,
    0x02, 0x01, 0x12, 0x03, 0x4f, 0x04, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x4f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x4f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4f, 0x13,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4f, 0x21, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x08, 0x12, 0x03, 0x4f, 0x23, 0x31, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x0c, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x4f, 0x24, 0x30, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x0c, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x4f, 0x24, 0x2a,
    0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0c, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x4f, 0x24, 0x2a, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0c, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x4f, 0x24, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x01, 0x08,
    0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x4f, 0x2b, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02,
    0x02, 0x12, 0x03, 0x50, 0x04, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x50, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x05, 0x12, 0x03, 0x50,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x03, 0x50, 0x13, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x03, 0x50, 0x21, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x08, 0x12, 0x03, 0x50, 0x23, 0x31, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x0c, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x50, 0x24, 0x30, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x0c, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x50, 0x24, 0x2a, 0x0a,
    0x11, 0x0a, 0x0a, 0x04, 0x0c, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x50,
    0x24, 0x2a, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0c, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x50, 0x24, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x02, 0x08, 0xe7,
    0x07, 0x00, 0x03, 0x12, 0x03, 0x50, 0x2b, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x03,
    0x12, 0x03, 0x51, 0x04, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x51, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x05, 0x12, 0x03, 0x51, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x01, 0x12, 0x03, 0x51, 0x13, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x03, 0x12, 0x03, 0x51, 0x21, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x03, 0x08, 0x12, 0x03, 0x51, 0x23, 0x31, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x0c, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x51, 0x24, 0x30, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x0c, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x51, 0x24, 0x2a, 0x0a, 0x11,
    0x0a, 0x0a, 0x04, 0x0c, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x51, 0x24,
    0x2a, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0c, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x51, 0x24, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x03, 0x08, 0xe7, 0x07,
    0x00, 0x03, 0x12, 0x03, 0x51, 0x2b, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x04, 0x12,
    0x03, 0x52, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x04, 0x12, 0x03, 0x52,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x05, 0x12, 0x03, 0x52, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x01, 0x12, 0x03, 0x52, 0x14, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x03, 0x12, 0x03, 0x52, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x04, 0x08, 0x12, 0x03, 0x52, 0x25, 0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0c,
    0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x52, 0x26, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x0c, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x52, 0x26, 0x2c, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x0c, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x52, 0x26, 0x2c,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0c, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x52, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x52, 0x2d, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x05, 0x12, 0x03,
    0x53, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x04, 0x12, 0x03, 0x53, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x05, 0x12, 0x03, 0x53, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x01, 0x12, 0x03, 0x53, 0x14, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x05, 0x03, 0x12, 0x03, 0x53, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x05, 0x08, 0x12, 0x03, 0x53, 0x25, 0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0c, 0x02,
    0x05, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x53, 0x26, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c,
    0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x53, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a,
    0x04, 0x0c, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x53, 0x26, 0x2c, 0x0a,
    0x12, 0x0a, 0x0b, 0x04, 0x0c, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x53, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03,
    0x12, 0x03, 0x53, 0x2d, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x06, 0x12, 0x03, 0x54,
    0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x04, 0x12, 0x03, 0x54, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x05, 0x12, 0x03, 0x54, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x01, 0x12, 0x03, 0x54, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x06, 0x03, 0x12, 0x03, 0x54, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x06, 0x08, 0x12, 0x03, 0x54, 0x25, 0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0c, 0x02, 0x06,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x54, 0x26, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02,
    0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x54, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04,
    0x0c, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x54, 0x26, 0x2c, 0x0a, 0x12,
    0x0a, 0x0b, 0x04, 0x0c, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x54,
    0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12,
    0x03, 0x54, 0x2d, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x07, 0x12, 0x03, 0x55, 0x04,
    0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x07, 0x04, 0x12, 0x03, 0x55, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x07, 0x05, 0x12, 0x03, 0x55, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x07, 0x01, 0x12, 0x03, 0x55, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x07, 0x03, 0x12, 0x03, 0x55, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x07, 0x08, 0x12, 0x03, 0x55, 0x25, 0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0c, 0x02, 0x07, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x55, 0x26, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x07,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x55, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0c,
    0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x55, 0x26, 0x2c, 0x0a, 0x12, 0x0a,
    0x0b, 0x04, 0x0c, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x55, 0x26,
    0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03,
    0x55, 0x2d, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x08, 0x12, 0x03, 0x56, 0x04, 0x36,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x08, 0x04, 0x12, 0x03, 0x56, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x08, 0x05, 0x12, 0x03, 0x56, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x08, 0x01, 0x12, 0x03, 0x56, 0x15, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x08, 0x03, 0x12, 0x03, 0x56, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x08,
    0x08, 0x12, 0x03, 0x56, 0x27, 0x35, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0c, 0x02, 0x08, 0x08, 0xe7,
    0x07, 0x00, 0x12, 0x03, 0x56, 0x28, 0x34, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x08, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x56, 0x28, 0x2e, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0c, 0x02,
    0x08, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x56, 0x28, 0x2e, 0x0a, 0x12, 0x0a, 0x0b,
    0x04, 0x0c, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x56, 0x28, 0x2e,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x56,
    0x2f, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x09, 0x12, 0x03, 0x57, 0x04, 0x37, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x09, 0x04, 0x12, 0x03, 0x57, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x09, 0x05, 0x12, 0x03, 0x57, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x09, 0x01, 0x12, 0x03, 0x57, 0x15, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x09, 0x03, 0x12, 0x03, 0x57, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x09, 0x08,
    0x12, 0x03, 0x57, 0x28, 0x36, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0c, 0x02, 0x09, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x57, 0x29, 0x35, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x09, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x57, 0x29, 0x2f, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0c, 0x02, 0x09,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x57, 0x29, 0x2f, 0x0a, 0x12, 0x0a, 0x0b, 0x04,
    0x0c, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x57, 0x29, 0x2f, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x57, 0x30,
    0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x0a, 0x12, 0x03, 0x58, 0x04, 0x39, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x58, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x58, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x0a, 0x01, 0x12, 0x03, 0x58, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0a,
    0x03, 0x12, 0x03, 0x58, 0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0a, 0x08, 0x12,
    0x03, 0x58, 0x2a, 0x38, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0c, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x58, 0x2b, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x0a, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x58, 0x2b, 0x31, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0c, 0x02, 0x0a, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x58, 0x2b, 0x31, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0c,
    0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x58, 0x2b, 0x31, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x0c, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x58, 0x32, 0x37,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x0b, 0x12, 0x03, 0x59, 0x04, 0x39, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x59, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x59, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x0b, 0x01, 0x12, 0x03, 0x59, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0b, 0x03,
    0x12, 0x03, 0x59, 0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0b, 0x08, 0x12, 0x03,
    0x59, 0x2a, 0x38, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0c, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x12,
    0x03, 0x59, 0x2b, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x59, 0x2b, 0x31, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0c, 0x02, 0x0b, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x59, 0x2b, 0x31, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0c, 0x02,
    0x0b, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x59, 0x2b, 0x31, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x0c, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x59, 0x32, 0x37, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x0c, 0x12, 0x03, 0x5a, 0x04, 0x31, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x5a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x0c, 0x05, 0x12, 0x03, 0x5a, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0c,
    0x01, 0x12, 0x03, 0x5a, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0c, 0x03, 0x12,
    0x03, 0x5a, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0c, 0x08, 0x12, 0x03, 0x5a,
    0x22, 0x30, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0c, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03,
    0x5a, 0x23, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x12, 0x03, 0x5a, 0x23, 0x29, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0c, 0x02, 0x0c, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x5a, 0x23, 0x29, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0c, 0x02, 0x0c,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5a, 0x23, 0x29, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x0c, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x5a, 0x2a, 0x2f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0c, 0x02, 0x0d, 0x12, 0x03, 0x5b, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x5b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x0d, 0x05, 0x12, 0x03, 0x5b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0d, 0x01,
    0x12, 0x03, 0x5b, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0d, 0x03, 0x12, 0x03,
    0x5b, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x0e, 0x12, 0x03, 0x5c, 0x04, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x5c, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x5c, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x5c, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x0e, 0x03, 0x12, 0x03, 0x5c, 0x21, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x0f,
    0x12, 0x03, 0x5d, 0x04, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0f, 0x04, 0x12, 0x03,
    0x5d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0f, 0x06, 0x12, 0x03, 0x5d, 0x0d,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x5d, 0x20, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x5d, 0x2d, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x0f, 0x08, 0x12, 0x03, 0x5d, 0x30, 0x3e, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x0c, 0x02, 0x0f, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x5d, 0x31, 0x3d, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x0c, 0x02, 0x0f, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x5d, 0x31, 0x37, 0x0a, 0x11,
    0x0a, 0x0a, 0x04, 0x0c, 0x02, 0x0f, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x5d, 0x31,
    0x37, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0c, 0x02, 0x0f, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x5d, 0x31, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0c, 0x02, 0x0f, 0x08, 0xe7, 0x07,
    0x00, 0x03, 0x12, 0x03, 0x5d, 0x38, 0x3d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x04, 0x60,
    0x00, 0x71, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x03, 0x60, 0x08, 0x1f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x61, 0x04, 0x33, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x61, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x61, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x61, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x61, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x08, 0x12, 0x03, 0x61,
    0x25, 0x32, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03,
    0x61, 0x26, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x12, 0x03, 0x61, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x00, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x61, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x61, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x0d, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x61, 0x2d, 0x31, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x03, 0x62, 0x04, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x01, 0x04, 0x12, 0x03, 0x62, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x62, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x62, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x62, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x08, 0x12, 0x03, 0x62, 0x23,
    0x30, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x62,
    0x24, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x03, 0x62, 0x24, 0x2a, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x62, 0x24, 0x2a, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x01, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x62, 0x24, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x0d, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x62, 0x2b, 0x2f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0d, 0x02, 0x02, 0x12, 0x03, 0x63, 0x04, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x63, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x63, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x63, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x03, 0x12, 0x03, 0x63,
    0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x08, 0x12, 0x03, 0x63, 0x23, 0x30,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x63, 0x24,
    0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x63, 0x24, 0x2a, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x63, 0x24, 0x2a, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x02, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x63, 0x24, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d,
    0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x63, 0x2b, 0x2f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0d, 0x02, 0x03, 0x12, 0x03, 0x64, 0x04, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x64, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x64, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x64, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x03, 0x12, 0x03, 0x64, 0x21,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x08, 0x12, 0x03, 0x64, 0x23, 0x30, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x64, 0x24, 0x2f,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x64,
    0x24, 0x2a, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x64, 0x24, 0x2a, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x03, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x64, 0x24, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02,
    0x03, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x64, 0x2b, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0d, 0x02, 0x04, 0x12, 0x03, 0x65, 0x04, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04,
    0x04, 0x12, 0x03, 0x65, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x05, 0x12,
    0x03, 0x65, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x01, 0x12, 0x03, 0x65,
    0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x03, 0x12, 0x03, 0x65, 0x23, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x08, 0x12, 0x03, 0x65, 0x25, 0x32, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x0d, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x65, 0x26, 0x31, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x65, 0x26,
    0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x65, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x65, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x04,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x65, 0x2d, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d,
    0x02, 0x05, 0x12, 0x03, 0x66, 0x04, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x04,
    0x12, 0x03, 0x66, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x05, 0x12, 0x03,
    0x66, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x01, 0x12, 0x03, 0x66, 0x14,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x03, 0x12, 0x03, 0x66, 0x23, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x08, 0x12, 0x03, 0x66, 0x25, 0x32, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x0d, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x66, 0x26, 0x31, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x0d, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x66, 0x26, 0x2c,
    0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x66, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x66, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x05, 0x08,
    0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x66, 0x2d, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02,
    0x06, 0x12, 0x03, 0x67, 0x04, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x04, 0x12,
    0x03, 0x67, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x05, 0x12, 0x03, 0x67,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x01, 0x12, 0x03, 0x67, 0x14, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x03, 0x12, 0x03, 0x67, 0x23, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x08, 0x12, 0x03, 0x67, 0x25, 0x32, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x0d, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x67, 0x26, 0x31, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x0d, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x67, 0x26, 0x2c, 0x0a,
    0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x67,
    0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x67, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x06, 0x08, 0xe7,
    0x07, 0x00, 0x03, 0x12, 0x03, 0x67, 0x2d, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x07,
    0x12, 0x03, 0x68, 0x04, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x04, 0x12, 0x03,
    0x68, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x05, 0x12, 0x03, 0x68, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x01, 0x12, 0x03, 0x68, 0x14, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x03, 0x12, 0x03, 0x68, 0x23, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x07, 0x08, 0x12, 0x03, 0x68, 0x25, 0x32, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x0d, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x68, 0x26, 0x31, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x0d, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x68, 0x26, 0x2c, 0x0a, 0x11,
    0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x68, 0x26,
    0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x68, 0x26, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x07, 0x08, 0xe7, 0x07,
    0x00, 0x03, 0x12, 0x03, 0x68, 0x2d, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x08, 0x12,
    0x03, 0x69, 0x04, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x04, 0x12, 0x03, 0x69,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x05, 0x12, 0x03, 0x69, 0x0d, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x01, 0x12, 0x03, 0x69, 0x15, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08, 0x03, 0x12, 0x03, 0x69, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x08, 0x08, 0x12, 0x03, 0x69, 0x27, 0x34, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d,
    0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x69, 0x28, 0x33, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x0d, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x69, 0x28, 0x2e, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x0d, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x69, 0x28, 0x2e,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x69, 0x28, 0x2e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x69, 0x2f, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x09, 0x12, 0x03,
    0x6a, 0x04, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x04, 0x12, 0x03, 0x6a, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x05, 0x12, 0x03, 0x6a, 0x0d, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x01, 0x12, 0x03, 0x6a, 0x15, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x09, 0x03, 0x12, 0x03, 0x6a, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x09, 0x08, 0x12, 0x03, 0x6a, 0x28, 0x35, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02,
    0x09, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x6a, 0x29, 0x34, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d,
    0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x6a, 0x29, 0x2f, 0x0a, 0x11, 0x0a, 0x0a,
    0x04, 0x0d, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x6a, 0x29, 0x2f, 0x0a,
    0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x6a, 0x29, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x03,
    0x12, 0x03, 0x6a, 0x30, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0a, 0x12, 0x03, 0x6b,
    0x04, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x6b, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x6b, 0x0d, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x6b, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x6b, 0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x0a, 0x08, 0x12, 0x03, 0x6b, 0x2a, 0x37, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x0a,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x6b, 0x2b, 0x36, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02,
    0x0a, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x6b, 0x2b, 0x31, 0x0a, 0x11, 0x0a, 0x0a, 0x04,
    0x0d, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x6b, 0x2b, 0x31, 0x0a, 0x12,
    0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6b,
    0x2b, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12,
    0x03, 0x6b, 0x32, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0b, 0x12, 0x03, 0x6c, 0x04,
    0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x6c, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x6c, 0x0d, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x6c, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x6c, 0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x0b, 0x08, 0x12, 0x03, 0x6c, 0x2a, 0x37, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x0b, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x6c, 0x2b, 0x36, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x0b,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x6c, 0x2b, 0x31, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d,
    0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x6c, 0x2b, 0x31, 0x0a, 0x12, 0x0a,
    0x0b, 0x04, 0x0d, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6c, 0x2b,
    0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x0b, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03,
    0x6c, 0x32, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0c, 0x12, 0x03, 0x6d, 0x04, 0x30,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x6d, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x6d, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x6d, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x0c, 0x03, 0x12, 0x03, 0x6d, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0c,
    0x08, 0x12, 0x03, 0x6d, 0x22, 0x2f, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x0c, 0x08, 0xe7,
    0x07, 0x00, 0x12, 0x03, 0x6d, 0x23, 0x2e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x0c, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x6d, 0x23, 0x29, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02,
    0x0c, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x6d, 0x23, 0x29, 0x0a, 0x12, 0x0a, 0x0b,
    0x04, 0x0d, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6d, 0x23, 0x29,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x0c, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x6d,
    0x2a, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0d, 0x12, 0x03, 0x6e, 0x04, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x6e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x6e, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x6e, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x0d, 0x03, 0x12, 0x03, 0x6e, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x0e, 0x12,
    0x03, 0x6f, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x6f,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x6f, 0x0d, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x6f, 0x13, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x6f, 0x21, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0d, 0x02, 0x0f, 0x12, 0x03, 0x70, 0x04, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x0f, 0x04, 0x12, 0x03, 0x70, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0f, 0x06,
    0x12, 0x03, 0x70, 0x0d, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0f, 0x01, 0x12, 0x03,
    0x70, 0x20, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x70, 0x2d,
    0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x0f, 0x08, 0x12, 0x03, 0x70, 0x30, 0x3d, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x0d, 0x02, 0x0f, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x70, 0x31, 0x3c,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02, 0x0f, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x70,
    0x31, 0x37, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0d, 0x02, 0x0f, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x70, 0x31, 0x37, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0d, 0x02, 0x0f, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x70, 0x31, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0d, 0x02,
    0x0f, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x70, 0x38, 0x3c, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x0e, 0x12, 0x04, 0x73, 0x00, 0x74, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x03,
    0x73, 0x08, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x04, 0x76, 0x00, 0x78, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x03, 0x76, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x00, 0x12, 0x03, 0x77, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x77, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x77, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x03, 0x77,
    0x15, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x03, 0x77, 0x1c, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x08, 0x12, 0x03, 0x77, 0x1e, 0x2b, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x0f, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x77, 0x1f, 0x2a, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x0f, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x77, 0x1f,
    0x25, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x0f, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x77, 0x1f, 0x25, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0f, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x77, 0x1f, 0x25, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0f, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x77, 0x26, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x10,
    0x12, 0x04, 0x7a, 0x00, 0x7d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x03, 0x7a,
    0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x03, 0x7b, 0x04, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7b, 0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x7b, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12,
    0x03, 0x7c, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7c,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x03, 0x7c, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7c, 0x14, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7c, 0x1a, 0x1b,
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

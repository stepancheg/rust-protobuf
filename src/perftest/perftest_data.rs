// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Test1 {
    // message fields
    value: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

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
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 value = 1;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i32) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value<'a>(&self) -> i32 {
        self.value.unwrap_or(0)
    }
}

impl ::protobuf::Message for Test1 {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.value.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value {
            try!(os.write_int32(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
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
                    "value",
                    Test1::has_value,
                    Test1::get_value,
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
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Test1 {
    fn eq(&self, other: &Test1) -> bool {
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Test1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestRepeatedBool {
    // message fields
    values: ::std::vec::Vec<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TestRepeatedBool {
    pub fn new() -> TestRepeatedBool {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestRepeatedBool {
        static mut instance: ::protobuf::lazy::Lazy<TestRepeatedBool> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestRepeatedBool,
        };
        unsafe {
            instance.get(|| {
                TestRepeatedBool {
                    values: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated bool values = 1;

    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::std::vec::Vec<bool>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<bool> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.values, ::std::vec::Vec::new())
    }

    pub fn get_values<'a>(&'a self) -> &'a [bool] {
        &self.values
    }
}

impl ::protobuf::Message for TestRepeatedBool {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.values));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += 2 * self.values.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.values.iter() {
            try!(os.write_bool(1, *v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestRepeatedBool>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestRepeatedBool {
    fn new() -> TestRepeatedBool {
        TestRepeatedBool::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestRepeatedBool>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_bool_accessor(
                    "values",
                    TestRepeatedBool::get_values,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestRepeatedBool>(
                    "TestRepeatedBool",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestRepeatedBool {
    fn clear(&mut self) {
        self.clear_values();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestRepeatedBool {
    fn eq(&self, other: &TestRepeatedBool) -> bool {
        self.values == other.values &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestRepeatedBool {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestRepeatedPackedInt32 {
    // message fields
    values: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TestRepeatedPackedInt32 {
    pub fn new() -> TestRepeatedPackedInt32 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestRepeatedPackedInt32 {
        static mut instance: ::protobuf::lazy::Lazy<TestRepeatedPackedInt32> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestRepeatedPackedInt32,
        };
        unsafe {
            instance.get(|| {
                TestRepeatedPackedInt32 {
                    values: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated int32 values = 1;

    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::std::vec::Vec<i32>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.values, ::std::vec::Vec::new())
    }

    pub fn get_values<'a>(&'a self) -> &'a [i32] {
        &self.values
    }
}

impl ::protobuf::Message for TestRepeatedPackedInt32 {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.values));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.values.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(1, &self.values);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.values.is_empty() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.values)));
            for v in self.values.iter() {
                try!(os.write_int32_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestRepeatedPackedInt32>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestRepeatedPackedInt32 {
    fn new() -> TestRepeatedPackedInt32 {
        TestRepeatedPackedInt32::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestRepeatedPackedInt32>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "values",
                    TestRepeatedPackedInt32::get_values,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestRepeatedPackedInt32>(
                    "TestRepeatedPackedInt32",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestRepeatedPackedInt32 {
    fn clear(&mut self) {
        self.clear_values();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestRepeatedPackedInt32 {
    fn eq(&self, other: &TestRepeatedPackedInt32) -> bool {
        self.values == other.values &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestRepeatedPackedInt32 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestRepeatedMessages {
    // message fields
    messages1: ::protobuf::RepeatedField<TestRepeatedMessages>,
    messages2: ::protobuf::RepeatedField<TestRepeatedMessages>,
    messages3: ::protobuf::RepeatedField<TestRepeatedMessages>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TestRepeatedMessages {
    pub fn new() -> TestRepeatedMessages {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestRepeatedMessages {
        static mut instance: ::protobuf::lazy::Lazy<TestRepeatedMessages> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestRepeatedMessages,
        };
        unsafe {
            instance.get(|| {
                TestRepeatedMessages {
                    messages1: ::protobuf::RepeatedField::new(),
                    messages2: ::protobuf::RepeatedField::new(),
                    messages3: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .TestRepeatedMessages messages1 = 1;

    pub fn clear_messages1(&mut self) {
        self.messages1.clear();
    }

    // Param is passed by value, moved
    pub fn set_messages1(&mut self, v: ::protobuf::RepeatedField<TestRepeatedMessages>) {
        self.messages1 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_messages1<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TestRepeatedMessages> {
        &mut self.messages1
    }

    // Take field
    pub fn take_messages1(&mut self) -> ::protobuf::RepeatedField<TestRepeatedMessages> {
        ::std::mem::replace(&mut self.messages1, ::protobuf::RepeatedField::new())
    }

    pub fn get_messages1<'a>(&'a self) -> &'a [TestRepeatedMessages] {
        &self.messages1
    }

    // repeated .TestRepeatedMessages messages2 = 2;

    pub fn clear_messages2(&mut self) {
        self.messages2.clear();
    }

    // Param is passed by value, moved
    pub fn set_messages2(&mut self, v: ::protobuf::RepeatedField<TestRepeatedMessages>) {
        self.messages2 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_messages2<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TestRepeatedMessages> {
        &mut self.messages2
    }

    // Take field
    pub fn take_messages2(&mut self) -> ::protobuf::RepeatedField<TestRepeatedMessages> {
        ::std::mem::replace(&mut self.messages2, ::protobuf::RepeatedField::new())
    }

    pub fn get_messages2<'a>(&'a self) -> &'a [TestRepeatedMessages] {
        &self.messages2
    }

    // repeated .TestRepeatedMessages messages3 = 3;

    pub fn clear_messages3(&mut self) {
        self.messages3.clear();
    }

    // Param is passed by value, moved
    pub fn set_messages3(&mut self, v: ::protobuf::RepeatedField<TestRepeatedMessages>) {
        self.messages3 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_messages3<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TestRepeatedMessages> {
        &mut self.messages3
    }

    // Take field
    pub fn take_messages3(&mut self) -> ::protobuf::RepeatedField<TestRepeatedMessages> {
        ::std::mem::replace(&mut self.messages3, ::protobuf::RepeatedField::new())
    }

    pub fn get_messages3<'a>(&'a self) -> &'a [TestRepeatedMessages] {
        &self.messages3
    }
}

impl ::protobuf::Message for TestRepeatedMessages {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.messages1));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.messages2));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.messages3));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.messages1.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.messages2.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.messages3.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.messages1.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.messages2.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.messages3.iter() {
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

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestRepeatedMessages>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestRepeatedMessages {
    fn new() -> TestRepeatedMessages {
        TestRepeatedMessages::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestRepeatedMessages>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "messages1",
                    TestRepeatedMessages::get_messages1,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "messages2",
                    TestRepeatedMessages::get_messages2,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "messages3",
                    TestRepeatedMessages::get_messages3,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestRepeatedMessages>(
                    "TestRepeatedMessages",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestRepeatedMessages {
    fn clear(&mut self) {
        self.clear_messages1();
        self.clear_messages2();
        self.clear_messages3();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestRepeatedMessages {
    fn eq(&self, other: &TestRepeatedMessages) -> bool {
        self.messages1 == other.messages1 &&
        self.messages2 == other.messages2 &&
        self.messages3 == other.messages3 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestRepeatedMessages {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestOptionalMessages {
    // message fields
    message1: ::protobuf::SingularPtrField<TestOptionalMessages>,
    message2: ::protobuf::SingularPtrField<TestOptionalMessages>,
    message3: ::protobuf::SingularPtrField<TestOptionalMessages>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TestOptionalMessages {
    pub fn new() -> TestOptionalMessages {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestOptionalMessages {
        static mut instance: ::protobuf::lazy::Lazy<TestOptionalMessages> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestOptionalMessages,
        };
        unsafe {
            instance.get(|| {
                TestOptionalMessages {
                    message1: ::protobuf::SingularPtrField::none(),
                    message2: ::protobuf::SingularPtrField::none(),
                    message3: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .TestOptionalMessages message1 = 1;

    pub fn clear_message1(&mut self) {
        self.message1.clear();
    }

    pub fn has_message1(&self) -> bool {
        self.message1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message1(&mut self, v: TestOptionalMessages) {
        self.message1 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message1<'a>(&'a mut self) -> &'a mut TestOptionalMessages {
        if self.message1.is_none() {
            self.message1.set_default();
        };
        self.message1.as_mut().unwrap()
    }

    // Take field
    pub fn take_message1(&mut self) -> TestOptionalMessages {
        self.message1.take().unwrap_or_else(|| TestOptionalMessages::new())
    }

    pub fn get_message1<'a>(&'a self) -> &'a TestOptionalMessages {
        self.message1.as_ref().unwrap_or_else(|| TestOptionalMessages::default_instance())
    }

    // optional .TestOptionalMessages message2 = 2;

    pub fn clear_message2(&mut self) {
        self.message2.clear();
    }

    pub fn has_message2(&self) -> bool {
        self.message2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message2(&mut self, v: TestOptionalMessages) {
        self.message2 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message2<'a>(&'a mut self) -> &'a mut TestOptionalMessages {
        if self.message2.is_none() {
            self.message2.set_default();
        };
        self.message2.as_mut().unwrap()
    }

    // Take field
    pub fn take_message2(&mut self) -> TestOptionalMessages {
        self.message2.take().unwrap_or_else(|| TestOptionalMessages::new())
    }

    pub fn get_message2<'a>(&'a self) -> &'a TestOptionalMessages {
        self.message2.as_ref().unwrap_or_else(|| TestOptionalMessages::default_instance())
    }

    // optional .TestOptionalMessages message3 = 3;

    pub fn clear_message3(&mut self) {
        self.message3.clear();
    }

    pub fn has_message3(&self) -> bool {
        self.message3.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message3(&mut self, v: TestOptionalMessages) {
        self.message3 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message3<'a>(&'a mut self) -> &'a mut TestOptionalMessages {
        if self.message3.is_none() {
            self.message3.set_default();
        };
        self.message3.as_mut().unwrap()
    }

    // Take field
    pub fn take_message3(&mut self) -> TestOptionalMessages {
        self.message3.take().unwrap_or_else(|| TestOptionalMessages::new())
    }

    pub fn get_message3<'a>(&'a self) -> &'a TestOptionalMessages {
        self.message3.as_ref().unwrap_or_else(|| TestOptionalMessages::default_instance())
    }
}

impl ::protobuf::Message for TestOptionalMessages {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.message1.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.message2.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.message3.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.message1.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.message2.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.message3.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message1.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.message2.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.message3.as_ref() {
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

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestOptionalMessages>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestOptionalMessages {
    fn new() -> TestOptionalMessages {
        TestOptionalMessages::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestOptionalMessages>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "message1",
                    TestOptionalMessages::has_message1,
                    TestOptionalMessages::get_message1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "message2",
                    TestOptionalMessages::has_message2,
                    TestOptionalMessages::get_message2,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "message3",
                    TestOptionalMessages::has_message3,
                    TestOptionalMessages::get_message3,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestOptionalMessages>(
                    "TestOptionalMessages",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestOptionalMessages {
    fn clear(&mut self) {
        self.clear_message1();
        self.clear_message2();
        self.clear_message3();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestOptionalMessages {
    fn eq(&self, other: &TestOptionalMessages) -> bool {
        self.message1 == other.message1 &&
        self.message2 == other.message2 &&
        self.message3 == other.message3 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestOptionalMessages {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TestStrings {
    // message fields
    s1: ::protobuf::SingularField<::std::string::String>,
    s2: ::protobuf::SingularField<::std::string::String>,
    s3: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TestStrings {
    pub fn new() -> TestStrings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestStrings {
        static mut instance: ::protobuf::lazy::Lazy<TestStrings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestStrings,
        };
        unsafe {
            instance.get(|| {
                TestStrings {
                    s1: ::protobuf::SingularField::none(),
                    s2: ::protobuf::SingularField::none(),
                    s3: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string s1 = 1;

    pub fn clear_s1(&mut self) {
        self.s1.clear();
    }

    pub fn has_s1(&self) -> bool {
        self.s1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s1(&mut self, v: ::std::string::String) {
        self.s1 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_s1<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.s1.is_none() {
            self.s1.set_default();
        };
        self.s1.as_mut().unwrap()
    }

    // Take field
    pub fn take_s1(&mut self) -> ::std::string::String {
        self.s1.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_s1<'a>(&'a self) -> &'a str {
        match self.s1.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string s2 = 2;

    pub fn clear_s2(&mut self) {
        self.s2.clear();
    }

    pub fn has_s2(&self) -> bool {
        self.s2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s2(&mut self, v: ::std::string::String) {
        self.s2 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_s2<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.s2.is_none() {
            self.s2.set_default();
        };
        self.s2.as_mut().unwrap()
    }

    // Take field
    pub fn take_s2(&mut self) -> ::std::string::String {
        self.s2.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_s2<'a>(&'a self) -> &'a str {
        match self.s2.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string s3 = 3;

    pub fn clear_s3(&mut self) {
        self.s3.clear();
    }

    pub fn has_s3(&self) -> bool {
        self.s3.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s3(&mut self, v: ::std::string::String) {
        self.s3 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_s3<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.s3.is_none() {
            self.s3.set_default();
        };
        self.s3.as_mut().unwrap()
    }

    // Take field
    pub fn take_s3(&mut self) -> ::std::string::String {
        self.s3.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_s3<'a>(&'a self) -> &'a str {
        match self.s3.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for TestStrings {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.s1.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.s2.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.s3.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.s1.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.s2.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.s3.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.s1.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.s2.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.s3.as_ref() {
            try!(os.write_string(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TestStrings>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TestStrings {
    fn new() -> TestStrings {
        TestStrings::new()
    }

    fn descriptor_static(_: ::std::option::Option<TestStrings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "s1",
                    TestStrings::has_s1,
                    TestStrings::get_s1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "s2",
                    TestStrings::has_s2,
                    TestStrings::get_s2,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "s3",
                    TestStrings::has_s3,
                    TestStrings::get_s3,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestStrings>(
                    "TestStrings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TestStrings {
    fn clear(&mut self) {
        self.clear_s1();
        self.clear_s2();
        self.clear_s3();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TestStrings {
    fn eq(&self, other: &TestStrings) -> bool {
        self.s1 == other.s1 &&
        self.s2 == other.s2 &&
        self.s3 == other.s3 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TestStrings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PerftestData {
    // message fields
    test1: ::protobuf::RepeatedField<Test1>,
    test_repeated_bool: ::protobuf::RepeatedField<TestRepeatedBool>,
    test_repeated_messages: ::protobuf::RepeatedField<TestRepeatedMessages>,
    test_optional_messages: ::protobuf::RepeatedField<TestOptionalMessages>,
    test_strings: ::protobuf::RepeatedField<TestStrings>,
    test_repeated_packed_int32: ::protobuf::RepeatedField<TestRepeatedPackedInt32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PerftestData {
    pub fn new() -> PerftestData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PerftestData {
        static mut instance: ::protobuf::lazy::Lazy<PerftestData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PerftestData,
        };
        unsafe {
            instance.get(|| {
                PerftestData {
                    test1: ::protobuf::RepeatedField::new(),
                    test_repeated_bool: ::protobuf::RepeatedField::new(),
                    test_repeated_messages: ::protobuf::RepeatedField::new(),
                    test_optional_messages: ::protobuf::RepeatedField::new(),
                    test_strings: ::protobuf::RepeatedField::new(),
                    test_repeated_packed_int32: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Test1 test1 = 1;

    pub fn clear_test1(&mut self) {
        self.test1.clear();
    }

    // Param is passed by value, moved
    pub fn set_test1(&mut self, v: ::protobuf::RepeatedField<Test1>) {
        self.test1 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_test1<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Test1> {
        &mut self.test1
    }

    // Take field
    pub fn take_test1(&mut self) -> ::protobuf::RepeatedField<Test1> {
        ::std::mem::replace(&mut self.test1, ::protobuf::RepeatedField::new())
    }

    pub fn get_test1<'a>(&'a self) -> &'a [Test1] {
        &self.test1
    }

    // repeated .TestRepeatedBool test_repeated_bool = 2;

    pub fn clear_test_repeated_bool(&mut self) {
        self.test_repeated_bool.clear();
    }

    // Param is passed by value, moved
    pub fn set_test_repeated_bool(&mut self, v: ::protobuf::RepeatedField<TestRepeatedBool>) {
        self.test_repeated_bool = v;
    }

    // Mutable pointer to the field.
    pub fn mut_test_repeated_bool<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TestRepeatedBool> {
        &mut self.test_repeated_bool
    }

    // Take field
    pub fn take_test_repeated_bool(&mut self) -> ::protobuf::RepeatedField<TestRepeatedBool> {
        ::std::mem::replace(&mut self.test_repeated_bool, ::protobuf::RepeatedField::new())
    }

    pub fn get_test_repeated_bool<'a>(&'a self) -> &'a [TestRepeatedBool] {
        &self.test_repeated_bool
    }

    // repeated .TestRepeatedMessages test_repeated_messages = 3;

    pub fn clear_test_repeated_messages(&mut self) {
        self.test_repeated_messages.clear();
    }

    // Param is passed by value, moved
    pub fn set_test_repeated_messages(&mut self, v: ::protobuf::RepeatedField<TestRepeatedMessages>) {
        self.test_repeated_messages = v;
    }

    // Mutable pointer to the field.
    pub fn mut_test_repeated_messages<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TestRepeatedMessages> {
        &mut self.test_repeated_messages
    }

    // Take field
    pub fn take_test_repeated_messages(&mut self) -> ::protobuf::RepeatedField<TestRepeatedMessages> {
        ::std::mem::replace(&mut self.test_repeated_messages, ::protobuf::RepeatedField::new())
    }

    pub fn get_test_repeated_messages<'a>(&'a self) -> &'a [TestRepeatedMessages] {
        &self.test_repeated_messages
    }

    // repeated .TestOptionalMessages test_optional_messages = 4;

    pub fn clear_test_optional_messages(&mut self) {
        self.test_optional_messages.clear();
    }

    // Param is passed by value, moved
    pub fn set_test_optional_messages(&mut self, v: ::protobuf::RepeatedField<TestOptionalMessages>) {
        self.test_optional_messages = v;
    }

    // Mutable pointer to the field.
    pub fn mut_test_optional_messages<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TestOptionalMessages> {
        &mut self.test_optional_messages
    }

    // Take field
    pub fn take_test_optional_messages(&mut self) -> ::protobuf::RepeatedField<TestOptionalMessages> {
        ::std::mem::replace(&mut self.test_optional_messages, ::protobuf::RepeatedField::new())
    }

    pub fn get_test_optional_messages<'a>(&'a self) -> &'a [TestOptionalMessages] {
        &self.test_optional_messages
    }

    // repeated .TestStrings test_strings = 5;

    pub fn clear_test_strings(&mut self) {
        self.test_strings.clear();
    }

    // Param is passed by value, moved
    pub fn set_test_strings(&mut self, v: ::protobuf::RepeatedField<TestStrings>) {
        self.test_strings = v;
    }

    // Mutable pointer to the field.
    pub fn mut_test_strings<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TestStrings> {
        &mut self.test_strings
    }

    // Take field
    pub fn take_test_strings(&mut self) -> ::protobuf::RepeatedField<TestStrings> {
        ::std::mem::replace(&mut self.test_strings, ::protobuf::RepeatedField::new())
    }

    pub fn get_test_strings<'a>(&'a self) -> &'a [TestStrings] {
        &self.test_strings
    }

    // repeated .TestRepeatedPackedInt32 test_repeated_packed_int32 = 6;

    pub fn clear_test_repeated_packed_int32(&mut self) {
        self.test_repeated_packed_int32.clear();
    }

    // Param is passed by value, moved
    pub fn set_test_repeated_packed_int32(&mut self, v: ::protobuf::RepeatedField<TestRepeatedPackedInt32>) {
        self.test_repeated_packed_int32 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_test_repeated_packed_int32<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TestRepeatedPackedInt32> {
        &mut self.test_repeated_packed_int32
    }

    // Take field
    pub fn take_test_repeated_packed_int32(&mut self) -> ::protobuf::RepeatedField<TestRepeatedPackedInt32> {
        ::std::mem::replace(&mut self.test_repeated_packed_int32, ::protobuf::RepeatedField::new())
    }

    pub fn get_test_repeated_packed_int32<'a>(&'a self) -> &'a [TestRepeatedPackedInt32] {
        &self.test_repeated_packed_int32
    }
}

impl ::protobuf::Message for PerftestData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.test1));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.test_repeated_bool));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.test_repeated_messages));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.test_optional_messages));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.test_strings));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.test_repeated_packed_int32));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.test1.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.test_repeated_bool.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.test_repeated_messages.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.test_optional_messages.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.test_strings.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.test_repeated_packed_int32.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.test1.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.test_repeated_bool.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.test_repeated_messages.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.test_optional_messages.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.test_strings.iter() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.test_repeated_packed_int32.iter() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PerftestData>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PerftestData {
    fn new() -> PerftestData {
        PerftestData::new()
    }

    fn descriptor_static(_: ::std::option::Option<PerftestData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "test1",
                    PerftestData::get_test1,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "test_repeated_bool",
                    PerftestData::get_test_repeated_bool,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "test_repeated_messages",
                    PerftestData::get_test_repeated_messages,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "test_optional_messages",
                    PerftestData::get_test_optional_messages,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "test_strings",
                    PerftestData::get_test_strings,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "test_repeated_packed_int32",
                    PerftestData::get_test_repeated_packed_int32,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PerftestData>(
                    "PerftestData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PerftestData {
    fn clear(&mut self) {
        self.clear_test1();
        self.clear_test_repeated_bool();
        self.clear_test_repeated_messages();
        self.clear_test_optional_messages();
        self.clear_test_strings();
        self.clear_test_repeated_packed_int32();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PerftestData {
    fn eq(&self, other: &PerftestData) -> bool {
        self.test1 == other.test1 &&
        self.test_repeated_bool == other.test_repeated_bool &&
        self.test_repeated_messages == other.test_repeated_messages &&
        self.test_optional_messages == other.test_optional_messages &&
        self.test_strings == other.test_strings &&
        self.test_repeated_packed_int32 == other.test_repeated_packed_int32 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PerftestData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x70, 0x65, 0x72, 0x66, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x16, 0x0a, 0x05, 0x54, 0x65, 0x73, 0x74, 0x31, 0x12, 0x0d,
    0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x22, 0x22, 0x0a,
    0x10, 0x54, 0x65, 0x73, 0x74, 0x52, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x42, 0x6f, 0x6f,
    0x6c, 0x12, 0x0e, 0x0a, 0x06, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x08, 0x22, 0x2d, 0x0a, 0x17, 0x54, 0x65, 0x73, 0x74, 0x52, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65,
    0x64, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x64, 0x49, 0x6e, 0x74, 0x33, 0x32, 0x12, 0x12, 0x0a, 0x06,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x05, 0x42, 0x02, 0x10, 0x01,
    0x22, 0x94, 0x01, 0x0a, 0x14, 0x54, 0x65, 0x73, 0x74, 0x52, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65,
    0x64, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x12, 0x28, 0x0a, 0x09, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x31, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x54,
    0x65, 0x73, 0x74, 0x52, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x12, 0x28, 0x0a, 0x09, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x32,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x52, 0x65, 0x70,
    0x65, 0x61, 0x74, 0x65, 0x64, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x12, 0x28, 0x0a,
    0x09, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x33, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x15, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x52, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x22, 0x91, 0x01, 0x0a, 0x14, 0x54, 0x65, 0x73, 0x74,
    0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73,
    0x12, 0x27, 0x0a, 0x08, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x31, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x15, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61,
    0x6c, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x12, 0x27, 0x0a, 0x08, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x32, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x54, 0x65,
    0x73, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x73, 0x12, 0x27, 0x0a, 0x08, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x33, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x61, 0x6c, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x22, 0x31, 0x0a, 0x0b, 0x54,
    0x65, 0x73, 0x74, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x0a, 0x0a, 0x02, 0x73, 0x31,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0a, 0x0a, 0x02, 0x73, 0x32, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x12, 0x0a, 0x0a, 0x02, 0x73, 0x33, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x22, 0xa4,
    0x02, 0x0a, 0x0c, 0x50, 0x65, 0x72, 0x66, 0x74, 0x65, 0x73, 0x74, 0x44, 0x61, 0x74, 0x61, 0x12,
    0x15, 0x0a, 0x05, 0x74, 0x65, 0x73, 0x74, 0x31, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06,
    0x2e, 0x54, 0x65, 0x73, 0x74, 0x31, 0x12, 0x2d, 0x0a, 0x12, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x72,
    0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x62, 0x6f, 0x6f, 0x6c, 0x18, 0x02, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x11, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x52, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65,
    0x64, 0x42, 0x6f, 0x6f, 0x6c, 0x12, 0x35, 0x0a, 0x16, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x72, 0x65,
    0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x18,
    0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x52, 0x65, 0x70, 0x65,
    0x61, 0x74, 0x65, 0x64, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x12, 0x35, 0x0a, 0x16,
    0x74, 0x65, 0x73, 0x74, 0x5f, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x5f, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x54,
    0x65, 0x73, 0x74, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x12, 0x22, 0x0a, 0x0c, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x73, 0x74, 0x72, 0x69,
    0x6e, 0x67, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x54, 0x65, 0x73, 0x74,
    0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x3c, 0x0a, 0x1a, 0x74, 0x65, 0x73, 0x74, 0x5f,
    0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x64, 0x5f,
    0x69, 0x6e, 0x74, 0x33, 0x32, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x54, 0x65,
    0x73, 0x74, 0x52, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x64,
    0x49, 0x6e, 0x74, 0x33, 0x32, 0x4a, 0xf4, 0x0b, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x25, 0x01,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x00, 0x00, 0x02, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x00, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x01, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x01, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x01, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x01, 0x13, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x01, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x04, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01,
    0x12, 0x03, 0x04, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x05,
    0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x05, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x05, 0x0d, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02,
    0x12, 0x04, 0x08, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x08,
    0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x09, 0x04, 0x30, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x09, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x08,
    0x12, 0x03, 0x09, 0x1e, 0x2f, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x09, 0x20, 0x2d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x09, 0x20, 0x26, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x02, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x20, 0x26, 0x0a, 0x12, 0x0a, 0x0b, 0x04,
    0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x20, 0x26, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x09, 0x29,
    0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x0c, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x00, 0x12, 0x03, 0x0d, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x0d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0d,
    0x0d, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x22, 0x2b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x2e, 0x2f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x0e, 0x0d, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0e, 0x22, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0e, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x04, 0x30,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0f, 0x0d, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x22, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x2e, 0x2f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04,
    0x12, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x12, 0x08, 0x1c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x13, 0x04, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x13, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x13, 0x0d, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x13, 0x22, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x13, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x14,
    0x04, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x14, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x14, 0x0d, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x22, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x02, 0x12, 0x03, 0x15, 0x04, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x15, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x15, 0x0d, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x15, 0x22,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x15, 0x2d, 0x2e, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x18, 0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x05, 0x01, 0x12, 0x03, 0x18, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12,
    0x03, 0x19, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x19,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x19, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x14, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x1a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x1a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x1a, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1a, 0x19,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x1b, 0x04, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x1b, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x1e, 0x00,
    0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x14, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x1f, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x1f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x1f, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x1f, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x20, 0x04, 0x35,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x20, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x20, 0x0d, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x20, 0x1e, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x20, 0x33, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02,
    0x12, 0x03, 0x21, 0x04, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x21, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x06, 0x12, 0x03, 0x21, 0x0d,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x21, 0x22, 0x38, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x21, 0x3b, 0x3c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x22, 0x04, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x22, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03,
    0x06, 0x12, 0x03, 0x22, 0x0d, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x22, 0x22, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x22,
    0x3b, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03, 0x23, 0x04, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12, 0x03, 0x23, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x04, 0x06, 0x12, 0x03, 0x23, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x23, 0x19, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x23, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x05, 0x12,
    0x03, 0x24, 0x04, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x04, 0x12, 0x03, 0x24,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x06, 0x12, 0x03, 0x24, 0x0d, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01, 0x12, 0x03, 0x24, 0x25, 0x3f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03, 0x24, 0x42, 0x43,
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

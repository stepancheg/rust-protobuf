// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]


#[deriving(Clone,Default)]
pub struct Test1 {
    value: ::std::option::Option<i32>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Test1 {
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
                    value: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional int32 value = 1;

    pub fn clear_value(&mut self) {
        self.value = None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i32) {
        self.value = Some(v);
    }

    pub fn get_value(&self) -> i32 {
        self.value.unwrap_or(0)
    }
}

impl ::protobuf::Message for Test1 {
    fn new() -> Test1 {
        Test1::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.value = Some(tmp);
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
        for value in self.value.iter() {
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
        match self.value {
            Some(v) => {
                try!(os.write_int32(1, v));
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
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Test1>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Test1_value_acc as &'static ::protobuf::reflect::FieldAccessor<Test1>) });
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

impl ::std::fmt::Show for Test1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Test1_value_acc_type;
static Test1_value_acc: Test1_value_acc_type = Test1_value_acc_type;

impl ::protobuf::reflect::FieldAccessor<Test1> for Test1_value_acc_type {
    fn name(&self) -> &'static str {
        "value"
    }

    fn has_field(&self, m: &Test1) -> bool {
        m.has_value()
    }

    fn get_i32(&self, m: &Test1) -> i32 {
        m.get_value()
    }
}

#[deriving(Clone,Default)]
pub struct TestRepeatedBool {
    values: ::std::vec::Vec<bool>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestRepeatedBool {
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
    pub fn mut_values(&'a mut self) -> &'a mut ::std::vec::Vec<bool> {
        &mut self.values
    }

    pub fn get_values(&'a self) -> &'a [bool] {
        self.values.as_slice()
    }
}

impl ::protobuf::Message for TestRepeatedBool {
    fn new() -> TestRepeatedBool {
        TestRepeatedBool::new()
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
                            self.values.push(try!(is.read_bool()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                        };
                        self.values.push(try!(is.read_bool()));
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
        my_size += 2 * self.values.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        for v in self.values.iter() {
            try!(os.write_bool(1, *v));
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
    fn descriptor_static(_: ::std::option::Option<TestRepeatedBool>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestRepeatedBool>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestRepeatedBool_values_acc as &'static ::protobuf::reflect::FieldAccessor<TestRepeatedBool>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestRepeatedBool>(
                    "TestRepeatedBool",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestRepeatedBool>()
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

impl ::std::fmt::Show for TestRepeatedBool {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestRepeatedBool_values_acc_type;
static TestRepeatedBool_values_acc: TestRepeatedBool_values_acc_type = TestRepeatedBool_values_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestRepeatedBool> for TestRepeatedBool_values_acc_type {
    fn name(&self) -> &'static str {
        "values"
    }

    fn len_field(&self, m: &TestRepeatedBool) -> uint {
        m.get_values().len()
    }

    fn get_rep_bool<'a>(&self, m: &'a TestRepeatedBool) -> &'a [bool] {
        m.get_values()
    }
}

#[deriving(Clone,Default)]
pub struct TestRepeatedPackedInt32 {
    values: ::std::vec::Vec<i32>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestRepeatedPackedInt32 {
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
    pub fn mut_values(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.values
    }

    pub fn get_values(&'a self) -> &'a [i32] {
        self.values.as_slice()
    }
}

impl ::protobuf::Message for TestRepeatedPackedInt32 {
    fn new() -> TestRepeatedPackedInt32 {
        TestRepeatedPackedInt32::new()
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
                            self.values.push(try!(is.read_int32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                        };
                        self.values.push(try!(is.read_int32()));
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
        if !self.values.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(1, self.values.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        if !self.values.is_empty() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(self.values.as_slice())));
            for v in self.values.iter() {
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
    fn descriptor_static(_: ::std::option::Option<TestRepeatedPackedInt32>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestRepeatedPackedInt32>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestRepeatedPackedInt32_values_acc as &'static ::protobuf::reflect::FieldAccessor<TestRepeatedPackedInt32>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestRepeatedPackedInt32>(
                    "TestRepeatedPackedInt32",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestRepeatedPackedInt32>()
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

impl ::std::fmt::Show for TestRepeatedPackedInt32 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestRepeatedPackedInt32_values_acc_type;
static TestRepeatedPackedInt32_values_acc: TestRepeatedPackedInt32_values_acc_type = TestRepeatedPackedInt32_values_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestRepeatedPackedInt32> for TestRepeatedPackedInt32_values_acc_type {
    fn name(&self) -> &'static str {
        "values"
    }

    fn len_field(&self, m: &TestRepeatedPackedInt32) -> uint {
        m.get_values().len()
    }

    fn get_rep_i32<'a>(&self, m: &'a TestRepeatedPackedInt32) -> &'a [i32] {
        m.get_values()
    }
}

#[deriving(Clone,Default)]
pub struct TestRepeatedMessages {
    messages1: ::protobuf::RepeatedField<TestRepeatedMessages>,
    messages2: ::protobuf::RepeatedField<TestRepeatedMessages>,
    messages3: ::protobuf::RepeatedField<TestRepeatedMessages>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestRepeatedMessages {
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
    pub fn mut_messages1(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TestRepeatedMessages> {
        &mut self.messages1
    }

    pub fn get_messages1(&'a self) -> &'a [TestRepeatedMessages] {
        self.messages1.as_slice()
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
    pub fn mut_messages2(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TestRepeatedMessages> {
        &mut self.messages2
    }

    pub fn get_messages2(&'a self) -> &'a [TestRepeatedMessages] {
        self.messages2.as_slice()
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
    pub fn mut_messages3(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TestRepeatedMessages> {
        &mut self.messages3
    }

    pub fn get_messages3(&'a self) -> &'a [TestRepeatedMessages] {
        self.messages3.as_slice()
    }
}

impl ::protobuf::Message for TestRepeatedMessages {
    fn new() -> TestRepeatedMessages {
        TestRepeatedMessages::new()
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.messages1.push_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.messages2.push_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.messages3.push_default();
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
        for value in self.messages1.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.messages2.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.messages3.iter() {
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
        for v in self.messages1.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        for v in self.messages2.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        for v in self.messages3.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
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
    fn descriptor_static(_: ::std::option::Option<TestRepeatedMessages>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestRepeatedMessages>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestRepeatedMessages_messages1_acc as &'static ::protobuf::reflect::FieldAccessor<TestRepeatedMessages>) });
                fields.push(unsafe { ::std::mem::transmute(&TestRepeatedMessages_messages2_acc as &'static ::protobuf::reflect::FieldAccessor<TestRepeatedMessages>) });
                fields.push(unsafe { ::std::mem::transmute(&TestRepeatedMessages_messages3_acc as &'static ::protobuf::reflect::FieldAccessor<TestRepeatedMessages>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestRepeatedMessages>(
                    "TestRepeatedMessages",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestRepeatedMessages>()
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

impl ::std::fmt::Show for TestRepeatedMessages {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestRepeatedMessages_messages1_acc_type;
static TestRepeatedMessages_messages1_acc: TestRepeatedMessages_messages1_acc_type = TestRepeatedMessages_messages1_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestRepeatedMessages> for TestRepeatedMessages_messages1_acc_type {
    fn name(&self) -> &'static str {
        "messages1"
    }

    fn len_field(&self, m: &TestRepeatedMessages) -> uint {
        m.get_messages1().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a TestRepeatedMessages, index: uint) -> &'a ::protobuf::Message {
        &m.get_messages1()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct TestRepeatedMessages_messages2_acc_type;
static TestRepeatedMessages_messages2_acc: TestRepeatedMessages_messages2_acc_type = TestRepeatedMessages_messages2_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestRepeatedMessages> for TestRepeatedMessages_messages2_acc_type {
    fn name(&self) -> &'static str {
        "messages2"
    }

    fn len_field(&self, m: &TestRepeatedMessages) -> uint {
        m.get_messages2().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a TestRepeatedMessages, index: uint) -> &'a ::protobuf::Message {
        &m.get_messages2()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct TestRepeatedMessages_messages3_acc_type;
static TestRepeatedMessages_messages3_acc: TestRepeatedMessages_messages3_acc_type = TestRepeatedMessages_messages3_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestRepeatedMessages> for TestRepeatedMessages_messages3_acc_type {
    fn name(&self) -> &'static str {
        "messages3"
    }

    fn len_field(&self, m: &TestRepeatedMessages) -> uint {
        m.get_messages3().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a TestRepeatedMessages, index: uint) -> &'a ::protobuf::Message {
        &m.get_messages3()[index] as &'a ::protobuf::Message
    }
}

#[deriving(Clone,Default)]
pub struct TestOptionalMessages {
    message1: ::protobuf::SingularPtrField<TestOptionalMessages>,
    message2: ::protobuf::SingularPtrField<TestOptionalMessages>,
    message3: ::protobuf::SingularPtrField<TestOptionalMessages>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestOptionalMessages {
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
    pub fn mut_message1(&'a mut self) -> &'a mut TestOptionalMessages {
        if self.message1.is_none() {
            self.message1.set_default();
        };
        self.message1.as_mut().unwrap()
    }

    pub fn get_message1(&'a self) -> &'a TestOptionalMessages {
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
    pub fn mut_message2(&'a mut self) -> &'a mut TestOptionalMessages {
        if self.message2.is_none() {
            self.message2.set_default();
        };
        self.message2.as_mut().unwrap()
    }

    pub fn get_message2(&'a self) -> &'a TestOptionalMessages {
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
    pub fn mut_message3(&'a mut self) -> &'a mut TestOptionalMessages {
        if self.message3.is_none() {
            self.message3.set_default();
        };
        self.message3.as_mut().unwrap()
    }

    pub fn get_message3(&'a self) -> &'a TestOptionalMessages {
        self.message3.as_ref().unwrap_or_else(|| TestOptionalMessages::default_instance())
    }
}

impl ::protobuf::Message for TestOptionalMessages {
    fn new() -> TestOptionalMessages {
        TestOptionalMessages::new()
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.message1.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.message2.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
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
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.message1.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.message2.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.message3.iter() {
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
        match self.message1.as_ref() {
            Some(v) => {
                try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.message2.as_ref() {
            Some(v) => {
                try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.message3.as_ref() {
            Some(v) => {
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
    fn descriptor_static(_: ::std::option::Option<TestOptionalMessages>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestOptionalMessages>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestOptionalMessages_message1_acc as &'static ::protobuf::reflect::FieldAccessor<TestOptionalMessages>) });
                fields.push(unsafe { ::std::mem::transmute(&TestOptionalMessages_message2_acc as &'static ::protobuf::reflect::FieldAccessor<TestOptionalMessages>) });
                fields.push(unsafe { ::std::mem::transmute(&TestOptionalMessages_message3_acc as &'static ::protobuf::reflect::FieldAccessor<TestOptionalMessages>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestOptionalMessages>(
                    "TestOptionalMessages",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestOptionalMessages>()
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

impl ::std::fmt::Show for TestOptionalMessages {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestOptionalMessages_message1_acc_type;
static TestOptionalMessages_message1_acc: TestOptionalMessages_message1_acc_type = TestOptionalMessages_message1_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestOptionalMessages> for TestOptionalMessages_message1_acc_type {
    fn name(&self) -> &'static str {
        "message1"
    }

    fn has_field(&self, m: &TestOptionalMessages) -> bool {
        m.has_message1()
    }

    fn get_message<'a>(&self, m: &'a TestOptionalMessages) -> &'a ::protobuf::Message {
        m.get_message1() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct TestOptionalMessages_message2_acc_type;
static TestOptionalMessages_message2_acc: TestOptionalMessages_message2_acc_type = TestOptionalMessages_message2_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestOptionalMessages> for TestOptionalMessages_message2_acc_type {
    fn name(&self) -> &'static str {
        "message2"
    }

    fn has_field(&self, m: &TestOptionalMessages) -> bool {
        m.has_message2()
    }

    fn get_message<'a>(&self, m: &'a TestOptionalMessages) -> &'a ::protobuf::Message {
        m.get_message2() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct TestOptionalMessages_message3_acc_type;
static TestOptionalMessages_message3_acc: TestOptionalMessages_message3_acc_type = TestOptionalMessages_message3_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestOptionalMessages> for TestOptionalMessages_message3_acc_type {
    fn name(&self) -> &'static str {
        "message3"
    }

    fn has_field(&self, m: &TestOptionalMessages) -> bool {
        m.has_message3()
    }

    fn get_message<'a>(&self, m: &'a TestOptionalMessages) -> &'a ::protobuf::Message {
        m.get_message3() as &'a ::protobuf::Message
    }
}

#[deriving(Clone,Default)]
pub struct TestStrings {
    s1: ::protobuf::SingularField<::std::string::String>,
    s2: ::protobuf::SingularField<::std::string::String>,
    s3: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestStrings {
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
    pub fn mut_s1(&'a mut self) -> &'a mut ::std::string::String {
        if self.s1.is_none() {
            self.s1.set_default();
        };
        self.s1.as_mut().unwrap()
    }

    pub fn get_s1(&'a self) -> &'a str {
        match self.s1.as_ref() {
            Some(v) => v.as_slice(),
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
    pub fn mut_s2(&'a mut self) -> &'a mut ::std::string::String {
        if self.s2.is_none() {
            self.s2.set_default();
        };
        self.s2.as_mut().unwrap()
    }

    pub fn get_s2(&'a self) -> &'a str {
        match self.s2.as_ref() {
            Some(v) => v.as_slice(),
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
    pub fn mut_s3(&'a mut self) -> &'a mut ::std::string::String {
        if self.s3.is_none() {
            self.s3.set_default();
        };
        self.s3.as_mut().unwrap()
    }

    pub fn get_s3(&'a self) -> &'a str {
        match self.s3.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for TestStrings {
    fn new() -> TestStrings {
        TestStrings::new()
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.s1.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.s2.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
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
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.s1.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        for value in self.s2.iter() {
            my_size += ::protobuf::rt::string_size(2, value.as_slice());
        };
        for value in self.s3.iter() {
            my_size += ::protobuf::rt::string_size(3, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.s1.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        match self.s2.as_ref() {
            Some(v) => {
                try!(os.write_string(2, v.as_slice()));
            },
            None => {},
        };
        match self.s3.as_ref() {
            Some(v) => {
                try!(os.write_string(3, v.as_slice()));
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
    fn descriptor_static(_: ::std::option::Option<TestStrings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestStrings>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestStrings_s1_acc as &'static ::protobuf::reflect::FieldAccessor<TestStrings>) });
                fields.push(unsafe { ::std::mem::transmute(&TestStrings_s2_acc as &'static ::protobuf::reflect::FieldAccessor<TestStrings>) });
                fields.push(unsafe { ::std::mem::transmute(&TestStrings_s3_acc as &'static ::protobuf::reflect::FieldAccessor<TestStrings>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestStrings>(
                    "TestStrings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestStrings>()
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

impl ::std::fmt::Show for TestStrings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestStrings_s1_acc_type;
static TestStrings_s1_acc: TestStrings_s1_acc_type = TestStrings_s1_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestStrings> for TestStrings_s1_acc_type {
    fn name(&self) -> &'static str {
        "s1"
    }

    fn has_field(&self, m: &TestStrings) -> bool {
        m.has_s1()
    }

    fn get_str<'a>(&self, m: &'a TestStrings) -> &'a str {
        m.get_s1()
    }
}

#[allow(non_camel_case_types)]
struct TestStrings_s2_acc_type;
static TestStrings_s2_acc: TestStrings_s2_acc_type = TestStrings_s2_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestStrings> for TestStrings_s2_acc_type {
    fn name(&self) -> &'static str {
        "s2"
    }

    fn has_field(&self, m: &TestStrings) -> bool {
        m.has_s2()
    }

    fn get_str<'a>(&self, m: &'a TestStrings) -> &'a str {
        m.get_s2()
    }
}

#[allow(non_camel_case_types)]
struct TestStrings_s3_acc_type;
static TestStrings_s3_acc: TestStrings_s3_acc_type = TestStrings_s3_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestStrings> for TestStrings_s3_acc_type {
    fn name(&self) -> &'static str {
        "s3"
    }

    fn has_field(&self, m: &TestStrings) -> bool {
        m.has_s3()
    }

    fn get_str<'a>(&self, m: &'a TestStrings) -> &'a str {
        m.get_s3()
    }
}

#[deriving(Clone,Default)]
pub struct PerftestData {
    test1: ::protobuf::RepeatedField<Test1>,
    test_repeated_bool: ::protobuf::RepeatedField<TestRepeatedBool>,
    test_repeated_messages: ::protobuf::RepeatedField<TestRepeatedMessages>,
    test_optional_messages: ::protobuf::RepeatedField<TestOptionalMessages>,
    test_strings: ::protobuf::RepeatedField<TestStrings>,
    test_repeated_packed_int32: ::protobuf::RepeatedField<TestRepeatedPackedInt32>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> PerftestData {
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
    pub fn mut_test1(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Test1> {
        &mut self.test1
    }

    pub fn get_test1(&'a self) -> &'a [Test1] {
        self.test1.as_slice()
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
    pub fn mut_test_repeated_bool(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TestRepeatedBool> {
        &mut self.test_repeated_bool
    }

    pub fn get_test_repeated_bool(&'a self) -> &'a [TestRepeatedBool] {
        self.test_repeated_bool.as_slice()
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
    pub fn mut_test_repeated_messages(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TestRepeatedMessages> {
        &mut self.test_repeated_messages
    }

    pub fn get_test_repeated_messages(&'a self) -> &'a [TestRepeatedMessages] {
        self.test_repeated_messages.as_slice()
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
    pub fn mut_test_optional_messages(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TestOptionalMessages> {
        &mut self.test_optional_messages
    }

    pub fn get_test_optional_messages(&'a self) -> &'a [TestOptionalMessages] {
        self.test_optional_messages.as_slice()
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
    pub fn mut_test_strings(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TestStrings> {
        &mut self.test_strings
    }

    pub fn get_test_strings(&'a self) -> &'a [TestStrings] {
        self.test_strings.as_slice()
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
    pub fn mut_test_repeated_packed_int32(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TestRepeatedPackedInt32> {
        &mut self.test_repeated_packed_int32
    }

    pub fn get_test_repeated_packed_int32(&'a self) -> &'a [TestRepeatedPackedInt32] {
        self.test_repeated_packed_int32.as_slice()
    }
}

impl ::protobuf::Message for PerftestData {
    fn new() -> PerftestData {
        PerftestData::new()
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.test1.push_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.test_repeated_bool.push_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.test_repeated_messages.push_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.test_optional_messages.push_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.test_strings.push_default();
                    try!(is.merge_message(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.test_repeated_packed_int32.push_default();
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
        for value in self.test1.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.test_repeated_bool.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.test_repeated_messages.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.test_optional_messages.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.test_strings.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.test_repeated_packed_int32.iter() {
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
        for v in self.test1.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        for v in self.test_repeated_bool.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        for v in self.test_repeated_messages.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        for v in self.test_optional_messages.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        for v in self.test_strings.iter() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        for v in self.test_repeated_packed_int32.iter() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
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
    fn descriptor_static(_: ::std::option::Option<PerftestData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<PerftestData>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&PerftestData_test1_acc as &'static ::protobuf::reflect::FieldAccessor<PerftestData>) });
                fields.push(unsafe { ::std::mem::transmute(&PerftestData_test_repeated_bool_acc as &'static ::protobuf::reflect::FieldAccessor<PerftestData>) });
                fields.push(unsafe { ::std::mem::transmute(&PerftestData_test_repeated_messages_acc as &'static ::protobuf::reflect::FieldAccessor<PerftestData>) });
                fields.push(unsafe { ::std::mem::transmute(&PerftestData_test_optional_messages_acc as &'static ::protobuf::reflect::FieldAccessor<PerftestData>) });
                fields.push(unsafe { ::std::mem::transmute(&PerftestData_test_strings_acc as &'static ::protobuf::reflect::FieldAccessor<PerftestData>) });
                fields.push(unsafe { ::std::mem::transmute(&PerftestData_test_repeated_packed_int32_acc as &'static ::protobuf::reflect::FieldAccessor<PerftestData>) });
                ::protobuf::reflect::MessageDescriptor::new::<PerftestData>(
                    "PerftestData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<PerftestData>()
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

impl ::std::fmt::Show for PerftestData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct PerftestData_test1_acc_type;
static PerftestData_test1_acc: PerftestData_test1_acc_type = PerftestData_test1_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerftestData> for PerftestData_test1_acc_type {
    fn name(&self) -> &'static str {
        "test1"
    }

    fn len_field(&self, m: &PerftestData) -> uint {
        m.get_test1().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a PerftestData, index: uint) -> &'a ::protobuf::Message {
        &m.get_test1()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct PerftestData_test_repeated_bool_acc_type;
static PerftestData_test_repeated_bool_acc: PerftestData_test_repeated_bool_acc_type = PerftestData_test_repeated_bool_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerftestData> for PerftestData_test_repeated_bool_acc_type {
    fn name(&self) -> &'static str {
        "test_repeated_bool"
    }

    fn len_field(&self, m: &PerftestData) -> uint {
        m.get_test_repeated_bool().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a PerftestData, index: uint) -> &'a ::protobuf::Message {
        &m.get_test_repeated_bool()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct PerftestData_test_repeated_messages_acc_type;
static PerftestData_test_repeated_messages_acc: PerftestData_test_repeated_messages_acc_type = PerftestData_test_repeated_messages_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerftestData> for PerftestData_test_repeated_messages_acc_type {
    fn name(&self) -> &'static str {
        "test_repeated_messages"
    }

    fn len_field(&self, m: &PerftestData) -> uint {
        m.get_test_repeated_messages().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a PerftestData, index: uint) -> &'a ::protobuf::Message {
        &m.get_test_repeated_messages()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct PerftestData_test_optional_messages_acc_type;
static PerftestData_test_optional_messages_acc: PerftestData_test_optional_messages_acc_type = PerftestData_test_optional_messages_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerftestData> for PerftestData_test_optional_messages_acc_type {
    fn name(&self) -> &'static str {
        "test_optional_messages"
    }

    fn len_field(&self, m: &PerftestData) -> uint {
        m.get_test_optional_messages().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a PerftestData, index: uint) -> &'a ::protobuf::Message {
        &m.get_test_optional_messages()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct PerftestData_test_strings_acc_type;
static PerftestData_test_strings_acc: PerftestData_test_strings_acc_type = PerftestData_test_strings_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerftestData> for PerftestData_test_strings_acc_type {
    fn name(&self) -> &'static str {
        "test_strings"
    }

    fn len_field(&self, m: &PerftestData) -> uint {
        m.get_test_strings().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a PerftestData, index: uint) -> &'a ::protobuf::Message {
        &m.get_test_strings()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct PerftestData_test_repeated_packed_int32_acc_type;
static PerftestData_test_repeated_packed_int32_acc: PerftestData_test_repeated_packed_int32_acc_type = PerftestData_test_repeated_packed_int32_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerftestData> for PerftestData_test_repeated_packed_int32_acc_type {
    fn name(&self) -> &'static str {
        "test_repeated_packed_int32"
    }

    fn len_field(&self, m: &PerftestData) -> uint {
        m.get_test_repeated_packed_int32().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a PerftestData, index: uint) -> &'a ::protobuf::Message {
        &m.get_test_repeated_packed_int32()[index] as &'a ::protobuf::Message
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

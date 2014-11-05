// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]


#[deriving(Clone,PartialEq,Default)]
pub struct TestMessage {
    value: ::std::option::Option<i32>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestMessage {
    pub fn new() -> TestMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestMessage {
        static mut instance: ::protobuf::lazy::Lazy<TestMessage> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const TestMessage };
        unsafe {
            instance.get(|| {
                TestMessage {
                    value: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional int32 value = 10;

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

impl ::protobuf::Message for TestMessage {
    fn new() -> TestMessage {
        TestMessage::new()
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
        match self.value {
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
    fn descriptor_static(_: ::std::option::Option<TestMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestMessage>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestMessage_value_acc as &'static ::protobuf::reflect::FieldAccessor<TestMessage>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestMessage>(
                    "TestMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestMessage>()
    }
}

impl ::protobuf::Clear for TestMessage {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for TestMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestMessage_value_acc_type;
static TestMessage_value_acc: TestMessage_value_acc_type = TestMessage_value_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestMessage> for TestMessage_value_acc_type {
    fn name(&self) -> &'static str {
        "value"
    }

    fn has_field(&self, m: &TestMessage) -> bool {
        m.has_value()
    }

    fn get_i32(&self, m: &TestMessage) -> i32 {
        m.get_value()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct TestTypes {
    double_singular: ::std::option::Option<f64>,
    float_singular: ::std::option::Option<f32>,
    int32_singular: ::std::option::Option<i32>,
    int64_singular: ::std::option::Option<i64>,
    uint32_singular: ::std::option::Option<u32>,
    uint64_singular: ::std::option::Option<u64>,
    sint32_singular: ::std::option::Option<i32>,
    sint64_singular: ::std::option::Option<i64>,
    fixed32_singular: ::std::option::Option<u32>,
    fixed64_singular: ::std::option::Option<u64>,
    sfixed32_singular: ::std::option::Option<i32>,
    sfixed64_singular: ::std::option::Option<i64>,
    bool_singular: ::std::option::Option<bool>,
    string_singular: ::protobuf::SingularField<::std::string::String>,
    bytes_singular: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    test_enum_singular: ::std::option::Option<TestEnum>,
    test_message_singular: ::protobuf::SingularPtrField<TestMessage>,
    double_repeated: ::std::vec::Vec<f64>,
    float_repeated: ::std::vec::Vec<f32>,
    int32_repeated: ::std::vec::Vec<i32>,
    int64_repeated: ::std::vec::Vec<i64>,
    uint32_repeated: ::std::vec::Vec<u32>,
    uint64_repeated: ::std::vec::Vec<u64>,
    sint32_repeated: ::std::vec::Vec<i32>,
    sint64_repeated: ::std::vec::Vec<i64>,
    fixed32_repeated: ::std::vec::Vec<u32>,
    fixed64_repeated: ::std::vec::Vec<u64>,
    sfixed32_repeated: ::std::vec::Vec<i32>,
    sfixed64_repeated: ::std::vec::Vec<i64>,
    bool_repeated: ::std::vec::Vec<bool>,
    string_repeated: ::protobuf::RepeatedField<::std::string::String>,
    bytes_repeated: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    test_enum_repeated: ::std::vec::Vec<TestEnum>,
    test_message_repeated: ::protobuf::RepeatedField<TestMessage>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestTypes {
    pub fn new() -> TestTypes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestTypes {
        static mut instance: ::protobuf::lazy::Lazy<TestTypes> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const TestTypes };
        unsafe {
            instance.get(|| {
                TestTypes {
                    double_singular: ::std::option::None,
                    float_singular: ::std::option::None,
                    int32_singular: ::std::option::None,
                    int64_singular: ::std::option::None,
                    uint32_singular: ::std::option::None,
                    uint64_singular: ::std::option::None,
                    sint32_singular: ::std::option::None,
                    sint64_singular: ::std::option::None,
                    fixed32_singular: ::std::option::None,
                    fixed64_singular: ::std::option::None,
                    sfixed32_singular: ::std::option::None,
                    sfixed64_singular: ::std::option::None,
                    bool_singular: ::std::option::None,
                    string_singular: ::protobuf::SingularField::none(),
                    bytes_singular: ::protobuf::SingularField::none(),
                    test_enum_singular: ::std::option::None,
                    test_message_singular: ::protobuf::SingularPtrField::none(),
                    double_repeated: ::std::vec::Vec::new(),
                    float_repeated: ::std::vec::Vec::new(),
                    int32_repeated: ::std::vec::Vec::new(),
                    int64_repeated: ::std::vec::Vec::new(),
                    uint32_repeated: ::std::vec::Vec::new(),
                    uint64_repeated: ::std::vec::Vec::new(),
                    sint32_repeated: ::std::vec::Vec::new(),
                    sint64_repeated: ::std::vec::Vec::new(),
                    fixed32_repeated: ::std::vec::Vec::new(),
                    fixed64_repeated: ::std::vec::Vec::new(),
                    sfixed32_repeated: ::std::vec::Vec::new(),
                    sfixed64_repeated: ::std::vec::Vec::new(),
                    bool_repeated: ::std::vec::Vec::new(),
                    string_repeated: ::protobuf::RepeatedField::new(),
                    bytes_repeated: ::protobuf::RepeatedField::new(),
                    test_enum_repeated: ::std::vec::Vec::new(),
                    test_message_repeated: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional double double_singular = 1;

    pub fn clear_double_singular(&mut self) {
        self.double_singular = None;
    }

    pub fn has_double_singular(&self) -> bool {
        self.double_singular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_double_singular(&mut self, v: f64) {
        self.double_singular = Some(v);
    }

    pub fn get_double_singular(&self) -> f64 {
        self.double_singular.unwrap_or(0.)
    }

    // optional float float_singular = 2;

    pub fn clear_float_singular(&mut self) {
        self.float_singular = None;
    }

    pub fn has_float_singular(&self) -> bool {
        self.float_singular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_float_singular(&mut self, v: f32) {
        self.float_singular = Some(v);
    }

    pub fn get_float_singular(&self) -> f32 {
        self.float_singular.unwrap_or(0.)
    }

    // optional int32 int32_singular = 3;

    pub fn clear_int32_singular(&mut self) {
        self.int32_singular = None;
    }

    pub fn has_int32_singular(&self) -> bool {
        self.int32_singular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_int32_singular(&mut self, v: i32) {
        self.int32_singular = Some(v);
    }

    pub fn get_int32_singular(&self) -> i32 {
        self.int32_singular.unwrap_or(0)
    }

    // optional int64 int64_singular = 4;

    pub fn clear_int64_singular(&mut self) {
        self.int64_singular = None;
    }

    pub fn has_int64_singular(&self) -> bool {
        self.int64_singular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_int64_singular(&mut self, v: i64) {
        self.int64_singular = Some(v);
    }

    pub fn get_int64_singular(&self) -> i64 {
        self.int64_singular.unwrap_or(0)
    }

    // optional uint32 uint32_singular = 5;

    pub fn clear_uint32_singular(&mut self) {
        self.uint32_singular = None;
    }

    pub fn has_uint32_singular(&self) -> bool {
        self.uint32_singular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uint32_singular(&mut self, v: u32) {
        self.uint32_singular = Some(v);
    }

    pub fn get_uint32_singular(&self) -> u32 {
        self.uint32_singular.unwrap_or(0)
    }

    // optional uint64 uint64_singular = 6;

    pub fn clear_uint64_singular(&mut self) {
        self.uint64_singular = None;
    }

    pub fn has_uint64_singular(&self) -> bool {
        self.uint64_singular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uint64_singular(&mut self, v: u64) {
        self.uint64_singular = Some(v);
    }

    pub fn get_uint64_singular(&self) -> u64 {
        self.uint64_singular.unwrap_or(0)
    }

    // optional sint32 sint32_singular = 7;

    pub fn clear_sint32_singular(&mut self) {
        self.sint32_singular = None;
    }

    pub fn has_sint32_singular(&self) -> bool {
        self.sint32_singular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sint32_singular(&mut self, v: i32) {
        self.sint32_singular = Some(v);
    }

    pub fn get_sint32_singular(&self) -> i32 {
        self.sint32_singular.unwrap_or(0)
    }

    // optional sint64 sint64_singular = 8;

    pub fn clear_sint64_singular(&mut self) {
        self.sint64_singular = None;
    }

    pub fn has_sint64_singular(&self) -> bool {
        self.sint64_singular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sint64_singular(&mut self, v: i64) {
        self.sint64_singular = Some(v);
    }

    pub fn get_sint64_singular(&self) -> i64 {
        self.sint64_singular.unwrap_or(0)
    }

    // optional fixed32 fixed32_singular = 9;

    pub fn clear_fixed32_singular(&mut self) {
        self.fixed32_singular = None;
    }

    pub fn has_fixed32_singular(&self) -> bool {
        self.fixed32_singular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fixed32_singular(&mut self, v: u32) {
        self.fixed32_singular = Some(v);
    }

    pub fn get_fixed32_singular(&self) -> u32 {
        self.fixed32_singular.unwrap_or(0)
    }

    // optional fixed64 fixed64_singular = 10;

    pub fn clear_fixed64_singular(&mut self) {
        self.fixed64_singular = None;
    }

    pub fn has_fixed64_singular(&self) -> bool {
        self.fixed64_singular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fixed64_singular(&mut self, v: u64) {
        self.fixed64_singular = Some(v);
    }

    pub fn get_fixed64_singular(&self) -> u64 {
        self.fixed64_singular.unwrap_or(0)
    }

    // optional sfixed32 sfixed32_singular = 11;

    pub fn clear_sfixed32_singular(&mut self) {
        self.sfixed32_singular = None;
    }

    pub fn has_sfixed32_singular(&self) -> bool {
        self.sfixed32_singular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sfixed32_singular(&mut self, v: i32) {
        self.sfixed32_singular = Some(v);
    }

    pub fn get_sfixed32_singular(&self) -> i32 {
        self.sfixed32_singular.unwrap_or(0)
    }

    // optional sfixed64 sfixed64_singular = 12;

    pub fn clear_sfixed64_singular(&mut self) {
        self.sfixed64_singular = None;
    }

    pub fn has_sfixed64_singular(&self) -> bool {
        self.sfixed64_singular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sfixed64_singular(&mut self, v: i64) {
        self.sfixed64_singular = Some(v);
    }

    pub fn get_sfixed64_singular(&self) -> i64 {
        self.sfixed64_singular.unwrap_or(0)
    }

    // optional bool bool_singular = 13;

    pub fn clear_bool_singular(&mut self) {
        self.bool_singular = None;
    }

    pub fn has_bool_singular(&self) -> bool {
        self.bool_singular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bool_singular(&mut self, v: bool) {
        self.bool_singular = Some(v);
    }

    pub fn get_bool_singular(&self) -> bool {
        self.bool_singular.unwrap_or(false)
    }

    // optional string string_singular = 14;

    pub fn clear_string_singular(&mut self) {
        self.string_singular.clear();
    }

    pub fn has_string_singular(&self) -> bool {
        self.string_singular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_singular(&mut self, v: ::std::string::String) {
        self.string_singular = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_singular(&'a mut self) -> &'a mut ::std::string::String {
        if self.string_singular.is_none() {
            self.string_singular.set_default();
        };
        self.string_singular.as_mut().unwrap()
    }

    pub fn get_string_singular(&'a self) -> &'a str {
        match self.string_singular.as_ref() {
            Some(ref v) => v.as_slice(),
            None => "",
        }
    }

    // optional bytes bytes_singular = 15;

    pub fn clear_bytes_singular(&mut self) {
        self.bytes_singular.clear();
    }

    pub fn has_bytes_singular(&self) -> bool {
        self.bytes_singular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytes_singular(&mut self, v: ::std::vec::Vec<u8>) {
        self.bytes_singular = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bytes_singular(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.bytes_singular.is_none() {
            self.bytes_singular.set_default();
        };
        self.bytes_singular.as_mut().unwrap()
    }

    pub fn get_bytes_singular(&'a self) -> &'a [u8] {
        match self.bytes_singular.as_ref() {
            Some(ref v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional .TestEnum test_enum_singular = 16;

    pub fn clear_test_enum_singular(&mut self) {
        self.test_enum_singular = None;
    }

    pub fn has_test_enum_singular(&self) -> bool {
        self.test_enum_singular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_test_enum_singular(&mut self, v: TestEnum) {
        self.test_enum_singular = Some(v);
    }

    pub fn get_test_enum_singular(&self) -> TestEnum {
        self.test_enum_singular.unwrap_or(DARK)
    }

    // optional .TestMessage test_message_singular = 17;

    pub fn clear_test_message_singular(&mut self) {
        self.test_message_singular.clear();
    }

    pub fn has_test_message_singular(&self) -> bool {
        self.test_message_singular.is_some()
    }

    // Param is passed by value, moved
    pub fn set_test_message_singular(&mut self, v: TestMessage) {
        self.test_message_singular = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_test_message_singular(&'a mut self) -> &'a mut TestMessage {
        if self.test_message_singular.is_none() {
            self.test_message_singular.set_default();
        };
        self.test_message_singular.as_mut().unwrap()
    }

    pub fn get_test_message_singular(&'a self) -> &'a TestMessage {
        self.test_message_singular.as_ref().unwrap_or_else(|| TestMessage::default_instance())
    }

    // repeated double double_repeated = 31;

    pub fn clear_double_repeated(&mut self) {
        self.double_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_double_repeated(&mut self, v: ::std::vec::Vec<f64>) {
        self.double_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_double_repeated(&'a mut self) -> &'a mut ::std::vec::Vec<f64> {
        &mut self.double_repeated
    }

    pub fn get_double_repeated(&'a self) -> &'a [f64] {
        self.double_repeated.as_slice()
    }

    pub fn add_double_repeated(&mut self, v: f64) {
        self.double_repeated.push(v);
    }

    // repeated float float_repeated = 32;

    pub fn clear_float_repeated(&mut self) {
        self.float_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_float_repeated(&mut self, v: ::std::vec::Vec<f32>) {
        self.float_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_float_repeated(&'a mut self) -> &'a mut ::std::vec::Vec<f32> {
        &mut self.float_repeated
    }

    pub fn get_float_repeated(&'a self) -> &'a [f32] {
        self.float_repeated.as_slice()
    }

    pub fn add_float_repeated(&mut self, v: f32) {
        self.float_repeated.push(v);
    }

    // repeated int32 int32_repeated = 33;

    pub fn clear_int32_repeated(&mut self) {
        self.int32_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_int32_repeated(&mut self, v: ::std::vec::Vec<i32>) {
        self.int32_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_int32_repeated(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.int32_repeated
    }

    pub fn get_int32_repeated(&'a self) -> &'a [i32] {
        self.int32_repeated.as_slice()
    }

    pub fn add_int32_repeated(&mut self, v: i32) {
        self.int32_repeated.push(v);
    }

    // repeated int64 int64_repeated = 34;

    pub fn clear_int64_repeated(&mut self) {
        self.int64_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_int64_repeated(&mut self, v: ::std::vec::Vec<i64>) {
        self.int64_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_int64_repeated(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.int64_repeated
    }

    pub fn get_int64_repeated(&'a self) -> &'a [i64] {
        self.int64_repeated.as_slice()
    }

    pub fn add_int64_repeated(&mut self, v: i64) {
        self.int64_repeated.push(v);
    }

    // repeated uint32 uint32_repeated = 35;

    pub fn clear_uint32_repeated(&mut self) {
        self.uint32_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_uint32_repeated(&mut self, v: ::std::vec::Vec<u32>) {
        self.uint32_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uint32_repeated(&'a mut self) -> &'a mut ::std::vec::Vec<u32> {
        &mut self.uint32_repeated
    }

    pub fn get_uint32_repeated(&'a self) -> &'a [u32] {
        self.uint32_repeated.as_slice()
    }

    pub fn add_uint32_repeated(&mut self, v: u32) {
        self.uint32_repeated.push(v);
    }

    // repeated uint64 uint64_repeated = 36;

    pub fn clear_uint64_repeated(&mut self) {
        self.uint64_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_uint64_repeated(&mut self, v: ::std::vec::Vec<u64>) {
        self.uint64_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uint64_repeated(&'a mut self) -> &'a mut ::std::vec::Vec<u64> {
        &mut self.uint64_repeated
    }

    pub fn get_uint64_repeated(&'a self) -> &'a [u64] {
        self.uint64_repeated.as_slice()
    }

    pub fn add_uint64_repeated(&mut self, v: u64) {
        self.uint64_repeated.push(v);
    }

    // repeated sint32 sint32_repeated = 37;

    pub fn clear_sint32_repeated(&mut self) {
        self.sint32_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_sint32_repeated(&mut self, v: ::std::vec::Vec<i32>) {
        self.sint32_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sint32_repeated(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.sint32_repeated
    }

    pub fn get_sint32_repeated(&'a self) -> &'a [i32] {
        self.sint32_repeated.as_slice()
    }

    pub fn add_sint32_repeated(&mut self, v: i32) {
        self.sint32_repeated.push(v);
    }

    // repeated sint64 sint64_repeated = 38;

    pub fn clear_sint64_repeated(&mut self) {
        self.sint64_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_sint64_repeated(&mut self, v: ::std::vec::Vec<i64>) {
        self.sint64_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sint64_repeated(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.sint64_repeated
    }

    pub fn get_sint64_repeated(&'a self) -> &'a [i64] {
        self.sint64_repeated.as_slice()
    }

    pub fn add_sint64_repeated(&mut self, v: i64) {
        self.sint64_repeated.push(v);
    }

    // repeated fixed32 fixed32_repeated = 39;

    pub fn clear_fixed32_repeated(&mut self) {
        self.fixed32_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_fixed32_repeated(&mut self, v: ::std::vec::Vec<u32>) {
        self.fixed32_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fixed32_repeated(&'a mut self) -> &'a mut ::std::vec::Vec<u32> {
        &mut self.fixed32_repeated
    }

    pub fn get_fixed32_repeated(&'a self) -> &'a [u32] {
        self.fixed32_repeated.as_slice()
    }

    pub fn add_fixed32_repeated(&mut self, v: u32) {
        self.fixed32_repeated.push(v);
    }

    // repeated fixed64 fixed64_repeated = 40;

    pub fn clear_fixed64_repeated(&mut self) {
        self.fixed64_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_fixed64_repeated(&mut self, v: ::std::vec::Vec<u64>) {
        self.fixed64_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fixed64_repeated(&'a mut self) -> &'a mut ::std::vec::Vec<u64> {
        &mut self.fixed64_repeated
    }

    pub fn get_fixed64_repeated(&'a self) -> &'a [u64] {
        self.fixed64_repeated.as_slice()
    }

    pub fn add_fixed64_repeated(&mut self, v: u64) {
        self.fixed64_repeated.push(v);
    }

    // repeated sfixed32 sfixed32_repeated = 41;

    pub fn clear_sfixed32_repeated(&mut self) {
        self.sfixed32_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_sfixed32_repeated(&mut self, v: ::std::vec::Vec<i32>) {
        self.sfixed32_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sfixed32_repeated(&'a mut self) -> &'a mut ::std::vec::Vec<i32> {
        &mut self.sfixed32_repeated
    }

    pub fn get_sfixed32_repeated(&'a self) -> &'a [i32] {
        self.sfixed32_repeated.as_slice()
    }

    pub fn add_sfixed32_repeated(&mut self, v: i32) {
        self.sfixed32_repeated.push(v);
    }

    // repeated sfixed64 sfixed64_repeated = 42;

    pub fn clear_sfixed64_repeated(&mut self) {
        self.sfixed64_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_sfixed64_repeated(&mut self, v: ::std::vec::Vec<i64>) {
        self.sfixed64_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sfixed64_repeated(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.sfixed64_repeated
    }

    pub fn get_sfixed64_repeated(&'a self) -> &'a [i64] {
        self.sfixed64_repeated.as_slice()
    }

    pub fn add_sfixed64_repeated(&mut self, v: i64) {
        self.sfixed64_repeated.push(v);
    }

    // repeated bool bool_repeated = 43;

    pub fn clear_bool_repeated(&mut self) {
        self.bool_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_bool_repeated(&mut self, v: ::std::vec::Vec<bool>) {
        self.bool_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bool_repeated(&'a mut self) -> &'a mut ::std::vec::Vec<bool> {
        &mut self.bool_repeated
    }

    pub fn get_bool_repeated(&'a self) -> &'a [bool] {
        self.bool_repeated.as_slice()
    }

    pub fn add_bool_repeated(&mut self, v: bool) {
        self.bool_repeated.push(v);
    }

    // repeated string string_repeated = 44;

    pub fn clear_string_repeated(&mut self) {
        self.string_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_string_repeated(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.string_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_string_repeated(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.string_repeated
    }

    pub fn get_string_repeated(&'a self) -> &'a [::std::string::String] {
        self.string_repeated.as_slice()
    }

    pub fn add_string_repeated(&mut self, v: ::std::string::String) {
        self.string_repeated.push(v);
    }

    // repeated bytes bytes_repeated = 45;

    pub fn clear_bytes_repeated(&mut self) {
        self.bytes_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_bytes_repeated(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.bytes_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bytes_repeated(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.bytes_repeated
    }

    pub fn get_bytes_repeated(&'a self) -> &'a [::std::vec::Vec<u8>] {
        self.bytes_repeated.as_slice()
    }

    pub fn add_bytes_repeated(&mut self, v: ::std::vec::Vec<u8>) {
        self.bytes_repeated.push(v);
    }

    // repeated .TestEnum test_enum_repeated = 46;

    pub fn clear_test_enum_repeated(&mut self) {
        self.test_enum_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_test_enum_repeated(&mut self, v: ::std::vec::Vec<TestEnum>) {
        self.test_enum_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_test_enum_repeated(&'a mut self) -> &'a mut ::std::vec::Vec<TestEnum> {
        &mut self.test_enum_repeated
    }

    pub fn get_test_enum_repeated(&'a self) -> &'a [TestEnum] {
        self.test_enum_repeated.as_slice()
    }

    pub fn add_test_enum_repeated(&mut self, v: TestEnum) {
        self.test_enum_repeated.push(v);
    }

    // repeated .TestMessage test_message_repeated = 47;

    pub fn clear_test_message_repeated(&mut self) {
        self.test_message_repeated.clear();
    }

    // Param is passed by value, moved
    pub fn set_test_message_repeated(&mut self, v: ::protobuf::RepeatedField<TestMessage>) {
        self.test_message_repeated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_test_message_repeated(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TestMessage> {
        &mut self.test_message_repeated
    }

    pub fn get_test_message_repeated(&'a self) -> &'a [TestMessage] {
        self.test_message_repeated.as_slice()
    }

    pub fn add_test_message_repeated(&mut self, v: TestMessage) {
        self.test_message_repeated.push(v);
    }
}

impl ::protobuf::Message for TestTypes {
    fn new() -> TestTypes {
        TestTypes::new()
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
                    self.double_singular = Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_float());
                    self.float_singular = Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.int32_singular = Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.int64_singular = Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.uint32_singular = Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.uint64_singular = Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sint32());
                    self.sint32_singular = Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sint64());
                    self.sint64_singular = Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_fixed32());
                    self.fixed32_singular = Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.fixed64_singular = Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sfixed32());
                    self.sfixed32_singular = Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sfixed64());
                    self.sfixed64_singular = Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.bool_singular = Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.string_singular.set_default();
                    try!(is.read_string_into(tmp))
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.bytes_singular.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = TestEnum::new(try!(is.read_int32()));
                    self.test_enum_singular = Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.test_message_singular.set_default();
                    try!(is.merge_message(tmp))
                },
                31 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.double_repeated.push(try!(is.read_double()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.double_repeated.push(try!(is.read_double()));
                    }
                },
                32 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.float_repeated.push(try!(is.read_float()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.float_repeated.push(try!(is.read_float()));
                    }
                },
                33 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.int32_repeated.push(try!(is.read_int32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.int32_repeated.push(try!(is.read_int32()));
                    }
                },
                34 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.int64_repeated.push(try!(is.read_int64()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.int64_repeated.push(try!(is.read_int64()));
                    }
                },
                35 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.uint32_repeated.push(try!(is.read_uint32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.uint32_repeated.push(try!(is.read_uint32()));
                    }
                },
                36 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.uint64_repeated.push(try!(is.read_uint64()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.uint64_repeated.push(try!(is.read_uint64()));
                    }
                },
                37 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.sint32_repeated.push(try!(is.read_sint32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.sint32_repeated.push(try!(is.read_sint32()));
                    }
                },
                38 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.sint64_repeated.push(try!(is.read_sint64()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.sint64_repeated.push(try!(is.read_sint64()));
                    }
                },
                39 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.fixed32_repeated.push(try!(is.read_fixed32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.fixed32_repeated.push(try!(is.read_fixed32()));
                    }
                },
                40 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.fixed64_repeated.push(try!(is.read_fixed64()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.fixed64_repeated.push(try!(is.read_fixed64()));
                    }
                },
                41 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.sfixed32_repeated.push(try!(is.read_sfixed32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.sfixed32_repeated.push(try!(is.read_sfixed32()));
                    }
                },
                42 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.sfixed64_repeated.push(try!(is.read_sfixed64()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.sfixed64_repeated.push(try!(is.read_sfixed64()));
                    }
                },
                43 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.bool_repeated.push(try!(is.read_bool()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.bool_repeated.push(try!(is.read_bool()));
                    }
                },
                44 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.string_repeated.push_default();
                    try!(is.read_string_into(tmp))
                },
                45 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.bytes_repeated.push_default();
                    try!(is.read_bytes_into(tmp))
                },
                46 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.test_enum_repeated.push(TestEnum::new(try!(is.read_int32())));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                        };
                        self.test_enum_repeated.push(TestEnum::new(try!(is.read_int32())));
                    }
                },
                47 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.test_message_repeated.push_default();
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
        if self.double_singular.is_some() {
            my_size += 9;
        };
        if self.float_singular.is_some() {
            my_size += 5;
        };
        for value in self.int32_singular.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.int64_singular.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.uint32_singular.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.uint64_singular.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.sint32_singular.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.sint64_singular.iter() {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.fixed32_singular.is_some() {
            my_size += 5;
        };
        if self.fixed64_singular.is_some() {
            my_size += 9;
        };
        if self.sfixed32_singular.is_some() {
            my_size += 5;
        };
        if self.sfixed64_singular.is_some() {
            my_size += 9;
        };
        if self.bool_singular.is_some() {
            my_size += 2;
        };
        for value in self.string_singular.iter() {
            my_size += ::protobuf::rt::string_size(14, value.as_slice());
        };
        for value in self.bytes_singular.iter() {
            my_size += ::protobuf::rt::bytes_size(15, value.as_slice());
        };
        for value in self.test_enum_singular.iter() {
            my_size += ::protobuf::rt::enum_size(16, *value);
        };
        for value in self.test_message_singular.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += 10 * self.double_repeated.len() as u32;
        my_size += 6 * self.float_repeated.len() as u32;
        for value in self.int32_repeated.iter() {
            my_size += ::protobuf::rt::value_size(33, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.int64_repeated.iter() {
            my_size += ::protobuf::rt::value_size(34, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.uint32_repeated.iter() {
            my_size += ::protobuf::rt::value_size(35, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.uint64_repeated.iter() {
            my_size += ::protobuf::rt::value_size(36, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.sint32_repeated.iter() {
            my_size += ::protobuf::rt::value_size(37, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.sint64_repeated.iter() {
            my_size += ::protobuf::rt::value_size(38, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += 6 * self.fixed32_repeated.len() as u32;
        my_size += 10 * self.fixed64_repeated.len() as u32;
        my_size += 6 * self.sfixed32_repeated.len() as u32;
        my_size += 10 * self.sfixed64_repeated.len() as u32;
        my_size += 3 * self.bool_repeated.len() as u32;
        for value in self.string_repeated.iter() {
            my_size += ::protobuf::rt::string_size(44, value.as_slice());
        };
        for value in self.bytes_repeated.iter() {
            my_size += ::protobuf::rt::bytes_size(45, value.as_slice());
        };
        for value in self.test_enum_repeated.iter() {
            my_size += ::protobuf::rt::enum_size(46, *value);
        };
        for value in self.test_message_repeated.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.double_singular {
            Some(ref v) => {
                try!(os.write_double(1, *v));
            },
            None => {},
        };
        match self.float_singular {
            Some(ref v) => {
                try!(os.write_float(2, *v));
            },
            None => {},
        };
        match self.int32_singular {
            Some(ref v) => {
                try!(os.write_int32(3, *v));
            },
            None => {},
        };
        match self.int64_singular {
            Some(ref v) => {
                try!(os.write_int64(4, *v));
            },
            None => {},
        };
        match self.uint32_singular {
            Some(ref v) => {
                try!(os.write_uint32(5, *v));
            },
            None => {},
        };
        match self.uint64_singular {
            Some(ref v) => {
                try!(os.write_uint64(6, *v));
            },
            None => {},
        };
        match self.sint32_singular {
            Some(ref v) => {
                try!(os.write_sint32(7, *v));
            },
            None => {},
        };
        match self.sint64_singular {
            Some(ref v) => {
                try!(os.write_sint64(8, *v));
            },
            None => {},
        };
        match self.fixed32_singular {
            Some(ref v) => {
                try!(os.write_fixed32(9, *v));
            },
            None => {},
        };
        match self.fixed64_singular {
            Some(ref v) => {
                try!(os.write_fixed64(10, *v));
            },
            None => {},
        };
        match self.sfixed32_singular {
            Some(ref v) => {
                try!(os.write_sfixed32(11, *v));
            },
            None => {},
        };
        match self.sfixed64_singular {
            Some(ref v) => {
                try!(os.write_sfixed64(12, *v));
            },
            None => {},
        };
        match self.bool_singular {
            Some(ref v) => {
                try!(os.write_bool(13, *v));
            },
            None => {},
        };
        match self.string_singular.as_ref() {
            Some(ref v) => {
                try!(os.write_string(14, v.as_slice()));
            },
            None => {},
        };
        match self.bytes_singular.as_ref() {
            Some(ref v) => {
                try!(os.write_bytes(15, v.as_slice()));
            },
            None => {},
        };
        match self.test_enum_singular {
            Some(ref v) => {
                try!(os.write_enum(16, *v as i32));
            },
            None => {},
        };
        match self.test_message_singular.as_ref() {
            Some(ref v) => {
                try!(os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        for v in self.double_repeated.iter() {
            try!(os.write_double(31, *v));
        };
        for v in self.float_repeated.iter() {
            try!(os.write_float(32, *v));
        };
        for v in self.int32_repeated.iter() {
            try!(os.write_int32(33, *v));
        };
        for v in self.int64_repeated.iter() {
            try!(os.write_int64(34, *v));
        };
        for v in self.uint32_repeated.iter() {
            try!(os.write_uint32(35, *v));
        };
        for v in self.uint64_repeated.iter() {
            try!(os.write_uint64(36, *v));
        };
        for v in self.sint32_repeated.iter() {
            try!(os.write_sint32(37, *v));
        };
        for v in self.sint64_repeated.iter() {
            try!(os.write_sint64(38, *v));
        };
        for v in self.fixed32_repeated.iter() {
            try!(os.write_fixed32(39, *v));
        };
        for v in self.fixed64_repeated.iter() {
            try!(os.write_fixed64(40, *v));
        };
        for v in self.sfixed32_repeated.iter() {
            try!(os.write_sfixed32(41, *v));
        };
        for v in self.sfixed64_repeated.iter() {
            try!(os.write_sfixed64(42, *v));
        };
        for v in self.bool_repeated.iter() {
            try!(os.write_bool(43, *v));
        };
        for v in self.string_repeated.iter() {
            try!(os.write_string(44, v.as_slice()));
        };
        for v in self.bytes_repeated.iter() {
            try!(os.write_bytes(45, v.as_slice()));
        };
        for v in self.test_enum_repeated.iter() {
            try!(os.write_enum(46, *v as i32));
        };
        for v in self.test_message_repeated.iter() {
            try!(os.write_tag(47, ::protobuf::wire_format::WireTypeLengthDelimited));
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
    fn descriptor_static(_: ::std::option::Option<TestTypes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TestTypes>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_double_singular_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_float_singular_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_int32_singular_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_int64_singular_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_uint32_singular_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_uint64_singular_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_sint32_singular_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_sint64_singular_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_fixed32_singular_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_fixed64_singular_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_sfixed32_singular_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_sfixed64_singular_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_bool_singular_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_string_singular_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_bytes_singular_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_test_enum_singular_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_test_message_singular_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_double_repeated_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_float_repeated_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_int32_repeated_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_int64_repeated_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_uint32_repeated_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_uint64_repeated_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_sint32_repeated_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_sint64_repeated_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_fixed32_repeated_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_fixed64_repeated_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_sfixed32_repeated_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_sfixed64_repeated_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_bool_repeated_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_string_repeated_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_bytes_repeated_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_test_enum_repeated_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                fields.push(unsafe { ::std::mem::transmute(&TestTypes_test_message_repeated_acc as &'static ::protobuf::reflect::FieldAccessor<TestTypes>) });
                ::protobuf::reflect::MessageDescriptor::new::<TestTypes>(
                    "TestTypes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestTypes>()
    }
}

impl ::protobuf::Clear for TestTypes {
    fn clear(&mut self) {
        self.clear_double_singular();
        self.clear_float_singular();
        self.clear_int32_singular();
        self.clear_int64_singular();
        self.clear_uint32_singular();
        self.clear_uint64_singular();
        self.clear_sint32_singular();
        self.clear_sint64_singular();
        self.clear_fixed32_singular();
        self.clear_fixed64_singular();
        self.clear_sfixed32_singular();
        self.clear_sfixed64_singular();
        self.clear_bool_singular();
        self.clear_string_singular();
        self.clear_bytes_singular();
        self.clear_test_enum_singular();
        self.clear_test_message_singular();
        self.clear_double_repeated();
        self.clear_float_repeated();
        self.clear_int32_repeated();
        self.clear_int64_repeated();
        self.clear_uint32_repeated();
        self.clear_uint64_repeated();
        self.clear_sint32_repeated();
        self.clear_sint64_repeated();
        self.clear_fixed32_repeated();
        self.clear_fixed64_repeated();
        self.clear_sfixed32_repeated();
        self.clear_sfixed64_repeated();
        self.clear_bool_repeated();
        self.clear_string_repeated();
        self.clear_bytes_repeated();
        self.clear_test_enum_repeated();
        self.clear_test_message_repeated();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for TestTypes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TestTypes_double_singular_acc_type;
static TestTypes_double_singular_acc: TestTypes_double_singular_acc_type = TestTypes_double_singular_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_double_singular_acc_type {
    fn name(&self) -> &'static str {
        "double_singular"
    }

    fn has_field(&self, m: &TestTypes) -> bool {
        m.has_double_singular()
    }

    fn get_f64(&self, m: &TestTypes) -> f64 {
        m.get_double_singular()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_float_singular_acc_type;
static TestTypes_float_singular_acc: TestTypes_float_singular_acc_type = TestTypes_float_singular_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_float_singular_acc_type {
    fn name(&self) -> &'static str {
        "float_singular"
    }

    fn has_field(&self, m: &TestTypes) -> bool {
        m.has_float_singular()
    }

    fn get_f32(&self, m: &TestTypes) -> f32 {
        m.get_float_singular()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_int32_singular_acc_type;
static TestTypes_int32_singular_acc: TestTypes_int32_singular_acc_type = TestTypes_int32_singular_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_int32_singular_acc_type {
    fn name(&self) -> &'static str {
        "int32_singular"
    }

    fn has_field(&self, m: &TestTypes) -> bool {
        m.has_int32_singular()
    }

    fn get_i32(&self, m: &TestTypes) -> i32 {
        m.get_int32_singular()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_int64_singular_acc_type;
static TestTypes_int64_singular_acc: TestTypes_int64_singular_acc_type = TestTypes_int64_singular_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_int64_singular_acc_type {
    fn name(&self) -> &'static str {
        "int64_singular"
    }

    fn has_field(&self, m: &TestTypes) -> bool {
        m.has_int64_singular()
    }

    fn get_i64(&self, m: &TestTypes) -> i64 {
        m.get_int64_singular()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_uint32_singular_acc_type;
static TestTypes_uint32_singular_acc: TestTypes_uint32_singular_acc_type = TestTypes_uint32_singular_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_uint32_singular_acc_type {
    fn name(&self) -> &'static str {
        "uint32_singular"
    }

    fn has_field(&self, m: &TestTypes) -> bool {
        m.has_uint32_singular()
    }

    fn get_u32(&self, m: &TestTypes) -> u32 {
        m.get_uint32_singular()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_uint64_singular_acc_type;
static TestTypes_uint64_singular_acc: TestTypes_uint64_singular_acc_type = TestTypes_uint64_singular_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_uint64_singular_acc_type {
    fn name(&self) -> &'static str {
        "uint64_singular"
    }

    fn has_field(&self, m: &TestTypes) -> bool {
        m.has_uint64_singular()
    }

    fn get_u64(&self, m: &TestTypes) -> u64 {
        m.get_uint64_singular()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_sint32_singular_acc_type;
static TestTypes_sint32_singular_acc: TestTypes_sint32_singular_acc_type = TestTypes_sint32_singular_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_sint32_singular_acc_type {
    fn name(&self) -> &'static str {
        "sint32_singular"
    }

    fn has_field(&self, m: &TestTypes) -> bool {
        m.has_sint32_singular()
    }

    fn get_i32(&self, m: &TestTypes) -> i32 {
        m.get_sint32_singular()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_sint64_singular_acc_type;
static TestTypes_sint64_singular_acc: TestTypes_sint64_singular_acc_type = TestTypes_sint64_singular_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_sint64_singular_acc_type {
    fn name(&self) -> &'static str {
        "sint64_singular"
    }

    fn has_field(&self, m: &TestTypes) -> bool {
        m.has_sint64_singular()
    }

    fn get_i64(&self, m: &TestTypes) -> i64 {
        m.get_sint64_singular()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_fixed32_singular_acc_type;
static TestTypes_fixed32_singular_acc: TestTypes_fixed32_singular_acc_type = TestTypes_fixed32_singular_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_fixed32_singular_acc_type {
    fn name(&self) -> &'static str {
        "fixed32_singular"
    }

    fn has_field(&self, m: &TestTypes) -> bool {
        m.has_fixed32_singular()
    }

    fn get_u32(&self, m: &TestTypes) -> u32 {
        m.get_fixed32_singular()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_fixed64_singular_acc_type;
static TestTypes_fixed64_singular_acc: TestTypes_fixed64_singular_acc_type = TestTypes_fixed64_singular_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_fixed64_singular_acc_type {
    fn name(&self) -> &'static str {
        "fixed64_singular"
    }

    fn has_field(&self, m: &TestTypes) -> bool {
        m.has_fixed64_singular()
    }

    fn get_u64(&self, m: &TestTypes) -> u64 {
        m.get_fixed64_singular()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_sfixed32_singular_acc_type;
static TestTypes_sfixed32_singular_acc: TestTypes_sfixed32_singular_acc_type = TestTypes_sfixed32_singular_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_sfixed32_singular_acc_type {
    fn name(&self) -> &'static str {
        "sfixed32_singular"
    }

    fn has_field(&self, m: &TestTypes) -> bool {
        m.has_sfixed32_singular()
    }

    fn get_i32(&self, m: &TestTypes) -> i32 {
        m.get_sfixed32_singular()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_sfixed64_singular_acc_type;
static TestTypes_sfixed64_singular_acc: TestTypes_sfixed64_singular_acc_type = TestTypes_sfixed64_singular_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_sfixed64_singular_acc_type {
    fn name(&self) -> &'static str {
        "sfixed64_singular"
    }

    fn has_field(&self, m: &TestTypes) -> bool {
        m.has_sfixed64_singular()
    }

    fn get_i64(&self, m: &TestTypes) -> i64 {
        m.get_sfixed64_singular()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_bool_singular_acc_type;
static TestTypes_bool_singular_acc: TestTypes_bool_singular_acc_type = TestTypes_bool_singular_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_bool_singular_acc_type {
    fn name(&self) -> &'static str {
        "bool_singular"
    }

    fn has_field(&self, m: &TestTypes) -> bool {
        m.has_bool_singular()
    }

    fn get_bool(&self, m: &TestTypes) -> bool {
        m.get_bool_singular()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_string_singular_acc_type;
static TestTypes_string_singular_acc: TestTypes_string_singular_acc_type = TestTypes_string_singular_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_string_singular_acc_type {
    fn name(&self) -> &'static str {
        "string_singular"
    }

    fn has_field(&self, m: &TestTypes) -> bool {
        m.has_string_singular()
    }

    fn get_str<'a>(&self, m: &'a TestTypes) -> &'a str {
        m.get_string_singular()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_bytes_singular_acc_type;
static TestTypes_bytes_singular_acc: TestTypes_bytes_singular_acc_type = TestTypes_bytes_singular_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_bytes_singular_acc_type {
    fn name(&self) -> &'static str {
        "bytes_singular"
    }

    fn has_field(&self, m: &TestTypes) -> bool {
        m.has_bytes_singular()
    }

    fn get_bytes<'a>(&self, m: &'a TestTypes) -> &'a [u8] {
        m.get_bytes_singular()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_test_enum_singular_acc_type;
static TestTypes_test_enum_singular_acc: TestTypes_test_enum_singular_acc_type = TestTypes_test_enum_singular_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_test_enum_singular_acc_type {
    fn name(&self) -> &'static str {
        "test_enum_singular"
    }

    fn has_field(&self, m: &TestTypes) -> bool {
        m.has_test_enum_singular()
    }

    fn get_enum<'a>(&self, m: &TestTypes) -> &'static ::protobuf::reflect::EnumValueDescriptor {
        use protobuf::{ProtobufEnum};
        m.get_test_enum_singular().descriptor()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_test_message_singular_acc_type;
static TestTypes_test_message_singular_acc: TestTypes_test_message_singular_acc_type = TestTypes_test_message_singular_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_test_message_singular_acc_type {
    fn name(&self) -> &'static str {
        "test_message_singular"
    }

    fn has_field(&self, m: &TestTypes) -> bool {
        m.has_test_message_singular()
    }

    fn get_message<'a>(&self, m: &'a TestTypes) -> &'a ::protobuf::Message {
        m.get_test_message_singular() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_double_repeated_acc_type;
static TestTypes_double_repeated_acc: TestTypes_double_repeated_acc_type = TestTypes_double_repeated_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_double_repeated_acc_type {
    fn name(&self) -> &'static str {
        "double_repeated"
    }

    fn len_field(&self, m: &TestTypes) -> uint {
        m.get_double_repeated().len()
    }

    fn get_rep_f64<'a>(&self, m: &'a TestTypes) -> &'a [f64] {
        m.get_double_repeated()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_float_repeated_acc_type;
static TestTypes_float_repeated_acc: TestTypes_float_repeated_acc_type = TestTypes_float_repeated_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_float_repeated_acc_type {
    fn name(&self) -> &'static str {
        "float_repeated"
    }

    fn len_field(&self, m: &TestTypes) -> uint {
        m.get_float_repeated().len()
    }

    fn get_rep_f32<'a>(&self, m: &'a TestTypes) -> &'a [f32] {
        m.get_float_repeated()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_int32_repeated_acc_type;
static TestTypes_int32_repeated_acc: TestTypes_int32_repeated_acc_type = TestTypes_int32_repeated_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_int32_repeated_acc_type {
    fn name(&self) -> &'static str {
        "int32_repeated"
    }

    fn len_field(&self, m: &TestTypes) -> uint {
        m.get_int32_repeated().len()
    }

    fn get_rep_i32<'a>(&self, m: &'a TestTypes) -> &'a [i32] {
        m.get_int32_repeated()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_int64_repeated_acc_type;
static TestTypes_int64_repeated_acc: TestTypes_int64_repeated_acc_type = TestTypes_int64_repeated_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_int64_repeated_acc_type {
    fn name(&self) -> &'static str {
        "int64_repeated"
    }

    fn len_field(&self, m: &TestTypes) -> uint {
        m.get_int64_repeated().len()
    }

    fn get_rep_i64<'a>(&self, m: &'a TestTypes) -> &'a [i64] {
        m.get_int64_repeated()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_uint32_repeated_acc_type;
static TestTypes_uint32_repeated_acc: TestTypes_uint32_repeated_acc_type = TestTypes_uint32_repeated_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_uint32_repeated_acc_type {
    fn name(&self) -> &'static str {
        "uint32_repeated"
    }

    fn len_field(&self, m: &TestTypes) -> uint {
        m.get_uint32_repeated().len()
    }

    fn get_rep_u32<'a>(&self, m: &'a TestTypes) -> &'a [u32] {
        m.get_uint32_repeated()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_uint64_repeated_acc_type;
static TestTypes_uint64_repeated_acc: TestTypes_uint64_repeated_acc_type = TestTypes_uint64_repeated_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_uint64_repeated_acc_type {
    fn name(&self) -> &'static str {
        "uint64_repeated"
    }

    fn len_field(&self, m: &TestTypes) -> uint {
        m.get_uint64_repeated().len()
    }

    fn get_rep_u64<'a>(&self, m: &'a TestTypes) -> &'a [u64] {
        m.get_uint64_repeated()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_sint32_repeated_acc_type;
static TestTypes_sint32_repeated_acc: TestTypes_sint32_repeated_acc_type = TestTypes_sint32_repeated_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_sint32_repeated_acc_type {
    fn name(&self) -> &'static str {
        "sint32_repeated"
    }

    fn len_field(&self, m: &TestTypes) -> uint {
        m.get_sint32_repeated().len()
    }

    fn get_rep_i32<'a>(&self, m: &'a TestTypes) -> &'a [i32] {
        m.get_sint32_repeated()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_sint64_repeated_acc_type;
static TestTypes_sint64_repeated_acc: TestTypes_sint64_repeated_acc_type = TestTypes_sint64_repeated_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_sint64_repeated_acc_type {
    fn name(&self) -> &'static str {
        "sint64_repeated"
    }

    fn len_field(&self, m: &TestTypes) -> uint {
        m.get_sint64_repeated().len()
    }

    fn get_rep_i64<'a>(&self, m: &'a TestTypes) -> &'a [i64] {
        m.get_sint64_repeated()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_fixed32_repeated_acc_type;
static TestTypes_fixed32_repeated_acc: TestTypes_fixed32_repeated_acc_type = TestTypes_fixed32_repeated_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_fixed32_repeated_acc_type {
    fn name(&self) -> &'static str {
        "fixed32_repeated"
    }

    fn len_field(&self, m: &TestTypes) -> uint {
        m.get_fixed32_repeated().len()
    }

    fn get_rep_u32<'a>(&self, m: &'a TestTypes) -> &'a [u32] {
        m.get_fixed32_repeated()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_fixed64_repeated_acc_type;
static TestTypes_fixed64_repeated_acc: TestTypes_fixed64_repeated_acc_type = TestTypes_fixed64_repeated_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_fixed64_repeated_acc_type {
    fn name(&self) -> &'static str {
        "fixed64_repeated"
    }

    fn len_field(&self, m: &TestTypes) -> uint {
        m.get_fixed64_repeated().len()
    }

    fn get_rep_u64<'a>(&self, m: &'a TestTypes) -> &'a [u64] {
        m.get_fixed64_repeated()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_sfixed32_repeated_acc_type;
static TestTypes_sfixed32_repeated_acc: TestTypes_sfixed32_repeated_acc_type = TestTypes_sfixed32_repeated_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_sfixed32_repeated_acc_type {
    fn name(&self) -> &'static str {
        "sfixed32_repeated"
    }

    fn len_field(&self, m: &TestTypes) -> uint {
        m.get_sfixed32_repeated().len()
    }

    fn get_rep_i32<'a>(&self, m: &'a TestTypes) -> &'a [i32] {
        m.get_sfixed32_repeated()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_sfixed64_repeated_acc_type;
static TestTypes_sfixed64_repeated_acc: TestTypes_sfixed64_repeated_acc_type = TestTypes_sfixed64_repeated_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_sfixed64_repeated_acc_type {
    fn name(&self) -> &'static str {
        "sfixed64_repeated"
    }

    fn len_field(&self, m: &TestTypes) -> uint {
        m.get_sfixed64_repeated().len()
    }

    fn get_rep_i64<'a>(&self, m: &'a TestTypes) -> &'a [i64] {
        m.get_sfixed64_repeated()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_bool_repeated_acc_type;
static TestTypes_bool_repeated_acc: TestTypes_bool_repeated_acc_type = TestTypes_bool_repeated_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_bool_repeated_acc_type {
    fn name(&self) -> &'static str {
        "bool_repeated"
    }

    fn len_field(&self, m: &TestTypes) -> uint {
        m.get_bool_repeated().len()
    }

    fn get_rep_bool<'a>(&self, m: &'a TestTypes) -> &'a [bool] {
        m.get_bool_repeated()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_string_repeated_acc_type;
static TestTypes_string_repeated_acc: TestTypes_string_repeated_acc_type = TestTypes_string_repeated_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_string_repeated_acc_type {
    fn name(&self) -> &'static str {
        "string_repeated"
    }

    fn len_field(&self, m: &TestTypes) -> uint {
        m.get_string_repeated().len()
    }

    fn get_rep_str<'a>(&self, m: &'a TestTypes) -> &'a [::std::string::String] {
        m.get_string_repeated()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_bytes_repeated_acc_type;
static TestTypes_bytes_repeated_acc: TestTypes_bytes_repeated_acc_type = TestTypes_bytes_repeated_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_bytes_repeated_acc_type {
    fn name(&self) -> &'static str {
        "bytes_repeated"
    }

    fn len_field(&self, m: &TestTypes) -> uint {
        m.get_bytes_repeated().len()
    }

    fn get_rep_bytes<'a>(&self, m: &'a TestTypes) -> &'a [::std::vec::Vec<u8>] {
        m.get_bytes_repeated()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_test_enum_repeated_acc_type;
static TestTypes_test_enum_repeated_acc: TestTypes_test_enum_repeated_acc_type = TestTypes_test_enum_repeated_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_test_enum_repeated_acc_type {
    fn name(&self) -> &'static str {
        "test_enum_repeated"
    }

    fn len_field(&self, m: &TestTypes) -> uint {
        m.get_test_enum_repeated().len()
    }

    fn get_rep_enum_item<'a>(&self, m: &TestTypes, index: uint) -> &'static ::protobuf::reflect::EnumValueDescriptor {
        use protobuf::{ProtobufEnum};
        m.get_test_enum_repeated()[index].descriptor()
    }
}

#[allow(non_camel_case_types)]
struct TestTypes_test_message_repeated_acc_type;
static TestTypes_test_message_repeated_acc: TestTypes_test_message_repeated_acc_type = TestTypes_test_message_repeated_acc_type;

impl ::protobuf::reflect::FieldAccessor<TestTypes> for TestTypes_test_message_repeated_acc_type {
    fn name(&self) -> &'static str {
        "test_message_repeated"
    }

    fn len_field(&self, m: &TestTypes) -> uint {
        m.get_test_message_repeated().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a TestTypes, index: uint) -> &'a ::protobuf::Message {
        &m.get_test_message_repeated()[index] as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Eq,Show)]
pub enum TestEnum {
    DARK = 1,
    LIGHT = 2,
}

impl TestEnum {
    pub fn new(value: i32) -> TestEnum {
        match value {
            1 => DARK,
            2 => LIGHT,
            _ => panic!()
        }
    }
}

impl ::protobuf::ProtobufEnum for TestEnum {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn enum_descriptor_static(_: Option<TestEnum>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::EnumDescriptor };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("TestEnum", file_descriptor_proto())
            })
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x21, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x74, 0x65, 0x78, 0x74, 0x5f, 0x66, 0x6f, 0x72,
    0x6d, 0x61, 0x74, 0x5f, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0x1c, 0x0a, 0x0b, 0x54, 0x65, 0x73, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28,
    0x05, 0x22, 0xa1, 0x07, 0x0a, 0x09, 0x54, 0x65, 0x73, 0x74, 0x54, 0x79, 0x70, 0x65, 0x73, 0x12,
    0x17, 0x0a, 0x0f, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x5f, 0x73, 0x69, 0x6e, 0x67, 0x75, 0x6c,
    0x61, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x12, 0x16, 0x0a, 0x0e, 0x66, 0x6c, 0x6f, 0x61,
    0x74, 0x5f, 0x73, 0x69, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02,
    0x12, 0x16, 0x0a, 0x0e, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x73, 0x69, 0x6e, 0x67, 0x75, 0x6c,
    0x61, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x16, 0x0a, 0x0e, 0x69, 0x6e, 0x74, 0x36,
    0x34, 0x5f, 0x73, 0x69, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03,
    0x12, 0x17, 0x0a, 0x0f, 0x75, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x73, 0x69, 0x6e, 0x67, 0x75,
    0x6c, 0x61, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x17, 0x0a, 0x0f, 0x75, 0x69, 0x6e,
    0x74, 0x36, 0x34, 0x5f, 0x73, 0x69, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x04, 0x12, 0x17, 0x0a, 0x0f, 0x73, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x73, 0x69, 0x6e,
    0x67, 0x75, 0x6c, 0x61, 0x72, 0x18, 0x07, 0x20, 0x01, 0x28, 0x11, 0x12, 0x17, 0x0a, 0x0f, 0x73,
    0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x73, 0x69, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x18, 0x08,
    0x20, 0x01, 0x28, 0x12, 0x12, 0x18, 0x0a, 0x10, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f,
    0x73, 0x69, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x18, 0x09, 0x20, 0x01, 0x28, 0x07, 0x12, 0x18,
    0x0a, 0x10, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x73, 0x69, 0x6e, 0x67, 0x75, 0x6c,
    0x61, 0x72, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x06, 0x12, 0x19, 0x0a, 0x11, 0x73, 0x66, 0x69, 0x78,
    0x65, 0x64, 0x33, 0x32, 0x5f, 0x73, 0x69, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x18, 0x0b, 0x20,
    0x01, 0x28, 0x0f, 0x12, 0x19, 0x0a, 0x11, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f,
    0x73, 0x69, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x10, 0x12, 0x15,
    0x0a, 0x0d, 0x62, 0x6f, 0x6f, 0x6c, 0x5f, 0x73, 0x69, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x18,
    0x0d, 0x20, 0x01, 0x28, 0x08, 0x12, 0x17, 0x0a, 0x0f, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f,
    0x73, 0x69, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x09, 0x12, 0x16,
    0x0a, 0x0e, 0x62, 0x79, 0x74, 0x65, 0x73, 0x5f, 0x73, 0x69, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72,
    0x18, 0x0f, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x25, 0x0a, 0x12, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x65,
    0x6e, 0x75, 0x6d, 0x5f, 0x73, 0x69, 0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x18, 0x10, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x09, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x45, 0x6e, 0x75, 0x6d, 0x12, 0x2b, 0x0a,
    0x15, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x73, 0x69,
    0x6e, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x54,
    0x65, 0x73, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x17, 0x0a, 0x0f, 0x64, 0x6f,
    0x75, 0x62, 0x6c, 0x65, 0x5f, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x18, 0x1f, 0x20,
    0x03, 0x28, 0x01, 0x12, 0x16, 0x0a, 0x0e, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x5f, 0x72, 0x65, 0x70,
    0x65, 0x61, 0x74, 0x65, 0x64, 0x18, 0x20, 0x20, 0x03, 0x28, 0x02, 0x12, 0x16, 0x0a, 0x0e, 0x69,
    0x6e, 0x74, 0x33, 0x32, 0x5f, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x18, 0x21, 0x20,
    0x03, 0x28, 0x05, 0x12, 0x16, 0x0a, 0x0e, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x72, 0x65, 0x70,
    0x65, 0x61, 0x74, 0x65, 0x64, 0x18, 0x22, 0x20, 0x03, 0x28, 0x03, 0x12, 0x17, 0x0a, 0x0f, 0x75,
    0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x18, 0x23,
    0x20, 0x03, 0x28, 0x0d, 0x12, 0x17, 0x0a, 0x0f, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x5f, 0x72,
    0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x18, 0x24, 0x20, 0x03, 0x28, 0x04, 0x12, 0x17, 0x0a,
    0x0f, 0x73, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64,
    0x18, 0x25, 0x20, 0x03, 0x28, 0x11, 0x12, 0x17, 0x0a, 0x0f, 0x73, 0x69, 0x6e, 0x74, 0x36, 0x34,
    0x5f, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x18, 0x26, 0x20, 0x03, 0x28, 0x12, 0x12,
    0x18, 0x0a, 0x10, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f, 0x72, 0x65, 0x70, 0x65, 0x61,
    0x74, 0x65, 0x64, 0x18, 0x27, 0x20, 0x03, 0x28, 0x07, 0x12, 0x18, 0x0a, 0x10, 0x66, 0x69, 0x78,
    0x65, 0x64, 0x36, 0x34, 0x5f, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x18, 0x28, 0x20,
    0x03, 0x28, 0x06, 0x12, 0x19, 0x0a, 0x11, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x5f,
    0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x18, 0x29, 0x20, 0x03, 0x28, 0x0f, 0x12, 0x19,
    0x0a, 0x11, 0x73, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x72, 0x65, 0x70, 0x65, 0x61,
    0x74, 0x65, 0x64, 0x18, 0x2a, 0x20, 0x03, 0x28, 0x10, 0x12, 0x15, 0x0a, 0x0d, 0x62, 0x6f, 0x6f,
    0x6c, 0x5f, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x18, 0x2b, 0x20, 0x03, 0x28, 0x08,
    0x12, 0x17, 0x0a, 0x0f, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x72, 0x65, 0x70, 0x65, 0x61,
    0x74, 0x65, 0x64, 0x18, 0x2c, 0x20, 0x03, 0x28, 0x09, 0x12, 0x16, 0x0a, 0x0e, 0x62, 0x79, 0x74,
    0x65, 0x73, 0x5f, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x18, 0x2d, 0x20, 0x03, 0x28,
    0x0c, 0x12, 0x25, 0x0a, 0x12, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x65, 0x6e, 0x75, 0x6d, 0x5f, 0x72,
    0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x18, 0x2e, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x09, 0x2e,
    0x54, 0x65, 0x73, 0x74, 0x45, 0x6e, 0x75, 0x6d, 0x12, 0x2b, 0x0a, 0x15, 0x74, 0x65, 0x73, 0x74,
    0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65,
    0x64, 0x18, 0x2f, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x2a, 0x1f, 0x0a, 0x08, 0x54, 0x65, 0x73, 0x74, 0x45, 0x6e, 0x75,
    0x6d, 0x12, 0x08, 0x0a, 0x04, 0x44, 0x41, 0x52, 0x4b, 0x10, 0x01, 0x12, 0x09, 0x0a, 0x05, 0x4c,
    0x49, 0x47, 0x48, 0x54, 0x10, 0x02, 0x4a, 0x91, 0x14, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x2d,
    0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x00, 0x00, 0x03, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x00, 0x05, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x01, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x01, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x01,
    0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x02, 0x04, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x02, 0x04, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x02, 0x0c, 0x0d, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x05, 0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x05, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x04, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x06, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x06, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x06, 0x1b, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x09, 0x00, 0x2d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x09, 0x08, 0x11,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x04, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x0a, 0x14, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x0a, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0b,
    0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0b, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0b, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x13, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0b, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x0c, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x0c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x0c, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x13,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x24, 0x25, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x0d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x0d, 0x13, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x0d, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x04,
    0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0e, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0e, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0e, 0x14, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0e, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x05, 0x12, 0x03, 0x0f, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12,
    0x03, 0x0f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0f,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0f, 0x14, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x0f, 0x26, 0x27, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x10, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x06, 0x04, 0x12, 0x03, 0x10, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x06, 0x05, 0x12, 0x03, 0x10, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x01,
    0x12, 0x03, 0x10, 0x14, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03,
    0x10, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x07, 0x12, 0x03, 0x11, 0x04, 0x28,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x04, 0x12, 0x03, 0x11, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x05, 0x12, 0x03, 0x11, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x11, 0x14, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x07, 0x03, 0x12, 0x03, 0x11, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x08,
    0x12, 0x03, 0x12, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x04, 0x12, 0x03,
    0x12, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x05, 0x12, 0x03, 0x12, 0x0d,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x01, 0x12, 0x03, 0x12, 0x15, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x03, 0x12, 0x03, 0x12, 0x28, 0x29, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x09, 0x12, 0x03, 0x13, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x09, 0x04, 0x12, 0x03, 0x13, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09,
    0x05, 0x12, 0x03, 0x13, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x01, 0x12,
    0x03, 0x13, 0x15, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x03, 0x12, 0x03, 0x13,
    0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0a, 0x12, 0x03, 0x14, 0x04, 0x2d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x14, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x14, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x14, 0x16, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x0a, 0x03, 0x12, 0x03, 0x14, 0x2a, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0b, 0x12,
    0x03, 0x15, 0x04, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x15,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x15, 0x0d, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x15, 0x16, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x15, 0x2a, 0x2c, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x0c, 0x12, 0x03, 0x16, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x0c, 0x04, 0x12, 0x03, 0x16, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x05,
    0x12, 0x03, 0x16, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x01, 0x12, 0x03,
    0x16, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x16, 0x22,
    0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0d, 0x12, 0x03, 0x17, 0x04, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x17, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x17, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x0d, 0x01, 0x12, 0x03, 0x17, 0x14, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d,
    0x03, 0x12, 0x03, 0x17, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0e, 0x12, 0x03,
    0x18, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x18, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x18, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x18, 0x13, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x18, 0x24, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x0f, 0x12, 0x03, 0x19, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0f,
    0x04, 0x12, 0x03, 0x19, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0f, 0x06, 0x12,
    0x03, 0x19, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x19,
    0x16, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x19, 0x2b, 0x2d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x10, 0x12, 0x03, 0x1a, 0x04, 0x34, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x10, 0x04, 0x12, 0x03, 0x1a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x10, 0x06, 0x12, 0x03, 0x1a, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x10, 0x01, 0x12, 0x03, 0x1a, 0x19, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x10, 0x03,
    0x12, 0x03, 0x1a, 0x31, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x11, 0x12, 0x03, 0x1c,
    0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x11, 0x04, 0x12, 0x03, 0x1c, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x11, 0x05, 0x12, 0x03, 0x1c, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x11, 0x01, 0x12, 0x03, 0x1c, 0x14, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x11, 0x03, 0x12, 0x03, 0x1c, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x12, 0x12, 0x03, 0x1d, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x12, 0x04,
    0x12, 0x03, 0x1d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x12, 0x05, 0x12, 0x03,
    0x1d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x12, 0x01, 0x12, 0x03, 0x1d, 0x13,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x12, 0x03, 0x12, 0x03, 0x1d, 0x24, 0x26, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x13, 0x12, 0x03, 0x1e, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x13, 0x04, 0x12, 0x03, 0x1e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x13, 0x05, 0x12, 0x03, 0x1e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x13,
    0x01, 0x12, 0x03, 0x1e, 0x13, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x13, 0x03, 0x12,
    0x03, 0x1e, 0x24, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x14, 0x12, 0x03, 0x1f, 0x04,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x14, 0x04, 0x12, 0x03, 0x1f, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x14, 0x05, 0x12, 0x03, 0x1f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x14, 0x01, 0x12, 0x03, 0x1f, 0x13, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x14, 0x03, 0x12, 0x03, 0x1f, 0x24, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x15, 0x12, 0x03, 0x20, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x15, 0x04, 0x12,
    0x03, 0x20, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x15, 0x05, 0x12, 0x03, 0x20,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x15, 0x01, 0x12, 0x03, 0x20, 0x14, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x15, 0x03, 0x12, 0x03, 0x20, 0x26, 0x28, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x16, 0x12, 0x03, 0x21, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x16, 0x04, 0x12, 0x03, 0x21, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x16, 0x05, 0x12, 0x03, 0x21, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x16, 0x01,
    0x12, 0x03, 0x21, 0x14, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x16, 0x03, 0x12, 0x03,
    0x21, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x17, 0x12, 0x03, 0x22, 0x04, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x17, 0x04, 0x12, 0x03, 0x22, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x17, 0x05, 0x12, 0x03, 0x22, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x17, 0x01, 0x12, 0x03, 0x22, 0x14, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x17, 0x03, 0x12, 0x03, 0x22, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x18,
    0x12, 0x03, 0x23, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x18, 0x04, 0x12, 0x03,
    0x23, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x18, 0x05, 0x12, 0x03, 0x23, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x18, 0x01, 0x12, 0x03, 0x23, 0x14, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x18, 0x03, 0x12, 0x03, 0x23, 0x26, 0x28, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x19, 0x12, 0x03, 0x24, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x19, 0x04, 0x12, 0x03, 0x24, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x19,
    0x05, 0x12, 0x03, 0x24, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x19, 0x01, 0x12,
    0x03, 0x24, 0x15, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x19, 0x03, 0x12, 0x03, 0x24,
    0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x1a, 0x12, 0x03, 0x25, 0x04, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1a, 0x04, 0x12, 0x03, 0x25, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x1a, 0x05, 0x12, 0x03, 0x25, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x1a, 0x01, 0x12, 0x03, 0x25, 0x15, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x1a, 0x03, 0x12, 0x03, 0x25, 0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x1b, 0x12,
    0x03, 0x26, 0x04, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1b, 0x04, 0x12, 0x03, 0x26,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1b, 0x05, 0x12, 0x03, 0x26, 0x0d, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1b, 0x01, 0x12, 0x03, 0x26, 0x16, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x1b, 0x03, 0x12, 0x03, 0x26, 0x2a, 0x2c, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x1c, 0x12, 0x03, 0x27, 0x04, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x1c, 0x04, 0x12, 0x03, 0x27, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1c, 0x05,
    0x12, 0x03, 0x27, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1c, 0x01, 0x12, 0x03,
    0x27, 0x16, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1c, 0x03, 0x12, 0x03, 0x27, 0x2a,
    0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x1d, 0x12, 0x03, 0x28, 0x04, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x1d, 0x04, 0x12, 0x03, 0x28, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x1d, 0x05, 0x12, 0x03, 0x28, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x1d, 0x01, 0x12, 0x03, 0x28, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1d,
    0x03, 0x12, 0x03, 0x28, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x1e, 0x12, 0x03,
    0x29, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1e, 0x04, 0x12, 0x03, 0x29, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1e, 0x05, 0x12, 0x03, 0x29, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1e, 0x01, 0x12, 0x03, 0x29, 0x14, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x1e, 0x03, 0x12, 0x03, 0x29, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x1f, 0x12, 0x03, 0x2a, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1f,
    0x04, 0x12, 0x03, 0x2a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1f, 0x05, 0x12,
    0x03, 0x2a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1f, 0x01, 0x12, 0x03, 0x2a,
    0x13, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1f, 0x03, 0x12, 0x03, 0x2a, 0x24, 0x26,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x20, 0x12, 0x03, 0x2b, 0x04, 0x2e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x20, 0x04, 0x12, 0x03, 0x2b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x20, 0x06, 0x12, 0x03, 0x2b, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x20, 0x01, 0x12, 0x03, 0x2b, 0x16, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x20, 0x03,
    0x12, 0x03, 0x2b, 0x2b, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x21, 0x12, 0x03, 0x2c,
    0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x21, 0x04, 0x12, 0x03, 0x2c, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x21, 0x06, 0x12, 0x03, 0x2c, 0x0d, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x21, 0x01, 0x12, 0x03, 0x2c, 0x19, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x21, 0x03, 0x12, 0x03, 0x2c, 0x31, 0x33,
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

// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]


#[deriving(Clone,PartialEq,Default,Show)]
pub struct TestLiteRuntime {
    v: ::std::option::Option<i32>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TestLiteRuntime {
    pub fn new() -> TestLiteRuntime {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TestLiteRuntime {
        static mut instance: ::protobuf::lazy::Lazy<TestLiteRuntime> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const TestLiteRuntime };
        unsafe {
            instance.get(|| {
                TestLiteRuntime {
                    v: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional int32 v = 1;

    pub fn clear_v(&mut self) {
        self.v = None;
    }

    pub fn has_v(&self) -> bool {
        self.v.is_some()
    }

    // Param is passed by value, moved
    pub fn set_v(&mut self, v: i32) {
        self.v = Some(v);
    }

    pub fn get_v(&self) -> i32 {
        self.v.unwrap_or(0)
    }
}

impl ::protobuf::Message for TestLiteRuntime {
    fn new() -> TestLiteRuntime {
        TestLiteRuntime::new()
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
                        return ::std::result::Err(::protobuf::ProtobufWireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.v = Some(tmp);
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
        for value in self.v.iter() {
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
        match self.v {
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

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TestLiteRuntime>()
    }
}

impl ::protobuf::Clear for TestLiteRuntime {
    fn clear(&mut self) {
        self.clear_v();
        self.unknown_fields.clear();
    }
}

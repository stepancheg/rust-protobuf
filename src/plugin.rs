// This file is generated. Do not edit

use protobuf::*;
use protobuf::rt;
use descriptor::*;

#[deriving(Clone,Eq)]
pub struct CodeGeneratorRequest {
    file_to_generate: ~[~str],
    parameter: Option<~str>,
    proto_file: ~[FileDescriptorProto],
}

impl<'self> CodeGeneratorRequest {
    pub fn new() -> CodeGeneratorRequest {
        CodeGeneratorRequest {
            file_to_generate: ~[],
            parameter: None,
            proto_file: ~[],
        }
    }

    pub fn default_instance() -> &'static CodeGeneratorRequest {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: CodeGeneratorRequest = CodeGeneratorRequest {
//             file_to_generate: ~[],
//             parameter: None,
//             proto_file: ~[],
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for v in self.file_to_generate.iter() {
            os.write_string(1, *v);
        };
        match self.parameter {
            Some(ref v) => {
                os.write_string(2, *v);
            },
            None => {},
        };
        for v in self.proto_file.iter() {
            os.write_tag(15, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_file_to_generate(&mut self) {
        self.file_to_generate.clear();
    }

    // Param is passed by value, moved
    pub fn set_file_to_generate(&mut self, v: ~[~str]) {
        self.file_to_generate = v;
    }

    // Mutable pointer to the field.
    pub fn mut_file_to_generate(&'self mut self) -> &'self mut ~[~str] {
        &mut self.file_to_generate
    }

    pub fn get_file_to_generate(&'self self) -> &'self [~str] {
        rt::as_slice_tmp(&self.file_to_generate)
    }

    pub fn add_file_to_generate(&mut self, v: ~str) {
        self.file_to_generate.push(v);
    }

    pub fn clear_parameter(&mut self) {
        self.parameter = None;
    }

    pub fn has_parameter(&self) -> bool {
        self.parameter.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parameter(&mut self, v: ~str) {
        self.parameter = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parameter(&'self mut self) -> &'self mut ~str {
        if self.parameter.is_none() {
            self.parameter = Some(~"");
        };
        self.parameter.get_mut_ref()
    }

    pub fn get_parameter(&'self self) -> &'self str {
        match self.parameter {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_proto_file(&mut self) {
        self.proto_file.clear();
    }

    // Param is passed by value, moved
    pub fn set_proto_file(&mut self, v: ~[FileDescriptorProto]) {
        self.proto_file = v;
    }

    // Mutable pointer to the field.
    pub fn mut_proto_file(&'self mut self) -> &'self mut ~[FileDescriptorProto] {
        &mut self.proto_file
    }

    pub fn get_proto_file(&'self self) -> &'self [FileDescriptorProto] {
        rt::as_slice_tmp(&self.proto_file)
    }

    pub fn add_proto_file(&mut self, v: FileDescriptorProto) {
        self.proto_file.push(v);
    }
}

impl Message for CodeGeneratorRequest {
    fn new() -> CodeGeneratorRequest {
        CodeGeneratorRequest::new()
    }

    fn clear(&mut self) {
        self.clear_file_to_generate();
        self.clear_parameter();
        self.clear_proto_file();
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
                    let tmp = is.read_string();
                    self.file_to_generate.push(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.parameter = Some(tmp);
                },
                15 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = FileDescriptorProto::new();
                    is.merge_message(&mut tmp);
                    self.proto_file.push(tmp);
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
        for value in self.file_to_generate.iter() {
            my_size += rt::string_size(1, *value);
        };
        for value in self.parameter.iter() {
            my_size += rt::string_size(2, *value);
        };
        for value in self.proto_file.iter() {
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
pub struct CodeGeneratorResponse {
    error: Option<~str>,
    file: ~[CodeGeneratorResponse_File],
}

impl<'self> CodeGeneratorResponse {
    pub fn new() -> CodeGeneratorResponse {
        CodeGeneratorResponse {
            error: None,
            file: ~[],
        }
    }

    pub fn default_instance() -> &'static CodeGeneratorResponse {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: CodeGeneratorResponse = CodeGeneratorResponse {
//             error: None,
//             file: ~[],
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.error {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        for v in self.file.iter() {
            os.write_tag(15, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_error(&mut self) {
        self.error = None;
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ~str) {
        self.error = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&'self mut self) -> &'self mut ~str {
        if self.error.is_none() {
            self.error = Some(~"");
        };
        self.error.get_mut_ref()
    }

    pub fn get_error(&'self self) -> &'self str {
        match self.error {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_file(&mut self) {
        self.file.clear();
    }

    // Param is passed by value, moved
    pub fn set_file(&mut self, v: ~[CodeGeneratorResponse_File]) {
        self.file = v;
    }

    // Mutable pointer to the field.
    pub fn mut_file(&'self mut self) -> &'self mut ~[CodeGeneratorResponse_File] {
        &mut self.file
    }

    pub fn get_file(&'self self) -> &'self [CodeGeneratorResponse_File] {
        rt::as_slice_tmp(&self.file)
    }

    pub fn add_file(&mut self, v: CodeGeneratorResponse_File) {
        self.file.push(v);
    }
}

impl Message for CodeGeneratorResponse {
    fn new() -> CodeGeneratorResponse {
        CodeGeneratorResponse::new()
    }

    fn clear(&mut self) {
        self.clear_error();
        self.clear_file();
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
                    let tmp = is.read_string();
                    self.error = Some(tmp);
                },
                15 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = CodeGeneratorResponse_File::new();
                    is.merge_message(&mut tmp);
                    self.file.push(tmp);
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
        for value in self.error.iter() {
            my_size += rt::string_size(1, *value);
        };
        for value in self.file.iter() {
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
pub struct CodeGeneratorResponse_File {
    name: Option<~str>,
    insertion_point: Option<~str>,
    content: Option<~str>,
}

impl<'self> CodeGeneratorResponse_File {
    pub fn new() -> CodeGeneratorResponse_File {
        CodeGeneratorResponse_File {
            name: None,
            insertion_point: None,
            content: None,
        }
    }

    pub fn default_instance() -> &'static CodeGeneratorResponse_File {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: CodeGeneratorResponse_File = CodeGeneratorResponse_File {
//             name: None,
//             insertion_point: None,
//             content: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.name {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        match self.insertion_point {
            Some(ref v) => {
                os.write_string(2, *v);
            },
            None => {},
        };
        match self.content {
            Some(ref v) => {
                os.write_string(15, *v);
            },
            None => {},
        };
    }

    pub fn clear_name(&mut self) {
        self.name = None;
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ~str) {
        self.name = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&'self mut self) -> &'self mut ~str {
        if self.name.is_none() {
            self.name = Some(~"");
        };
        self.name.get_mut_ref()
    }

    pub fn get_name(&'self self) -> &'self str {
        match self.name {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_insertion_point(&mut self) {
        self.insertion_point = None;
    }

    pub fn has_insertion_point(&self) -> bool {
        self.insertion_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_insertion_point(&mut self, v: ~str) {
        self.insertion_point = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_insertion_point(&'self mut self) -> &'self mut ~str {
        if self.insertion_point.is_none() {
            self.insertion_point = Some(~"");
        };
        self.insertion_point.get_mut_ref()
    }

    pub fn get_insertion_point(&'self self) -> &'self str {
        match self.insertion_point {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_content(&mut self) {
        self.content = None;
    }

    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ~str) {
        self.content = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&'self mut self) -> &'self mut ~str {
        if self.content.is_none() {
            self.content = Some(~"");
        };
        self.content.get_mut_ref()
    }

    pub fn get_content(&'self self) -> &'self str {
        match self.content {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }
}

impl Message for CodeGeneratorResponse_File {
    fn new() -> CodeGeneratorResponse_File {
        CodeGeneratorResponse_File::new()
    }

    fn clear(&mut self) {
        self.clear_name();
        self.clear_insertion_point();
        self.clear_content();
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
                    let tmp = is.read_string();
                    self.name = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.insertion_point = Some(tmp);
                },
                15 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.content = Some(tmp);
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
        for value in self.name.iter() {
            my_size += rt::string_size(1, *value);
        };
        for value in self.insertion_point.iter() {
            my_size += rt::string_size(2, *value);
        };
        for value in self.content.iter() {
            my_size += rt::string_size(15, *value);
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

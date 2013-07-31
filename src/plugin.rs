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

impl CodeGeneratorRequest {
    pub fn new() -> CodeGeneratorRequest {
        CodeGeneratorRequest {
            file_to_generate: ~[],
            parameter: None,
            proto_file: ~[],
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for self.file_to_generate.iter().advance |v| {
            os.write_string(1, *v);
        };
        match self.parameter {
            Some(ref v) => {
                os.write_string(2, *v);
            },
            None => {},
        };
        for self.proto_file.iter().advance |v| {
            os.write_tag(15, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_file_to_generate(&mut self) {
        self.file_to_generate.clear();
    }

    pub fn clear_parameter(&mut self) {
        self.parameter = None;
    }

    pub fn clear_proto_file(&mut self) {
        self.proto_file.clear();
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
        for self.file_to_generate.iter().advance |value| {
            my_size += rt::string_size(1, *value);
        };
        for self.parameter.iter().advance |value| {
            my_size += rt::string_size(2, *value);
        };
        for self.proto_file.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    pub fn write_to(&self, os: &mut CodedOutputStream) {
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

impl CodeGeneratorResponse {
    pub fn new() -> CodeGeneratorResponse {
        CodeGeneratorResponse {
            error: None,
            file: ~[],
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.error {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        for self.file.iter().advance |v| {
            os.write_tag(15, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_error(&mut self) {
        self.error = None;
    }

    pub fn clear_file(&mut self) {
        self.file.clear();
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
        for self.error.iter().advance |value| {
            my_size += rt::string_size(1, *value);
        };
        for self.file.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    pub fn write_to(&self, os: &mut CodedOutputStream) {
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

impl CodeGeneratorResponse_File {
    pub fn new() -> CodeGeneratorResponse_File {
        CodeGeneratorResponse_File {
            name: None,
            insertion_point: None,
            content: None,
        }
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

    pub fn clear_insertion_point(&mut self) {
        self.insertion_point = None;
    }

    pub fn clear_content(&mut self) {
        self.content = None;
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
        for self.name.iter().advance |value| {
            my_size += rt::string_size(1, *value);
        };
        for self.insertion_point.iter().advance |value| {
            my_size += rt::string_size(2, *value);
        };
        for self.content.iter().advance |value| {
            my_size += rt::string_size(15, *value);
        };
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    pub fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes: ~[u32] = ~[];
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes, &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
    }
}

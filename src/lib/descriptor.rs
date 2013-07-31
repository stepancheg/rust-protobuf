// This file is generated. Do not edit

use protobuf::*;
use protobuf::rt;

#[deriving(Clone,Eq)]
pub struct FileDescriptorSet {
    file: ~[FileDescriptorProto],
}

impl FileDescriptorSet {
    pub fn new() -> FileDescriptorSet {
        FileDescriptorSet {
            file: ~[],
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for self.file.iter().advance |v| {
            os.write_tag(1, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_file(&mut self) {
        self.file.clear();
    }
}

impl Message for FileDescriptorSet {
    fn new() -> FileDescriptorSet {
        FileDescriptorSet::new()
    }

    fn clear(&mut self) {
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
                    let mut tmp = FileDescriptorProto::new();
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
pub struct FileDescriptorProto {
    name: Option<~str>,
    package: Option<~str>,
    dependency: ~[~str],
    public_dependency: ~[i32],
    weak_dependency: ~[i32],
    message_type: ~[DescriptorProto],
    enum_type: ~[EnumDescriptorProto],
    service: ~[ServiceDescriptorProto],
    extension: ~[FieldDescriptorProto],
    options: Option<FileOptions>,
    source_code_info: Option<SourceCodeInfo>,
}

impl FileDescriptorProto {
    pub fn new() -> FileDescriptorProto {
        FileDescriptorProto {
            name: None,
            package: None,
            dependency: ~[],
            public_dependency: ~[],
            weak_dependency: ~[],
            message_type: ~[],
            enum_type: ~[],
            service: ~[],
            extension: ~[],
            options: None,
            source_code_info: None,
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.name {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        match self.package {
            Some(ref v) => {
                os.write_string(2, *v);
            },
            None => {},
        };
        for self.dependency.iter().advance |v| {
            os.write_string(3, *v);
        };
        for self.public_dependency.iter().advance |v| {
            os.write_int32(10, *v);
        };
        for self.weak_dependency.iter().advance |v| {
            os.write_int32(11, *v);
        };
        for self.message_type.iter().advance |v| {
            os.write_tag(4, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        for self.enum_type.iter().advance |v| {
            os.write_tag(5, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        for self.service.iter().advance |v| {
            os.write_tag(6, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        for self.extension.iter().advance |v| {
            os.write_tag(7, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        match self.options {
            Some(ref v) => {
                os.write_tag(8, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
        match self.source_code_info {
            Some(ref v) => {
                os.write_tag(9, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
    }

    pub fn clear_name(&mut self) {
        self.name = None;
    }

    pub fn clear_package(&mut self) {
        self.package = None;
    }

    pub fn clear_dependency(&mut self) {
        self.dependency.clear();
    }

    pub fn clear_public_dependency(&mut self) {
        self.public_dependency.clear();
    }

    pub fn clear_weak_dependency(&mut self) {
        self.weak_dependency.clear();
    }

    pub fn clear_message_type(&mut self) {
        self.message_type.clear();
    }

    pub fn clear_enum_type(&mut self) {
        self.enum_type.clear();
    }

    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    pub fn clear_extension(&mut self) {
        self.extension.clear();
    }

    pub fn clear_options(&mut self) {
        self.options = None;
    }

    pub fn clear_source_code_info(&mut self) {
        self.source_code_info = None;
    }
}

impl Message for FileDescriptorProto {
    fn new() -> FileDescriptorProto {
        FileDescriptorProto::new()
    }

    fn clear(&mut self) {
        self.clear_name();
        self.clear_package();
        self.clear_dependency();
        self.clear_public_dependency();
        self.clear_weak_dependency();
        self.clear_message_type();
        self.clear_enum_type();
        self.clear_service();
        self.clear_extension();
        self.clear_options();
        self.clear_source_code_info();
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
                    self.package = Some(tmp);
                },
                3 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.dependency.push(tmp);
                },
                10 => {
                    if wire_type == wire_format::WireTypeLengthDelimited {
                        let len = is.read_raw_varint32();
                        let old_limit = is.push_limit(len);
                        while !is.eof() {
                            self.public_dependency.push(is.read_int32());
                        }
                        is.pop_limit(old_limit);
                    } else {
                        assert_eq!(wire_format::WireTypeVarint, wire_type);
                        self.public_dependency.push(is.read_int32());
                    }
                },
                11 => {
                    if wire_type == wire_format::WireTypeLengthDelimited {
                        let len = is.read_raw_varint32();
                        let old_limit = is.push_limit(len);
                        while !is.eof() {
                            self.weak_dependency.push(is.read_int32());
                        }
                        is.pop_limit(old_limit);
                    } else {
                        assert_eq!(wire_format::WireTypeVarint, wire_type);
                        self.weak_dependency.push(is.read_int32());
                    }
                },
                4 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = DescriptorProto::new();
                    is.merge_message(&mut tmp);
                    self.message_type.push(tmp);
                },
                5 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = EnumDescriptorProto::new();
                    is.merge_message(&mut tmp);
                    self.enum_type.push(tmp);
                },
                6 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = ServiceDescriptorProto::new();
                    is.merge_message(&mut tmp);
                    self.service.push(tmp);
                },
                7 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = FieldDescriptorProto::new();
                    is.merge_message(&mut tmp);
                    self.extension.push(tmp);
                },
                8 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = FileOptions::new();
                    is.merge_message(&mut tmp);
                    self.options = Some(tmp);
                },
                9 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = SourceCodeInfo::new();
                    is.merge_message(&mut tmp);
                    self.source_code_info = Some(tmp);
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
        for self.package.iter().advance |value| {
            my_size += rt::string_size(2, *value);
        };
        for self.dependency.iter().advance |value| {
            my_size += rt::string_size(3, *value);
        };
        for self.public_dependency.iter().advance |value| {
            my_size += rt::value_size(10, *value, wire_format::WireTypeVarint);
        };
        for self.weak_dependency.iter().advance |value| {
            my_size += rt::value_size(11, *value, wire_format::WireTypeVarint);
        };
        for self.message_type.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for self.enum_type.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for self.service.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for self.extension.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for self.options.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for self.source_code_info.iter().advance |value| {
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
pub struct DescriptorProto {
    name: Option<~str>,
    field: ~[FieldDescriptorProto],
    extension: ~[FieldDescriptorProto],
    nested_type: ~[DescriptorProto],
    enum_type: ~[EnumDescriptorProto],
    extension_range: ~[DescriptorProto_ExtensionRange],
    options: Option<MessageOptions>,
}

impl DescriptorProto {
    pub fn new() -> DescriptorProto {
        DescriptorProto {
            name: None,
            field: ~[],
            extension: ~[],
            nested_type: ~[],
            enum_type: ~[],
            extension_range: ~[],
            options: None,
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.name {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        for self.field.iter().advance |v| {
            os.write_tag(2, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        for self.extension.iter().advance |v| {
            os.write_tag(6, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        for self.nested_type.iter().advance |v| {
            os.write_tag(3, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        for self.enum_type.iter().advance |v| {
            os.write_tag(4, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        for self.extension_range.iter().advance |v| {
            os.write_tag(5, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        match self.options {
            Some(ref v) => {
                os.write_tag(7, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
    }

    pub fn clear_name(&mut self) {
        self.name = None;
    }

    pub fn clear_field(&mut self) {
        self.field.clear();
    }

    pub fn clear_extension(&mut self) {
        self.extension.clear();
    }

    pub fn clear_nested_type(&mut self) {
        self.nested_type.clear();
    }

    pub fn clear_enum_type(&mut self) {
        self.enum_type.clear();
    }

    pub fn clear_extension_range(&mut self) {
        self.extension_range.clear();
    }

    pub fn clear_options(&mut self) {
        self.options = None;
    }
}

impl Message for DescriptorProto {
    fn new() -> DescriptorProto {
        DescriptorProto::new()
    }

    fn clear(&mut self) {
        self.clear_name();
        self.clear_field();
        self.clear_extension();
        self.clear_nested_type();
        self.clear_enum_type();
        self.clear_extension_range();
        self.clear_options();
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
                    let mut tmp = FieldDescriptorProto::new();
                    is.merge_message(&mut tmp);
                    self.field.push(tmp);
                },
                6 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = FieldDescriptorProto::new();
                    is.merge_message(&mut tmp);
                    self.extension.push(tmp);
                },
                3 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = DescriptorProto::new();
                    is.merge_message(&mut tmp);
                    self.nested_type.push(tmp);
                },
                4 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = EnumDescriptorProto::new();
                    is.merge_message(&mut tmp);
                    self.enum_type.push(tmp);
                },
                5 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = DescriptorProto_ExtensionRange::new();
                    is.merge_message(&mut tmp);
                    self.extension_range.push(tmp);
                },
                7 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = MessageOptions::new();
                    is.merge_message(&mut tmp);
                    self.options = Some(tmp);
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
        for self.field.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for self.extension.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for self.nested_type.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for self.enum_type.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for self.extension_range.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for self.options.iter().advance |value| {
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
pub struct DescriptorProto_ExtensionRange {
    start: Option<i32>,
    end: Option<i32>,
}

impl DescriptorProto_ExtensionRange {
    pub fn new() -> DescriptorProto_ExtensionRange {
        DescriptorProto_ExtensionRange {
            start: None,
            end: None,
        }
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.start {
            Some(ref v) => {
                os.write_int32(1, *v);
            },
            None => {},
        };
        match self.end {
            Some(ref v) => {
                os.write_int32(2, *v);
            },
            None => {},
        };
    }

    pub fn clear_start(&mut self) {
        self.start = None;
    }

    pub fn clear_end(&mut self) {
        self.end = None;
    }
}

impl Message for DescriptorProto_ExtensionRange {
    fn new() -> DescriptorProto_ExtensionRange {
        DescriptorProto_ExtensionRange::new()
    }

    fn clear(&mut self) {
        self.clear_start();
        self.clear_end();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.start = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.end = Some(tmp);
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
        for self.start.iter().advance |value| {
            my_size += rt::value_size(1, *value, wire_format::WireTypeVarint);
        };
        for self.end.iter().advance |value| {
            my_size += rt::value_size(2, *value, wire_format::WireTypeVarint);
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
pub struct FieldDescriptorProto {
    name: Option<~str>,
    number: Option<i32>,
    label: Option<FieldDescriptorProto_Label>,
    field_type: Option<FieldDescriptorProto_Type>,
    type_name: Option<~str>,
    extendee: Option<~str>,
    default_value: Option<~str>,
    options: Option<FieldOptions>,
}

impl FieldDescriptorProto {
    pub fn new() -> FieldDescriptorProto {
        FieldDescriptorProto {
            name: None,
            number: None,
            label: None,
            field_type: None,
            type_name: None,
            extendee: None,
            default_value: None,
            options: None,
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.name {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        match self.number {
            Some(ref v) => {
                os.write_int32(3, *v);
            },
            None => {},
        };
        match self.label {
            Some(ref v) => {
                os.write_enum(4, *v as i32);
            },
            None => {},
        };
        match self.field_type {
            Some(ref v) => {
                os.write_enum(5, *v as i32);
            },
            None => {},
        };
        match self.type_name {
            Some(ref v) => {
                os.write_string(6, *v);
            },
            None => {},
        };
        match self.extendee {
            Some(ref v) => {
                os.write_string(2, *v);
            },
            None => {},
        };
        match self.default_value {
            Some(ref v) => {
                os.write_string(7, *v);
            },
            None => {},
        };
        match self.options {
            Some(ref v) => {
                os.write_tag(8, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
    }

    pub fn clear_name(&mut self) {
        self.name = None;
    }

    pub fn clear_number(&mut self) {
        self.number = None;
    }

    pub fn clear_label(&mut self) {
        self.label = None;
    }

    pub fn clear_field_type(&mut self) {
        self.field_type = None;
    }

    pub fn clear_type_name(&mut self) {
        self.type_name = None;
    }

    pub fn clear_extendee(&mut self) {
        self.extendee = None;
    }

    pub fn clear_default_value(&mut self) {
        self.default_value = None;
    }

    pub fn clear_options(&mut self) {
        self.options = None;
    }
}

impl Message for FieldDescriptorProto {
    fn new() -> FieldDescriptorProto {
        FieldDescriptorProto::new()
    }

    fn clear(&mut self) {
        self.clear_name();
        self.clear_number();
        self.clear_label();
        self.clear_field_type();
        self.clear_type_name();
        self.clear_extendee();
        self.clear_default_value();
        self.clear_options();
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
                3 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.number = Some(tmp);
                },
                4 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = FieldDescriptorProto_Label::new(is.read_int32());
                    self.label = Some(tmp);
                },
                5 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = FieldDescriptorProto_Type::new(is.read_int32());
                    self.field_type = Some(tmp);
                },
                6 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.type_name = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.extendee = Some(tmp);
                },
                7 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.default_value = Some(tmp);
                },
                8 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = FieldOptions::new();
                    is.merge_message(&mut tmp);
                    self.options = Some(tmp);
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
        for self.number.iter().advance |value| {
            my_size += rt::value_size(3, *value, wire_format::WireTypeVarint);
        };
        for self.label.iter().advance |value| {
            my_size += rt::enum_size(4, *value);
        };
        for self.field_type.iter().advance |value| {
            my_size += rt::enum_size(5, *value);
        };
        for self.type_name.iter().advance |value| {
            my_size += rt::string_size(6, *value);
        };
        for self.extendee.iter().advance |value| {
            my_size += rt::string_size(2, *value);
        };
        for self.default_value.iter().advance |value| {
            my_size += rt::string_size(7, *value);
        };
        for self.options.iter().advance |value| {
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
pub enum FieldDescriptorProto_Type {
    TYPE_DOUBLE = 1,
    TYPE_FLOAT = 2,
    TYPE_INT64 = 3,
    TYPE_UINT64 = 4,
    TYPE_INT32 = 5,
    TYPE_FIXED64 = 6,
    TYPE_FIXED32 = 7,
    TYPE_BOOL = 8,
    TYPE_STRING = 9,
    TYPE_GROUP = 10,
    TYPE_MESSAGE = 11,
    TYPE_BYTES = 12,
    TYPE_UINT32 = 13,
    TYPE_ENUM = 14,
    TYPE_SFIXED32 = 15,
    TYPE_SFIXED64 = 16,
    TYPE_SINT32 = 17,
    TYPE_SINT64 = 18,
}

impl FieldDescriptorProto_Type {
    pub fn new(value: i32) -> FieldDescriptorProto_Type {
        match value {
            1 => TYPE_DOUBLE,
            2 => TYPE_FLOAT,
            3 => TYPE_INT64,
            4 => TYPE_UINT64,
            5 => TYPE_INT32,
            6 => TYPE_FIXED64,
            7 => TYPE_FIXED32,
            8 => TYPE_BOOL,
            9 => TYPE_STRING,
            10 => TYPE_GROUP,
            11 => TYPE_MESSAGE,
            12 => TYPE_BYTES,
            13 => TYPE_UINT32,
            14 => TYPE_ENUM,
            15 => TYPE_SFIXED32,
            16 => TYPE_SFIXED64,
            17 => TYPE_SINT32,
            18 => TYPE_SINT64,
            _ => fail!()
        }
    }
}

impl ProtobufEnum for FieldDescriptorProto_Type {
    pub fn value(&self) -> i32 {
        *self as i32
    }
}

#[deriving(Clone,Eq)]
pub enum FieldDescriptorProto_Label {
    LABEL_OPTIONAL = 1,
    LABEL_REQUIRED = 2,
    LABEL_REPEATED = 3,
}

impl FieldDescriptorProto_Label {
    pub fn new(value: i32) -> FieldDescriptorProto_Label {
        match value {
            1 => LABEL_OPTIONAL,
            2 => LABEL_REQUIRED,
            3 => LABEL_REPEATED,
            _ => fail!()
        }
    }
}

impl ProtobufEnum for FieldDescriptorProto_Label {
    pub fn value(&self) -> i32 {
        *self as i32
    }
}

#[deriving(Clone,Eq)]
pub struct EnumDescriptorProto {
    name: Option<~str>,
    value: ~[EnumValueDescriptorProto],
    options: Option<EnumOptions>,
}

impl EnumDescriptorProto {
    pub fn new() -> EnumDescriptorProto {
        EnumDescriptorProto {
            name: None,
            value: ~[],
            options: None,
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.name {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        for self.value.iter().advance |v| {
            os.write_tag(2, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        match self.options {
            Some(ref v) => {
                os.write_tag(3, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
    }

    pub fn clear_name(&mut self) {
        self.name = None;
    }

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn clear_options(&mut self) {
        self.options = None;
    }
}

impl Message for EnumDescriptorProto {
    fn new() -> EnumDescriptorProto {
        EnumDescriptorProto::new()
    }

    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.clear_options();
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
                    let mut tmp = EnumValueDescriptorProto::new();
                    is.merge_message(&mut tmp);
                    self.value.push(tmp);
                },
                3 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = EnumOptions::new();
                    is.merge_message(&mut tmp);
                    self.options = Some(tmp);
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
        for self.value.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for self.options.iter().advance |value| {
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
pub struct EnumValueDescriptorProto {
    name: Option<~str>,
    number: Option<i32>,
    options: Option<EnumValueOptions>,
}

impl EnumValueDescriptorProto {
    pub fn new() -> EnumValueDescriptorProto {
        EnumValueDescriptorProto {
            name: None,
            number: None,
            options: None,
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.name {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        match self.number {
            Some(ref v) => {
                os.write_int32(2, *v);
            },
            None => {},
        };
        match self.options {
            Some(ref v) => {
                os.write_tag(3, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
    }

    pub fn clear_name(&mut self) {
        self.name = None;
    }

    pub fn clear_number(&mut self) {
        self.number = None;
    }

    pub fn clear_options(&mut self) {
        self.options = None;
    }
}

impl Message for EnumValueDescriptorProto {
    fn new() -> EnumValueDescriptorProto {
        EnumValueDescriptorProto::new()
    }

    fn clear(&mut self) {
        self.clear_name();
        self.clear_number();
        self.clear_options();
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
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.number = Some(tmp);
                },
                3 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = EnumValueOptions::new();
                    is.merge_message(&mut tmp);
                    self.options = Some(tmp);
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
        for self.number.iter().advance |value| {
            my_size += rt::value_size(2, *value, wire_format::WireTypeVarint);
        };
        for self.options.iter().advance |value| {
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
pub struct ServiceDescriptorProto {
    name: Option<~str>,
    method: ~[MethodDescriptorProto],
    options: Option<ServiceOptions>,
}

impl ServiceDescriptorProto {
    pub fn new() -> ServiceDescriptorProto {
        ServiceDescriptorProto {
            name: None,
            method: ~[],
            options: None,
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.name {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        for self.method.iter().advance |v| {
            os.write_tag(2, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        match self.options {
            Some(ref v) => {
                os.write_tag(3, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
    }

    pub fn clear_name(&mut self) {
        self.name = None;
    }

    pub fn clear_method(&mut self) {
        self.method.clear();
    }

    pub fn clear_options(&mut self) {
        self.options = None;
    }
}

impl Message for ServiceDescriptorProto {
    fn new() -> ServiceDescriptorProto {
        ServiceDescriptorProto::new()
    }

    fn clear(&mut self) {
        self.clear_name();
        self.clear_method();
        self.clear_options();
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
                    let mut tmp = MethodDescriptorProto::new();
                    is.merge_message(&mut tmp);
                    self.method.push(tmp);
                },
                3 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = ServiceOptions::new();
                    is.merge_message(&mut tmp);
                    self.options = Some(tmp);
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
        for self.method.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for self.options.iter().advance |value| {
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
pub struct MethodDescriptorProto {
    name: Option<~str>,
    input_type: Option<~str>,
    output_type: Option<~str>,
    options: Option<MethodOptions>,
}

impl MethodDescriptorProto {
    pub fn new() -> MethodDescriptorProto {
        MethodDescriptorProto {
            name: None,
            input_type: None,
            output_type: None,
            options: None,
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.name {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        match self.input_type {
            Some(ref v) => {
                os.write_string(2, *v);
            },
            None => {},
        };
        match self.output_type {
            Some(ref v) => {
                os.write_string(3, *v);
            },
            None => {},
        };
        match self.options {
            Some(ref v) => {
                os.write_tag(4, wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes, sizes_pos);
            },
            None => {},
        };
    }

    pub fn clear_name(&mut self) {
        self.name = None;
    }

    pub fn clear_input_type(&mut self) {
        self.input_type = None;
    }

    pub fn clear_output_type(&mut self) {
        self.output_type = None;
    }

    pub fn clear_options(&mut self) {
        self.options = None;
    }
}

impl Message for MethodDescriptorProto {
    fn new() -> MethodDescriptorProto {
        MethodDescriptorProto::new()
    }

    fn clear(&mut self) {
        self.clear_name();
        self.clear_input_type();
        self.clear_output_type();
        self.clear_options();
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
                    self.input_type = Some(tmp);
                },
                3 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.output_type = Some(tmp);
                },
                4 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = MethodOptions::new();
                    is.merge_message(&mut tmp);
                    self.options = Some(tmp);
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
        for self.input_type.iter().advance |value| {
            my_size += rt::string_size(2, *value);
        };
        for self.output_type.iter().advance |value| {
            my_size += rt::string_size(3, *value);
        };
        for self.options.iter().advance |value| {
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
pub struct FileOptions {
    java_package: Option<~str>,
    java_outer_classname: Option<~str>,
    java_multiple_files: Option<bool>,
    java_generate_equals_and_hash: Option<bool>,
    optimize_for: Option<FileOptions_OptimizeMode>,
    go_package: Option<~str>,
    cc_generic_services: Option<bool>,
    java_generic_services: Option<bool>,
    py_generic_services: Option<bool>,
    uninterpreted_option: ~[UninterpretedOption],
}

impl FileOptions {
    pub fn new() -> FileOptions {
        FileOptions {
            java_package: None,
            java_outer_classname: None,
            java_multiple_files: None,
            java_generate_equals_and_hash: None,
            optimize_for: None,
            go_package: None,
            cc_generic_services: None,
            java_generic_services: None,
            py_generic_services: None,
            uninterpreted_option: ~[],
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.java_package {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        match self.java_outer_classname {
            Some(ref v) => {
                os.write_string(8, *v);
            },
            None => {},
        };
        match self.java_multiple_files {
            Some(ref v) => {
                os.write_bool(10, *v);
            },
            None => {},
        };
        match self.java_generate_equals_and_hash {
            Some(ref v) => {
                os.write_bool(20, *v);
            },
            None => {},
        };
        match self.optimize_for {
            Some(ref v) => {
                os.write_enum(9, *v as i32);
            },
            None => {},
        };
        match self.go_package {
            Some(ref v) => {
                os.write_string(11, *v);
            },
            None => {},
        };
        match self.cc_generic_services {
            Some(ref v) => {
                os.write_bool(16, *v);
            },
            None => {},
        };
        match self.java_generic_services {
            Some(ref v) => {
                os.write_bool(17, *v);
            },
            None => {},
        };
        match self.py_generic_services {
            Some(ref v) => {
                os.write_bool(18, *v);
            },
            None => {},
        };
        for self.uninterpreted_option.iter().advance |v| {
            os.write_tag(999, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_java_package(&mut self) {
        self.java_package = None;
    }

    pub fn clear_java_outer_classname(&mut self) {
        self.java_outer_classname = None;
    }

    pub fn clear_java_multiple_files(&mut self) {
        self.java_multiple_files = None;
    }

    pub fn clear_java_generate_equals_and_hash(&mut self) {
        self.java_generate_equals_and_hash = None;
    }

    pub fn clear_optimize_for(&mut self) {
        self.optimize_for = None;
    }

    pub fn clear_go_package(&mut self) {
        self.go_package = None;
    }

    pub fn clear_cc_generic_services(&mut self) {
        self.cc_generic_services = None;
    }

    pub fn clear_java_generic_services(&mut self) {
        self.java_generic_services = None;
    }

    pub fn clear_py_generic_services(&mut self) {
        self.py_generic_services = None;
    }

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }
}

impl Message for FileOptions {
    fn new() -> FileOptions {
        FileOptions::new()
    }

    fn clear(&mut self) {
        self.clear_java_package();
        self.clear_java_outer_classname();
        self.clear_java_multiple_files();
        self.clear_java_generate_equals_and_hash();
        self.clear_optimize_for();
        self.clear_go_package();
        self.clear_cc_generic_services();
        self.clear_java_generic_services();
        self.clear_py_generic_services();
        self.clear_uninterpreted_option();
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
                    self.java_package = Some(tmp);
                },
                8 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.java_outer_classname = Some(tmp);
                },
                10 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.java_multiple_files = Some(tmp);
                },
                20 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.java_generate_equals_and_hash = Some(tmp);
                },
                9 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = FileOptions_OptimizeMode::new(is.read_int32());
                    self.optimize_for = Some(tmp);
                },
                11 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.go_package = Some(tmp);
                },
                16 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.cc_generic_services = Some(tmp);
                },
                17 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.java_generic_services = Some(tmp);
                },
                18 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.py_generic_services = Some(tmp);
                },
                999 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = UninterpretedOption::new();
                    is.merge_message(&mut tmp);
                    self.uninterpreted_option.push(tmp);
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
        for self.java_package.iter().advance |value| {
            my_size += rt::string_size(1, *value);
        };
        for self.java_outer_classname.iter().advance |value| {
            my_size += rt::string_size(8, *value);
        };
        if self.java_multiple_files.is_some() {
            my_size += 2;
        };
        if self.java_generate_equals_and_hash.is_some() {
            my_size += 3;
        };
        for self.optimize_for.iter().advance |value| {
            my_size += rt::enum_size(9, *value);
        };
        for self.go_package.iter().advance |value| {
            my_size += rt::string_size(11, *value);
        };
        if self.cc_generic_services.is_some() {
            my_size += 3;
        };
        if self.java_generic_services.is_some() {
            my_size += 3;
        };
        if self.py_generic_services.is_some() {
            my_size += 3;
        };
        for self.uninterpreted_option.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 2 + rt::compute_raw_varint32_size(len) + len;
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
pub enum FileOptions_OptimizeMode {
    SPEED = 1,
    CODE_SIZE = 2,
    LITE_RUNTIME = 3,
}

impl FileOptions_OptimizeMode {
    pub fn new(value: i32) -> FileOptions_OptimizeMode {
        match value {
            1 => SPEED,
            2 => CODE_SIZE,
            3 => LITE_RUNTIME,
            _ => fail!()
        }
    }
}

impl ProtobufEnum for FileOptions_OptimizeMode {
    pub fn value(&self) -> i32 {
        *self as i32
    }
}

#[deriving(Clone,Eq)]
pub struct MessageOptions {
    message_set_wire_format: Option<bool>,
    no_standard_descriptor_accessor: Option<bool>,
    uninterpreted_option: ~[UninterpretedOption],
}

impl MessageOptions {
    pub fn new() -> MessageOptions {
        MessageOptions {
            message_set_wire_format: None,
            no_standard_descriptor_accessor: None,
            uninterpreted_option: ~[],
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.message_set_wire_format {
            Some(ref v) => {
                os.write_bool(1, *v);
            },
            None => {},
        };
        match self.no_standard_descriptor_accessor {
            Some(ref v) => {
                os.write_bool(2, *v);
            },
            None => {},
        };
        for self.uninterpreted_option.iter().advance |v| {
            os.write_tag(999, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_message_set_wire_format(&mut self) {
        self.message_set_wire_format = None;
    }

    pub fn clear_no_standard_descriptor_accessor(&mut self) {
        self.no_standard_descriptor_accessor = None;
    }

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }
}

impl Message for MessageOptions {
    fn new() -> MessageOptions {
        MessageOptions::new()
    }

    fn clear(&mut self) {
        self.clear_message_set_wire_format();
        self.clear_no_standard_descriptor_accessor();
        self.clear_uninterpreted_option();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.message_set_wire_format = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.no_standard_descriptor_accessor = Some(tmp);
                },
                999 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = UninterpretedOption::new();
                    is.merge_message(&mut tmp);
                    self.uninterpreted_option.push(tmp);
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
        if self.message_set_wire_format.is_some() {
            my_size += 2;
        };
        if self.no_standard_descriptor_accessor.is_some() {
            my_size += 2;
        };
        for self.uninterpreted_option.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 2 + rt::compute_raw_varint32_size(len) + len;
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
pub struct FieldOptions {
    ctype: Option<FieldOptions_CType>,
    packed: Option<bool>,
    lazy: Option<bool>,
    deprecated: Option<bool>,
    experimental_map_key: Option<~str>,
    weak: Option<bool>,
    uninterpreted_option: ~[UninterpretedOption],
}

impl FieldOptions {
    pub fn new() -> FieldOptions {
        FieldOptions {
            ctype: None,
            packed: None,
            lazy: None,
            deprecated: None,
            experimental_map_key: None,
            weak: None,
            uninterpreted_option: ~[],
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.ctype {
            Some(ref v) => {
                os.write_enum(1, *v as i32);
            },
            None => {},
        };
        match self.packed {
            Some(ref v) => {
                os.write_bool(2, *v);
            },
            None => {},
        };
        match self.lazy {
            Some(ref v) => {
                os.write_bool(5, *v);
            },
            None => {},
        };
        match self.deprecated {
            Some(ref v) => {
                os.write_bool(3, *v);
            },
            None => {},
        };
        match self.experimental_map_key {
            Some(ref v) => {
                os.write_string(9, *v);
            },
            None => {},
        };
        match self.weak {
            Some(ref v) => {
                os.write_bool(10, *v);
            },
            None => {},
        };
        for self.uninterpreted_option.iter().advance |v| {
            os.write_tag(999, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_ctype(&mut self) {
        self.ctype = None;
    }

    pub fn clear_packed(&mut self) {
        self.packed = None;
    }

    pub fn clear_lazy(&mut self) {
        self.lazy = None;
    }

    pub fn clear_deprecated(&mut self) {
        self.deprecated = None;
    }

    pub fn clear_experimental_map_key(&mut self) {
        self.experimental_map_key = None;
    }

    pub fn clear_weak(&mut self) {
        self.weak = None;
    }

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }
}

impl Message for FieldOptions {
    fn new() -> FieldOptions {
        FieldOptions::new()
    }

    fn clear(&mut self) {
        self.clear_ctype();
        self.clear_packed();
        self.clear_lazy();
        self.clear_deprecated();
        self.clear_experimental_map_key();
        self.clear_weak();
        self.clear_uninterpreted_option();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = FieldOptions_CType::new(is.read_int32());
                    self.ctype = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.packed = Some(tmp);
                },
                5 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.lazy = Some(tmp);
                },
                3 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.deprecated = Some(tmp);
                },
                9 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.experimental_map_key = Some(tmp);
                },
                10 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.weak = Some(tmp);
                },
                999 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = UninterpretedOption::new();
                    is.merge_message(&mut tmp);
                    self.uninterpreted_option.push(tmp);
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
        for self.ctype.iter().advance |value| {
            my_size += rt::enum_size(1, *value);
        };
        if self.packed.is_some() {
            my_size += 2;
        };
        if self.lazy.is_some() {
            my_size += 2;
        };
        if self.deprecated.is_some() {
            my_size += 2;
        };
        for self.experimental_map_key.iter().advance |value| {
            my_size += rt::string_size(9, *value);
        };
        if self.weak.is_some() {
            my_size += 2;
        };
        for self.uninterpreted_option.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 2 + rt::compute_raw_varint32_size(len) + len;
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
pub enum FieldOptions_CType {
    STRING = 0,
    CORD = 1,
    STRING_PIECE = 2,
}

impl FieldOptions_CType {
    pub fn new(value: i32) -> FieldOptions_CType {
        match value {
            0 => STRING,
            1 => CORD,
            2 => STRING_PIECE,
            _ => fail!()
        }
    }
}

impl ProtobufEnum for FieldOptions_CType {
    pub fn value(&self) -> i32 {
        *self as i32
    }
}

#[deriving(Clone,Eq)]
pub struct EnumOptions {
    allow_alias: Option<bool>,
    uninterpreted_option: ~[UninterpretedOption],
}

impl EnumOptions {
    pub fn new() -> EnumOptions {
        EnumOptions {
            allow_alias: None,
            uninterpreted_option: ~[],
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.allow_alias {
            Some(ref v) => {
                os.write_bool(2, *v);
            },
            None => {},
        };
        for self.uninterpreted_option.iter().advance |v| {
            os.write_tag(999, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_allow_alias(&mut self) {
        self.allow_alias = None;
    }

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }
}

impl Message for EnumOptions {
    fn new() -> EnumOptions {
        EnumOptions::new()
    }

    fn clear(&mut self) {
        self.clear_allow_alias();
        self.clear_uninterpreted_option();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                2 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.allow_alias = Some(tmp);
                },
                999 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = UninterpretedOption::new();
                    is.merge_message(&mut tmp);
                    self.uninterpreted_option.push(tmp);
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
        if self.allow_alias.is_some() {
            my_size += 2;
        };
        for self.uninterpreted_option.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 2 + rt::compute_raw_varint32_size(len) + len;
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
pub struct EnumValueOptions {
    uninterpreted_option: ~[UninterpretedOption],
}

impl EnumValueOptions {
    pub fn new() -> EnumValueOptions {
        EnumValueOptions {
            uninterpreted_option: ~[],
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for self.uninterpreted_option.iter().advance |v| {
            os.write_tag(999, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }
}

impl Message for EnumValueOptions {
    fn new() -> EnumValueOptions {
        EnumValueOptions::new()
    }

    fn clear(&mut self) {
        self.clear_uninterpreted_option();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                999 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = UninterpretedOption::new();
                    is.merge_message(&mut tmp);
                    self.uninterpreted_option.push(tmp);
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
        for self.uninterpreted_option.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 2 + rt::compute_raw_varint32_size(len) + len;
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
pub struct ServiceOptions {
    uninterpreted_option: ~[UninterpretedOption],
}

impl ServiceOptions {
    pub fn new() -> ServiceOptions {
        ServiceOptions {
            uninterpreted_option: ~[],
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for self.uninterpreted_option.iter().advance |v| {
            os.write_tag(999, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }
}

impl Message for ServiceOptions {
    fn new() -> ServiceOptions {
        ServiceOptions::new()
    }

    fn clear(&mut self) {
        self.clear_uninterpreted_option();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                999 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = UninterpretedOption::new();
                    is.merge_message(&mut tmp);
                    self.uninterpreted_option.push(tmp);
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
        for self.uninterpreted_option.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 2 + rt::compute_raw_varint32_size(len) + len;
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
pub struct MethodOptions {
    uninterpreted_option: ~[UninterpretedOption],
}

impl MethodOptions {
    pub fn new() -> MethodOptions {
        MethodOptions {
            uninterpreted_option: ~[],
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for self.uninterpreted_option.iter().advance |v| {
            os.write_tag(999, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }
}

impl Message for MethodOptions {
    fn new() -> MethodOptions {
        MethodOptions::new()
    }

    fn clear(&mut self) {
        self.clear_uninterpreted_option();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                999 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = UninterpretedOption::new();
                    is.merge_message(&mut tmp);
                    self.uninterpreted_option.push(tmp);
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
        for self.uninterpreted_option.iter().advance |value| {
            let len = value.compute_sizes(sizes);
            my_size += 2 + rt::compute_raw_varint32_size(len) + len;
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
pub struct UninterpretedOption {
    name: ~[UninterpretedOption_NamePart],
    identifier_value: Option<~str>,
    positive_int_value: Option<u64>,
    negative_int_value: Option<i64>,
    double_value: Option<f64>,
    string_value: Option<~[u8]>,
    aggregate_value: Option<~str>,
}

impl UninterpretedOption {
    pub fn new() -> UninterpretedOption {
        UninterpretedOption {
            name: ~[],
            identifier_value: None,
            positive_int_value: None,
            negative_int_value: None,
            double_value: None,
            string_value: None,
            aggregate_value: None,
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for self.name.iter().advance |v| {
            os.write_tag(2, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        match self.identifier_value {
            Some(ref v) => {
                os.write_string(3, *v);
            },
            None => {},
        };
        match self.positive_int_value {
            Some(ref v) => {
                os.write_uint64(4, *v);
            },
            None => {},
        };
        match self.negative_int_value {
            Some(ref v) => {
                os.write_int64(5, *v);
            },
            None => {},
        };
        match self.double_value {
            Some(ref v) => {
                os.write_double(6, *v);
            },
            None => {},
        };
        match self.string_value {
            Some(ref v) => {
                os.write_bytes(7, *v);
            },
            None => {},
        };
        match self.aggregate_value {
            Some(ref v) => {
                os.write_string(8, *v);
            },
            None => {},
        };
    }

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn clear_identifier_value(&mut self) {
        self.identifier_value = None;
    }

    pub fn clear_positive_int_value(&mut self) {
        self.positive_int_value = None;
    }

    pub fn clear_negative_int_value(&mut self) {
        self.negative_int_value = None;
    }

    pub fn clear_double_value(&mut self) {
        self.double_value = None;
    }

    pub fn clear_string_value(&mut self) {
        self.string_value = None;
    }

    pub fn clear_aggregate_value(&mut self) {
        self.aggregate_value = None;
    }
}

impl Message for UninterpretedOption {
    fn new() -> UninterpretedOption {
        UninterpretedOption::new()
    }

    fn clear(&mut self) {
        self.clear_name();
        self.clear_identifier_value();
        self.clear_positive_int_value();
        self.clear_negative_int_value();
        self.clear_double_value();
        self.clear_string_value();
        self.clear_aggregate_value();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                2 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let mut tmp = UninterpretedOption_NamePart::new();
                    is.merge_message(&mut tmp);
                    self.name.push(tmp);
                },
                3 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.identifier_value = Some(tmp);
                },
                4 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_uint64();
                    self.positive_int_value = Some(tmp);
                },
                5 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int64();
                    self.negative_int_value = Some(tmp);
                },
                6 => {
                    assert_eq!(wire_format::WireTypeFixed64, wire_type);
                    let tmp = is.read_double();
                    self.double_value = Some(tmp);
                },
                7 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_bytes();
                    self.string_value = Some(tmp);
                },
                8 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.aggregate_value = Some(tmp);
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
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for self.identifier_value.iter().advance |value| {
            my_size += rt::string_size(3, *value);
        };
        for self.positive_int_value.iter().advance |value| {
            my_size += rt::value_size(4, *value, wire_format::WireTypeVarint);
        };
        for self.negative_int_value.iter().advance |value| {
            my_size += rt::value_size(5, *value, wire_format::WireTypeVarint);
        };
        if self.double_value.is_some() {
            my_size += 9;
        };
        for self.string_value.iter().advance |value| {
            my_size += rt::bytes_size(7, *value);
        };
        for self.aggregate_value.iter().advance |value| {
            my_size += rt::string_size(8, *value);
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
pub struct UninterpretedOption_NamePart {
    name_part: Option<~str>,
    is_extension: Option<bool>,
}

impl UninterpretedOption_NamePart {
    pub fn new() -> UninterpretedOption_NamePart {
        UninterpretedOption_NamePart {
            name_part: None,
            is_extension: None,
        }
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.name_part {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        match self.is_extension {
            Some(ref v) => {
                os.write_bool(2, *v);
            },
            None => {},
        };
    }

    pub fn clear_name_part(&mut self) {
        self.name_part = None;
    }

    pub fn clear_is_extension(&mut self) {
        self.is_extension = None;
    }
}

impl Message for UninterpretedOption_NamePart {
    fn new() -> UninterpretedOption_NamePart {
        UninterpretedOption_NamePart::new()
    }

    fn clear(&mut self) {
        self.clear_name_part();
        self.clear_is_extension();
    }

    fn is_initialized(&self) -> bool {
        if self.name_part.is_none() {
            return false;
        };
        if self.is_extension.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.name_part = Some(tmp);
                },
                2 => {
                    assert_eq!(wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.is_extension = Some(tmp);
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
        for self.name_part.iter().advance |value| {
            my_size += rt::string_size(1, *value);
        };
        if self.is_extension.is_some() {
            my_size += 2;
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
pub struct SourceCodeInfo {
    location: ~[SourceCodeInfo_Location],
}

impl SourceCodeInfo {
    pub fn new() -> SourceCodeInfo {
        SourceCodeInfo {
            location: ~[],
        }
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for self.location.iter().advance |v| {
            os.write_tag(1, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_location(&mut self) {
        self.location.clear();
    }
}

impl Message for SourceCodeInfo {
    fn new() -> SourceCodeInfo {
        SourceCodeInfo::new()
    }

    fn clear(&mut self) {
        self.clear_location();
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
                    let mut tmp = SourceCodeInfo_Location::new();
                    is.merge_message(&mut tmp);
                    self.location.push(tmp);
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
        for self.location.iter().advance |value| {
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
pub struct SourceCodeInfo_Location {
    path: ~[i32],
    span: ~[i32],
    leading_comments: Option<~str>,
    trailing_comments: Option<~str>,
}

impl SourceCodeInfo_Location {
    pub fn new() -> SourceCodeInfo_Location {
        SourceCodeInfo_Location {
            path: ~[],
            span: ~[],
            leading_comments: None,
            trailing_comments: None,
        }
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        if !self.path.is_empty() {
            os.write_tag(1, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(rt::vec_packed_data_size(self.path, wire_format::WireTypeVarint));
            for self.path.iter().advance |v| {
                os.write_int32_no_tag(*v);
            };
        };
        if !self.span.is_empty() {
            os.write_tag(2, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(rt::vec_packed_data_size(self.span, wire_format::WireTypeVarint));
            for self.span.iter().advance |v| {
                os.write_int32_no_tag(*v);
            };
        };
        match self.leading_comments {
            Some(ref v) => {
                os.write_string(3, *v);
            },
            None => {},
        };
        match self.trailing_comments {
            Some(ref v) => {
                os.write_string(4, *v);
            },
            None => {},
        };
    }

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn clear_span(&mut self) {
        self.span.clear();
    }

    pub fn clear_leading_comments(&mut self) {
        self.leading_comments = None;
    }

    pub fn clear_trailing_comments(&mut self) {
        self.trailing_comments = None;
    }
}

impl Message for SourceCodeInfo_Location {
    fn new() -> SourceCodeInfo_Location {
        SourceCodeInfo_Location::new()
    }

    fn clear(&mut self) {
        self.clear_path();
        self.clear_span();
        self.clear_leading_comments();
        self.clear_trailing_comments();
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    if wire_type == wire_format::WireTypeLengthDelimited {
                        let len = is.read_raw_varint32();
                        let old_limit = is.push_limit(len);
                        while !is.eof() {
                            self.path.push(is.read_int32());
                        }
                        is.pop_limit(old_limit);
                    } else {
                        assert_eq!(wire_format::WireTypeVarint, wire_type);
                        self.path.push(is.read_int32());
                    }
                },
                2 => {
                    if wire_type == wire_format::WireTypeLengthDelimited {
                        let len = is.read_raw_varint32();
                        let old_limit = is.push_limit(len);
                        while !is.eof() {
                            self.span.push(is.read_int32());
                        }
                        is.pop_limit(old_limit);
                    } else {
                        assert_eq!(wire_format::WireTypeVarint, wire_type);
                        self.span.push(is.read_int32());
                    }
                },
                3 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.leading_comments = Some(tmp);
                },
                4 => {
                    assert_eq!(wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = is.read_string();
                    self.trailing_comments = Some(tmp);
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
        my_size += rt::vec_packed_size(1, self.path, wire_format::WireTypeVarint);
        my_size += rt::vec_packed_size(2, self.span, wire_format::WireTypeVarint);
        for self.leading_comments.iter().advance |value| {
            my_size += rt::string_size(3, *value);
        };
        for self.trailing_comments.iter().advance |value| {
            my_size += rt::string_size(4, *value);
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

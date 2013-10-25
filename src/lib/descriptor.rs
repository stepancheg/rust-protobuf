// This file is generated. Do not edit

use protobuf::*;
use protobuf::rt;

#[deriving(Clone,Eq)]
pub struct FileDescriptorSet {
    file: ~[FileDescriptorProto],
}

impl<'self> FileDescriptorSet {
    pub fn new() -> FileDescriptorSet {
        FileDescriptorSet {
            file: ~[],
        }
    }

    pub fn default_instance() -> &'static FileDescriptorSet {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: FileDescriptorSet = FileDescriptorSet {
//             file: ~[],
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for v in self.file.iter() {
            os.write_tag(1, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_file(&mut self) {
        self.file.clear();
    }

    // Param is passed by value, moved
    pub fn set_file(&mut self, v: ~[FileDescriptorProto]) {
        self.file = v;
    }

    // Mutable pointer to the field.
    pub fn mut_file(&'self mut self) -> &'self mut ~[FileDescriptorProto] {
        &mut self.file
    }

    pub fn get_file(&'self self) -> &'self [FileDescriptorProto] {
        rt::as_slice_tmp(&self.file)
    }

    pub fn add_file(&mut self, v: FileDescriptorProto) {
        self.file.push(v);
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

impl<'self> FileDescriptorProto {
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

    pub fn default_instance() -> &'static FileDescriptorProto {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: FileDescriptorProto = FileDescriptorProto {
//             name: None,
//             package: None,
//             dependency: ~[],
//             public_dependency: ~[],
//             weak_dependency: ~[],
//             message_type: ~[],
//             enum_type: ~[],
//             service: ~[],
//             extension: ~[],
//             options: None,
//             source_code_info: None,
//         };
//         &'static instance
        fail!("TODO");
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
        for v in self.dependency.iter() {
            os.write_string(3, *v);
        };
        for v in self.public_dependency.iter() {
            os.write_int32(10, *v);
        };
        for v in self.weak_dependency.iter() {
            os.write_int32(11, *v);
        };
        for v in self.message_type.iter() {
            os.write_tag(4, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        for v in self.enum_type.iter() {
            os.write_tag(5, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        for v in self.service.iter() {
            os.write_tag(6, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        for v in self.extension.iter() {
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

    pub fn clear_package(&mut self) {
        self.package = None;
    }

    pub fn has_package(&self) -> bool {
        self.package.is_some()
    }

    // Param is passed by value, moved
    pub fn set_package(&mut self, v: ~str) {
        self.package = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_package(&'self mut self) -> &'self mut ~str {
        if self.package.is_none() {
            self.package = Some(~"");
        };
        self.package.get_mut_ref()
    }

    pub fn get_package(&'self self) -> &'self str {
        match self.package {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_dependency(&mut self) {
        self.dependency.clear();
    }

    // Param is passed by value, moved
    pub fn set_dependency(&mut self, v: ~[~str]) {
        self.dependency = v;
    }

    // Mutable pointer to the field.
    pub fn mut_dependency(&'self mut self) -> &'self mut ~[~str] {
        &mut self.dependency
    }

    pub fn get_dependency(&'self self) -> &'self [~str] {
        rt::as_slice_tmp(&self.dependency)
    }

    pub fn add_dependency(&mut self, v: ~str) {
        self.dependency.push(v);
    }

    pub fn clear_public_dependency(&mut self) {
        self.public_dependency.clear();
    }

    // Param is passed by value, moved
    pub fn set_public_dependency(&mut self, v: ~[i32]) {
        self.public_dependency = v;
    }

    // Mutable pointer to the field.
    pub fn mut_public_dependency(&'self mut self) -> &'self mut ~[i32] {
        &mut self.public_dependency
    }

    pub fn get_public_dependency(&'self self) -> &'self [i32] {
        rt::as_slice_tmp(&self.public_dependency)
    }

    pub fn add_public_dependency(&mut self, v: i32) {
        self.public_dependency.push(v);
    }

    pub fn clear_weak_dependency(&mut self) {
        self.weak_dependency.clear();
    }

    // Param is passed by value, moved
    pub fn set_weak_dependency(&mut self, v: ~[i32]) {
        self.weak_dependency = v;
    }

    // Mutable pointer to the field.
    pub fn mut_weak_dependency(&'self mut self) -> &'self mut ~[i32] {
        &mut self.weak_dependency
    }

    pub fn get_weak_dependency(&'self self) -> &'self [i32] {
        rt::as_slice_tmp(&self.weak_dependency)
    }

    pub fn add_weak_dependency(&mut self, v: i32) {
        self.weak_dependency.push(v);
    }

    pub fn clear_message_type(&mut self) {
        self.message_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_message_type(&mut self, v: ~[DescriptorProto]) {
        self.message_type = v;
    }

    // Mutable pointer to the field.
    pub fn mut_message_type(&'self mut self) -> &'self mut ~[DescriptorProto] {
        &mut self.message_type
    }

    pub fn get_message_type(&'self self) -> &'self [DescriptorProto] {
        rt::as_slice_tmp(&self.message_type)
    }

    pub fn add_message_type(&mut self, v: DescriptorProto) {
        self.message_type.push(v);
    }

    pub fn clear_enum_type(&mut self) {
        self.enum_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_enum_type(&mut self, v: ~[EnumDescriptorProto]) {
        self.enum_type = v;
    }

    // Mutable pointer to the field.
    pub fn mut_enum_type(&'self mut self) -> &'self mut ~[EnumDescriptorProto] {
        &mut self.enum_type
    }

    pub fn get_enum_type(&'self self) -> &'self [EnumDescriptorProto] {
        rt::as_slice_tmp(&self.enum_type)
    }

    pub fn add_enum_type(&mut self, v: EnumDescriptorProto) {
        self.enum_type.push(v);
    }

    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: ~[ServiceDescriptorProto]) {
        self.service = v;
    }

    // Mutable pointer to the field.
    pub fn mut_service(&'self mut self) -> &'self mut ~[ServiceDescriptorProto] {
        &mut self.service
    }

    pub fn get_service(&'self self) -> &'self [ServiceDescriptorProto] {
        rt::as_slice_tmp(&self.service)
    }

    pub fn add_service(&mut self, v: ServiceDescriptorProto) {
        self.service.push(v);
    }

    pub fn clear_extension(&mut self) {
        self.extension.clear();
    }

    // Param is passed by value, moved
    pub fn set_extension(&mut self, v: ~[FieldDescriptorProto]) {
        self.extension = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extension(&'self mut self) -> &'self mut ~[FieldDescriptorProto] {
        &mut self.extension
    }

    pub fn get_extension(&'self self) -> &'self [FieldDescriptorProto] {
        rt::as_slice_tmp(&self.extension)
    }

    pub fn add_extension(&mut self, v: FieldDescriptorProto) {
        self.extension.push(v);
    }

    pub fn clear_options(&mut self) {
        self.options = None;
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: FileOptions) {
        self.options = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&'self mut self) -> &'self mut FileOptions {
        if self.options.is_none() {
            self.options = Some(FileOptions::new());
        };
        self.options.get_mut_ref()
    }

    pub fn get_options(&'self self) -> &'self FileOptions {
        match self.options {
            Some(ref v) => v,
            None => FileOptions::default_instance(),
        }
    }

    pub fn clear_source_code_info(&mut self) {
        self.source_code_info = None;
    }

    pub fn has_source_code_info(&self) -> bool {
        self.source_code_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_code_info(&mut self, v: SourceCodeInfo) {
        self.source_code_info = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source_code_info(&'self mut self) -> &'self mut SourceCodeInfo {
        if self.source_code_info.is_none() {
            self.source_code_info = Some(SourceCodeInfo::new());
        };
        self.source_code_info.get_mut_ref()
    }

    pub fn get_source_code_info(&'self self) -> &'self SourceCodeInfo {
        match self.source_code_info {
            Some(ref v) => v,
            None => SourceCodeInfo::default_instance(),
        }
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
        for value in self.name.iter() {
            my_size += rt::string_size(1, *value);
        };
        for value in self.package.iter() {
            my_size += rt::string_size(2, *value);
        };
        for value in self.dependency.iter() {
            my_size += rt::string_size(3, *value);
        };
        for value in self.public_dependency.iter() {
            my_size += rt::value_size(10, *value, wire_format::WireTypeVarint);
        };
        for value in self.weak_dependency.iter() {
            my_size += rt::value_size(11, *value, wire_format::WireTypeVarint);
        };
        for value in self.message_type.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.enum_type.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.service.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.extension.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.options.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.source_code_info.iter() {
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
pub struct DescriptorProto {
    name: Option<~str>,
    field: ~[FieldDescriptorProto],
    extension: ~[FieldDescriptorProto],
    nested_type: ~[DescriptorProto],
    enum_type: ~[EnumDescriptorProto],
    extension_range: ~[DescriptorProto_ExtensionRange],
    options: Option<MessageOptions>,
}

impl<'self> DescriptorProto {
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

    pub fn default_instance() -> &'static DescriptorProto {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: DescriptorProto = DescriptorProto {
//             name: None,
//             field: ~[],
//             extension: ~[],
//             nested_type: ~[],
//             enum_type: ~[],
//             extension_range: ~[],
//             options: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.name {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        for v in self.field.iter() {
            os.write_tag(2, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        for v in self.extension.iter() {
            os.write_tag(6, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        for v in self.nested_type.iter() {
            os.write_tag(3, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        for v in self.enum_type.iter() {
            os.write_tag(4, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
        for v in self.extension_range.iter() {
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

    pub fn clear_field(&mut self) {
        self.field.clear();
    }

    // Param is passed by value, moved
    pub fn set_field(&mut self, v: ~[FieldDescriptorProto]) {
        self.field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_field(&'self mut self) -> &'self mut ~[FieldDescriptorProto] {
        &mut self.field
    }

    pub fn get_field(&'self self) -> &'self [FieldDescriptorProto] {
        rt::as_slice_tmp(&self.field)
    }

    pub fn add_field(&mut self, v: FieldDescriptorProto) {
        self.field.push(v);
    }

    pub fn clear_extension(&mut self) {
        self.extension.clear();
    }

    // Param is passed by value, moved
    pub fn set_extension(&mut self, v: ~[FieldDescriptorProto]) {
        self.extension = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extension(&'self mut self) -> &'self mut ~[FieldDescriptorProto] {
        &mut self.extension
    }

    pub fn get_extension(&'self self) -> &'self [FieldDescriptorProto] {
        rt::as_slice_tmp(&self.extension)
    }

    pub fn add_extension(&mut self, v: FieldDescriptorProto) {
        self.extension.push(v);
    }

    pub fn clear_nested_type(&mut self) {
        self.nested_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_nested_type(&mut self, v: ~[DescriptorProto]) {
        self.nested_type = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nested_type(&'self mut self) -> &'self mut ~[DescriptorProto] {
        &mut self.nested_type
    }

    pub fn get_nested_type(&'self self) -> &'self [DescriptorProto] {
        rt::as_slice_tmp(&self.nested_type)
    }

    pub fn add_nested_type(&mut self, v: DescriptorProto) {
        self.nested_type.push(v);
    }

    pub fn clear_enum_type(&mut self) {
        self.enum_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_enum_type(&mut self, v: ~[EnumDescriptorProto]) {
        self.enum_type = v;
    }

    // Mutable pointer to the field.
    pub fn mut_enum_type(&'self mut self) -> &'self mut ~[EnumDescriptorProto] {
        &mut self.enum_type
    }

    pub fn get_enum_type(&'self self) -> &'self [EnumDescriptorProto] {
        rt::as_slice_tmp(&self.enum_type)
    }

    pub fn add_enum_type(&mut self, v: EnumDescriptorProto) {
        self.enum_type.push(v);
    }

    pub fn clear_extension_range(&mut self) {
        self.extension_range.clear();
    }

    // Param is passed by value, moved
    pub fn set_extension_range(&mut self, v: ~[DescriptorProto_ExtensionRange]) {
        self.extension_range = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extension_range(&'self mut self) -> &'self mut ~[DescriptorProto_ExtensionRange] {
        &mut self.extension_range
    }

    pub fn get_extension_range(&'self self) -> &'self [DescriptorProto_ExtensionRange] {
        rt::as_slice_tmp(&self.extension_range)
    }

    pub fn add_extension_range(&mut self, v: DescriptorProto_ExtensionRange) {
        self.extension_range.push(v);
    }

    pub fn clear_options(&mut self) {
        self.options = None;
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: MessageOptions) {
        self.options = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&'self mut self) -> &'self mut MessageOptions {
        if self.options.is_none() {
            self.options = Some(MessageOptions::new());
        };
        self.options.get_mut_ref()
    }

    pub fn get_options(&'self self) -> &'self MessageOptions {
        match self.options {
            Some(ref v) => v,
            None => MessageOptions::default_instance(),
        }
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
        for value in self.name.iter() {
            my_size += rt::string_size(1, *value);
        };
        for value in self.field.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.extension.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.nested_type.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.enum_type.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.extension_range.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.options.iter() {
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
pub struct DescriptorProto_ExtensionRange {
    start: Option<i32>,
    end: Option<i32>,
}

impl<'self> DescriptorProto_ExtensionRange {
    pub fn new() -> DescriptorProto_ExtensionRange {
        DescriptorProto_ExtensionRange {
            start: None,
            end: None,
        }
    }

    pub fn default_instance() -> &'static DescriptorProto_ExtensionRange {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: DescriptorProto_ExtensionRange = DescriptorProto_ExtensionRange {
//             start: None,
//             end: None,
//         };
//         &'static instance
        fail!("TODO");
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

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: i32) {
        self.start = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start(&'self mut self) -> &'self mut i32 {
        if self.start.is_none() {
            self.start = Some(0);
        };
        self.start.get_mut_ref()
    }

    pub fn get_start(&self) -> i32 {
        self.start.unwrap_or(0)
    }

    pub fn clear_end(&mut self) {
        self.end = None;
    }

    pub fn has_end(&self) -> bool {
        self.end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: i32) {
        self.end = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end(&'self mut self) -> &'self mut i32 {
        if self.end.is_none() {
            self.end = Some(0);
        };
        self.end.get_mut_ref()
    }

    pub fn get_end(&self) -> i32 {
        self.end.unwrap_or(0)
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
        for value in self.start.iter() {
            my_size += rt::value_size(1, *value, wire_format::WireTypeVarint);
        };
        for value in self.end.iter() {
            my_size += rt::value_size(2, *value, wire_format::WireTypeVarint);
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

impl<'self> FieldDescriptorProto {
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

    pub fn default_instance() -> &'static FieldDescriptorProto {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: FieldDescriptorProto = FieldDescriptorProto {
//             name: None,
//             number: None,
//             label: None,
//             field_type: None,
//             type_name: None,
//             extendee: None,
//             default_value: None,
//             options: None,
//         };
//         &'static instance
        fail!("TODO");
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

    pub fn clear_number(&mut self) {
        self.number = None;
    }

    pub fn has_number(&self) -> bool {
        self.number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: i32) {
        self.number = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_number(&'self mut self) -> &'self mut i32 {
        if self.number.is_none() {
            self.number = Some(0);
        };
        self.number.get_mut_ref()
    }

    pub fn get_number(&self) -> i32 {
        self.number.unwrap_or(0)
    }

    pub fn clear_label(&mut self) {
        self.label = None;
    }

    pub fn has_label(&self) -> bool {
        self.label.is_some()
    }

    // Param is passed by value, moved
    pub fn set_label(&mut self, v: FieldDescriptorProto_Label) {
        self.label = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_label(&'self mut self) -> &'self mut FieldDescriptorProto_Label {
        if self.label.is_none() {
            self.label = Some(FieldDescriptorProto_Label::new(0));
        };
        self.label.get_mut_ref()
    }

    pub fn get_label(&self) -> FieldDescriptorProto_Label {
        self.label.unwrap_or(FieldDescriptorProto_Label::new(0))
    }

    pub fn clear_field_type(&mut self) {
        self.field_type = None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: FieldDescriptorProto_Type) {
        self.field_type = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&'self mut self) -> &'self mut FieldDescriptorProto_Type {
        if self.field_type.is_none() {
            self.field_type = Some(FieldDescriptorProto_Type::new(0));
        };
        self.field_type.get_mut_ref()
    }

    pub fn get_field_type(&self) -> FieldDescriptorProto_Type {
        self.field_type.unwrap_or(FieldDescriptorProto_Type::new(0))
    }

    pub fn clear_type_name(&mut self) {
        self.type_name = None;
    }

    pub fn has_type_name(&self) -> bool {
        self.type_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_type_name(&mut self, v: ~str) {
        self.type_name = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_type_name(&'self mut self) -> &'self mut ~str {
        if self.type_name.is_none() {
            self.type_name = Some(~"");
        };
        self.type_name.get_mut_ref()
    }

    pub fn get_type_name(&'self self) -> &'self str {
        match self.type_name {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_extendee(&mut self) {
        self.extendee = None;
    }

    pub fn has_extendee(&self) -> bool {
        self.extendee.is_some()
    }

    // Param is passed by value, moved
    pub fn set_extendee(&mut self, v: ~str) {
        self.extendee = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_extendee(&'self mut self) -> &'self mut ~str {
        if self.extendee.is_none() {
            self.extendee = Some(~"");
        };
        self.extendee.get_mut_ref()
    }

    pub fn get_extendee(&'self self) -> &'self str {
        match self.extendee {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_default_value(&mut self) {
        self.default_value = None;
    }

    pub fn has_default_value(&self) -> bool {
        self.default_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_default_value(&mut self, v: ~str) {
        self.default_value = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_default_value(&'self mut self) -> &'self mut ~str {
        if self.default_value.is_none() {
            self.default_value = Some(~"");
        };
        self.default_value.get_mut_ref()
    }

    pub fn get_default_value(&'self self) -> &'self str {
        match self.default_value {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_options(&mut self) {
        self.options = None;
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: FieldOptions) {
        self.options = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&'self mut self) -> &'self mut FieldOptions {
        if self.options.is_none() {
            self.options = Some(FieldOptions::new());
        };
        self.options.get_mut_ref()
    }

    pub fn get_options(&'self self) -> &'self FieldOptions {
        match self.options {
            Some(ref v) => v,
            None => FieldOptions::default_instance(),
        }
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
        for value in self.name.iter() {
            my_size += rt::string_size(1, *value);
        };
        for value in self.number.iter() {
            my_size += rt::value_size(3, *value, wire_format::WireTypeVarint);
        };
        for value in self.label.iter() {
            my_size += rt::enum_size(4, *value);
        };
        for value in self.field_type.iter() {
            my_size += rt::enum_size(5, *value);
        };
        for value in self.type_name.iter() {
            my_size += rt::string_size(6, *value);
        };
        for value in self.extendee.iter() {
            my_size += rt::string_size(2, *value);
        };
        for value in self.default_value.iter() {
            my_size += rt::string_size(7, *value);
        };
        for value in self.options.iter() {
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
    fn value(&self) -> i32 {
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
    fn value(&self) -> i32 {
        *self as i32
    }
}

#[deriving(Clone,Eq)]
pub struct EnumDescriptorProto {
    name: Option<~str>,
    value: ~[EnumValueDescriptorProto],
    options: Option<EnumOptions>,
}

impl<'self> EnumDescriptorProto {
    pub fn new() -> EnumDescriptorProto {
        EnumDescriptorProto {
            name: None,
            value: ~[],
            options: None,
        }
    }

    pub fn default_instance() -> &'static EnumDescriptorProto {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: EnumDescriptorProto = EnumDescriptorProto {
//             name: None,
//             value: ~[],
//             options: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.name {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        for v in self.value.iter() {
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

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ~[EnumValueDescriptorProto]) {
        self.value = v;
    }

    // Mutable pointer to the field.
    pub fn mut_value(&'self mut self) -> &'self mut ~[EnumValueDescriptorProto] {
        &mut self.value
    }

    pub fn get_value(&'self self) -> &'self [EnumValueDescriptorProto] {
        rt::as_slice_tmp(&self.value)
    }

    pub fn add_value(&mut self, v: EnumValueDescriptorProto) {
        self.value.push(v);
    }

    pub fn clear_options(&mut self) {
        self.options = None;
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: EnumOptions) {
        self.options = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&'self mut self) -> &'self mut EnumOptions {
        if self.options.is_none() {
            self.options = Some(EnumOptions::new());
        };
        self.options.get_mut_ref()
    }

    pub fn get_options(&'self self) -> &'self EnumOptions {
        match self.options {
            Some(ref v) => v,
            None => EnumOptions::default_instance(),
        }
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
        for value in self.name.iter() {
            my_size += rt::string_size(1, *value);
        };
        for value in self.value.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.options.iter() {
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
pub struct EnumValueDescriptorProto {
    name: Option<~str>,
    number: Option<i32>,
    options: Option<EnumValueOptions>,
}

impl<'self> EnumValueDescriptorProto {
    pub fn new() -> EnumValueDescriptorProto {
        EnumValueDescriptorProto {
            name: None,
            number: None,
            options: None,
        }
    }

    pub fn default_instance() -> &'static EnumValueDescriptorProto {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: EnumValueDescriptorProto = EnumValueDescriptorProto {
//             name: None,
//             number: None,
//             options: None,
//         };
//         &'static instance
        fail!("TODO");
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

    pub fn clear_number(&mut self) {
        self.number = None;
    }

    pub fn has_number(&self) -> bool {
        self.number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: i32) {
        self.number = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_number(&'self mut self) -> &'self mut i32 {
        if self.number.is_none() {
            self.number = Some(0);
        };
        self.number.get_mut_ref()
    }

    pub fn get_number(&self) -> i32 {
        self.number.unwrap_or(0)
    }

    pub fn clear_options(&mut self) {
        self.options = None;
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: EnumValueOptions) {
        self.options = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&'self mut self) -> &'self mut EnumValueOptions {
        if self.options.is_none() {
            self.options = Some(EnumValueOptions::new());
        };
        self.options.get_mut_ref()
    }

    pub fn get_options(&'self self) -> &'self EnumValueOptions {
        match self.options {
            Some(ref v) => v,
            None => EnumValueOptions::default_instance(),
        }
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
        for value in self.name.iter() {
            my_size += rt::string_size(1, *value);
        };
        for value in self.number.iter() {
            my_size += rt::value_size(2, *value, wire_format::WireTypeVarint);
        };
        for value in self.options.iter() {
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
pub struct ServiceDescriptorProto {
    name: Option<~str>,
    method: ~[MethodDescriptorProto],
    options: Option<ServiceOptions>,
}

impl<'self> ServiceDescriptorProto {
    pub fn new() -> ServiceDescriptorProto {
        ServiceDescriptorProto {
            name: None,
            method: ~[],
            options: None,
        }
    }

    pub fn default_instance() -> &'static ServiceDescriptorProto {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: ServiceDescriptorProto = ServiceDescriptorProto {
//             name: None,
//             method: ~[],
//             options: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.name {
            Some(ref v) => {
                os.write_string(1, *v);
            },
            None => {},
        };
        for v in self.method.iter() {
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

    pub fn clear_method(&mut self) {
        self.method.clear();
    }

    // Param is passed by value, moved
    pub fn set_method(&mut self, v: ~[MethodDescriptorProto]) {
        self.method = v;
    }

    // Mutable pointer to the field.
    pub fn mut_method(&'self mut self) -> &'self mut ~[MethodDescriptorProto] {
        &mut self.method
    }

    pub fn get_method(&'self self) -> &'self [MethodDescriptorProto] {
        rt::as_slice_tmp(&self.method)
    }

    pub fn add_method(&mut self, v: MethodDescriptorProto) {
        self.method.push(v);
    }

    pub fn clear_options(&mut self) {
        self.options = None;
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: ServiceOptions) {
        self.options = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&'self mut self) -> &'self mut ServiceOptions {
        if self.options.is_none() {
            self.options = Some(ServiceOptions::new());
        };
        self.options.get_mut_ref()
    }

    pub fn get_options(&'self self) -> &'self ServiceOptions {
        match self.options {
            Some(ref v) => v,
            None => ServiceOptions::default_instance(),
        }
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
        for value in self.name.iter() {
            my_size += rt::string_size(1, *value);
        };
        for value in self.method.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.options.iter() {
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
pub struct MethodDescriptorProto {
    name: Option<~str>,
    input_type: Option<~str>,
    output_type: Option<~str>,
    options: Option<MethodOptions>,
}

impl<'self> MethodDescriptorProto {
    pub fn new() -> MethodDescriptorProto {
        MethodDescriptorProto {
            name: None,
            input_type: None,
            output_type: None,
            options: None,
        }
    }

    pub fn default_instance() -> &'static MethodDescriptorProto {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: MethodDescriptorProto = MethodDescriptorProto {
//             name: None,
//             input_type: None,
//             output_type: None,
//             options: None,
//         };
//         &'static instance
        fail!("TODO");
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

    pub fn clear_input_type(&mut self) {
        self.input_type = None;
    }

    pub fn has_input_type(&self) -> bool {
        self.input_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input_type(&mut self, v: ~str) {
        self.input_type = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_input_type(&'self mut self) -> &'self mut ~str {
        if self.input_type.is_none() {
            self.input_type = Some(~"");
        };
        self.input_type.get_mut_ref()
    }

    pub fn get_input_type(&'self self) -> &'self str {
        match self.input_type {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_output_type(&mut self) {
        self.output_type = None;
    }

    pub fn has_output_type(&self) -> bool {
        self.output_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_output_type(&mut self, v: ~str) {
        self.output_type = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_output_type(&'self mut self) -> &'self mut ~str {
        if self.output_type.is_none() {
            self.output_type = Some(~"");
        };
        self.output_type.get_mut_ref()
    }

    pub fn get_output_type(&'self self) -> &'self str {
        match self.output_type {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_options(&mut self) {
        self.options = None;
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: MethodOptions) {
        self.options = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&'self mut self) -> &'self mut MethodOptions {
        if self.options.is_none() {
            self.options = Some(MethodOptions::new());
        };
        self.options.get_mut_ref()
    }

    pub fn get_options(&'self self) -> &'self MethodOptions {
        match self.options {
            Some(ref v) => v,
            None => MethodOptions::default_instance(),
        }
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
        for value in self.name.iter() {
            my_size += rt::string_size(1, *value);
        };
        for value in self.input_type.iter() {
            my_size += rt::string_size(2, *value);
        };
        for value in self.output_type.iter() {
            my_size += rt::string_size(3, *value);
        };
        for value in self.options.iter() {
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

impl<'self> FileOptions {
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

    pub fn default_instance() -> &'static FileOptions {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: FileOptions = FileOptions {
//             java_package: None,
//             java_outer_classname: None,
//             java_multiple_files: None,
//             java_generate_equals_and_hash: None,
//             optimize_for: None,
//             go_package: None,
//             cc_generic_services: None,
//             java_generic_services: None,
//             py_generic_services: None,
//             uninterpreted_option: ~[],
//         };
//         &'static instance
        fail!("TODO");
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
        for v in self.uninterpreted_option.iter() {
            os.write_tag(999, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_java_package(&mut self) {
        self.java_package = None;
    }

    pub fn has_java_package(&self) -> bool {
        self.java_package.is_some()
    }

    // Param is passed by value, moved
    pub fn set_java_package(&mut self, v: ~str) {
        self.java_package = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_java_package(&'self mut self) -> &'self mut ~str {
        if self.java_package.is_none() {
            self.java_package = Some(~"");
        };
        self.java_package.get_mut_ref()
    }

    pub fn get_java_package(&'self self) -> &'self str {
        match self.java_package {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_java_outer_classname(&mut self) {
        self.java_outer_classname = None;
    }

    pub fn has_java_outer_classname(&self) -> bool {
        self.java_outer_classname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_java_outer_classname(&mut self, v: ~str) {
        self.java_outer_classname = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_java_outer_classname(&'self mut self) -> &'self mut ~str {
        if self.java_outer_classname.is_none() {
            self.java_outer_classname = Some(~"");
        };
        self.java_outer_classname.get_mut_ref()
    }

    pub fn get_java_outer_classname(&'self self) -> &'self str {
        match self.java_outer_classname {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_java_multiple_files(&mut self) {
        self.java_multiple_files = None;
    }

    pub fn has_java_multiple_files(&self) -> bool {
        self.java_multiple_files.is_some()
    }

    // Param is passed by value, moved
    pub fn set_java_multiple_files(&mut self, v: bool) {
        self.java_multiple_files = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_java_multiple_files(&'self mut self) -> &'self mut bool {
        if self.java_multiple_files.is_none() {
            self.java_multiple_files = Some(false);
        };
        self.java_multiple_files.get_mut_ref()
    }

    pub fn get_java_multiple_files(&self) -> bool {
        self.java_multiple_files.unwrap_or(false)
    }

    pub fn clear_java_generate_equals_and_hash(&mut self) {
        self.java_generate_equals_and_hash = None;
    }

    pub fn has_java_generate_equals_and_hash(&self) -> bool {
        self.java_generate_equals_and_hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_java_generate_equals_and_hash(&mut self, v: bool) {
        self.java_generate_equals_and_hash = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_java_generate_equals_and_hash(&'self mut self) -> &'self mut bool {
        if self.java_generate_equals_and_hash.is_none() {
            self.java_generate_equals_and_hash = Some(false);
        };
        self.java_generate_equals_and_hash.get_mut_ref()
    }

    pub fn get_java_generate_equals_and_hash(&self) -> bool {
        self.java_generate_equals_and_hash.unwrap_or(false)
    }

    pub fn clear_optimize_for(&mut self) {
        self.optimize_for = None;
    }

    pub fn has_optimize_for(&self) -> bool {
        self.optimize_for.is_some()
    }

    // Param is passed by value, moved
    pub fn set_optimize_for(&mut self, v: FileOptions_OptimizeMode) {
        self.optimize_for = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_optimize_for(&'self mut self) -> &'self mut FileOptions_OptimizeMode {
        if self.optimize_for.is_none() {
            self.optimize_for = Some(FileOptions_OptimizeMode::new(0));
        };
        self.optimize_for.get_mut_ref()
    }

    pub fn get_optimize_for(&self) -> FileOptions_OptimizeMode {
        self.optimize_for.unwrap_or(FileOptions_OptimizeMode::new(0))
    }

    pub fn clear_go_package(&mut self) {
        self.go_package = None;
    }

    pub fn has_go_package(&self) -> bool {
        self.go_package.is_some()
    }

    // Param is passed by value, moved
    pub fn set_go_package(&mut self, v: ~str) {
        self.go_package = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_go_package(&'self mut self) -> &'self mut ~str {
        if self.go_package.is_none() {
            self.go_package = Some(~"");
        };
        self.go_package.get_mut_ref()
    }

    pub fn get_go_package(&'self self) -> &'self str {
        match self.go_package {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_cc_generic_services(&mut self) {
        self.cc_generic_services = None;
    }

    pub fn has_cc_generic_services(&self) -> bool {
        self.cc_generic_services.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cc_generic_services(&mut self, v: bool) {
        self.cc_generic_services = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cc_generic_services(&'self mut self) -> &'self mut bool {
        if self.cc_generic_services.is_none() {
            self.cc_generic_services = Some(false);
        };
        self.cc_generic_services.get_mut_ref()
    }

    pub fn get_cc_generic_services(&self) -> bool {
        self.cc_generic_services.unwrap_or(false)
    }

    pub fn clear_java_generic_services(&mut self) {
        self.java_generic_services = None;
    }

    pub fn has_java_generic_services(&self) -> bool {
        self.java_generic_services.is_some()
    }

    // Param is passed by value, moved
    pub fn set_java_generic_services(&mut self, v: bool) {
        self.java_generic_services = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_java_generic_services(&'self mut self) -> &'self mut bool {
        if self.java_generic_services.is_none() {
            self.java_generic_services = Some(false);
        };
        self.java_generic_services.get_mut_ref()
    }

    pub fn get_java_generic_services(&self) -> bool {
        self.java_generic_services.unwrap_or(false)
    }

    pub fn clear_py_generic_services(&mut self) {
        self.py_generic_services = None;
    }

    pub fn has_py_generic_services(&self) -> bool {
        self.py_generic_services.is_some()
    }

    // Param is passed by value, moved
    pub fn set_py_generic_services(&mut self, v: bool) {
        self.py_generic_services = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_py_generic_services(&'self mut self) -> &'self mut bool {
        if self.py_generic_services.is_none() {
            self.py_generic_services = Some(false);
        };
        self.py_generic_services.get_mut_ref()
    }

    pub fn get_py_generic_services(&self) -> bool {
        self.py_generic_services.unwrap_or(false)
    }

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }

    // Param is passed by value, moved
    pub fn set_uninterpreted_option(&mut self, v: ~[UninterpretedOption]) {
        self.uninterpreted_option = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uninterpreted_option(&'self mut self) -> &'self mut ~[UninterpretedOption] {
        &mut self.uninterpreted_option
    }

    pub fn get_uninterpreted_option(&'self self) -> &'self [UninterpretedOption] {
        rt::as_slice_tmp(&self.uninterpreted_option)
    }

    pub fn add_uninterpreted_option(&mut self, v: UninterpretedOption) {
        self.uninterpreted_option.push(v);
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
        for value in self.java_package.iter() {
            my_size += rt::string_size(1, *value);
        };
        for value in self.java_outer_classname.iter() {
            my_size += rt::string_size(8, *value);
        };
        if self.java_multiple_files.is_some() {
            my_size += 2;
        };
        if self.java_generate_equals_and_hash.is_some() {
            my_size += 3;
        };
        for value in self.optimize_for.iter() {
            my_size += rt::enum_size(9, *value);
        };
        for value in self.go_package.iter() {
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
        for value in self.uninterpreted_option.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 2 + rt::compute_raw_varint32_size(len) + len;
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
    fn value(&self) -> i32 {
        *self as i32
    }
}

#[deriving(Clone,Eq)]
pub struct MessageOptions {
    message_set_wire_format: Option<bool>,
    no_standard_descriptor_accessor: Option<bool>,
    uninterpreted_option: ~[UninterpretedOption],
}

impl<'self> MessageOptions {
    pub fn new() -> MessageOptions {
        MessageOptions {
            message_set_wire_format: None,
            no_standard_descriptor_accessor: None,
            uninterpreted_option: ~[],
        }
    }

    pub fn default_instance() -> &'static MessageOptions {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: MessageOptions = MessageOptions {
//             message_set_wire_format: None,
//             no_standard_descriptor_accessor: None,
//             uninterpreted_option: ~[],
//         };
//         &'static instance
        fail!("TODO");
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
        for v in self.uninterpreted_option.iter() {
            os.write_tag(999, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_message_set_wire_format(&mut self) {
        self.message_set_wire_format = None;
    }

    pub fn has_message_set_wire_format(&self) -> bool {
        self.message_set_wire_format.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_set_wire_format(&mut self, v: bool) {
        self.message_set_wire_format = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message_set_wire_format(&'self mut self) -> &'self mut bool {
        if self.message_set_wire_format.is_none() {
            self.message_set_wire_format = Some(false);
        };
        self.message_set_wire_format.get_mut_ref()
    }

    pub fn get_message_set_wire_format(&self) -> bool {
        self.message_set_wire_format.unwrap_or(false)
    }

    pub fn clear_no_standard_descriptor_accessor(&mut self) {
        self.no_standard_descriptor_accessor = None;
    }

    pub fn has_no_standard_descriptor_accessor(&self) -> bool {
        self.no_standard_descriptor_accessor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_no_standard_descriptor_accessor(&mut self, v: bool) {
        self.no_standard_descriptor_accessor = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_no_standard_descriptor_accessor(&'self mut self) -> &'self mut bool {
        if self.no_standard_descriptor_accessor.is_none() {
            self.no_standard_descriptor_accessor = Some(false);
        };
        self.no_standard_descriptor_accessor.get_mut_ref()
    }

    pub fn get_no_standard_descriptor_accessor(&self) -> bool {
        self.no_standard_descriptor_accessor.unwrap_or(false)
    }

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }

    // Param is passed by value, moved
    pub fn set_uninterpreted_option(&mut self, v: ~[UninterpretedOption]) {
        self.uninterpreted_option = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uninterpreted_option(&'self mut self) -> &'self mut ~[UninterpretedOption] {
        &mut self.uninterpreted_option
    }

    pub fn get_uninterpreted_option(&'self self) -> &'self [UninterpretedOption] {
        rt::as_slice_tmp(&self.uninterpreted_option)
    }

    pub fn add_uninterpreted_option(&mut self, v: UninterpretedOption) {
        self.uninterpreted_option.push(v);
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
        for value in self.uninterpreted_option.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 2 + rt::compute_raw_varint32_size(len) + len;
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
pub struct FieldOptions {
    ctype: Option<FieldOptions_CType>,
    packed: Option<bool>,
    lazy: Option<bool>,
    deprecated: Option<bool>,
    experimental_map_key: Option<~str>,
    weak: Option<bool>,
    uninterpreted_option: ~[UninterpretedOption],
}

impl<'self> FieldOptions {
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

    pub fn default_instance() -> &'static FieldOptions {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: FieldOptions = FieldOptions {
//             ctype: None,
//             packed: None,
//             lazy: None,
//             deprecated: None,
//             experimental_map_key: None,
//             weak: None,
//             uninterpreted_option: ~[],
//         };
//         &'static instance
        fail!("TODO");
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
        for v in self.uninterpreted_option.iter() {
            os.write_tag(999, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_ctype(&mut self) {
        self.ctype = None;
    }

    pub fn has_ctype(&self) -> bool {
        self.ctype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ctype(&mut self, v: FieldOptions_CType) {
        self.ctype = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ctype(&'self mut self) -> &'self mut FieldOptions_CType {
        if self.ctype.is_none() {
            self.ctype = Some(FieldOptions_CType::new(0));
        };
        self.ctype.get_mut_ref()
    }

    pub fn get_ctype(&self) -> FieldOptions_CType {
        self.ctype.unwrap_or(FieldOptions_CType::new(0))
    }

    pub fn clear_packed(&mut self) {
        self.packed = None;
    }

    pub fn has_packed(&self) -> bool {
        self.packed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packed(&mut self, v: bool) {
        self.packed = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_packed(&'self mut self) -> &'self mut bool {
        if self.packed.is_none() {
            self.packed = Some(false);
        };
        self.packed.get_mut_ref()
    }

    pub fn get_packed(&self) -> bool {
        self.packed.unwrap_or(false)
    }

    pub fn clear_lazy(&mut self) {
        self.lazy = None;
    }

    pub fn has_lazy(&self) -> bool {
        self.lazy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lazy(&mut self, v: bool) {
        self.lazy = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lazy(&'self mut self) -> &'self mut bool {
        if self.lazy.is_none() {
            self.lazy = Some(false);
        };
        self.lazy.get_mut_ref()
    }

    pub fn get_lazy(&self) -> bool {
        self.lazy.unwrap_or(false)
    }

    pub fn clear_deprecated(&mut self) {
        self.deprecated = None;
    }

    pub fn has_deprecated(&self) -> bool {
        self.deprecated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deprecated(&mut self, v: bool) {
        self.deprecated = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deprecated(&'self mut self) -> &'self mut bool {
        if self.deprecated.is_none() {
            self.deprecated = Some(false);
        };
        self.deprecated.get_mut_ref()
    }

    pub fn get_deprecated(&self) -> bool {
        self.deprecated.unwrap_or(false)
    }

    pub fn clear_experimental_map_key(&mut self) {
        self.experimental_map_key = None;
    }

    pub fn has_experimental_map_key(&self) -> bool {
        self.experimental_map_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_experimental_map_key(&mut self, v: ~str) {
        self.experimental_map_key = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_experimental_map_key(&'self mut self) -> &'self mut ~str {
        if self.experimental_map_key.is_none() {
            self.experimental_map_key = Some(~"");
        };
        self.experimental_map_key.get_mut_ref()
    }

    pub fn get_experimental_map_key(&'self self) -> &'self str {
        match self.experimental_map_key {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_weak(&mut self) {
        self.weak = None;
    }

    pub fn has_weak(&self) -> bool {
        self.weak.is_some()
    }

    // Param is passed by value, moved
    pub fn set_weak(&mut self, v: bool) {
        self.weak = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_weak(&'self mut self) -> &'self mut bool {
        if self.weak.is_none() {
            self.weak = Some(false);
        };
        self.weak.get_mut_ref()
    }

    pub fn get_weak(&self) -> bool {
        self.weak.unwrap_or(false)
    }

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }

    // Param is passed by value, moved
    pub fn set_uninterpreted_option(&mut self, v: ~[UninterpretedOption]) {
        self.uninterpreted_option = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uninterpreted_option(&'self mut self) -> &'self mut ~[UninterpretedOption] {
        &mut self.uninterpreted_option
    }

    pub fn get_uninterpreted_option(&'self self) -> &'self [UninterpretedOption] {
        rt::as_slice_tmp(&self.uninterpreted_option)
    }

    pub fn add_uninterpreted_option(&mut self, v: UninterpretedOption) {
        self.uninterpreted_option.push(v);
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
        for value in self.ctype.iter() {
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
        for value in self.experimental_map_key.iter() {
            my_size += rt::string_size(9, *value);
        };
        if self.weak.is_some() {
            my_size += 2;
        };
        for value in self.uninterpreted_option.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 2 + rt::compute_raw_varint32_size(len) + len;
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
    fn value(&self) -> i32 {
        *self as i32
    }
}

#[deriving(Clone,Eq)]
pub struct EnumOptions {
    allow_alias: Option<bool>,
    uninterpreted_option: ~[UninterpretedOption],
}

impl<'self> EnumOptions {
    pub fn new() -> EnumOptions {
        EnumOptions {
            allow_alias: None,
            uninterpreted_option: ~[],
        }
    }

    pub fn default_instance() -> &'static EnumOptions {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: EnumOptions = EnumOptions {
//             allow_alias: None,
//             uninterpreted_option: ~[],
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        match self.allow_alias {
            Some(ref v) => {
                os.write_bool(2, *v);
            },
            None => {},
        };
        for v in self.uninterpreted_option.iter() {
            os.write_tag(999, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_allow_alias(&mut self) {
        self.allow_alias = None;
    }

    pub fn has_allow_alias(&self) -> bool {
        self.allow_alias.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allow_alias(&mut self, v: bool) {
        self.allow_alias = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allow_alias(&'self mut self) -> &'self mut bool {
        if self.allow_alias.is_none() {
            self.allow_alias = Some(false);
        };
        self.allow_alias.get_mut_ref()
    }

    pub fn get_allow_alias(&self) -> bool {
        self.allow_alias.unwrap_or(false)
    }

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }

    // Param is passed by value, moved
    pub fn set_uninterpreted_option(&mut self, v: ~[UninterpretedOption]) {
        self.uninterpreted_option = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uninterpreted_option(&'self mut self) -> &'self mut ~[UninterpretedOption] {
        &mut self.uninterpreted_option
    }

    pub fn get_uninterpreted_option(&'self self) -> &'self [UninterpretedOption] {
        rt::as_slice_tmp(&self.uninterpreted_option)
    }

    pub fn add_uninterpreted_option(&mut self, v: UninterpretedOption) {
        self.uninterpreted_option.push(v);
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
        for value in self.uninterpreted_option.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 2 + rt::compute_raw_varint32_size(len) + len;
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
pub struct EnumValueOptions {
    uninterpreted_option: ~[UninterpretedOption],
}

impl<'self> EnumValueOptions {
    pub fn new() -> EnumValueOptions {
        EnumValueOptions {
            uninterpreted_option: ~[],
        }
    }

    pub fn default_instance() -> &'static EnumValueOptions {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: EnumValueOptions = EnumValueOptions {
//             uninterpreted_option: ~[],
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for v in self.uninterpreted_option.iter() {
            os.write_tag(999, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }

    // Param is passed by value, moved
    pub fn set_uninterpreted_option(&mut self, v: ~[UninterpretedOption]) {
        self.uninterpreted_option = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uninterpreted_option(&'self mut self) -> &'self mut ~[UninterpretedOption] {
        &mut self.uninterpreted_option
    }

    pub fn get_uninterpreted_option(&'self self) -> &'self [UninterpretedOption] {
        rt::as_slice_tmp(&self.uninterpreted_option)
    }

    pub fn add_uninterpreted_option(&mut self, v: UninterpretedOption) {
        self.uninterpreted_option.push(v);
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
        for value in self.uninterpreted_option.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 2 + rt::compute_raw_varint32_size(len) + len;
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
pub struct ServiceOptions {
    uninterpreted_option: ~[UninterpretedOption],
}

impl<'self> ServiceOptions {
    pub fn new() -> ServiceOptions {
        ServiceOptions {
            uninterpreted_option: ~[],
        }
    }

    pub fn default_instance() -> &'static ServiceOptions {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: ServiceOptions = ServiceOptions {
//             uninterpreted_option: ~[],
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for v in self.uninterpreted_option.iter() {
            os.write_tag(999, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }

    // Param is passed by value, moved
    pub fn set_uninterpreted_option(&mut self, v: ~[UninterpretedOption]) {
        self.uninterpreted_option = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uninterpreted_option(&'self mut self) -> &'self mut ~[UninterpretedOption] {
        &mut self.uninterpreted_option
    }

    pub fn get_uninterpreted_option(&'self self) -> &'self [UninterpretedOption] {
        rt::as_slice_tmp(&self.uninterpreted_option)
    }

    pub fn add_uninterpreted_option(&mut self, v: UninterpretedOption) {
        self.uninterpreted_option.push(v);
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
        for value in self.uninterpreted_option.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 2 + rt::compute_raw_varint32_size(len) + len;
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
pub struct MethodOptions {
    uninterpreted_option: ~[UninterpretedOption],
}

impl<'self> MethodOptions {
    pub fn new() -> MethodOptions {
        MethodOptions {
            uninterpreted_option: ~[],
        }
    }

    pub fn default_instance() -> &'static MethodOptions {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: MethodOptions = MethodOptions {
//             uninterpreted_option: ~[],
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for v in self.uninterpreted_option.iter() {
            os.write_tag(999, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }

    // Param is passed by value, moved
    pub fn set_uninterpreted_option(&mut self, v: ~[UninterpretedOption]) {
        self.uninterpreted_option = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uninterpreted_option(&'self mut self) -> &'self mut ~[UninterpretedOption] {
        &mut self.uninterpreted_option
    }

    pub fn get_uninterpreted_option(&'self self) -> &'self [UninterpretedOption] {
        rt::as_slice_tmp(&self.uninterpreted_option)
    }

    pub fn add_uninterpreted_option(&mut self, v: UninterpretedOption) {
        self.uninterpreted_option.push(v);
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
        for value in self.uninterpreted_option.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 2 + rt::compute_raw_varint32_size(len) + len;
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
pub struct UninterpretedOption {
    name: ~[UninterpretedOption_NamePart],
    identifier_value: Option<~str>,
    positive_int_value: Option<u64>,
    negative_int_value: Option<i64>,
    double_value: Option<f64>,
    string_value: Option<~[u8]>,
    aggregate_value: Option<~str>,
}

impl<'self> UninterpretedOption {
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

    pub fn default_instance() -> &'static UninterpretedOption {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: UninterpretedOption = UninterpretedOption {
//             name: ~[],
//             identifier_value: None,
//             positive_int_value: None,
//             negative_int_value: None,
//             double_value: None,
//             string_value: None,
//             aggregate_value: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for v in self.name.iter() {
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

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ~[UninterpretedOption_NamePart]) {
        self.name = v;
    }

    // Mutable pointer to the field.
    pub fn mut_name(&'self mut self) -> &'self mut ~[UninterpretedOption_NamePart] {
        &mut self.name
    }

    pub fn get_name(&'self self) -> &'self [UninterpretedOption_NamePart] {
        rt::as_slice_tmp(&self.name)
    }

    pub fn add_name(&mut self, v: UninterpretedOption_NamePart) {
        self.name.push(v);
    }

    pub fn clear_identifier_value(&mut self) {
        self.identifier_value = None;
    }

    pub fn has_identifier_value(&self) -> bool {
        self.identifier_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_identifier_value(&mut self, v: ~str) {
        self.identifier_value = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_identifier_value(&'self mut self) -> &'self mut ~str {
        if self.identifier_value.is_none() {
            self.identifier_value = Some(~"");
        };
        self.identifier_value.get_mut_ref()
    }

    pub fn get_identifier_value(&'self self) -> &'self str {
        match self.identifier_value {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_positive_int_value(&mut self) {
        self.positive_int_value = None;
    }

    pub fn has_positive_int_value(&self) -> bool {
        self.positive_int_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_positive_int_value(&mut self, v: u64) {
        self.positive_int_value = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_positive_int_value(&'self mut self) -> &'self mut u64 {
        if self.positive_int_value.is_none() {
            self.positive_int_value = Some(0);
        };
        self.positive_int_value.get_mut_ref()
    }

    pub fn get_positive_int_value(&self) -> u64 {
        self.positive_int_value.unwrap_or(0)
    }

    pub fn clear_negative_int_value(&mut self) {
        self.negative_int_value = None;
    }

    pub fn has_negative_int_value(&self) -> bool {
        self.negative_int_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_negative_int_value(&mut self, v: i64) {
        self.negative_int_value = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_negative_int_value(&'self mut self) -> &'self mut i64 {
        if self.negative_int_value.is_none() {
            self.negative_int_value = Some(0);
        };
        self.negative_int_value.get_mut_ref()
    }

    pub fn get_negative_int_value(&self) -> i64 {
        self.negative_int_value.unwrap_or(0)
    }

    pub fn clear_double_value(&mut self) {
        self.double_value = None;
    }

    pub fn has_double_value(&self) -> bool {
        self.double_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_double_value(&mut self, v: f64) {
        self.double_value = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_double_value(&'self mut self) -> &'self mut f64 {
        if self.double_value.is_none() {
            self.double_value = Some(0.);
        };
        self.double_value.get_mut_ref()
    }

    pub fn get_double_value(&self) -> f64 {
        self.double_value.unwrap_or(0.)
    }

    pub fn clear_string_value(&mut self) {
        self.string_value = None;
    }

    pub fn has_string_value(&self) -> bool {
        self.string_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_value(&mut self, v: ~[u8]) {
        self.string_value = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_value(&'self mut self) -> &'self mut ~[u8] {
        if self.string_value.is_none() {
            self.string_value = Some(~[]);
        };
        self.string_value.get_mut_ref()
    }

    pub fn get_string_value(&'self self) -> &'self [u8] {
        match self.string_value {
            Some(ref v) => rt::as_slice_tmp(v),
            None => &'self [],
        }
    }

    pub fn clear_aggregate_value(&mut self) {
        self.aggregate_value = None;
    }

    pub fn has_aggregate_value(&self) -> bool {
        self.aggregate_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_aggregate_value(&mut self, v: ~str) {
        self.aggregate_value = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_aggregate_value(&'self mut self) -> &'self mut ~str {
        if self.aggregate_value.is_none() {
            self.aggregate_value = Some(~"");
        };
        self.aggregate_value.get_mut_ref()
    }

    pub fn get_aggregate_value(&'self self) -> &'self str {
        match self.aggregate_value {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
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
        for value in self.name.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.identifier_value.iter() {
            my_size += rt::string_size(3, *value);
        };
        for value in self.positive_int_value.iter() {
            my_size += rt::value_size(4, *value, wire_format::WireTypeVarint);
        };
        for value in self.negative_int_value.iter() {
            my_size += rt::value_size(5, *value, wire_format::WireTypeVarint);
        };
        if self.double_value.is_some() {
            my_size += 9;
        };
        for value in self.string_value.iter() {
            my_size += rt::bytes_size(7, *value);
        };
        for value in self.aggregate_value.iter() {
            my_size += rt::string_size(8, *value);
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
pub struct UninterpretedOption_NamePart {
    name_part: Option<~str>,
    is_extension: Option<bool>,
}

impl<'self> UninterpretedOption_NamePart {
    pub fn new() -> UninterpretedOption_NamePart {
        UninterpretedOption_NamePart {
            name_part: None,
            is_extension: None,
        }
    }

    pub fn default_instance() -> &'static UninterpretedOption_NamePart {
//         // doesn't work, because rust master has broken static constants that contains None of ~str
//         // https://github.com/mozilla/rust/issues/8578
//         // TODO: should at least keep static without ~str
//         static instance: UninterpretedOption_NamePart = UninterpretedOption_NamePart {
//             name_part: None,
//             is_extension: None,
//         };
//         &'static instance
        fail!("TODO");
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

    pub fn has_name_part(&self) -> bool {
        self.name_part.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name_part(&mut self, v: ~str) {
        self.name_part = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name_part(&'self mut self) -> &'self mut ~str {
        if self.name_part.is_none() {
            self.name_part = Some(~"");
        };
        self.name_part.get_mut_ref()
    }

    pub fn get_name_part(&'self self) -> &'self str {
        match self.name_part {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_is_extension(&mut self) {
        self.is_extension = None;
    }

    pub fn has_is_extension(&self) -> bool {
        self.is_extension.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_extension(&mut self, v: bool) {
        self.is_extension = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_is_extension(&'self mut self) -> &'self mut bool {
        if self.is_extension.is_none() {
            self.is_extension = Some(false);
        };
        self.is_extension.get_mut_ref()
    }

    pub fn get_is_extension(&self) -> bool {
        self.is_extension.unwrap_or(false)
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
        for value in self.name_part.iter() {
            my_size += rt::string_size(1, *value);
        };
        if self.is_extension.is_some() {
            my_size += 2;
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
pub struct SourceCodeInfo {
    location: ~[SourceCodeInfo_Location],
}

impl<'self> SourceCodeInfo {
    pub fn new() -> SourceCodeInfo {
        SourceCodeInfo {
            location: ~[],
        }
    }

    pub fn default_instance() -> &'static SourceCodeInfo {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: SourceCodeInfo = SourceCodeInfo {
//             location: ~[],
//         };
//         &'static instance
        fail!("TODO");
    }

    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        for v in self.location.iter() {
            os.write_tag(1, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(sizes[*sizes_pos]);
            *sizes_pos += 1;
            v.write_to_with_computed_sizes(os, sizes, sizes_pos);
        };
    }

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: ~[SourceCodeInfo_Location]) {
        self.location = v;
    }

    // Mutable pointer to the field.
    pub fn mut_location(&'self mut self) -> &'self mut ~[SourceCodeInfo_Location] {
        &mut self.location
    }

    pub fn get_location(&'self self) -> &'self [SourceCodeInfo_Location] {
        rt::as_slice_tmp(&self.location)
    }

    pub fn add_location(&mut self, v: SourceCodeInfo_Location) {
        self.location.push(v);
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
        for value in self.location.iter() {
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
pub struct SourceCodeInfo_Location {
    path: ~[i32],
    span: ~[i32],
    leading_comments: Option<~str>,
    trailing_comments: Option<~str>,
}

impl<'self> SourceCodeInfo_Location {
    pub fn new() -> SourceCodeInfo_Location {
        SourceCodeInfo_Location {
            path: ~[],
            span: ~[],
            leading_comments: None,
            trailing_comments: None,
        }
    }

    pub fn default_instance() -> &'static SourceCodeInfo_Location {
//         // doesn't work, because rust doen't implement static constants of types like ~str
//         // https://github.com/mozilla/rust/issues/8406
//         static instance: SourceCodeInfo_Location = SourceCodeInfo_Location {
//             path: ~[],
//             span: ~[],
//             leading_comments: None,
//             trailing_comments: None,
//         };
//         &'static instance
        fail!("TODO");
    }

    #[allow(unused_variable)]
    pub fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        if !self.path.is_empty() {
            os.write_tag(1, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(rt::vec_packed_data_size(self.path, wire_format::WireTypeVarint));
            for v in self.path.iter() {
                os.write_int32_no_tag(*v);
            };
        };
        if !self.span.is_empty() {
            os.write_tag(2, wire_format::WireTypeLengthDelimited);
            os.write_raw_varint32(rt::vec_packed_data_size(self.span, wire_format::WireTypeVarint));
            for v in self.span.iter() {
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

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ~[i32]) {
        self.path = v;
    }

    // Mutable pointer to the field.
    pub fn mut_path(&'self mut self) -> &'self mut ~[i32] {
        &mut self.path
    }

    pub fn get_path(&'self self) -> &'self [i32] {
        rt::as_slice_tmp(&self.path)
    }

    pub fn add_path(&mut self, v: i32) {
        self.path.push(v);
    }

    pub fn clear_span(&mut self) {
        self.span.clear();
    }

    // Param is passed by value, moved
    pub fn set_span(&mut self, v: ~[i32]) {
        self.span = v;
    }

    // Mutable pointer to the field.
    pub fn mut_span(&'self mut self) -> &'self mut ~[i32] {
        &mut self.span
    }

    pub fn get_span(&'self self) -> &'self [i32] {
        rt::as_slice_tmp(&self.span)
    }

    pub fn add_span(&mut self, v: i32) {
        self.span.push(v);
    }

    pub fn clear_leading_comments(&mut self) {
        self.leading_comments = None;
    }

    pub fn has_leading_comments(&self) -> bool {
        self.leading_comments.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leading_comments(&mut self, v: ~str) {
        self.leading_comments = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_leading_comments(&'self mut self) -> &'self mut ~str {
        if self.leading_comments.is_none() {
            self.leading_comments = Some(~"");
        };
        self.leading_comments.get_mut_ref()
    }

    pub fn get_leading_comments(&'self self) -> &'self str {
        match self.leading_comments {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
    }

    pub fn clear_trailing_comments(&mut self) {
        self.trailing_comments = None;
    }

    pub fn has_trailing_comments(&self) -> bool {
        self.trailing_comments.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trailing_comments(&mut self, v: ~str) {
        self.trailing_comments = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_trailing_comments(&'self mut self) -> &'self mut ~str {
        if self.trailing_comments.is_none() {
            self.trailing_comments = Some(~"");
        };
        self.trailing_comments.get_mut_ref()
    }

    pub fn get_trailing_comments(&'self self) -> &'self str {
        match self.trailing_comments {
            Some(ref v) => v.as_slice(),
            None => &'self "",
        }
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
        for value in self.leading_comments.iter() {
            my_size += rt::string_size(3, *value);
        };
        for value in self.trailing_comments.iter() {
            my_size += rt::string_size(4, *value);
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

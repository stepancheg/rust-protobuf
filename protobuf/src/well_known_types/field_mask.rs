// This file is generated by rust-protobuf 2.7.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `google/protobuf/field_mask.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct FieldMask {
    // message fields
    pub paths: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a FieldMask {
    fn default() -> &'a FieldMask {
        <FieldMask as ::protobuf::Message>::default_instance()
    }
}

impl FieldMask {
    pub fn new() -> FieldMask {
        ::std::default::Default::default()
    }

    // repeated string paths = 1;


    pub fn get_paths(&self) -> &[::std::string::String] {
        &self.paths
    }
    pub fn clear_paths(&mut self) {
        self.paths.clear();
    }

    // Param is passed by value, moved
    pub fn set_paths(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.paths = v;
    }

    // Mutable pointer to the field.
    pub fn mut_paths(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.paths
    }

    // Take field
    pub fn take_paths(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.paths, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for FieldMask {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.paths)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.paths {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.paths {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> FieldMask {
        FieldMask::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "paths",
                    |m: &FieldMask| { &m.paths },
                    |m: &mut FieldMask| { &mut m.paths },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FieldMask>(
                    "FieldMask",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static FieldMask {
        static mut instance: ::protobuf::lazy::Lazy<FieldMask> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FieldMask,
        };
        unsafe {
            instance.get(FieldMask::new)
        }
    }
}

impl ::protobuf::Clear for FieldMask {
    fn clear(&mut self) {
        self.paths.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FieldMask {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FieldMask {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20google/protobuf/field_mask.proto\x12\x0fgoogle.protobuf\"!\n\tFiel\
    dMask\x12\x14\n\x05paths\x18\x01\x20\x03(\tR\x05pathsBN\n\x13com.google.\
    protobufB\x0eFieldMaskProtoP\x01\xa2\x02\x03GPB\xaa\x02\x1eGoogle.Protob\
    uf.WellKnownTypesJ\x929\n\x07\x12\x05\x1e\0\xf4\x01\x01\n\xcc\x0c\n\x01\
    \x0c\x12\x03\x1e\0\x122\xc1\x0c\x20Protocol\x20Buffers\x20-\x20Google's\
    \x20data\x20interchange\x20format\n\x20Copyright\x202008\x20Google\x20In\
    c.\x20\x20All\x20rights\x20reserved.\n\x20https://developers.google.com/\
    protocol-buffers/\n\n\x20Redistribution\x20and\x20use\x20in\x20source\
    \x20and\x20binary\x20forms,\x20with\x20or\x20without\n\x20modification,\
    \x20are\x20permitted\x20provided\x20that\x20the\x20following\x20conditio\
    ns\x20are\n\x20met:\n\n\x20\x20\x20\x20\x20*\x20Redistributions\x20of\
    \x20source\x20code\x20must\x20retain\x20the\x20above\x20copyright\n\x20n\
    otice,\x20this\x20list\x20of\x20conditions\x20and\x20the\x20following\
    \x20disclaimer.\n\x20\x20\x20\x20\x20*\x20Redistributions\x20in\x20binar\
    y\x20form\x20must\x20reproduce\x20the\x20above\n\x20copyright\x20notice,\
    \x20this\x20list\x20of\x20conditions\x20and\x20the\x20following\x20discl\
    aimer\n\x20in\x20the\x20documentation\x20and/or\x20other\x20materials\
    \x20provided\x20with\x20the\n\x20distribution.\n\x20\x20\x20\x20\x20*\
    \x20Neither\x20the\x20name\x20of\x20Google\x20Inc.\x20nor\x20the\x20name\
    s\x20of\x20its\n\x20contributors\x20may\x20be\x20used\x20to\x20endorse\
    \x20or\x20promote\x20products\x20derived\x20from\n\x20this\x20software\
    \x20without\x20specific\x20prior\x20written\x20permission.\n\n\x20THIS\
    \x20SOFTWARE\x20IS\x20PROVIDED\x20BY\x20THE\x20COPYRIGHT\x20HOLDERS\x20A\
    ND\x20CONTRIBUTORS\n\x20\"AS\x20IS\"\x20AND\x20ANY\x20EXPRESS\x20OR\x20I\
    MPLIED\x20WARRANTIES,\x20INCLUDING,\x20BUT\x20NOT\n\x20LIMITED\x20TO,\
    \x20THE\x20IMPLIED\x20WARRANTIES\x20OF\x20MERCHANTABILITY\x20AND\x20FITN\
    ESS\x20FOR\n\x20A\x20PARTICULAR\x20PURPOSE\x20ARE\x20DISCLAIMED.\x20IN\
    \x20NO\x20EVENT\x20SHALL\x20THE\x20COPYRIGHT\n\x20OWNER\x20OR\x20CONTRIB\
    UTORS\x20BE\x20LIABLE\x20FOR\x20ANY\x20DIRECT,\x20INDIRECT,\x20INCIDENTA\
    L,\n\x20SPECIAL,\x20EXEMPLARY,\x20OR\x20CONSEQUENTIAL\x20DAMAGES\x20(INC\
    LUDING,\x20BUT\x20NOT\n\x20LIMITED\x20TO,\x20PROCUREMENT\x20OF\x20SUBSTI\
    TUTE\x20GOODS\x20OR\x20SERVICES;\x20LOSS\x20OF\x20USE,\n\x20DATA,\x20OR\
    \x20PROFITS;\x20OR\x20BUSINESS\x20INTERRUPTION)\x20HOWEVER\x20CAUSED\x20\
    AND\x20ON\x20ANY\n\x20THEORY\x20OF\x20LIABILITY,\x20WHETHER\x20IN\x20CON\
    TRACT,\x20STRICT\x20LIABILITY,\x20OR\x20TORT\n\x20(INCLUDING\x20NEGLIGEN\
    CE\x20OR\x20OTHERWISE)\x20ARISING\x20IN\x20ANY\x20WAY\x20OUT\x20OF\x20TH\
    E\x20USE\n\x20OF\x20THIS\x20SOFTWARE,\x20EVEN\x20IF\x20ADVISED\x20OF\x20\
    THE\x20POSSIBILITY\x20OF\x20SUCH\x20DAMAGE.\n\n\x08\n\x01\x02\x12\x03\
    \x20\x08\x17\n\x08\n\x01\x08\x12\x03\"\0;\n\t\n\x02\x08%\x12\x03\"\0;\n\
    \x08\n\x01\x08\x12\x03#\0,\n\t\n\x02\x08\x01\x12\x03#\0,\n\x08\n\x01\x08\
    \x12\x03$\0/\n\t\n\x02\x08\x08\x12\x03$\0/\n\x08\n\x01\x08\x12\x03%\0\"\
    \n\t\n\x02\x08\n\x12\x03%\0\"\n\x08\n\x01\x08\x12\x03&\0!\n\t\n\x02\x08$\
    \x12\x03&\0!\n\xcd*\n\x02\x04\0\x12\x06\xf1\x01\0\xf4\x01\x01\x1a\xbe*\
    \x20`FieldMask`\x20represents\x20a\x20set\x20of\x20symbolic\x20field\x20\
    paths,\x20for\x20example:\n\n\x20\x20\x20\x20\x20paths:\x20\"f.a\"\n\x20\
    \x20\x20\x20\x20paths:\x20\"f.b.d\"\n\n\x20Here\x20`f`\x20represents\x20\
    a\x20field\x20in\x20some\x20root\x20message,\x20`a`\x20and\x20`b`\n\x20f\
    ields\x20in\x20the\x20message\x20found\x20in\x20`f`,\x20and\x20`d`\x20a\
    \x20field\x20found\x20in\x20the\n\x20message\x20in\x20`f.b`.\n\n\x20Fiel\
    d\x20masks\x20are\x20used\x20to\x20specify\x20a\x20subset\x20of\x20field\
    s\x20that\x20should\x20be\n\x20returned\x20by\x20a\x20get\x20operation\
    \x20or\x20modified\x20by\x20an\x20update\x20operation.\n\x20Field\x20mas\
    ks\x20also\x20have\x20a\x20custom\x20JSON\x20encoding\x20(see\x20below).\
    \n\n\x20#\x20Field\x20Masks\x20in\x20Projections\n\n\x20When\x20used\x20\
    in\x20the\x20context\x20of\x20a\x20projection,\x20a\x20response\x20messa\
    ge\x20or\n\x20sub-message\x20is\x20filtered\x20by\x20the\x20API\x20to\
    \x20only\x20contain\x20those\x20fields\x20as\n\x20specified\x20in\x20the\
    \x20mask.\x20For\x20example,\x20if\x20the\x20mask\x20in\x20the\x20previo\
    us\n\x20example\x20is\x20applied\x20to\x20a\x20response\x20message\x20as\
    \x20follows:\n\n\x20\x20\x20\x20\x20f\x20{\n\x20\x20\x20\x20\x20\x20\x20\
    a\x20:\x2022\n\x20\x20\x20\x20\x20\x20\x20b\x20{\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20d\x20:\x201\n\x20\x20\x20\x20\x20\x20\x20\x20\x20x\x20:\
    \x202\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\x20\x20y\x20:\
    \x2013\n\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20z:\x208\n\n\x20The\
    \x20result\x20will\x20not\x20contain\x20specific\x20values\x20for\x20fie\
    lds\x20x,y\x20and\x20z\n\x20(their\x20value\x20will\x20be\x20set\x20to\
    \x20the\x20default,\x20and\x20omitted\x20in\x20proto\x20text\n\x20output\
    ):\n\n\n\x20\x20\x20\x20\x20f\x20{\n\x20\x20\x20\x20\x20\x20\x20a\x20:\
    \x2022\n\x20\x20\x20\x20\x20\x20\x20b\x20{\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20d\x20:\x201\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\
    }\n\n\x20A\x20repeated\x20field\x20is\x20not\x20allowed\x20except\x20at\
    \x20the\x20last\x20position\x20of\x20a\n\x20paths\x20string.\n\n\x20If\
    \x20a\x20FieldMask\x20object\x20is\x20not\x20present\x20in\x20a\x20get\
    \x20operation,\x20the\n\x20operation\x20applies\x20to\x20all\x20fields\
    \x20(as\x20if\x20a\x20FieldMask\x20of\x20all\x20fields\n\x20had\x20been\
    \x20specified).\n\n\x20Note\x20that\x20a\x20field\x20mask\x20does\x20not\
    \x20necessarily\x20apply\x20to\x20the\n\x20top-level\x20response\x20mess\
    age.\x20In\x20case\x20of\x20a\x20REST\x20get\x20operation,\x20the\n\x20f\
    ield\x20mask\x20applies\x20directly\x20to\x20the\x20response,\x20but\x20\
    in\x20case\x20of\x20a\x20REST\n\x20list\x20operation,\x20the\x20mask\x20\
    instead\x20applies\x20to\x20each\x20individual\x20message\n\x20in\x20the\
    \x20returned\x20resource\x20list.\x20In\x20case\x20of\x20a\x20REST\x20cu\
    stom\x20method,\n\x20other\x20definitions\x20may\x20be\x20used.\x20Where\
    \x20the\x20mask\x20applies\x20will\x20be\n\x20clearly\x20documented\x20t\
    ogether\x20with\x20its\x20declaration\x20in\x20the\x20API.\x20\x20In\n\
    \x20any\x20case,\x20the\x20effect\x20on\x20the\x20returned\x20resource/r\
    esources\x20is\x20required\n\x20behavior\x20for\x20APIs.\n\n\x20#\x20Fie\
    ld\x20Masks\x20in\x20Update\x20Operations\n\n\x20A\x20field\x20mask\x20i\
    n\x20update\x20operations\x20specifies\x20which\x20fields\x20of\x20the\n\
    \x20targeted\x20resource\x20are\x20going\x20to\x20be\x20updated.\x20The\
    \x20API\x20is\x20required\n\x20to\x20only\x20change\x20the\x20values\x20\
    of\x20the\x20fields\x20as\x20specified\x20in\x20the\x20mask\n\x20and\x20\
    leave\x20the\x20others\x20untouched.\x20If\x20a\x20resource\x20is\x20pas\
    sed\x20in\x20to\n\x20describe\x20the\x20updated\x20values,\x20the\x20API\
    \x20ignores\x20the\x20values\x20of\x20all\n\x20fields\x20not\x20covered\
    \x20by\x20the\x20mask.\n\n\x20If\x20a\x20repeated\x20field\x20is\x20spec\
    ified\x20for\x20an\x20update\x20operation,\x20the\x20existing\n\x20repea\
    ted\x20values\x20in\x20the\x20target\x20resource\x20will\x20be\x20overwr\
    itten\x20by\x20the\x20new\x20values.\n\x20Note\x20that\x20a\x20repeated\
    \x20field\x20is\x20only\x20allowed\x20in\x20the\x20last\x20position\x20o\
    f\x20a\x20`paths`\n\x20string.\n\n\x20If\x20a\x20sub-message\x20is\x20sp\
    ecified\x20in\x20the\x20last\x20position\x20of\x20the\x20field\x20mask\
    \x20for\x20an\n\x20update\x20operation,\x20then\x20the\x20existing\x20su\
    b-message\x20in\x20the\x20target\x20resource\x20is\n\x20overwritten.\x20\
    Given\x20the\x20target\x20message:\n\n\x20\x20\x20\x20\x20f\x20{\n\x20\
    \x20\x20\x20\x20\x20\x20b\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20d\
    \x20:\x201\n\x20\x20\x20\x20\x20\x20\x20\x20\x20x\x20:\x202\n\x20\x20\
    \x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\x20\x20c\x20:\x201\n\x20\x20\
    \x20\x20\x20}\n\n\x20And\x20an\x20update\x20message:\n\n\x20\x20\x20\x20\
    \x20f\x20{\n\x20\x20\x20\x20\x20\x20\x20b\x20{\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20d\x20:\x2010\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\
    \x20\x20}\n\n\x20then\x20if\x20the\x20field\x20mask\x20is:\n\n\x20\x20pa\
    ths:\x20\"f.b\"\n\n\x20then\x20the\x20result\x20will\x20be:\n\n\x20\x20\
    \x20\x20\x20f\x20{\n\x20\x20\x20\x20\x20\x20\x20b\x20{\n\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20d\x20:\x2010\n\x20\x20\x20\x20\x20\x20\x20}\n\x20\
    \x20\x20\x20\x20\x20\x20c\x20:\x201\n\x20\x20\x20\x20\x20}\n\n\x20Howeve\
    r,\x20if\x20the\x20update\x20mask\x20was:\n\n\x20\x20paths:\x20\"f.b.d\"\
    \n\n\x20then\x20the\x20result\x20would\x20be:\n\n\x20\x20\x20\x20\x20f\
    \x20{\n\x20\x20\x20\x20\x20\x20\x20b\x20{\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20d\x20:\x2010\n\x20\x20\x20\x20\x20\x20\x20\x20\x20x\x20:\x202\n\
    \x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20\x20\x20c\x20:\x201\n\
    \x20\x20\x20\x20\x20}\n\n\x20In\x20order\x20to\x20reset\x20a\x20field's\
    \x20value\x20to\x20the\x20default,\x20the\x20field\x20must\n\x20be\x20in\
    \x20the\x20mask\x20and\x20set\x20to\x20the\x20default\x20value\x20in\x20\
    the\x20provided\x20resource.\n\x20Hence,\x20in\x20order\x20to\x20reset\
    \x20all\x20fields\x20of\x20a\x20resource,\x20provide\x20a\x20default\n\
    \x20instance\x20of\x20the\x20resource\x20and\x20set\x20all\x20fields\x20\
    in\x20the\x20mask,\x20or\x20do\n\x20not\x20provide\x20a\x20mask\x20as\
    \x20described\x20below.\n\n\x20If\x20a\x20field\x20mask\x20is\x20not\x20\
    present\x20on\x20update,\x20the\x20operation\x20applies\x20to\n\x20all\
    \x20fields\x20(as\x20if\x20a\x20field\x20mask\x20of\x20all\x20fields\x20\
    has\x20been\x20specified).\n\x20Note\x20that\x20in\x20the\x20presence\
    \x20of\x20schema\x20evolution,\x20this\x20may\x20mean\x20that\n\x20field\
    s\x20the\x20client\x20does\x20not\x20know\x20and\x20has\x20therefore\x20\
    not\x20filled\x20into\n\x20the\x20request\x20will\x20be\x20reset\x20to\
    \x20their\x20default.\x20If\x20this\x20is\x20unwanted\n\x20behavior,\x20\
    a\x20specific\x20service\x20may\x20require\x20a\x20client\x20to\x20alway\
    s\x20specify\n\x20a\x20field\x20mask,\x20producing\x20an\x20error\x20if\
    \x20not.\n\n\x20As\x20with\x20get\x20operations,\x20the\x20location\x20o\
    f\x20the\x20resource\x20which\n\x20describes\x20the\x20updated\x20values\
    \x20in\x20the\x20request\x20message\x20depends\x20on\x20the\n\x20operati\
    on\x20kind.\x20In\x20any\x20case,\x20the\x20effect\x20of\x20the\x20field\
    \x20mask\x20is\n\x20required\x20to\x20be\x20honored\x20by\x20the\x20API.\
    \n\n\x20##\x20Considerations\x20for\x20HTTP\x20REST\n\n\x20The\x20HTTP\
    \x20kind\x20of\x20an\x20update\x20operation\x20which\x20uses\x20a\x20fie\
    ld\x20mask\x20must\n\x20be\x20set\x20to\x20PATCH\x20instead\x20of\x20PUT\
    \x20in\x20order\x20to\x20satisfy\x20HTTP\x20semantics\n\x20(PUT\x20must\
    \x20only\x20be\x20used\x20for\x20full\x20updates).\n\n\x20#\x20JSON\x20E\
    ncoding\x20of\x20Field\x20Masks\n\n\x20In\x20JSON,\x20a\x20field\x20mask\
    \x20is\x20encoded\x20as\x20a\x20single\x20string\x20where\x20paths\x20ar\
    e\n\x20separated\x20by\x20a\x20comma.\x20Fields\x20name\x20in\x20each\
    \x20path\x20are\x20converted\n\x20to/from\x20lower-camel\x20naming\x20co\
    nventions.\n\n\x20As\x20an\x20example,\x20consider\x20the\x20following\
    \x20message\x20declarations:\n\n\x20\x20\x20\x20\x20message\x20Profile\
    \x20{\n\x20\x20\x20\x20\x20\x20\x20User\x20user\x20=\x201;\n\x20\x20\x20\
    \x20\x20\x20\x20Photo\x20photo\x20=\x202;\n\x20\x20\x20\x20\x20}\n\x20\
    \x20\x20\x20\x20message\x20User\x20{\n\x20\x20\x20\x20\x20\x20\x20string\
    \x20display_name\x20=\x201;\n\x20\x20\x20\x20\x20\x20\x20string\x20addre\
    ss\x20=\x202;\n\x20\x20\x20\x20\x20}\n\n\x20In\x20proto\x20a\x20field\
    \x20mask\x20for\x20`Profile`\x20may\x20look\x20as\x20such:\n\n\x20\x20\
    \x20\x20\x20mask\x20{\n\x20\x20\x20\x20\x20\x20\x20paths:\x20\"user.disp\
    lay_name\"\n\x20\x20\x20\x20\x20\x20\x20paths:\x20\"photo\"\n\x20\x20\
    \x20\x20\x20}\n\n\x20In\x20JSON,\x20the\x20same\x20mask\x20is\x20represe\
    nted\x20as\x20below:\n\n\x20\x20\x20\x20\x20{\n\x20\x20\x20\x20\x20\x20\
    \x20mask:\x20\"user.displayName,photo\"\n\x20\x20\x20\x20\x20}\n\n\x20#\
    \x20Field\x20Masks\x20and\x20Oneof\x20Fields\n\n\x20Field\x20masks\x20tr\
    eat\x20fields\x20in\x20oneofs\x20just\x20as\x20regular\x20fields.\x20Con\
    sider\x20the\n\x20following\x20message:\n\n\x20\x20\x20\x20\x20message\
    \x20SampleMessage\x20{\n\x20\x20\x20\x20\x20\x20\x20oneof\x20test_oneof\
    \x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20string\x20name\x20=\x204;\n\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20SubMessage\x20sub_message\x20=\x209;\
    \n\x20\x20\x20\x20\x20\x20\x20}\n\x20\x20\x20\x20\x20}\n\n\x20The\x20fie\
    ld\x20mask\x20can\x20be:\n\n\x20\x20\x20\x20\x20mask\x20{\n\x20\x20\x20\
    \x20\x20\x20\x20paths:\x20\"name\"\n\x20\x20\x20\x20\x20}\n\n\x20Or:\n\n\
    \x20\x20\x20\x20\x20mask\x20{\n\x20\x20\x20\x20\x20\x20\x20paths:\x20\"s\
    ub_message\"\n\x20\x20\x20\x20\x20}\n\n\x20Note\x20that\x20oneof\x20type\
    \x20names\x20(\"test_oneof\"\x20in\x20this\x20case)\x20cannot\x20be\x20u\
    sed\x20in\n\x20paths.\n\n\x0b\n\x03\x04\0\x01\x12\x04\xf1\x01\x08\x11\n,\
    \n\x04\x04\0\x02\0\x12\x04\xf3\x01\x02\x1c\x1a\x1e\x20The\x20set\x20of\
    \x20field\x20mask\x20paths.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\xf3\x01\
    \x02\n\n\r\n\x05\x04\0\x02\0\x05\x12\x04\xf3\x01\x0b\x11\n\r\n\x05\x04\0\
    \x02\0\x01\x12\x04\xf3\x01\x12\x17\n\r\n\x05\x04\0\x02\0\x03\x12\x04\xf3\
    \x01\x1a\x1bb\x06proto3\
";

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

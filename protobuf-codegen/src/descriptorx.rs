//! Utilities to work with descriptor

use protobuf::descriptor::EnumValueDescriptorProto;
use protobuf::descriptor::FieldDescriptorProto;

use rust;
use ident::RustIdent;


pub trait EnumValueDescriptorEx {
    fn rust_name(&self) -> RustIdent;
}

impl EnumValueDescriptorEx for EnumValueDescriptorProto {
    fn rust_name(&self) -> RustIdent {
        let mut r = String::new();
        if rust::is_rust_keyword(self.get_name()) {
            r.push_str("value_");
        }
        r.push_str(self.get_name());
        RustIdent::new(&r)
    }
}

pub trait FieldDescriptorProtoExt {
    fn rust_name(&self) -> String;
}

impl FieldDescriptorProtoExt for FieldDescriptorProto {
    fn rust_name(&self) -> String {
        if rust::is_rust_keyword(self.get_name()) {
            format!("field_{}", self.get_name())
        } else {
            self.get_name().to_string()
        }
    }
}

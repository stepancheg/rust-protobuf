//! Utilities to work with descriptor

use protobuf::descriptor::FieldDescriptorProto;

use rust;


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

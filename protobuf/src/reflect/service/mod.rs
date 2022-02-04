use crate::descriptor::MethodDescriptorProto;
use crate::descriptor::ServiceDescriptorProto;
use crate::reflect::FileDescriptor;
use crate::reflect::MessageDescriptor;

/// Dynamic representation of service type.
///
/// Rust-protobuf does not support services (it is not an RPC library),
/// but it support querying service description. Which might be useful
/// for example to generate source files for the services.
/// or to perform invocations dynamically.
#[derive(Clone, Eq, PartialEq)]
pub struct ServiceDescriptor {
    file_descriptor: FileDescriptor,
    index: usize,
}

impl ServiceDescriptor {
    pub(crate) fn new(file_descriptor: FileDescriptor, index: usize) -> ServiceDescriptor {
        ServiceDescriptor {
            file_descriptor,
            index,
        }
    }

    /// Proto snippet describing this service.
    pub fn get_proto(&self) -> &ServiceDescriptorProto {
        &self.file_descriptor.proto().service[self.index]
    }

    /// Method descriptors of this service.
    pub fn methods(&self) -> Vec<MethodDescriptor> {
        let value_len = self.get_proto().method.len();
        (0..value_len)
            .map(move |index| MethodDescriptor {
                service_descriptor: self.clone(),
                index,
            })
            .collect()
    }
}

/// Service method descriptor.
pub struct MethodDescriptor {
    service_descriptor: ServiceDescriptor,
    index: usize,
}

impl MethodDescriptor {
    /// Proto snippet describing this method.
    pub fn get_proto(&self) -> &MethodDescriptorProto {
        &self.service_descriptor.get_proto().method[self.index]
    }

    /// Method input type.
    pub fn input_type(&self) -> MessageDescriptor {
        // TODO
        unimplemented!()
    }

    /// Method output type.
    pub fn output_type(&self) -> MessageDescriptor {
        // TODO
        unimplemented!()
    }
}

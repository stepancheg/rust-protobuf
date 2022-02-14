use protobuf::descriptor::DescriptorProto;
use protobuf::descriptor::EnumDescriptorProto;
use protobuf::descriptor::FileDescriptorProto;
use protobuf::descriptor::ServiceDescriptorProto;
use protobuf::reflect::FileDescriptor;

use crate::model;
use crate::model::WithLoc;
use crate::pure::convert::Resolver;
use crate::ProtobufAbsPathRef;
use crate::ProtobufIdentRef;

pub(crate) struct OptionResoler<'a> {
    pub(crate) resolver: &'a Resolver<'a>,
    pub(crate) descriptor_without_options: FileDescriptor,
}

impl<'a> OptionResoler<'a> {
    fn service(
        &self,
        service_proto: &mut ServiceDescriptorProto,
        service_model: &WithLoc<model::Service>,
    ) -> anyhow::Result<()> {
        service_proto.options = self
            .resolver
            .service_options(&service_model.options)?
            .into();
        Ok(())
    }

    fn enumeration(
        &self,
        scope: &ProtobufAbsPathRef,
        enum_proto: &mut EnumDescriptorProto,
        enum_model: &WithLoc<model::Enumeration>,
    ) -> anyhow::Result<()> {
        enum_proto.options = self
            .resolver
            .enum_options(scope, &enum_model.options)?
            .into();
        Ok(())
    }

    fn message(
        &self,
        scope: &ProtobufAbsPathRef,
        message_proto: &mut DescriptorProto,
        message_model: &WithLoc<model::Message>,
    ) -> anyhow::Result<()> {
        message_proto.options = self
            .resolver
            .message_options(scope, &message_model.options)?
            .into();

        let mut nested_scope = scope.to_owned();
        nested_scope.push_simple(ProtobufIdentRef::new(&message_proto.get_name()));

        for nested_message_model in &message_model.messages {
            let nested_message_proto = message_proto
                .nested_type
                .iter_mut()
                .find(|nested_message_proto| {
                    nested_message_proto.get_name() == nested_message_model.name
                })
                .unwrap();
            self.message(&nested_scope, nested_message_proto, nested_message_model)?;
        }

        for nested_enum_model in &message_model.enums {
            let nested_enum_proto = message_proto
                .enum_type
                .iter_mut()
                .find(|nested_enum_proto| nested_enum_proto.get_name() == nested_enum_model.name)
                .unwrap();
            self.enumeration(&nested_scope, nested_enum_proto, nested_enum_model)?;
        }

        Ok(())
    }

    pub(crate) fn file(&self, output: &mut FileDescriptorProto) -> anyhow::Result<()> {
        // TODO: use it to resolve messages.
        let _ = &self.descriptor_without_options;

        for message_model in &self.resolver.current_file.messages {
            let message_proto = output
                .message_type
                .iter_mut()
                .find(|m| m.get_name() == message_model.name)
                .unwrap();
            self.message(
                &self.resolver.current_file.package,
                message_proto,
                message_model,
            )?;
        }

        for enum_model in &self.resolver.current_file.enums {
            let enum_proto = output
                .enum_type
                .iter_mut()
                .find(|e| e.get_name() == enum_model.name)
                .unwrap();
            self.enumeration(&self.resolver.current_file.package, enum_proto, enum_model)?;
        }

        for service_proto in &mut output.service {
            let service_model = self
                .resolver
                .current_file
                .services
                .iter()
                .find(|s| s.name == service_proto.get_name())
                .unwrap();
            self.service(service_proto, service_model)?;
        }

        output.options = self
            .resolver
            .file_options(
                &self.resolver.current_file.package,
                &self.resolver.current_file.options,
            )?
            .into();

        Ok(())
    }
}

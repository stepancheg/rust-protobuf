use std::fmt;

use protobuf::reflect::EnumDescriptor;
use protobuf::reflect::FieldDescriptor;
use protobuf::reflect::FileDescriptor;
use protobuf::reflect::MessageDescriptor;
use protobuf::reflect::OneofDescriptor;

use crate::customize::CustomizeCallback;
use crate::Customize;

#[derive(Clone)]
pub(crate) struct CustomizeElemCtx<'a> {
    pub(crate) for_elem: Customize,
    pub(crate) for_children: Customize,
    pub(crate) callback: &'a dyn CustomizeCallback,
}

impl<'a> fmt::Debug for CustomizeElemCtx<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CustomizeElemCtx")
            .field("for_elem", &self.for_elem)
            .field("for_children", &self.for_children)
            .finish_non_exhaustive()
    }
}

impl<'a> CustomizeElemCtx<'a> {
    pub(crate) fn child(
        &self,
        elem_from_rustproto: &Customize,
        elem_descriptor: &impl DescriptorForCustomize,
    ) -> CustomizeElemCtx<'a> {
        let mut for_elem = self.for_children.clone();
        for_elem.update_with(elem_from_rustproto);

        let for_children = for_elem.clone();

        for_elem.update_with(&elem_descriptor.customize(self.callback));

        CustomizeElemCtx {
            for_elem,
            for_children,
            callback: self.callback,
        }
    }
}

pub(crate) trait DescriptorForCustomize {
    fn customize(&self, callback: &dyn CustomizeCallback) -> Customize;
}

impl DescriptorForCustomize for MessageDescriptor {
    fn customize(&self, callback: &dyn CustomizeCallback) -> Customize {
        callback.message(self)
    }
}

impl DescriptorForCustomize for FieldDescriptor {
    fn customize(&self, callback: &dyn CustomizeCallback) -> Customize {
        callback.field(self)
    }
}

impl DescriptorForCustomize for EnumDescriptor {
    fn customize(&self, callback: &dyn CustomizeCallback) -> Customize {
        callback.enumeration(self)
    }
}

impl DescriptorForCustomize for OneofDescriptor {
    fn customize(&self, callback: &dyn CustomizeCallback) -> Customize {
        callback.oneof(self)
    }
}

impl DescriptorForCustomize for FileDescriptor {
    fn customize(&self, callback: &dyn CustomizeCallback) -> Customize {
        callback.file(self)
    }
}

pub(crate) struct SpecialFieldPseudoDescriptor<'a> {
    pub(crate) message: &'a MessageDescriptor,
    pub(crate) field: &'a str,
}

impl<'a> DescriptorForCustomize for SpecialFieldPseudoDescriptor<'a> {
    fn customize(&self, callback: &dyn CustomizeCallback) -> Customize {
        callback.special_field(self.message, self.field)
    }
}

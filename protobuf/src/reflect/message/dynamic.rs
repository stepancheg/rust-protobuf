use crate::descriptor::FileDescriptorProto;
use crate::reflect::file::building::FileDescriptorBuilding;
use crate::reflect::message::index::MessageIndex;
use crate::reflect::message::path::MessagePath;

#[derive(Debug)]
pub(crate) struct DynamicMessageDescriptor {
    pub indices: MessageIndex,
}

impl DynamicMessageDescriptor {
    pub fn new(
        proto: &FileDescriptorProto,
        path: &MessagePath,
        building: &FileDescriptorBuilding,
    ) -> crate::Result<DynamicMessageDescriptor> {
        let m = path.eval(proto).unwrap();
        let indices = MessageIndex::index(m, building)?;

        Ok(DynamicMessageDescriptor { indices })
    }
}

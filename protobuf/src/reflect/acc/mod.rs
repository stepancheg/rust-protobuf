use crate::reflect::acc::v2::AccessorV2;

pub(crate) mod v2;

pub(crate) enum Accessor {
    V2(AccessorV2),
}

/// Accessor object is constructed in generated code.
/// Should not be used directly.
pub struct FieldAccessor {
    pub(crate) name: &'static str,
    pub(crate) accessor: Accessor,
}

impl FieldAccessor {
    pub(crate) fn new_v2(name: &'static str, accessor: AccessorV2) -> FieldAccessor {
        FieldAccessor {
            name,
            accessor: Accessor::V2(accessor),
        }
    }
}

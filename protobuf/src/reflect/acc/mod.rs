use crate::reflect::acc::v2::AccessorV2;

pub(crate) mod v2;

pub(crate) enum GeneratedFieldAccessor {
    V2(AccessorV2),
}

/// Accessor object is constructed in generated code.
/// Should not be used directly.
pub struct FieldAccessor {
    pub(crate) name: &'static str,
    pub(crate) accessor: GeneratedFieldAccessor,
}

impl FieldAccessor {
    pub(crate) fn new_v2(name: &'static str, accessor: AccessorV2) -> FieldAccessor {
        FieldAccessor {
            name,
            accessor: GeneratedFieldAccessor::V2(accessor),
        }
    }
}

use crate::reflect::accessor::map::MapFieldAccessorHolder;
use crate::reflect::accessor::repeated::RepeatedFieldAccessorHolder;
use crate::reflect::accessor::singular::SingularFieldAccessorHolder;

pub(crate) mod map;
pub(crate) mod repeated;
pub(crate) mod singular;

pub(crate) enum AccessorKind {
    Singular(SingularFieldAccessorHolder),
    Repeated(RepeatedFieldAccessorHolder),
    Map(MapFieldAccessorHolder),
}

/// Accessor object is constructed in generated code.
/// Should not be used directly.
pub struct FieldAccessor {
    pub(crate) name: &'static str,
    pub(crate) accessor: AccessorKind,
}

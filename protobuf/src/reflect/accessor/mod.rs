use reflect::accessor::singular::SingularFieldAccessorHolder;
use reflect::accessor::repeated::RepeatedFieldAccessorHolder;
use reflect::accessor::map::MapFieldAccessorHolder;

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

use reflect::accessor::map::MapFieldAccessor;
use reflect::accessor::repeated::RepeatedFieldAccessor;
use reflect::accessor::singular::SingularFieldAccessor;

pub(crate) mod repeated;
pub(crate) mod singular;
pub(crate) mod map;


pub(crate) enum AccessorKind {
    Singular(Box<SingularFieldAccessor>),
    Repeated(Box<RepeatedFieldAccessor>),
    Map(Box<MapFieldAccessor>),
}

/// Accessor object is constructed in generated code.
/// Should not be used directly.
pub struct FieldAccessor {
    pub(crate) name: &'static str,
    pub(crate) accessor: AccessorKind,
}

use crate::reflect::accessor::singular::GetOptionImplHasGetCopy;
use crate::reflect::accessor::singular::GetOptionImplHasGetRef;
use crate::reflect::accessor::singular::GetOptionImplHasGetRefDeref;
use crate::reflect::accessor::singular::GetOrDefaultGetCopy;
use crate::reflect::accessor::singular::GetOrDefaultGetRef;
use crate::reflect::accessor::singular::GetOrDefaultGetRefDeref;
use crate::reflect::accessor::singular::MutOrDefaultGetMut;
use crate::reflect::accessor::singular::MutOrDefaultUnmplemented;
use crate::reflect::accessor::singular::SetImplSetField;
use crate::reflect::accessor::singular::SingularFieldAccessorHolder;
use crate::reflect::accessor::singular::SingularFieldAccessorImpl;
use crate::reflect::accessor::AccessorKind;
use crate::reflect::accessor::FieldAccessor;
use crate::reflect::runtime_types::RuntimeType;
use crate::reflect::runtime_types::RuntimeTypeMessage;
use crate::reflect::runtime_types::RuntimeTypeWithDeref;
use crate::reflect::types::ProtobufType;
use crate::reflect::types::ProtobufTypeMessage;
use crate::Message;
use std::marker;

/// Make accessor for `Copy` field
pub fn make_oneof_copy_has_get_set_accessors<M, V>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: fn(&M) -> <V::RuntimeType as RuntimeType>::Value,
    set: fn(&mut M, <V::RuntimeType as RuntimeType>::Value),
) -> FieldAccessor
where
    M: Message + 'static,
    V: ProtobufType + 'static,
    <V::RuntimeType as RuntimeType>::Value: Copy,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(SingularFieldAccessorImpl::<M, V, _, _, _, _> {
                get_option_impl: GetOptionImplHasGetCopy::<M, V::RuntimeType> { has, get },
                get_or_default_impl: GetOrDefaultGetCopy::<M, V::RuntimeType> { get_field: get },
                mut_or_default_impl: MutOrDefaultUnmplemented::new(),
                set_impl: SetImplSetField::<M, V::RuntimeType> { set_field: set },
                _marker: marker::PhantomData,
            }),
            element_type: V::dynamic(),
        }),
    }
}

/// Make accessor for `oneof` field
pub fn make_oneof_deref_has_get_set_accessor<M, F>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: for<'a> fn(&'a M) -> &'a <F::RuntimeType as RuntimeTypeWithDeref>::DerefTarget,
    set: fn(&mut M, <F::RuntimeType as RuntimeType>::Value),
) -> FieldAccessor
where
    M: Message + 'static,
    F: ProtobufType,
    F::RuntimeType: RuntimeTypeWithDeref,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(SingularFieldAccessorImpl::<M, F, _, _, _, _> {
                get_option_impl: GetOptionImplHasGetRefDeref::<M, F::RuntimeType> { has, get },
                get_or_default_impl: GetOrDefaultGetRefDeref::<M, F::RuntimeType> {
                    get_field: get,
                },
                mut_or_default_impl: MutOrDefaultUnmplemented::new(),
                set_impl: SetImplSetField::<M, F::RuntimeType> { set_field: set },
                _marker: marker::PhantomData,
            }),
            element_type: F::dynamic(),
        }),
    }
}

/// Make accessor for `oneof` `message` field
pub fn make_oneof_message_has_get_mut_set_accessor<M, F>(
    name: &'static str,
    has_field: fn(&M) -> bool,
    get_field: for<'a> fn(&'a M) -> &'a F,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut F,
    set_field: fn(&mut M, F),
) -> FieldAccessor
where
    M: Message + 'static,
    F: Message + Default + Clone + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(
                SingularFieldAccessorImpl::<M, ProtobufTypeMessage<F>, _, _, _, _> {
                    get_option_impl: GetOptionImplHasGetRef::<M, RuntimeTypeMessage<F>> {
                        get: get_field,
                        has: has_field,
                    },
                    get_or_default_impl: GetOrDefaultGetRef::<M, RuntimeTypeMessage<F>> {
                        get_field,
                    },
                    mut_or_default_impl: MutOrDefaultGetMut::<M, RuntimeTypeMessage<F>> {
                        mut_field,
                    },
                    set_impl: SetImplSetField::<M, RuntimeTypeMessage<F>> { set_field },
                    _marker: marker::PhantomData,
                },
            ),
            element_type: ProtobufTypeMessage::<F>::dynamic(),
        }),
    }
}

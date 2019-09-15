use std::marker;

use crate::core::Message;
use crate::enums::ProtobufEnum;
use crate::enums::ProtobufEnumOrUnknown;
use crate::reflect::accessor::AccessorKind;
use crate::reflect::accessor::FieldAccessor;
use crate::reflect::runtime_types::RuntimeType;
use crate::reflect::runtime_types::RuntimeTypeEnumOrUnknown;
use crate::reflect::runtime_types::RuntimeTypeWithDeref;
use crate::reflect::type_dynamic::ProtobufTypeDynamic;
use crate::reflect::types::ProtobufType;
use crate::reflect::types::ProtobufTypeEnumOrUnknown;
use crate::reflect::value::ReflectValueMut;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::singular::OptionLike;

pub(crate) mod oneof;

/// This trait should not be used directly, use `FieldDescriptor` instead
pub(crate) trait SingularFieldAccessor: Send + Sync + 'static {
    fn get_field<'a>(&self, m: &'a dyn Message) -> Option<ReflectValueRef<'a>>;
    fn get_field_or_default<'a>(&self, m: &'a dyn Message) -> ReflectValueRef<'a>;
    fn mut_field_or_default<'a>(&self, m: &'a mut dyn Message) -> ReflectValueMut<'a>;
    fn set_field(&self, m: &mut dyn Message, value: ReflectValueBox);
}

pub(crate) struct SingularFieldAccessorHolder {
    pub accessor: Box<dyn SingularFieldAccessor>,
    pub element_type: &'static dyn ProtobufTypeDynamic,
}

trait GetOptionImpl<M>: Send + Sync + 'static {
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>>;
}

trait GetOrDefaultImpl<M>: Send + Sync + 'static {
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a>;
}

trait MutOrDefaultImpl<M>: Send + Sync + 'static {
    fn mut_singular_field_or_default_impl<'a>(&self, m: &'a mut M) -> ReflectValueMut<'a>;
}

trait SetImpl<M>: Send + Sync + 'static {
    fn set_singular_field(&self, m: &mut M, value: ReflectValueBox);
}

struct MutOrDefaultUnmplemented<M>
where
    M: Message,
{
    _marker: marker::PhantomData<M>,
}

impl<M> MutOrDefaultUnmplemented<M>
where
    M: Message,
{
    fn new() -> MutOrDefaultUnmplemented<M> {
        MutOrDefaultUnmplemented {
            _marker: marker::PhantomData,
        }
    }
}

impl<M> MutOrDefaultImpl<M> for MutOrDefaultUnmplemented<M>
where
    M: Message,
{
    fn mut_singular_field_or_default_impl<'a>(&self, _m: &'a mut M) -> ReflectValueMut<'a> {
        unimplemented!()
    }
}

struct SingularFieldAccessorImpl<M, V, G, D, E, S>
where
    M: Message,
    V: ProtobufType,
    G: GetOptionImpl<M>,
    D: GetOrDefaultImpl<M>,
    E: MutOrDefaultImpl<M>,
    S: SetImpl<M>,
{
    get_option_impl: G,
    get_or_default_impl: D,
    mut_or_default_impl: E,
    set_impl: S,
    _marker: marker::PhantomData<(M, V)>,
}

impl<M, V, G, D, E, S> SingularFieldAccessor for SingularFieldAccessorImpl<M, V, G, D, E, S>
where
    M: Message,
    V: ProtobufType,
    G: GetOptionImpl<M>,
    D: GetOrDefaultImpl<M>,
    E: MutOrDefaultImpl<M>,
    S: SetImpl<M>,
{
    fn get_field<'a>(&self, m: &'a dyn Message) -> Option<ReflectValueRef<'a>> {
        let m = m.downcast_ref().unwrap();
        self.get_option_impl.get_reflect_impl(m)
    }

    fn get_field_or_default<'a>(&self, m: &'a dyn Message) -> ReflectValueRef<'a> {
        let m = m.downcast_ref().unwrap();
        self.get_or_default_impl
            .get_singular_field_or_default_impl(m)
    }

    fn mut_field_or_default<'a>(&self, m: &'a mut dyn Message) -> ReflectValueMut<'a> {
        let m = m.downcast_mut().unwrap();
        self.mut_or_default_impl
            .mut_singular_field_or_default_impl(m)
    }

    fn set_field(&self, m: &mut dyn Message, value: ReflectValueBox) {
        let m = m.downcast_mut().unwrap();
        self.set_impl.set_singular_field(m, value)
    }
}

struct GetOptionImplFieldPointer<M, V>
where
    M: Message,
    V: RuntimeType,
{
    get_field: for<'a> fn(&'a M) -> &'a V::Value,
}

struct GetOptionImplOptionFieldPointer<M, V, O>
where
    M: Message,
    V: RuntimeType,
    O: OptionLike<V::Value> + Sync + Send + 'static,
{
    get_field: for<'a> fn(&'a M) -> &'a O,
    _marker: marker::PhantomData<V>,
}

struct GetOptionImplHasGetRef<M, V>
where
    M: Message,
    V: RuntimeType,
{
    has: for<'a> fn(&'a M) -> bool,
    get: for<'a> fn(&'a M) -> &'a V::Value,
}

struct GetOptionImplHasGetRefDeref<M, V>
where
    M: Message,
    V: RuntimeTypeWithDeref,
{
    has: for<'a> fn(&'a M) -> bool,
    get: for<'a> fn(&'a M) -> &'a V::DerefTarget,
}

struct GetOptionImplHasGetCopy<M, V>
where
    M: Message,
    V: RuntimeType,
{
    has: for<'a> fn(&'a M) -> bool,
    get: for<'a> fn(&'a M) -> V::Value,
}

impl<M, V> GetOptionImpl<M> for GetOptionImplFieldPointer<M, V>
where
    M: Message,
    V: RuntimeType,
{
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        let v = (self.get_field)(m);
        if V::is_non_zero(v) {
            Some(V::as_ref(v))
        } else {
            None
        }
    }
}

impl<M, V, O> GetOptionImpl<M> for GetOptionImplOptionFieldPointer<M, V, O>
where
    M: Message,
    V: RuntimeType,
    O: OptionLike<V::Value> + Sync + Send + 'static,
{
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        (self.get_field)(m).as_option_ref().map(V::as_ref)
    }
}

impl<M, V> GetOptionImpl<M> for GetOptionImplHasGetRef<M, V>
where
    M: Message,
    V: RuntimeType,
{
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        if (self.has)(m) {
            Some(V::as_ref((self.get)(m)))
        } else {
            None
        }
    }
}

impl<M, V> GetOptionImpl<M> for GetOptionImplHasGetRefDeref<M, V>
where
    M: Message,
    V: RuntimeTypeWithDeref,
{
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        if (self.has)(m) {
            Some(V::defef_as_ref((self.get)(m)))
        } else {
            None
        }
    }
}

impl<M, V> GetOptionImpl<M> for GetOptionImplHasGetCopy<M, V>
where
    M: Message,
    V: RuntimeType,
{
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        if (self.has)(m) {
            Some(V::into_static_value_ref((self.get)(m)))
        } else {
            None
        }
    }
}

struct GetOrDefaultGetRef<M, V>
where
    M: Message,
    V: RuntimeType,
{
    get_field: for<'a> fn(&'a M) -> &'a V::Value,
}

struct GetOrDefaultGetRefDeref<M, V>
where
    M: Message,
    V: RuntimeTypeWithDeref,
{
    get_field: for<'a> fn(&'a M) -> &'a V::DerefTarget,
}

struct GetOrDefaultOptionRefTypeDefault<M, V, O>
where
    M: Message,
    V: RuntimeType,
    O: OptionLike<V::Value> + Sync + Send + 'static,
{
    get_field: for<'a> fn(&'a M) -> &'a O,
    _marker: marker::PhantomData<V>,
}

struct GetOrDefaultGetCopy<M, V>
where
    M: Message,
    V: RuntimeType,
{
    get_field: for<'a> fn(&'a M) -> V::Value,
}

impl<M, V> GetOrDefaultImpl<M> for GetOrDefaultGetRef<M, V>
where
    M: Message,
    V: RuntimeType,
{
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        V::as_ref((self.get_field)(m))
    }
}

impl<M, V, O> GetOrDefaultImpl<M> for GetOrDefaultOptionRefTypeDefault<M, V, O>
where
    M: Message,
    V: RuntimeType,
    O: OptionLike<V::Value> + Sync + Send + 'static,
{
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        match (self.get_field)(m).as_option_ref() {
            Some(v) => V::as_ref(v),
            None => V::default_value_ref(),
        }
    }
}

impl<M, V> GetOrDefaultImpl<M> for GetOrDefaultGetRefDeref<M, V>
where
    M: Message,
    V: RuntimeTypeWithDeref,
{
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        V::defef_as_ref((self.get_field)(m))
    }
}

impl<M, V> GetOrDefaultImpl<M> for GetOrDefaultGetCopy<M, V>
where
    M: Message,
    V: RuntimeType,
{
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        V::into_static_value_ref((self.get_field)(m))
    }
}

struct MutOrDefaultGetMut<M, V>
where
    M: Message,
    V: RuntimeType,
{
    mut_field: for<'a> fn(&'a mut M) -> &'a mut V::Value,
}

impl<M, V> MutOrDefaultImpl<M> for MutOrDefaultGetMut<M, V>
where
    M: Message,
    V: RuntimeType,
{
    fn mut_singular_field_or_default_impl<'a>(&self, m: &'a mut M) -> ReflectValueMut<'a> {
        V::as_mut((self.mut_field)(m))
    }
}

struct MutOrDefaultOptionMut<M, V, O>
where
    M: Message,
    V: RuntimeType,
    O: OptionLike<V::Value> + Sync + Send + 'static,
{
    mut_field: for<'a> fn(&'a mut M) -> &'a mut O,
    _marker: marker::PhantomData<V>,
}

impl<M, V, O> MutOrDefaultImpl<M> for MutOrDefaultOptionMut<M, V, O>
where
    M: Message,
    V: RuntimeType,
    O: OptionLike<V::Value> + Sync + Send + 'static,
{
    fn mut_singular_field_or_default_impl<'a>(&self, m: &'a mut M) -> ReflectValueMut<'a> {
        let option = (self.mut_field)(m);
        if option.as_option_ref().is_none() {
            option.set_value(V::Value::default());
        }
        V::as_mut(option.as_option_mut().unwrap())
    }
}

struct SetImplFieldPointer<M, V>
where
    M: Message,
    V: RuntimeType,
{
    mut_field: for<'a> fn(&'a mut M) -> &'a mut V::Value,
}

struct SetImplOptionFieldPointer<M, V, O>
where
    M: Message,
    V: RuntimeType,
    O: OptionLike<V::Value> + Sync + Send + 'static,
{
    mut_field: for<'a> fn(&'a mut M) -> &'a mut O,
    _marker: marker::PhantomData<V>,
}

struct SetImplSetField<M, V>
where
    M: Message,
    V: RuntimeType,
{
    set_field: for<'a> fn(&'a mut M, V::Value),
}

impl<M, V> SetImpl<M> for SetImplFieldPointer<M, V>
where
    M: Message,
    V: RuntimeType,
{
    fn set_singular_field(&self, m: &mut M, value: ReflectValueBox) {
        V::set_from_value_box((self.mut_field)(m), value);
    }
}

impl<M, V, O> SetImpl<M> for SetImplOptionFieldPointer<M, V, O>
where
    M: Message,
    V: RuntimeType,
    O: OptionLike<V::Value> + Sync + Send + 'static,
{
    fn set_singular_field(&self, m: &mut M, value: ReflectValueBox) {
        (self.mut_field)(m).set_value(V::from_value_box(value));
    }
}

impl<M, V> SetImpl<M> for SetImplSetField<M, V>
where
    M: Message,
    V: RuntimeType,
{
    fn set_singular_field(&self, m: &mut M, value: ReflectValueBox) {
        let value = value.downcast::<V::Value>().expect("message");
        (self.set_field)(m, value)
    }
}

/// Make accessor for option or option-like field
pub fn make_option_accessor<M, V, O>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a O,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut O,
) -> FieldAccessor
where
    M: Message + 'static,
    V: ProtobufType + 'static,
    O: OptionLike<<V::RuntimeType as RuntimeType>::Value> + Send + Sync + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(SingularFieldAccessorImpl::<M, V, _, _, _, _> {
                get_option_impl: GetOptionImplOptionFieldPointer::<M, V::RuntimeType, _> {
                    get_field,
                    _marker: marker::PhantomData,
                },
                get_or_default_impl: GetOrDefaultOptionRefTypeDefault::<M, V::RuntimeType, _> {
                    get_field,
                    _marker: marker::PhantomData,
                },
                mut_or_default_impl: MutOrDefaultOptionMut::<M, V::RuntimeType, _> {
                    mut_field,
                    _marker: marker::PhantomData,
                },
                set_impl: SetImplOptionFieldPointer::<M, V::RuntimeType, _> {
                    mut_field,
                    _marker: marker::PhantomData,
                },
                _marker: marker::PhantomData,
            }),
            element_type: V::dynamic(),
        }),
    }
}

/// Make accessor for option-like field
pub fn make_option_get_copy_accessor<M, V, O>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a O,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut O,
    get_value: fn(&M) -> <V::RuntimeType as RuntimeType>::Value,
) -> FieldAccessor
where
    M: Message + 'static,
    V: ProtobufType + 'static,
    O: OptionLike<<V::RuntimeType as RuntimeType>::Value> + Send + Sync + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(SingularFieldAccessorImpl::<M, V, _, _, _, _> {
                get_option_impl: GetOptionImplOptionFieldPointer::<M, V::RuntimeType, O> {
                    get_field,
                    _marker: marker::PhantomData,
                },
                get_or_default_impl: GetOrDefaultGetCopy::<M, V::RuntimeType> {
                    get_field: get_value,
                },
                mut_or_default_impl: MutOrDefaultUnmplemented::new(),
                set_impl: SetImplOptionFieldPointer::<M, V::RuntimeType, O> {
                    mut_field,
                    _marker: marker::PhantomData,
                },
                _marker: marker::PhantomData,
            }),
            element_type: V::dynamic(),
        }),
    }
}

struct GetOrDefaultEnum<M, E: ProtobufEnum> {
    get_field: for<'a> fn(&'a M) -> &'a Option<ProtobufEnumOrUnknown<E>>,
    default_value: E,
}

impl<M: Message, E: ProtobufEnum> GetOrDefaultImpl<M> for GetOrDefaultEnum<M, E> {
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        ReflectValueRef::Enum(
            E::enum_descriptor_static(),
            match (self.get_field)(m) {
                Some(e) => e.value(),
                None => self.default_value.value(),
            },
        )
    }
}

/// Make accessor for enum field
pub fn make_option_enum_accessor<M, E>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a Option<ProtobufEnumOrUnknown<E>>,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut Option<ProtobufEnumOrUnknown<E>>,
    default_value: E,
) -> FieldAccessor
where
    M: Message + 'static,
    E: ProtobufEnum,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(SingularFieldAccessorImpl::<
                M,
                ProtobufTypeEnumOrUnknown<E>,
                _,
                _,
                _,
                _,
            > {
                get_option_impl: GetOptionImplOptionFieldPointer::<
                    M,
                    RuntimeTypeEnumOrUnknown<E>,
                    Option<ProtobufEnumOrUnknown<E>>,
                > {
                    get_field,
                    _marker: marker::PhantomData,
                },
                get_or_default_impl: GetOrDefaultEnum::<M, E> {
                    get_field,
                    default_value,
                },
                mut_or_default_impl: MutOrDefaultUnmplemented::new(),
                set_impl: SetImplOptionFieldPointer::<
                    M,
                    RuntimeTypeEnumOrUnknown<E>,
                    Option<ProtobufEnumOrUnknown<E>>,
                > {
                    mut_field,
                    _marker: marker::PhantomData,
                },
                _marker: marker::PhantomData,
            }),
            element_type: ProtobufTypeEnumOrUnknown::<E>::dynamic(),
        }),
    }
}

/// String or bytes field
pub fn make_option_get_ref_accessor<M, V, O>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a O,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut O,
    get_value: for<'a> fn(&'a M) -> &'a <V::RuntimeType as RuntimeTypeWithDeref>::DerefTarget,
) -> FieldAccessor
where
    M: Message + 'static,
    V: ProtobufType + 'static,
    V::RuntimeType: RuntimeTypeWithDeref,
    O: OptionLike<<V::RuntimeType as RuntimeType>::Value> + Send + Sync + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(SingularFieldAccessorImpl::<M, V, _, _, _, _> {
                get_option_impl: GetOptionImplOptionFieldPointer::<M, V::RuntimeType, O> {
                    get_field,
                    _marker: marker::PhantomData,
                },
                get_or_default_impl: GetOrDefaultGetRefDeref::<M, V::RuntimeType> {
                    get_field: get_value,
                },
                mut_or_default_impl: MutOrDefaultUnmplemented::new(),
                set_impl: SetImplOptionFieldPointer::<M, V::RuntimeType, O> {
                    mut_field,
                    _marker: marker::PhantomData,
                },
                _marker: marker::PhantomData,
            }),
            element_type: V::dynamic(),
        }),
    }
}

/// Make accessor for simple field
pub fn make_simple_field_accessor<M, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a <V::RuntimeType as RuntimeType>::Value,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut <V::RuntimeType as RuntimeType>::Value,
) -> FieldAccessor
where
    M: Message + 'static,
    V: ProtobufType + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(SingularFieldAccessorImpl::<M, V, _, _, _, _> {
                get_option_impl: GetOptionImplFieldPointer::<M, V::RuntimeType> { get_field },
                get_or_default_impl: GetOrDefaultGetRef::<M, V::RuntimeType> { get_field },
                mut_or_default_impl: MutOrDefaultGetMut::<M, V::RuntimeType> { mut_field },
                set_impl: SetImplFieldPointer::<M, V::RuntimeType> { mut_field },
                _marker: marker::PhantomData,
            }),
            element_type: V::dynamic(),
        }),
    }
}

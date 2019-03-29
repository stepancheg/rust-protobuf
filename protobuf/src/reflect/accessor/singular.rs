use reflect::accessor::AccessorKind;
use reflect::accessor::FieldAccessor;
use reflect::runtime_types::RuntimeType;
use reflect::runtime_types::RuntimeTypeMessage;
use reflect::runtime_types::RuntimeTypeWithDeref;
use reflect::type_dynamic::ProtobufTypeDynamic;
use reflect::types::ProtobufType;
use reflect::types::ProtobufTypeMessage;
use reflect::ReflectValueBox;
use reflect::ReflectValueRef;
use singular::OptionLike;
use std::marker;
use Message;

/// This trait should not be used directly, use `FieldDescriptor` instead
pub(crate) trait SingularFieldAccessor: Send + Sync + 'static {
    fn get_reflect<'a>(&self, m: &'a Message) -> Option<ReflectValueRef<'a>>;

    fn get_singular_field_or_default<'a>(&self, m: &'a Message) -> ReflectValueRef<'a>;
    fn set_singular_field(&self, m: &mut Message, value: ReflectValueBox);
}

pub(crate) struct SingularFieldAccessorHolder {
    pub accessor: Box<SingularFieldAccessor>,
    pub element_type: &'static ProtobufTypeDynamic,
}

trait GetOptionImpl<M>: Send + Sync + 'static {
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>>;
}

trait GetOrDefaultImpl<M>: Send + Sync + 'static {
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a>;
}

trait SetImpl<M>: Send + Sync + 'static {
    fn set_singular_field(&self, m: &mut M, value: ReflectValueBox);
}

struct SingularFieldAccessorImpl<M, V, G, D, S>
where
    M: Message,
    V: ProtobufType,
    G: GetOptionImpl<M>,
    D: GetOrDefaultImpl<M>,
    S: SetImpl<M>,
{
    get_option_impl: G,
    get_or_default_impl: D,
    set_impl: S,
    _marker: marker::PhantomData<(M, V)>,
}

impl<M, V, G, D, S> SingularFieldAccessor for SingularFieldAccessorImpl<M, V, G, D, S>
where
    M: Message,
    V: ProtobufType,
    G: GetOptionImpl<M>,
    D: GetOrDefaultImpl<M>,
    S: SetImpl<M>,
{
    fn get_reflect<'a>(&self, m: &'a Message) -> Option<ReflectValueRef<'a>> {
        let m = m.downcast_ref().unwrap();
        self.get_option_impl.get_reflect_impl(m)
    }

    fn get_singular_field_or_default<'a>(&self, m: &'a Message) -> ReflectValueRef<'a> {
        let m = m.downcast_ref().unwrap();
        self.get_or_default_impl
            .get_singular_field_or_default_impl(m)
    }

    fn set_singular_field(&self, m: &mut Message, value: ReflectValueBox) {
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

// TODO: make_singular_xxx_accessor are used only for oneof fields
// oneof codegen should be changed

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
            accessor: Box::new(SingularFieldAccessorImpl::<M, V, _, _, _> {
                get_option_impl: GetOptionImplHasGetCopy::<M, V::RuntimeType> { has, get },
                get_or_default_impl: GetOrDefaultGetCopy::<M, V::RuntimeType> { get_field: get },
                set_impl: SetImplSetField::<M, V::RuntimeType> { set_field: set },
                _marker: marker::PhantomData,
            }),
            element_type: V::dynamic(),
        }),
    }
}

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
            accessor: Box::new(SingularFieldAccessorImpl::<M, F, _, _, _> {
                get_option_impl: GetOptionImplHasGetRefDeref::<M, F::RuntimeType> { has, get },
                get_or_default_impl: GetOrDefaultGetRefDeref::<M, F::RuntimeType> { get_field: get },
                set_impl: SetImplSetField::<M, F::RuntimeType> { set_field: set },
                _marker: marker::PhantomData,
            }),
            element_type: F::dynamic(),
        }),
    }
}

pub fn make_oneof_message_has_get_mut_set_accessor<M, F>(
    name: &'static str,
    has_field: fn(&M) -> bool,
    get_field: for<'a> fn(&'a M) -> &'a F,
    _mut_field: for<'a> fn(&'a mut M) -> &'a mut F,
    set_field: fn(&mut M, F),
) -> FieldAccessor
where
    M: Message + 'static,
    F: Message + Default + Clone + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(SingularFieldAccessorImpl::<
                M,
                ProtobufTypeMessage<F>,
                _,
                _,
                _,
            > {
                get_option_impl: GetOptionImplHasGetRef::<M, RuntimeTypeMessage<F>> {
                    get: get_field,
                    has: has_field,
                },
                get_or_default_impl: GetOrDefaultGetRef::<M, RuntimeTypeMessage<F>> { get_field },
                set_impl: SetImplSetField::<M, RuntimeTypeMessage<F>> { set_field },
                _marker: marker::PhantomData,
            }),
            element_type: ProtobufTypeMessage::<F>::dynamic(),
        }),
    }
}

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
            accessor: Box::new(SingularFieldAccessorImpl::<M, V, _, _, _> {
                get_option_impl: GetOptionImplOptionFieldPointer::<M, V::RuntimeType, _> {
                    get_field,
                    _marker: marker::PhantomData,
                },
                get_or_default_impl: GetOrDefaultOptionRefTypeDefault::<M, V::RuntimeType, _> {
                    get_field,
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
            accessor: Box::new(SingularFieldAccessorImpl::<M, V, _, _, _> {
                get_option_impl: GetOptionImplOptionFieldPointer::<M, V::RuntimeType, O> {
                    get_field,
                    _marker: marker::PhantomData,
                },
                get_or_default_impl: GetOrDefaultGetCopy::<M, V::RuntimeType> {
                    get_field: get_value,
                },
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
            accessor: Box::new(SingularFieldAccessorImpl::<M, V, _, _, _> {
                get_option_impl: GetOptionImplOptionFieldPointer::<M, V::RuntimeType, O> {
                    get_field,
                    _marker: marker::PhantomData,
                },
                get_or_default_impl: GetOrDefaultGetRefDeref::<M, V::RuntimeType> {
                    get_field: get_value,
                },
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
            accessor: Box::new(SingularFieldAccessorImpl::<M, V, _, _, _> {
                get_option_impl: GetOptionImplFieldPointer::<M, V::RuntimeType> { get_field },
                get_or_default_impl: GetOrDefaultGetRef::<M, V::RuntimeType> { get_field },
                set_impl: SetImplFieldPointer::<M, V::RuntimeType> { mut_field },
                _marker: marker::PhantomData,
            }),
            element_type: V::dynamic(),
        }),
    }
}

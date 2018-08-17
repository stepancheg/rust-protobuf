use Message;
use reflect::EnumValueDescriptor;
use reflect::ReflectValueRef;
use reflect::ReflectValueBox;
use reflect::runtime_types::RuntimeType;
use reflect::types::ProtobufType;
use core::message_down_cast;
use reflect::accessor::FieldAccessor;
use reflect::accessor::AccessorKind;
use reflect::types::ProtobufTypeString;
use reflect::types::ProtobufTypeBytes;
use reflect::types::ProtobufTypeMessage;
use singular::OptionLike;
use reflect::runtime_types::RuntimeTypeWithDeref;
use reflect::type_dynamic::ProtobufTypeDynamic;
use core::message_down_cast_mut;
use std::marker;


/// This trait should not be used directly, use `FieldDescriptor` instead
pub(crate) trait SingularFieldAccessor : Send + Sync + 'static {
    fn protobuf_type(&self) -> &'static ProtobufTypeDynamic;
    fn get_reflect<'a>(&self, m: &'a Message) -> Option<ReflectValueRef<'a>>;

    fn get_singular_field_or_default<'a>(&self, m: &'a Message) -> ReflectValueRef<'a>;
    fn set_singular_field(&self, m: &mut Message, value: ReflectValueBox);
}

trait GetMutSetSingularMessage<M> : Send + Sync + 'static {
    fn get_message<'a>(&self, m: &'a M) -> &'a Message;
    fn mut_message<'a>(&self, m: &'a mut M) -> &'a mut Message;
    fn set_message(&self, m: &mut M, field: Box<Message>);
}

trait GetSingularEnum<M> {
    fn get_enum(&self, m: &M) -> &'static EnumValueDescriptor;
}

trait GetOptionImpl<M> : Send + Sync + 'static {
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>>;
}

trait GetOrDefaultImpl<M> : Send + Sync + 'static {
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a>;
}

trait SetImpl<M> : Send + Sync + 'static {
    fn set_singular_field(&self, m: &mut M, value: ReflectValueBox);
}

struct SingularFieldAccessorImpl<M, V, G, D, S>
    where
        M : Message,
        V : ProtobufType,
        G : GetOptionImpl<M>,
        D : GetOrDefaultImpl<M>,
        S : SetImpl<M>,
{
    get_option_impl: G,
    get_or_default_impl: D,
    set_impl: S,
    _marker: marker::PhantomData<(M, V)>,
}

impl<M, V, G, D, S> SingularFieldAccessor for SingularFieldAccessorImpl<M, V, G, D, S>
    where
        M : Message,
        V : ProtobufType,
        G : GetOptionImpl<M>,
        D : GetOrDefaultImpl<M>,
        S : SetImpl<M>,
{
    fn protobuf_type(&self) -> &'static ProtobufTypeDynamic {
        V::dynamic()
    }

    fn get_reflect<'a>(&self, m: &'a Message) -> Option<ReflectValueRef<'a>> {
        let m = message_down_cast(m);
        self.get_option_impl.get_reflect_impl(m)
    }

    fn get_singular_field_or_default<'a>(&self, m: &'a Message) -> ReflectValueRef<'a> {
        let m = message_down_cast(m);
        self.get_or_default_impl.get_singular_field_or_default_impl(m)
    }

    fn set_singular_field(&self, m: &mut Message, value: ReflectValueBox) {
        let m = message_down_cast_mut(m);
        self.set_impl.set_singular_field(m, value)
    }
}



struct GetOptionImplFieldPointer<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    get_field: for<'a> fn(&'a M) -> &'a <V::RuntimeType as RuntimeType>::Value,
}

struct GetOptionImplOptionFieldPointer<M, V, O>
    where
        M : Message,
        V : ProtobufType,
        O : OptionLike<<V::RuntimeType as RuntimeType>::Value> + Sync + Send + 'static,
{
    get_field: for<'a> fn(&'a M) -> &'a O,
    _marker: marker::PhantomData<V>,
}

struct GetOptionImplHasGetRef<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    has: for<'a> fn(&'a M) -> bool,
    get: for<'a> fn(&'a M) -> &'a <V::RuntimeType as RuntimeType>::Value,
}

struct GetOptionImplHasGetRefDeref<M, V>
    where
        M : Message,
        V : ProtobufType,
        V::RuntimeType : RuntimeTypeWithDeref,
{
    has: for<'a> fn(&'a M) -> bool,
    get: for<'a> fn(&'a M) -> &'a <V::RuntimeType as RuntimeTypeWithDeref>::DerefTarget,
}

struct GetOptionImplHasGetCopy<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    has: for<'a> fn(&'a M) -> bool,
    get: for<'a> fn(&'a M) -> <V::RuntimeType as RuntimeType>::Value,
}

impl<M, V> GetOptionImpl<M> for GetOptionImplFieldPointer<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        let v = V::RuntimeType::as_ref((self.get_field)(m));
        if v.is_non_zero() {
            Some(v)
        } else {
            None
        }
    }
}

impl<M, V, O> GetOptionImpl<M> for GetOptionImplOptionFieldPointer<M, V, O>
    where
        M : Message,
        V : ProtobufType,
        O : OptionLike<<V::RuntimeType as RuntimeType>::Value> + Sync + Send + 'static,
{
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        let m = message_down_cast(m);
        (self.get_field)(m)
            .as_option_ref()
            .map(V::RuntimeType::as_ref)
    }
}

impl<M, V> GetOptionImpl<M> for GetOptionImplHasGetRef<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        if (self.has)(m) {
            Some(<V::RuntimeType as RuntimeType>::as_ref((self.get)(m)))
        } else {
            None
        }
    }
}

impl<M, V> GetOptionImpl<M> for GetOptionImplHasGetRefDeref<M, V>
    where
        M : Message,
        V : ProtobufType,
        V::RuntimeType : RuntimeTypeWithDeref,
{
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        if (self.has)(m) {
            Some(<V::RuntimeType as RuntimeTypeWithDeref>::defef_as_ref((self.get)(m)))
        } else {
            None
        }
    }
}

impl<M, V> GetOptionImpl<M> for GetOptionImplHasGetCopy<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        if (self.has)(m) {
            Some(<V::RuntimeType as RuntimeType>::into_static_value_ref((self.get)(m)))
        } else {
            None
        }
    }
}

struct GetOrDefaultGetRef<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    get_field: for<'a> fn(&'a M) -> &'a <V::RuntimeType as RuntimeType>::Value,
}

struct GetOrDefaultGetRefDeref<M, V>
    where
        M : Message,
        V : ProtobufType,
        V::RuntimeType : RuntimeTypeWithDeref,
{
    get_field: for<'a> fn(&'a M) -> &'a <V::RuntimeType as RuntimeTypeWithDeref>::DerefTarget,
}

struct GetOrDefaultOptionRefTypeDefault<M, V, O>
    where
        M : Message,
        V : ProtobufType,
        O : OptionLike<<V::RuntimeType as RuntimeType>::Value> + Sync + Send + 'static,
{
    get_field: for<'a> fn(&'a M) -> &'a O,
    _marker: marker::PhantomData<V>,
}

struct GetOrDefaultGetCopy<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    get_field: for<'a> fn(&'a M) -> <V::RuntimeType as RuntimeType>::Value,
}

impl<M, V> GetOrDefaultImpl<M> for GetOrDefaultGetRef<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        V::RuntimeType::as_ref((self.get_field)(m))
    }
}

impl<M, V, O> GetOrDefaultImpl<M> for GetOrDefaultOptionRefTypeDefault<M, V, O>
    where
        M : Message,
        V : ProtobufType,
        O : OptionLike<<V::RuntimeType as RuntimeType>::Value> + Sync + Send + 'static,
{
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        match (self.get_field)(m).as_option_ref() {
            Some(v) => V::RuntimeType::as_ref(v),
            None => V::RuntimeType::default_value_ref(),
        }
    }
}

impl<M, V> GetOrDefaultImpl<M> for GetOrDefaultGetRefDeref<M, V>
    where
        M : Message,
        V : ProtobufType,
        V::RuntimeType : RuntimeTypeWithDeref,
{
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        <V::RuntimeType as RuntimeTypeWithDeref>::defef_as_ref((self.get_field)(m))
    }
}

impl<M, V> GetOrDefaultImpl<M> for GetOrDefaultGetCopy<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        <V::RuntimeType as RuntimeType>::into_static_value_ref((self.get_field)(m))
    }
}


struct SetImplFieldPointer<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    mut_field: for<'a> fn(&'a mut M) -> &'a mut <V::RuntimeType as RuntimeType>::Value,
}

struct SetImplOptionFieldPointer<M, V, O>
    where
        M : Message,
        V : ProtobufType,
        O : OptionLike<<V::RuntimeType as RuntimeType>::Value> + Sync + Send + 'static,
{
    mut_field: for<'a> fn(&'a mut M) -> &'a mut O,
    _marker: marker::PhantomData<V>,
}

struct SetImplSetField<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    set_field: for<'a> fn(&'a mut M, <V::RuntimeType as RuntimeType>::Value),
}

impl<M, V> SetImpl<M> for SetImplFieldPointer<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    fn set_singular_field(&self, m: &mut M, value: ReflectValueBox) {
        V::RuntimeType::set_from_value_box((self.mut_field)(m), value);
    }
}

impl<M, V, O> SetImpl<M> for SetImplOptionFieldPointer<M, V, O>
    where
        M : Message,
        V : ProtobufType,
        O : OptionLike<<V::RuntimeType as RuntimeType>::Value> + Sync + Send + 'static,
{
    fn set_singular_field(&self, m: &mut M, value: ReflectValueBox) {
        (self.mut_field)(m).set_value(V::RuntimeType::from_value_box(value));
    }
}

impl<M, V> SetImpl<M> for SetImplSetField<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    fn set_singular_field(&self, m: &mut M, value: ReflectValueBox) {
        let value = value.downcast::<<V::RuntimeType as RuntimeType>::Value>().expect("message");
        (self.set_field)(m, value)
    }
}




// TODO: make_singular_xxx_accessor are used only for oneof fields
// oneof codegen should be changed

pub fn make_singular_copy_has_get_set_accessor<M, V>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: fn(&M) -> <V::RuntimeType as RuntimeType>::Value,
    set: fn(&mut M, <V::RuntimeType as RuntimeType>::Value),
) -> FieldAccessor
    where
        M : Message + 'static,
        V : ProtobufType + 'static,
        <V::RuntimeType as RuntimeType>::Value : Copy,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(Box::new(SingularFieldAccessorImpl::<M, V, _, _, _> {
            get_option_impl: GetOptionImplHasGetCopy::<M, V> { has, get },
            get_or_default_impl: GetOrDefaultGetCopy::<M, V> { get_field: get },
            set_impl: SetImplSetField::<M, V> { set_field: set },
            _marker: marker::PhantomData,
        }))
    }
}

pub fn make_singular_string_has_get_set_accessor<M : Message + 'static>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: for<'a> fn(&'a M) -> &'a str,
    set: fn(&mut M, String),
) -> FieldAccessor {
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(Box::new(SingularFieldAccessorImpl::<M, ProtobufTypeString, _, _, _> {
            get_option_impl: GetOptionImplHasGetRefDeref::<M, ProtobufTypeString> { has, get },
            get_or_default_impl: GetOrDefaultGetRefDeref::<M, ProtobufTypeString> { get_field: get },
            set_impl: SetImplSetField::<M, ProtobufTypeString> { set_field: set },
            _marker: marker::PhantomData,
        }))
    }
}

pub fn make_singular_bytes_has_get_set_accessor<M : Message + 'static>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: for<'a> fn(&'a M) -> &'a [u8],
    set: fn(&mut M, Vec<u8>),
) -> FieldAccessor {
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(Box::new(SingularFieldAccessorImpl::<M, ProtobufTypeBytes, _, _, _> {
            get_option_impl: GetOptionImplHasGetRefDeref::<M, ProtobufTypeBytes> { has, get },
            get_or_default_impl: GetOrDefaultGetRefDeref::<M, ProtobufTypeBytes> { get_field: get },
            set_impl: SetImplSetField::<M, ProtobufTypeBytes> { set_field: set },
            _marker: marker::PhantomData,
        }))
    }
}

pub fn make_singular_message_has_get_mut_set_accessor<M, F>(
    name: &'static str,
    has_field: fn(&M) -> bool,
    get_field: for<'a> fn(&'a M) -> &'a F,
    _mut_field: for<'a> fn(&'a mut M) -> &'a mut F,
    set_field: fn(&mut M, F),
) -> FieldAccessor
    where M : Message + 'static, F : Message + Default + Clone + 'static
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(Box::new(SingularFieldAccessorImpl::<M, ProtobufTypeMessage<F>, _, _, _> {
            get_option_impl: GetOptionImplHasGetRef::<M, ProtobufTypeMessage<F>> { get: get_field, has: has_field },
            get_or_default_impl: GetOrDefaultGetRef::<M, ProtobufTypeMessage<F>> { get_field },
            set_impl: SetImplSetField::<M, ProtobufTypeMessage<F>> { set_field },
            _marker: marker::PhantomData,
        }))
    }
}

pub fn make_option_accessor<M, V, O>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a O,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut O,
) -> FieldAccessor
    where
        M : Message + 'static,
        V : ProtobufType + 'static,
        O : OptionLike<<V::RuntimeType as RuntimeType>::Value> + Send + Sync + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(Box::new(SingularFieldAccessorImpl::<M, V, _, _, _> {
            get_option_impl: GetOptionImplOptionFieldPointer::<M, V, _> { get_field, _marker: marker::PhantomData },
            get_or_default_impl: GetOrDefaultOptionRefTypeDefault::<M, V, _> { get_field, _marker: marker::PhantomData },
            set_impl: SetImplOptionFieldPointer::<M, V, _> { mut_field, _marker: marker::PhantomData },
            _marker: marker::PhantomData,
        }))
    }
}

pub fn make_option_get_copy_accessor<M, V, O>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a O,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut O,
    get_value: fn(&M) -> <V::RuntimeType as RuntimeType>::Value,
) -> FieldAccessor
    where
        M : Message + 'static,
        V : ProtobufType + 'static,
        O : OptionLike<<V::RuntimeType as RuntimeType>::Value> + Send + Sync + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(Box::new(SingularFieldAccessorImpl::<M, V, _, _, _> {
            get_option_impl: GetOptionImplOptionFieldPointer::<M, V, O> { get_field, _marker: marker::PhantomData },
            get_or_default_impl: GetOrDefaultGetCopy::<M, V> { get_field: get_value },
            set_impl: SetImplOptionFieldPointer::<M, V, O> { mut_field, _marker: marker::PhantomData },
            _marker: marker::PhantomData,
        }))
    }
}

pub fn make_option_get_ref_accessor<M, V, O>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a O,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut O,
    get_value: for<'a> fn(&'a M) -> &'a <V::RuntimeType as RuntimeTypeWithDeref>::DerefTarget,
) -> FieldAccessor
    where
        M : Message + 'static,
        V : ProtobufType + 'static,
        V::RuntimeType : RuntimeTypeWithDeref,
        O : OptionLike<<V::RuntimeType as RuntimeType>::Value> + Send + Sync + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(Box::new(SingularFieldAccessorImpl::<M, V, _, _, _> {
            get_option_impl: GetOptionImplOptionFieldPointer::<M, V, O> { get_field, _marker: marker::PhantomData },
            get_or_default_impl: GetOrDefaultGetRefDeref::<M, V> { get_field: get_value },
            set_impl: SetImplOptionFieldPointer::<M, V, O> { mut_field, _marker: marker::PhantomData },
            _marker: marker::PhantomData,
        }))
    }
}

pub fn make_simple_field_accessor<M, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a <V::RuntimeType as RuntimeType>::Value,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut <V::RuntimeType as RuntimeType>::Value,
) -> FieldAccessor
    where
        M : Message + 'static,
        V : ProtobufType + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Singular(Box::new(SingularFieldAccessorImpl::<M, V, _, _, _> {
            get_option_impl: GetOptionImplFieldPointer::<M, V> { get_field },
            get_or_default_impl: GetOrDefaultGetRef::<M, V> { get_field },
            set_impl: SetImplFieldPointer::<M, V> { mut_field },
            _marker: marker::PhantomData,
        }))
    }
}

use std::fmt;
use std::marker;

use crate::core::Message;
use crate::repeated::RepeatedField;

use crate::reflect::accessor::AccessorKind;
use crate::reflect::accessor::FieldAccessor;
use crate::reflect::repeated::ReflectRepeated;
use crate::reflect::repeated::ReflectRepeatedMut;
use crate::reflect::repeated::ReflectRepeatedRef;
use crate::reflect::runtime_types::RuntimeType;
use crate::reflect::type_dynamic::ProtobufTypeDynamic;
use crate::reflect::types::ProtobufType;
use crate::reflect::ProtobufValue;

pub(crate) trait RepeatedFieldAccessor: Send + Sync + 'static {
    fn get_reflect<'a>(&self, m: &'a dyn Message) -> ReflectRepeatedRef<'a>;
    fn mut_reflect<'a>(&self, m: &'a mut dyn Message) -> ReflectRepeatedMut<'a>;
}

pub(crate) struct RepeatedFieldAccessorHolder {
    pub accessor: Box<dyn RepeatedFieldAccessor>,
    pub element_type: &'static dyn ProtobufTypeDynamic,
}

trait RepeatedFieldGetMut<M, R: ?Sized>: Send + Sync + 'static
where
    M: Message + 'static,
{
    fn get_field<'a>(&self, message: &'a M) -> &'a R;
    fn mut_field<'a>(&self, message: &'a mut M) -> &'a mut R;
}

struct RepeatedFieldGetMutImpl<M, L>
where
    M: Message + 'static,
{
    get_field: for<'a> fn(&'a M) -> &'a L,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut L,
}

impl<M, V> RepeatedFieldGetMut<M, dyn ReflectRepeated> for RepeatedFieldGetMutImpl<M, Vec<V>>
where
    M: Message + 'static,
    V: ProtobufValue + fmt::Debug + 'static,
{
    fn get_field<'a>(&self, m: &'a M) -> &'a dyn ReflectRepeated {
        (self.get_field)(m) as &dyn ReflectRepeated
    }

    fn mut_field<'a>(&self, m: &'a mut M) -> &'a mut dyn ReflectRepeated {
        (self.mut_field)(m) as &mut dyn ReflectRepeated
    }
}

impl<M, V> RepeatedFieldGetMut<M, dyn ReflectRepeated>
    for RepeatedFieldGetMutImpl<M, RepeatedField<V>>
where
    M: Message + 'static,
    V: ProtobufValue + fmt::Debug + 'static,
{
    fn get_field<'a>(&self, m: &'a M) -> &'a dyn ReflectRepeated {
        (self.get_field)(m) as &dyn ReflectRepeated
    }

    fn mut_field<'a>(&self, m: &'a mut M) -> &'a mut dyn ReflectRepeated {
        (self.mut_field)(m) as &mut dyn ReflectRepeated
    }
}

struct RepeatedFieldAccessorImpl<M, V>
where
    M: Message,
    V: ProtobufType,
{
    fns: Box<dyn RepeatedFieldGetMut<M, dyn ReflectRepeated>>,
    _marker: marker::PhantomData<V>,
}

impl<M, V> RepeatedFieldAccessor for RepeatedFieldAccessorImpl<M, V>
where
    M: Message,
    V: ProtobufType,
{
    fn get_reflect<'a>(&self, m: &'a dyn Message) -> ReflectRepeatedRef<'a> {
        let m = m.downcast_ref().unwrap();
        let repeated = self.fns.get_field(m);
        ReflectRepeatedRef {
            repeated,
            dynamic: V::RuntimeType::dynamic(),
        }
    }

    fn mut_reflect<'a>(&self, m: &'a mut dyn Message) -> ReflectRepeatedMut<'a> {
        let m = m.downcast_mut().unwrap();
        let repeated = self.fns.mut_field(m);
        ReflectRepeatedMut {
            repeated,
            dynamic: V::RuntimeType::dynamic(),
        }
    }
}

/// Make accessor for `Vec` field
pub fn make_vec_accessor<M, V>(
    name: &'static str,
    get_vec: for<'a> fn(&'a M) -> &'a Vec<<V::RuntimeType as RuntimeType>::Value>,
    mut_vec: for<'a> fn(&'a mut M) -> &'a mut Vec<<V::RuntimeType as RuntimeType>::Value>,
) -> FieldAccessor
where
    M: Message + 'static,
    V: ProtobufType + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Repeated(RepeatedFieldAccessorHolder {
            accessor: Box::new(RepeatedFieldAccessorImpl::<M, V> {
                fns: Box::new(RepeatedFieldGetMutImpl::<
                    M,
                    Vec<<V::RuntimeType as RuntimeType>::Value>,
                > {
                    get_field: get_vec,
                    mut_field: mut_vec,
                }),
                _marker: marker::PhantomData::<V>,
            }),
            element_type: V::dynamic(),
        }),
    }
}

/// Make accessor for `RepeatedField`
pub fn make_repeated_field_accessor<M, V>(
    name: &'static str,
    get_vec: for<'a> fn(&'a M) -> &'a RepeatedField<<V::RuntimeType as RuntimeType>::Value>,
    mut_vec: for<'a> fn(&'a mut M) -> &'a mut RepeatedField<<V::RuntimeType as RuntimeType>::Value>,
) -> FieldAccessor
where
    M: Message + 'static,
    V: ProtobufType + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Repeated(RepeatedFieldAccessorHolder {
            accessor: Box::new(RepeatedFieldAccessorImpl::<M, V> {
                fns: Box::new(RepeatedFieldGetMutImpl::<
                    M,
                    RepeatedField<<V::RuntimeType as RuntimeType>::Value>,
                > {
                    get_field: get_vec,
                    mut_field: mut_vec,
                }),
                _marker: marker::PhantomData::<V>,
            }),
            element_type: V::dynamic(),
        }),
    }
}

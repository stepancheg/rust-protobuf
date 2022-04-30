use std::any::TypeId;
use std::mem;

use crate::reflect::ProtobufValue;

pub(crate) enum VecMutVariant<'a> {
    U32(&'a mut Vec<u32>),
    U64(&'a mut Vec<u64>),
    I32(&'a mut Vec<i32>),
    I64(&'a mut Vec<i64>),
    F32(&'a mut Vec<f32>),
    F64(&'a mut Vec<f64>),
    Bool(&'a mut Vec<bool>),
}

impl<'a> VecMutVariant<'a> {
    pub(crate) fn downcast<V: ProtobufValue>(vec: &mut Vec<V>) -> Option<VecMutVariant<'a>> {
        // SAFETY: we check type before transmuting.
        if TypeId::of::<Vec<V>>() == TypeId::of::<Vec<u32>>() {
            Some(VecMutVariant::U32(unsafe { mem::transmute(vec) }))
        } else if TypeId::of::<Vec<V>>() == TypeId::of::<Vec<u64>>() {
            Some(VecMutVariant::U64(unsafe { mem::transmute(vec) }))
        } else if TypeId::of::<Vec<V>>() == TypeId::of::<Vec<i32>>() {
            Some(VecMutVariant::I32(unsafe { mem::transmute(vec) }))
        } else if TypeId::of::<Vec<V>>() == TypeId::of::<Vec<i64>>() {
            Some(VecMutVariant::I64(unsafe { mem::transmute(vec) }))
        } else if TypeId::of::<Vec<V>>() == TypeId::of::<Vec<f32>>() {
            Some(VecMutVariant::F32(unsafe { mem::transmute(vec) }))
        } else if TypeId::of::<Vec<V>>() == TypeId::of::<Vec<f64>>() {
            Some(VecMutVariant::F64(unsafe { mem::transmute(vec) }))
        } else if TypeId::of::<Vec<V>>() == TypeId::of::<Vec<bool>>() {
            Some(VecMutVariant::Bool(unsafe { mem::transmute(vec) }))
        } else {
            None
        }
    }
}

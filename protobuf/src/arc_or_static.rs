use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug)]
pub enum ArcOrStatic<T: 'static> {
    Arc(Arc<T>),
    Static(&'static T),
}

impl<T> Deref for ArcOrStatic<T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            ArcOrStatic::Arc(t) => &**t,
            ArcOrStatic::Static(t) => t,
        }
    }
}

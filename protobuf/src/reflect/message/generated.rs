//! Generated messages reflection support.

use crate::message::Message;
use std::marker;

/// Sized to dynamic reflection operations.
pub(crate) trait MessageFactory: Send + Sync + 'static {
    fn new_instance(&self) -> Box<dyn Message>;
    fn default_instance(&self) -> &dyn Message;
    fn clone(&self, message: &dyn Message) -> Box<dyn Message>;
    fn eq(&self, a: &dyn Message, b: &dyn Message) -> bool;
}

/// The only message factory implementation.
pub(crate) struct MessageFactoryImpl<M>(pub marker::PhantomData<M>);

impl<M> MessageFactory for MessageFactoryImpl<M>
where
    M: 'static + Message + Default + Clone + PartialEq,
{
    fn new_instance(&self) -> Box<dyn Message> {
        let m: M = Default::default();
        Box::new(m)
    }

    fn default_instance(&self) -> &dyn Message {
        M::default_instance() as &dyn Message
    }

    fn clone(&self, message: &dyn Message) -> Box<dyn Message> {
        let m: &M = message.downcast_ref().expect("wrong message type");
        Box::new(m.clone())
    }

    fn eq(&self, a: &dyn Message, b: &dyn Message) -> bool {
        let a: &M = a.downcast_ref().expect("wrong message type");
        let b: &M = b.downcast_ref().expect("wrong message type");
        a == b
    }
}

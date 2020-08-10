use crate::reflect::dynamic::DynamicMessage;
use crate::reflect::reflect_eq::{ReflectEq, ReflectEqMode};
use crate::reflect::MessageDescriptor;
use crate::Message;
use std::ops::Deref;

#[derive(Clone, Debug)]
enum MessageRefImpl<'a> {
    Message(&'a dyn Message),
    EmptyDynamic(DynamicMessage),
}

/// Wrapper around either [`Message`] reference or a container for an empty dynamic message.
#[derive(Clone, Debug)]
pub struct MessageRef<'a> {
    imp: MessageRefImpl<'a>,
}

impl<'a> From<&'a dyn Message> for MessageRef<'a> {
    fn from(m: &'a dyn Message) -> Self {
        MessageRef {
            imp: MessageRefImpl::Message(m),
        }
    }
}

impl<'a, M: Message> From<&'a M> for MessageRef<'a> {
    fn from(m: &'a M) -> Self {
        MessageRef {
            imp: MessageRefImpl::Message(m),
        }
    }
}

impl<'a> ReflectEq for MessageRef<'a> {
    fn reflect_eq(&self, that: &Self, mode: &ReflectEqMode) -> bool {
        let ad = self.descriptor();
        let bd = that.descriptor();
        ad == bd && ad.reflect_eq(&**self, &**that, mode)
    }
}

impl<'a> MessageRef<'a> {
    /// Wrap a message.
    pub fn new(message: &'a dyn Message) -> MessageRef<'a> {
        MessageRef {
            imp: MessageRefImpl::Message(message),
        }
    }

    /// Default (empty) instance of given message type.
    pub fn default_instance(message: &MessageDescriptor) -> MessageRef<'static> {
        // Note we create a native generated instance for generated types
        // and dynamic message for dynamic types.
        match message.default_instance() {
            Some(m) => MessageRef::new(m),
            None => MessageRef {
                imp: MessageRefImpl::EmptyDynamic(DynamicMessage::new(message.clone())),
            },
        }
    }
}

impl<'a> Deref for MessageRef<'a> {
    type Target = dyn Message;

    fn deref(&self) -> &dyn Message {
        match &self.imp {
            MessageRefImpl::Message(m) => *m,
            MessageRefImpl::EmptyDynamic(e) => e,
        }
    }
}

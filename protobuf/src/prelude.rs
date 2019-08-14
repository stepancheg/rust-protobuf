//! Prelude, should be imported by default when protobuf is used.

use crate::core::Message;
use crate::singular::OptionLike;
use crate::singular::SingularPtrField;

/// Trait is implemented by types which hold fields of generated messages.
pub trait MessageField<M: Message + Default> {
    /// Get a message reference or default instance.
    fn get_message(&self) -> &M;
    /// Get a message reference, and initialize field with empty message
    /// if field is not yet initialized.
    fn mut_message(&mut self) -> &mut M;
    /// Set message to the field.
    fn set_message(&mut self, message: M);
    /// Set message field to default value.
    fn set_default(&mut self) -> &mut M;
}

impl<M: Message + Default> MessageField<M> for SingularPtrField<M> {
    fn get_message(&self) -> &M {
        match self.as_ref() {
            Some(m) => m,
            None => M::default_instance(),
        }
    }

    fn mut_message(&mut self) -> &mut M {
        if self.is_none() {
            self.set_value(Default::default());
        }

        self.as_mut().unwrap()
    }

    fn set_message(&mut self, message: M) {
        self.set_value(message);
    }

    fn set_default(&mut self) -> &mut M {
        if self.is_none() {
            self.set_message(Default::default());
        } else {
            self.as_mut().unwrap().clear();
        }

        self.mut_message()
    }
}

impl<M: Message + Default> MessageField<M> for Option<Box<M>> {
    fn get_message(&self) -> &M {
        match self {
            Some(m) => m,
            None => M::default_instance(),
        }
    }

    fn mut_message(&mut self) -> &mut M {
        if self.is_none() {
            *self = Some(Box::new(Default::default()));
        }

        self.as_mut().unwrap()
    }

    fn set_message(&mut self, message: M) {
        if self.is_some() {
            **self.as_mut().unwrap() = message;
        } else {
            *self = Some(Box::new(message));
        }
    }

    fn set_default(&mut self) -> &mut M {
        if self.is_some() {
            let p = &mut **self.as_mut().unwrap();
            Message::clear(p);
            p
        } else {
            *self = Some(Box::new(Default::default()));
            self.as_mut().unwrap()
        }
    }
}

impl<M: Message + Default> MessageField<M> for Option<M> {
    fn get_message(&self) -> &M {
        match self {
            Some(m) => m,
            None => Message::default_instance(),
        }
    }

    fn mut_message(&mut self) -> &mut M {
        if self.is_none() {
            *self = Some(Default::default());
        }

        self.as_mut().unwrap()
    }

    fn set_message(&mut self, message: M) {
        *self = Some(message);
    }

    fn set_default(&mut self) -> &mut M {
        if self.is_some() {
            let p = self.as_mut().unwrap();
            p.clear();
            p
        } else {
            *self = Some(Default::default());
            self.as_mut().unwrap()
        }
    }
}

#![doc(hidden)]

use std::fmt;
use std::mem;
use std::ops::Deref;

/// Identifier in `.proto` file
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
#[doc(hidden)]
pub struct ProtobufIdent(String);

#[derive(Eq, PartialEq, Debug, Hash)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ProtobufIdentRef(str);

impl Deref for ProtobufIdentRef {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl Deref for ProtobufIdent {
    type Target = ProtobufIdentRef;

    fn deref(&self) -> &ProtobufIdentRef {
        ProtobufIdentRef::new(&self.0)
    }
}

impl From<&'_ str> for ProtobufIdent {
    fn from(s: &str) -> Self {
        ProtobufIdent::new(s)
    }
}

impl From<String> for ProtobufIdent {
    fn from(s: String) -> Self {
        ProtobufIdent::new(&s)
    }
}

impl Into<String> for ProtobufIdent {
    fn into(self) -> String {
        self.0
    }
}

impl fmt::Display for ProtobufIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.get(), f)
    }
}

impl ProtobufIdentRef {
    pub fn new<'a>(ident: &'a str) -> &'a ProtobufIdentRef {
        assert!(!ident.is_empty());
        // SAFETY: ProtobufIdentRef is repr(transparent)
        unsafe { mem::transmute(ident) }
    }

    pub fn as_str(&self) -> &str {
        &*self
    }

    pub fn to_owned(&self) -> ProtobufIdent {
        ProtobufIdent(self.0.to_owned())
    }
}

impl ProtobufIdent {
    pub fn as_ref(&self) -> &ProtobufIdentRef {
        ProtobufIdentRef::new(&self.0)
    }

    pub fn new(s: &str) -> ProtobufIdent {
        assert!(!s.is_empty());
        assert!(!s.contains("/"));
        assert!(!s.contains("."));
        assert!(!s.contains(":"));
        assert!(!s.contains("("));
        assert!(!s.contains(")"));
        ProtobufIdent(s.to_owned())
    }

    pub fn get(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

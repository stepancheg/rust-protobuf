use std::fmt;
use std::iter;


/// Valid Rust identifier
#[derive(Eq, PartialEq, Debug, Clone)]
pub(crate) struct RustIdent(String);

impl RustIdent {
    pub fn new(s: &str) -> RustIdent {
        assert!(!s.is_empty());
        assert!(!s.contains("/"), "{}", s);
        assert!(!s.contains("."), "{}", s);
        assert!(!s.contains(":"), "{}", s);
        RustIdent(s.to_owned())
    }
    
    pub fn super_ident() -> RustIdent {
        RustIdent::new("super")
    }

    pub fn get(&self) -> &str {
        &self.0
    }

    pub fn to_path(&self) -> RustIdentWithPath {
        RustIdentWithPath::new(&self.0)
    }
}

impl fmt::Display for RustIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.get(), f)
    }
}

impl From<&'_ str> for RustIdent {
    fn from(s: &str) -> Self {
        RustIdent::new(s)
    }
}

impl From<String> for RustIdent {
    fn from(s: String) -> Self {
        RustIdent::new(&s)
    }
}

#[derive(Default, Eq, PartialEq, Debug, Clone)]
pub(crate) struct RustPath {
    absolute: bool,
    path: Vec<RustIdent>,
}

impl RustPath {
    pub fn is_absolute(&self) -> bool {
        self.absolute
    }

    pub fn is_empty(&self) -> bool {
        assert!(!self.absolute);
        self.path.is_empty()
    }

    pub fn relative_from_components<I : IntoIterator<Item = RustIdent>>(i: I) -> RustPath {
        RustPath {
            absolute: false,
            path: i.into_iter().collect(),
        }
    }

    pub fn with_ident(self, ident: RustIdent) -> RustIdentWithPath {
        RustIdentWithPath {
            path: self,
            ident,
        }
    }

    pub fn first(&self) -> Option<RustIdent> {
        assert!(!self.absolute);
        self.path.iter().cloned().next()
    }

    pub fn remove_first(&mut self) -> Option<RustIdent> {
        assert!(!self.absolute);
        if self.path.is_empty() {
            None
        } else {
            Some(self.path.remove(0))
        }
    }

    pub fn to_reverse(&self) -> RustPath {
        assert!(!self.absolute);
        RustPath::relative_from_components(iter::repeat(RustIdent::super_ident()).take(self.path.len()))
    }

    pub fn prepend_ident(&mut self, ident: RustIdent) {
        assert!(!self.absolute);
        self.path.insert(0, ident);
    }

    pub fn append(mut self, path: RustPath) -> RustPath {
        assert!(!path.absolute);
        for c in path.path {
            self.path.push(c);
        }
        self
    }

    pub fn append_ident(mut self, ident: RustIdent) -> RustPath {
        self.path.push(ident);
        self
    }

    pub fn append_with_ident(self, path: RustIdentWithPath) -> RustIdentWithPath {
        self
            .append(path.path)
            .with_ident(path.ident)
    }
}

impl From<&'_ str> for RustPath {
    fn from(s: &str) -> Self {
        let (s, absolute) = if s.starts_with("::") {
            (&s[2..], true)
        } else {
            (s, false)
        };
        let path = s.split("::").map(RustIdent::from).collect();
        RustPath {
            absolute,
            path,
        }
    }
}

impl fmt::Display for RustPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.absolute {
            write!(f, "::")?;
        }
        for (i, c) in self.path.iter().enumerate() {
            if i != 0 {
                write!(f, "::")?;
            }
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub(crate) struct RustIdentWithPath {
    pub path: RustPath,
    pub ident: RustIdent,
}

impl RustIdentWithPath {
    pub fn new(s: &str) -> RustIdentWithPath {
        let mut path = RustPath::from(s);
        let ident = path.path.pop().unwrap();
        RustIdentWithPath {
            path,
            ident,
        }
    }

    pub fn prepend_ident(&mut self, ident: RustIdent) {
        self.path.prepend_ident(ident)
    }

    pub fn to_path(&self) -> RustPath {
        self.path.clone().append_ident(self.ident.clone())
    }
}

impl From<&'_ str> for RustIdentWithPath {
    fn from(s: &str) -> Self {
        RustIdentWithPath::new(s)
    }
}

impl From<String> for RustIdentWithPath {
    fn from(s: String) -> Self {
        RustIdentWithPath::new(&s)
    }
}

impl fmt::Display for RustIdentWithPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.to_path(), f)
    }
}

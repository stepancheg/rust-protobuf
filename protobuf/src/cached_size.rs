#[cfg(feature = "with-serde")]
use serde::ser::{Serialize, Serializer, SerializeStruct};
#[cfg(feature = "with-serde")]
use serde::{Deserializer, Deserialize};
#[cfg(feature = "with-serde")]
use serde::de;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::hash::Hash;
use std::hash::Hasher;
#[cfg(feature = "with-serde")]
use std::fmt;

/// Cached size field used in generated code.
/// It is always equal to itself to simplify generated code.
/// (Generated code can use `#[derive(Eq)]`).
#[derive(Debug, Default)]
pub struct CachedSize {
    size: AtomicUsize,
}

impl CachedSize {
    pub fn get(&self) -> u32 {
        self.size.load(Ordering::Relaxed) as u32
    }

    pub fn set(&self, size: u32) {
        self.size.store(size as usize, Ordering::Relaxed)
    }
}

#[cfg(feature = "with-serde")]
impl Serialize for CachedSize {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("CachedSize", 1)?;
        s.serialize_field("size", &self.size.load(Ordering::Relaxed))?;
        s.end()
    }
}

#[cfg(feature = "with-serde")]
impl<'de> Deserialize<'de> for CachedSize {
    fn deserialize<D>(deserializer: D) -> Result<CachedSize, D::Error>
    where
        D: Deserializer<'de>,
    {

        struct CachedSizeVisitor;

        impl<'de> de::Visitor<'de> for CachedSizeVisitor
        {
            type Value = CachedSize;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`size`")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CachedSize, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                if let Some("size") = map.next_key()? {
                    return Ok(CachedSize { size: AtomicUsize::new(map.next_value()?) })
                }
                return Err(de::Error::missing_field("size not found"));
            }
        }

        deserializer.deserialize_u64(CachedSizeVisitor)
    }
}

impl Clone for CachedSize {
    fn clone(&self) -> CachedSize {
        CachedSize {
            size: AtomicUsize::new(self.size.load(Ordering::Relaxed))
        }
    }
}

impl PartialEq<CachedSize> for CachedSize {
    fn eq(&self, _other: &CachedSize) -> bool {
        true
    }
}

impl Eq for CachedSize {}

impl Hash for CachedSize {
    fn hash<H : Hasher>(&self, _state: &mut H) {
        // ignore cached size in cache computation
    }
}

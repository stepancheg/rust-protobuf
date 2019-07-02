use std::{mem, time::Duration};

use crate::well_known_types;

/// Slice from `vec[vec.len()..vec.capacity()]`
pub unsafe fn remaining_capacity_as_slice_mut<A>(vec: &mut Vec<A>) -> &mut [A] {
    let range = vec.len()..vec.capacity();
    vec.get_unchecked_mut(range)
}

pub unsafe fn remove_lifetime_mut<A: ?Sized>(a: &mut A) -> &'static mut A {
    mem::transmute(a)
}

macro_rules! map_rust_type_to_protobuf {
    ($rust:path, $protobuf:path) => {
        impl From<$protobuf> for $rust {
            fn from(inner: $protobuf) -> Self {
                inner.value
            }
        }

        impl From<$rust> for $protobuf {
            fn from(inner: $rust) -> Self {
                let mut value = Self::new();
                value.value = inner;
                value
            }
        }
    };
}

// Map some of well known types to Rust equivalents.

map_rust_type_to_protobuf! { bool, well_known_types::BoolValue }
map_rust_type_to_protobuf! { Vec<u8>, well_known_types::BytesValue }
map_rust_type_to_protobuf! { f64, well_known_types::DoubleValue }
map_rust_type_to_protobuf! { f32, well_known_types::FloatValue }
map_rust_type_to_protobuf! { i32, well_known_types::Int32Value }
map_rust_type_to_protobuf! { i64, well_known_types::Int64Value }
map_rust_type_to_protobuf! { u32, well_known_types::UInt32Value }
map_rust_type_to_protobuf! { u64, well_known_types::UInt64Value }
map_rust_type_to_protobuf! { String, well_known_types::StringValue }

impl From<well_known_types::Duration> for Duration {
    fn from(inner: well_known_types::Duration) -> Self {
        Duration::new(inner.seconds as u64, inner.nanos as u32)
    }
}

impl From<Duration> for well_known_types::Duration {
    fn from(inner: Duration) -> Self {
        let mut value = well_known_types::Duration::new();
        value.seconds = inner.as_secs() as i64;
        value.nanos = inner.subsec_nanos() as i32;
        value
    }
}

impl From<well_known_types::Empty> for () {
    fn from(_inner: well_known_types::Empty) -> Self {
        ()
    }
}

impl From<()> for well_known_types::Empty {
    fn from(_inner: ()) -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remaining_capacity_as_slice_mut() {
        let mut v = Vec::with_capacity(5);
        v.push(10);
        v.push(11);
        v.push(12);
        unsafe {
            {
                let s = remaining_capacity_as_slice_mut(&mut v);
                assert_eq!(2, s.len());
                s[0] = 13;
                s[1] = 14;
            }
            v.set_len(5);
        }
        assert_eq!(vec![10, 11, 12, 13, 14], v);
    }

    #[test]
    fn test_duration_protobuf_to_rust_mapping() {
        let mut proto_duration = well_known_types::Duration::new();
        proto_duration.seconds = 1_000;
        proto_duration.nanos = 100;

        let rust_duration = Duration::from(proto_duration);
        assert_eq!(rust_duration, Duration::new(1_000, 100));
    }

    #[test]
    fn test_duration_rust_to_protobuf_mapping() {
        let rust_duration = Duration::new(1_000, 100);

        let proto_duration = well_known_types::Duration::from(rust_duration);
        assert_eq!(proto_duration.seconds, 1_000);
        assert_eq!(proto_duration.nanos, 100);
    }    
}

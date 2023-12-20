cfg_if::cfg_if! {
    if #[cfg(feature = "btreemaps")] {
        /// Rust type to map protobuf maps into.
        pub type Map<K, V> = std::collections::BTreeMap<K, V>;

        /// Key constraints for `Map`. BTreeMap keys must implement `Ord`
        pub trait KeyConstraint: Ord {}
        impl<T: Ord> KeyConstraint for T {}

        /// Iterator type of `Map`
        pub(crate) type Iter<'a, K, V> = std::collections::btree_map::Iter<'a, K, V>;
    } else {
        /// Rust type to map protobuf maps into.
        pub type Map<K, V> = std::collections::HashMap<K, V>;

        /// Key constraints for `Map`. HashMap keys must implement `std::hash::Hash`
        pub trait KeyConstraint: std::hash::Hash {}
        impl<T: std::hash::Hash> KeyConstraint for T {}

        /// Iterator type of `Map`
        pub(crate) type Iter<'a, K, V> = std::collections::hash_map::Iter<'a, K, V>;
    }
}

//! Generated files are imported from here.
//!
//! For the demonstration we generate descriptors twice, with
//! as pure rust codegen, and with codegen dependent on `protoc` binary.

pub mod generated_with_pure {
    include!(concat!(env!("OUT_DIR"), "/generated_with_pure/mod.rs"));
}

pub mod generated_with_native {
    include!(concat!(env!("OUT_DIR"), "/generated_with_native/mod.rs"));
}

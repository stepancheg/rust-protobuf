#[crate_id(name = "protobuf#0.1.0")];
#[crate_type = "lib"];
#[feature(globs)];
#[feature(managed_boxes)];
#[allow(dead_code)];

#[desc = "protobuf implementation for rust"];
#[license = "BSD"];

pub use core::*;

mod core;
pub mod descriptor;
pub mod codegen;
pub mod rt;
mod misc;
mod zigzag;
mod hex;

#[cfg(test)]
mod shrug;
#[cfg(test)]
mod test;

// so `use protobuf::*` could work in descriptor mod
pub mod protobuf {
    pub use descriptor;
    pub use codegen;
    pub use core::*;
    pub use rt;
}

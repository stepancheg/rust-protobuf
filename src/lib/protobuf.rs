#[link(name = "protobuf", vers = "0.1.0")];
#[crate_type = "lib"];

#[desc = "protobuf implementation for rust"];
#[license = "BSD"];
#[author = "Stepan Koltsov"];

pub use core::*;

pub mod core;
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
mod protobuf {
    pub use core::*;
    pub use rt;
}

/// Which parse to use to parse `.proto` files.
pub enum WhichParser {
    /// Pure Rust parser implemented by this crate.
    Pure,
    /// Parse `.proto` files using `protoc --descriptor_set_out=...` command.
    Protoc,
}

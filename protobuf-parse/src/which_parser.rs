/// Which parse to use to parse `.proto` files.
#[derive(Debug, Copy, Clone)]
pub(crate) enum WhichParser {
    /// Pure Rust parser implemented by this crate.
    Pure,
    /// Parse `.proto` files using `protoc --descriptor_set_out=...` command.
    Protoc,
}

impl Default for WhichParser {
    fn default() -> Self {
        WhichParser::Pure
    }
}

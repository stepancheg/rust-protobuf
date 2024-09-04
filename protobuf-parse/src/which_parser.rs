/// Which parse to use to parse `.proto` files.
#[derive(Debug, Copy, Clone)]
#[derive(Default)]
pub(crate) enum WhichParser {
    /// Pure Rust parser implemented by this crate.
    #[default]
    Pure,
    /// Parse `.proto` files using `protoc --descriptor_set_out=...` command.
    Protoc,
}


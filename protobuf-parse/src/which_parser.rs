/// Which parse to use (when possible).
pub enum WhichParser {
    /// Pure Rust parser.
    Pure,
    /// Parse using external `protoc` command.
    Protoc,
}

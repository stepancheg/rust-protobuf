# rust-protobuf examples

This directory contains examples for rust-protobuf version 3.

Rust-protobuf 3 is currently alpha version, and will be released
in a few weeks.

Rust-protobuf version 2 has similar API, but examples won't work
as is with version 2.

## Crate references

In the examples, protobuf crates are referenced by relative paths:

```toml
protobuf = { path = "../../protobuf" }
```

in your project versions should be used instead like:

```toml
protobuf = "3.0.0-alpha.8"
```

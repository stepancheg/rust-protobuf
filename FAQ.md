# Frequently Asked Questions

## Encoding and Decoding

Asked in [stepancheg::rust-protobuf#270](https://github.com/stepancheg/rust-protobuf/issues/270)

### How do I convert a message to bytes?

Each protobuf message has a `val [result] = [message].write_to_bytes()` which returns either `ProtobufResult<Vec<u8>>` or `ProtobufResult<bytes:Bytes>` (if you enable the `bytes` crate feature).

Normal error handling then applies, i.e. `[result].unwrap()` or 

```rust
match [result] {
  Err(e) => { println!("I've got an error {:?}", e); return; }
  Ok(bytes) => { ... }
}
```

### How do I convert bytes to a message?

There are two options:

1. Create a Message

Create a protobuf message, which you can later convert to the type that you require.

```rust
let bytes: Vec<u8> = myBytes;

// create a protobuf::Message
let message: protobuf::Message = protobuf::parse_from_bytes().unwrap();

// TODO cleanly convert message to myproto::MyMessage

```

2. You already have a `mut [message]`

If you already have a mutable message, you can merge bytes into the message.

```rust
let mut my_message = myproto::MyMessage::new();
let bytes: Vec<u8> = myBytes;

// merge bytes into message
my_message.merge_from_bytes(bytes.as_ref()); // don't need `as_ref` if you already have &[u8]
```
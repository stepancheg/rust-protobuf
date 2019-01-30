# Frequently asked questions

## Questions about Protocol Buffers

Protocol Buffers is designed and maintained by Google.

Google has very thorough documentation including [FAQ](https://developers.google.com/protocol-buffers/docs/faq).

If your question is not rust-protobuf-specific, it's likely you can find your answer there.

## How to serialize a message to bytes?

The easiest way to do it is to invoke `write_to_bytes` function:

```
message.write_to_bytes()
```

## How to deserialize a message from a byte array?

```
let my_message: MyMessage = protobuf::parse_from_bytes(bytes).unwrap()
```

## What is `cached_size` field?

Before serializing protobuf need to know sizes of all nested structures.

So serialization of protobuf message is done in two steps:
* first, compute sizes of all nested messages
* second, write the message tree with each message prepended by serialized size

So before serialization protobuf implementation stores size of a message into `cached_size` field.

(The same strategy used at least in Google's C++ and Java implementation of Protocol Buffers)

Sometimes it's annoying. Previously rust-protobuf stored message sizes in external buffer,
but it was slightly worse performance-wise.

rust-protobuf could have an option to skip generation of `cached_size`
and use external buffer when there's no such field. If you need that could be useful to you,
please open an issue.

## What is `unknown_fields` field?

When unknown field is encountered during parsing (e. g. field added in newer version of `.proto` file)
this field data is stored in `unknown_fields`.
The purpose of this field is to be able to perform loseless read-modify-write operation
even with older version of `proto` file which does not know about new fields.

## You answer is not there?

Feel free to open an issue with a question.

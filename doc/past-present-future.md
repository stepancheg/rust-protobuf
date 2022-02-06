# Past, present and future of rust-protobuf

## Past

I started rust-protobuf 7 years ago (time flies). I wanted to implement something and found that there's no protobuf implementation in Rust.

So I decided to implement it. Implementation took quite a lot of time, and I didn't implement that other project I wanted rust-protobuf to be used for.

That was fun time. Rust was very far from being stable, breaking changes occured weekly, and patching rust-protobuf was not trivial because rust-protobuf itself contains generated code, and fixing for rust updates required patching both library, code generator **and generated code** inside protobuf. I usually used `sed` to patch generated code.

There are still references to these old times. For example, `Debug` trait was called `Show` in early rust, and this name is still [mentioned in code generator](https://github.com/stepancheg/rust-protobuf/blob/188596b3d78b381d49a753cdb3ecb5fbc9382b0d/protobuf-codegen/src/gen/message.rs#L535).

Many API features of rust-protobuf which seem to be strange, can be explained by historical reasons. For example, in rust-protobuf version 2 `Debug` implementation of messages outputs protobuf text format, not standard rust debugging output.

Another example, in the old times `cargo` didn't have `build.rs` files, so files needed to be generated manually and probably committed to the repository, and code generator was optimized for storing generated files in source tree.

## Present

There are two version of rust-protobuf now: 2 and 3-alpha.

Version 2 is stable, it provides good backwards compatibility.

Version 3-alpha contains multiple improvements (mostly fixing past mistakes). It implements:
* proper runtime reflection
* dynamic messages
* json and text format printing and parsing
* better mapping of proto messages to rust module namespaces
* various API cleanups

Version 3 of rust-protobuf is mostly done. It requires about 20 hours of work to release it.

Version 3 is backward incompatible with version 2. It is not possible to implement certain feartures of rust-protobuf 3 in version 2 without breaking backwards compatibility.

### Prost

Some time ago people started prost project, which is an alternative implementation or protobuf. It has some advantages and some drawbacks.

Here I tried to do [the comparison](https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-examples/vs-prost). I see some things rust-protobuf does better. And I learned a few tricks from prost, and implemented them in rust-protobuf.

Long time ago there were some pull requests to rust-protobuf which I had to reject for reasons like they broke backwards compatibility, or changed API to similar API.

Perhaps that was the reason why prost project started. I don't know. We didn't discuss it.

## Future

The future is foggy. Technically, I could release rust-protobuf 3. But:
* it would be moral obligation to maintain it, and I have full time job unrelated to rust-protobuf
* having more than one protobuf implementation is not necessarily good

And if prost is considered superior to rust-protobuf, maybe it would be better to keep maintaining rust-protobuf at version 2 and never release version 3. And recommend using prost instead.

Advice (in reddit comment, or in [email](mailto:stepan.koltsov@gmail.com)) would be appreciated.

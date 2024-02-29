fn main() {
    protobuf_codegen::Codegen::new()
        .includes(["../../google-protobuf-all-protos/protobuf/protobuf-git/conformance", "../../google-protobuf-all-protos/protobuf/protobuf-git/src/google/protobuf"])
        .inputs(["../../google-protobuf-all-protos/protobuf/protobuf-git/conformance/conformance.proto", "../../google-protobuf-all-protos/protobuf/protobuf-git/src/google/protobuf/test_messages_proto2.proto", "../../google-protobuf-all-protos/protobuf/protobuf-git/src/google/protobuf/test_messages_proto3.proto"])
        .cargo_out_dir("protos")
        .run_from_script();
}

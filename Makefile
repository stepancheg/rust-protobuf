export PATH := target:$(PATH)

.PHONY:
all:

.PHONY:
test-protoc:
	protoc --rust_out src/lib src/proto/shrug.proto
	protoc --rust_out src/lib src/proto/test_root.proto
	protoc --rust_out src/lib src/proto/test_nonunique_enum.proto
	protoc --rust_out src/lib src/proto/text_format_test_data.proto

.PHONY:
regenerate:
	protoc --rust_out src -Isrc/proto src/proto/google/protobuf/*.proto
	mv src/descriptor.rs src/lib/


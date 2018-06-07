all:
	protobuf/regenerate_if_proto3.sh
	cp protobuf-dev/src/{lib.rs,enums.rs,rt.rs,stream.rs} protobuf/src/
	cp protobuf-dev/src/reflect/* protobuf/src/reflect/ -a
	cargo build

clean:
	rm -rf protobuf
	git checkout HEAD protobuf

all:
	protobuf/regenerate_if_proto3.sh
	rm protobuf -rf
	cp protobuf-dev protobuf -a
	cargo build

clean:
	rm -rf protobuf
	git checkout HEAD protobuf

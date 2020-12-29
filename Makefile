gen_go_proto:
	protoc -I proto/ proto/*.proto --go_out=plugins=grpc:proto/.


build_linux:
	cargo build --release
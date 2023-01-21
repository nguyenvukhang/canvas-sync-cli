include ../../.env

BIN=./build/canvas_sync

build:
	@mkdir -p build
	@cd build && cmake .. && cmake --build .
	@mv ./build/compile_commands.json .
	@make run

# to include this, run `make build_rust` before the `cmake --build` command
build_rust:
	@cd embed && cargo build --release
	@rm -f ./build/libembed*
	@cp ./embed/target/release/libembed* ./build

run:
	@CANVAS_TOKEN=$(CANVAS_TOKEN) BIN=$(BIN) sh ./scripts/debug

.PHONY: build

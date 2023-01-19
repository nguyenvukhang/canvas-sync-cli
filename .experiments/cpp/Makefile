build:
	@make debug
	mkdir -p build
	cd build && cmake .. && cmake --build .
	@make build_rust
	@make run

build_rust:
	cd embed && cargo build --release
	rm -f ./build/libembed*
	@cp ./embed/target/release/libembed* ./build

run:
	@echo '~~~~~~~~~~~~~~~~~~~~~~~~~~ RUN CANVAS SYNC'
	@./build/canvas_sync

debug:
	@echo '~~~~~~~~~~~~~~~~~~~~~~~~~~ DEBUG'
	@ls build/lib*

.PHONY: build

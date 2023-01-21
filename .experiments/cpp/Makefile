MAKEFILE_PATH := $(abspath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR  := $(dir $(MAKEFILE_PATH))

CMAKE_PRG ?= $(shell (command -v cmake3 || echo cmake))
CMAKE_BUILD_TYPE ?= Release # as opposed to Debug
CMAKE_FLAGS := -DCMAKE_BUILD_TYPE=$(CMAKE_BUILD_TYPE)
CANVAS_SYNC_PRG := $(MAKEFILE_DIR)/build/canvas_sync

include ../../.env

BIN=./build/canvas_sync

develop:
	@make --no-print-directory build
	@make run_prg
	# @make debug_prg
	@make save_config

build:
	@mkdir -p $(MAKEFILE_DIR)/build # create build directory
	@cmake \
		-B $(MAKEFILE_DIR)/build -S $(MAKEFILE_DIR) \
		$(CMAKE_FLAGS)
	@cmake --build $(MAKEFILE_DIR)/build # run the build

# save the build command for LSP use
save_config:
	@mv $(MAKEFILE_DIR)/build/compile_commands.json .

# to include this, run `make build_rust` before the `cmake --build` command
build_rust:
	@cd embed && cargo build --release
	@rm -f ./build/libembed*
	@cp ./embed/target/release/libembed* ./build

run_prg:
	@CANVAS_TOKEN=$(CANVAS_TOKEN) BIN=$(BIN) sh ./scripts/debug

debug_prg:
	@CANVAS_TOKEN=$(CANVAS_TOKEN) lldb $(BIN)

.PHONY: build build_rust build_release build_debug

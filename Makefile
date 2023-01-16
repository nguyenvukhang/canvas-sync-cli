include .env
BIN=./target/debug/canvas-sync

quick:
	cargo build || exit 1
	@echo "~~~~~~~~~~~~~~ BREAK TOKEN ~~~~~~~~~~~~~"
	@ RUST_LOG=canvas_sync=debug $(BIN) set-token hello
	@echo "~~~~~~~~~~~~~~ NO COMMAND ~~~~~~~~~~~~~~"
	@ RUST_LOG=canvas_sync=debug $(BIN)
	@echo "~~~~~~~~~~~~~~ FIX TOKEN ~~~~~~~~~~~~~~~"
	@ RUST_LOG=canvas_sync=debug $(BIN) set-token $(CANVAS_TOKEN)
	@echo "~~~~~~~~~~~~~~ CONFIG ~~~~~~~~~~~~~~~~~~"
	@ RUST_LOG=canvas_sync=debug $(BIN) config
	@echo "~~~~~~~~~~~~~~ FETCH ~~~~~~~~~~~~~~~~~~~"
	@ RUST_LOG=canvas_sync=debug $(BIN) fetch
	@echo "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"

include .env
BIN=./target/debug/canvas-sync

main:
	cargo build || exit 1
	@echo "~~~~~~~~~~~~~~ FETCH ~~~~~~~~~~~~~~~~~~~"
	@ RUST_LOG=canvas_sync=debug $(BIN) fetch
	@echo "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"

null:
	cargo build || exit 1
	@ $(BIN)

quiet:
	cargo build || exit 1
	@echo "~~~~~~~~~~~~~~ FETCH ~~~~~~~~~~~~~~~~~~~"
	@ $(BIN) fetch
	@echo "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"

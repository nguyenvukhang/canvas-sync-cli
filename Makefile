include .env
BIN_DIR:=$(HOME)/dots/personal/.local/bin
BIN:=canvas-sync


main:
	cargo build || exit 1
	@echo "~~~~~~~~~~~~~~ FETCH ~~~~~~~~~~~~~~~~~~~"
	@ RUST_LOG=canvas_sync=debug ./target/debug/$(BIN) fetch
	@echo "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"

quiet:
	cargo build || exit 1
	@echo "~~~~~~~~~~~~~~ FETCH ~~~~~~~~~~~~~~~~~~~"
	@ ./target/debug/$(BIN) fetch
	@echo "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"

load_bin:
	@rm -f $(BIN_DIR)/$(BIN)
	@cp ./target/release/$(BIN) $(BIN_DIR)

all:
	cargo build --release
	make load_bin

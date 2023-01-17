include .env
BIN=./target/debug/canvas-sync

	# @echo "~~~~~~~~~~~~~~ BREAK TOKEN ~~~~~~~~~~~~~"
	# @ RUST_LOG=canvas_sync=debug $(BIN) set-token hello
	# @echo "~~~~~~~~~~~~~~ NO COMMAND ~~~~~~~~~~~~~~"
	# @ RUST_LOG=canvas_sync=debug $(BIN)
	# @echo "~~~~~~~~~~~~~~ FIX TOKEN ~~~~~~~~~~~~~~~"
	# @ RUST_LOG=canvas_sync=debug $(BIN) set-token $(CANVAS_TOKEN)
	# @echo "~~~~~~~~~~~~~~ NO COMMAND ~~~~~~~~~~~~~~"
	# @ RUST_LOG=canvas_sync=debug $(BIN)
	# @echo "~~~~~~~~~~~~~~ CONFIG ~~~~~~~~~~~~~~~~~~"
	# @ RUST_LOG=canvas_sync=debug $(BIN) config
	
verbose:
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

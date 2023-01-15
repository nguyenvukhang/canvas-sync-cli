include .env

quick:
	cargo build || exit 1
	@echo "~~~~~~~~~~~~~~~~~~~~~~~~~"
	@CANVAS_TOKEN=$(CANVAS_TOKEN) \
		RUST_LOG=canvas_sync=debug \
		./target/debug/canvas-sync
	@echo "~~~~~~~~~~~~~~~~~~~~~~~~~"

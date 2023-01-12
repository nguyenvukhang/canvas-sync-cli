include .env

quick:
	cargo build || exit 1
	@echo "~~~~~~~~~~~~~~~~~~~~~~~~~"
	@CANVAS_TOKEN=$(CANVAS_TOKEN) ./target/debug/canvas-sync
	@echo "~~~~~~~~~~~~~~~~~~~~~~~~~"

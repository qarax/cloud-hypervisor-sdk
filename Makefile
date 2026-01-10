.PHONY: help build test fmt clippy clean regenerate doc setup-examples run-example

help:
	@echo "Cloud Hypervisor SDK - Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  make build          - Build the library"
	@echo "  make test           - Run tests"
	@echo "  make fmt            - Format code"
	@echo "  make clippy         - Run clippy linter"
	@echo "  make doc            - Generate and open documentation"
	@echo "  make setup-examples - Download test images for examples"
	@echo "  make run-example    - Run the simple_vm_cirros example"
	@echo "  make regenerate     - Regenerate SDK from upstream OpenAPI spec"
	@echo "  make clean          - Clean build artifacts"

build:
	cargo build

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy

doc:
	cargo doc --open

setup-examples:
	./examples/fetch_test_images.sh

run-example: setup-examples
	cargo run --example simple_vm_cirros

regenerate:
	./regenerate.sh

clean:
	cargo clean

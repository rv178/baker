BINARY=bake
all: dev

release:
	@cargo build --release 
	@cp ./target/release/$(BINARY) ./bin/$(BINARY)

dev:
	@cargo build
	@cp ./target/debug/$(BINARY) ./bin/$(BINARY)

check:
	@cargo fmt
	@cargo check

clean:
	@cargo clean

setup:
	@mkdir bin
	@rustup install stable
	@rustup default stable

.PHONY: check clean setup all build

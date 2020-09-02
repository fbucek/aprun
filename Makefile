#.PHONY: doc

all: clean check test build doc

check:
	cargo check
	cargo clippy
	cargo fmt

test: 
	cargo test

build:
	cargo build

doc:
	cargo doc --no-deps --document-private-items --open

clean: 
	cargo clean --doc

publish: all
	cd aprun && cargo publish

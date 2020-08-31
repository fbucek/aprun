#.PHONY: doc

all: clean test build doc

test: 
	cargo test

build:
	cargo build

doc:
	cargo doc --no-deps --document-private-items --open

clean: 
	cargo clean --doc

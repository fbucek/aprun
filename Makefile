#.PHONY: doc

doc:
	cargo doc --no-deps --document-private-items --open

clean: 
	cargo clean --doc

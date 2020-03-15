test:
	cargo test --all-features

lint:
	cargo clippy -- -D warnings

.PHONY: check test

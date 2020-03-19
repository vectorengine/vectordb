fmt:
	cargo fmt

test:
	cargo test --all-features

lint:
	cargo clippy -- -D warnings

clean:
	rm -rf target/

.PHONY: fmt lint test clean

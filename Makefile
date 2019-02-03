
.PHONY: all fmt test doc lint clean

all:
	cargo run --example test

fmt:
	cargo fmt

test:
	cargo test -- --nocapture
	#cargo test

doc:
	cargo doc --open

lint:
	cargo clippy

clean:
	cargo clean

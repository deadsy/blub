
.PHONY: all bench fmt test doc lint clean

all:
	cargo run --example test

release:
	cargo run --release --example test

bench:
	cargo bench

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

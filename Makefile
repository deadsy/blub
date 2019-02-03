
all:
	cargo run --example test

fmt:
	cargo fmt

doc:
	cargo doc --open

lint:
	cargo clippy

clean:
	cargo clean

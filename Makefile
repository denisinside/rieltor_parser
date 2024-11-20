build:
	cargo build
	cargo build --release

run:
	cargo run --release $(args)

test:
	cargo test

clippy:
	cargo clippy
	cargo clippy --release
	cargo clippy --tests

fmt:
	cargo fmt --all

clean:
	cargo clean

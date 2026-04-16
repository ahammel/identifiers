p:
	cargo fmt --all

l:
	cargo clippy --all

t:
	cargo test --all

c check:
	cargo check --all

b build:
	cargo build --all

r run:
	cargo run

br build-release:
	cargo build --release --all

.PHONY: p l t c check b build r run br build-release

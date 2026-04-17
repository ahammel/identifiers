b build:
	cargo build --all

br build-release:
	cargo build --release --all

p pretty:
	cargo fmt --all

l lint:
	cargo clippy --all

t test:
	cargo test --all

c check:
	cargo check --all

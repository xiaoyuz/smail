.PHONY: check-server test-server fmt-server

check-server:
	cargo check --all --all-targets --all-features
	cargo fmt -- --check
	cargo clippy --all-targets --all-features -- -D clippy::all

fmt-server:
	cargo clippy --fix --allow-dirty --allow-staged
	cargo fmt

test-server:
	cargo test --all
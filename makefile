.PHONY: build docker fmt lint test

build:
	cargo build -r

docker:
	todo

fmt:
	cargo +nightly fmt

lint:
	cargo clippy --all-targets --all-features -- -D warnings -D clippy::all

test:
	cargo test

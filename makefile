all: test

build:
	cargo build

run:
	cargo run

test:
	cargo test -- --nocapture

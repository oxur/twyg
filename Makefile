default: all

all: build test run-demos

build:
	@cargo build

test:
	@cargo test

run-demos:
	@cargo run --example=color-caller
	@cargo run --example=no-caller
	@cargo run --example=no-color

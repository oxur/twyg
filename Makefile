default: all

all: build test run-demos

build:
	@cargo build

test:
	@cargo test

demos:
	@cargo run --example=colour-caller
	@cargo run --example=no-caller
	@cargo run --example=no-colour
	@cargo run --example=from-config

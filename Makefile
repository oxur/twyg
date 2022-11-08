default: all

all: deps build test demos

build:
	@cargo build

test:
	@cargo test

demos:
	@cargo run --example=colour-caller
	@cargo run --example=no-caller
	@cargo run --example=no-colour
	@cargo run --example=from-config

deps:
	@cargo update

publish:
	@cargo publish

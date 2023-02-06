default: all

all: deps build test demos

auth:
	@echo "Copy and paste the following in the terminal where you"
	@echo "will be executing cargo commands:"
	@echo
	@echo '    eval $$(ssh-agent -s) && ssh-add'
	@echo

build:
	@cargo build

lint:
	@cargo +nightly clippy --version
	@cargo +nightly clippy --all-targets --all-features -- --no-deps -D clippy::all

cicd-lint:
	@cargo clippy --version
	@cargo clippy --all-targets --all-features -- --no-deps -D clippy::all

test:
	@cargo test

examples:
	@cargo run --example=colour-caller
	@cargo run --example=no-caller
	@cargo run --example=no-colour
	@cargo run --example=from-config

demos: examples

deps:
	@cargo update

publish:
	@cargo publish

nightly:
	@rustup toolchain install nightly

.PHONY: default all auth build lint test examples demos deps publish

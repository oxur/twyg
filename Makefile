default: all

all: deps build lint check test demos

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

check:
	@cargo +nightly udeps

test:
	@cargo test

examples:
	@echo
	@echo ">>> With Colour & Caller <<<"
	@cargo run --example=colour-caller
	@echo
	@echo ">>> Without Caller <<<"
	@cargo run --example=no-caller
	@echo
	@echo ">>> Without Colour <<<"
	@cargo run --example=no-colour
	@echo
	@echo ">>> From Config <<<"
	@cargo run --example=from-config
	@echo
	@echo ">>> From Config (using confyg library) <<<"
	@cargo run --example=from-confyg
	@echo
	@echo ">>> To stderr <<<"
	@cargo run --example=stderr
	@echo

demos: examples

deps:
	@cargo update

publish:
	@cargo publish

nightly:
	@rustup toolchain install nightly

install-udeps:
	@echo ">> Setting up cargo udeps ..."
	@cargo install cargo-udeps --locked

.PHONY: default all auth build lint test examples demos deps publish

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

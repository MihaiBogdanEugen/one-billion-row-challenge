## help: Prints this help message
help:
	@echo "\nOne Billion Row Challenge\nUsage: \n"
	@sed -n "s/^##//p" ${MAKEFILE_LIST} | column -t -s ":" |  sed -e "s/^/ /"

## build: Build the local packages and all of its dependencies with optimizations (release mode)
build:
	cargo build --release

## generate-input:  Generate input file
generate-input: build
	cargo run --release --bin generate_input -- --size $(size)

## run: Run the solver for the obrc
run: build
	cargo run --release --bin obrc -- --input-path $(input-path) --solution $(solution) 

## test: Run the tests for the obrc
test: build
	cargo test --verbose

## bench: Run the criterion benchmark for measurements_1000000000.txt
bench:
	cargo bench

## update: Update dependencies listed in Cargo.lock
update:
	cargo update

## check: Analyze the current package and report errors, but don't build object files
check:
	cargo check --verbose

## clean: Clean the current package and all build artifacts
clean:
	@rm -rdf Cargo.lock && cargo clean

## fmt: Format all Rust files of the current crate
fmt:
	cargo fmt --all

## clippy: Run cargo clippy for static ckecks
clippy:
	cargo clippy --all-targets --all-features --verbose

.PHONY: help build generate-input run bench update check clean fmt clippy
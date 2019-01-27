.PHONY: integration
integration: build test
	bats integration

.PHONY: build
build:
	cargo build

.PHONY: test
test:
	cargo test

.PHONY: release
release:
	cargo build --release

.PHONY: performance
performance: release
	hyperfine --warmup 10 ./target/release/roadrunner

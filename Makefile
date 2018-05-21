.PHONY: integration
integration: build
	bats integration

.PHONY: build
build:
	cargo build

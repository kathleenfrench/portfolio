
## rust ops

.PHONY: build
build: ## compile the rust binary
	@cd api && cargo build

.PHONY: run
run: ## run the rust app locally
	@cd api && cargo run

.PHONY: test
test: ## run rust tests
	@cd api && cargo test

.PHONY: release
release: ## compile a release build
	@cd api && cargo build --release

.PHONY: check
check: ## verify the rust bin is able to be compiled
	@cd app && cargo check

## frontend ops

.PHONY: help
help: ## lists some available makefile commands
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.DEFAULT_GOAL := help
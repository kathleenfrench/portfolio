
## rust ops

.PHONY: build
build: ## compile the rust binary
	@cargo build

.PHONY: run
run: ## run the rust app locally
	@cargo run

.PHONY: test
test: ## run rust tests
	@cargo test

.PHONY: release
release: ## compile a release build
	@cargo build --release

.PHONY: check
check: ## verify the rust bin is able to be compiled
	@cargo check

## frontend ops

.PHONY: js
js: ## compile js
	@cd web && npx spack

.PHONY: clean
clean: ## remove compiled js assets
	@rm -rf dist

.PHONY: help
help: ## lists some available makefile commands
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.DEFAULT_GOAL := help
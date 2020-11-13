
## rust ops

.PHONY: run
run: ## run the rust app locally
	@cd api && cargo run

.PHONY: test
test: ## run rust tests
	@cd api && cargo test

## frontend ops

.PHONY: help
help: ## lists some available makefile commands
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.DEFAULT_GOAL := help
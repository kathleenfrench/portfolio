
########################################## local dev

.PHONY: local
local: run dist ## compiles/bundles all code, starts the rust server
	@echo "success! visit the site in the browser at https://127.0.0.1:3000!"

########################################## rust server

.PHONY: build
build: ## compile the rust server binary
	@cargo build

.PHONY: run
run: ## run the rust app server locally
	@cargo run

.PHONY: lint
lint: ## lint the rust code
	@cargo fmt

.PHONY: test
test: lint ## run rust tests
	@cargo test

.PHONY: release
release: ## compile a release build
	@cargo build --release

.PHONY: check
check: ## verify the rust server bin is able to be compiled
	@echo "checking server binary..."
	@cargo check
	@echo "checking pterm lib..."
	@cd pterm && cargo check

.PHONY: watch
watch: ## run the hot-reload server for the rust backend
	@systemfd --no-pid -s http::3000 -- cargo watch -x run

########################################## frontend ops

web/package.json:
	@npm install -C web

.PHONY: dist
dist: clean | web/package.json ## build and bundle all assets (js, css, html)
	@npm --prefix web run build

########################################## utils

.PHONY: clean
clean: ## remove generated assets
	@rm -rf dist

.PHONY: help
help: ## lists some available makefile commands
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.DEFAULT_GOAL := help
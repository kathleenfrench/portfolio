
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

.PHONY: watch
watch: ## run the hot-reload server for rust
	@systemfd --no-pid -s http::3000 -- cargo watch -x run

## frontend ops

DIST_DIR := $(PWD)/dist

${DIST_DIR}:
	@if [ ! -d $(DIST_DIR) ]; then \
		echo "creating /dist directory..."; \
		mkdir -p $(DIST_DIR); \
	fi;

.PHONY: css
css: | ${DIST_DIR} ## bundle css
	@cd web && npm run css

.PHONY: js
js: | ${DIST_DIR} ## compile js
	@cd web && npx spack

.PHONY: assets
assets: js css ## compile all frontend assets to /dist
	@echo successfully compiled assets!

.PHONY: clean
clean: ## remove compiled js assets
	@echo "nuking /dist directory..."
	@rm -rf dist

.PHONY: help
help: ## lists some available makefile commands
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.DEFAULT_GOAL := help
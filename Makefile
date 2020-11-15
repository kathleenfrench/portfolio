
## local

.PHONY: local
local: run assets | cert ## configures ssl, compiles/bundles all code, starts the rust server
	@echo "success! visit the site in the browser at https://127.0.0.1:3000!"

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

.PHONY: css
css: ## bundle css
	@cd web && npm run css

.PHONY: hot-css
hot-css: ## hot reload css scripts
	@cd web && npm run css-watch

.PHONY: js
js: ## compile js
	@cd web && npx spack

.PHONY: assets
assets: js css ## compile all frontend assets to /static/assets
	@echo successfully compiled assets!

## utils

.PHONY: clean
clean: ## remove generated assets
	@echo "nuking generated directories..."
	@rm -rf static/assets/js
	@rm -rf static/assets/css

.PHONY: cert
cert: | cert.pem key.pem ## create a local self-signed cert for dev and install it
	@mkcert --cert-file cert.pem --key-file key.pem localhost dev.local 127.0.0.1 ::1
	@mkcert -install

.PHONY: help
help: ## lists some available makefile commands
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.DEFAULT_GOAL := help
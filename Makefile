########################################## config

# the wasm-bindgen CLI version must exactly match the wasm-bindgen crate in
# web/pterm/Cargo.lock, so read it from there instead of hardcoding it
WASM_BINDGEN_VERSION := $(shell grep -A1 '^name = "wasm-bindgen"$$' web/pterm/Cargo.lock | sed -n 's/version = "\(.*\)"/\1/p')
PTERM_WASM := web/pterm/target/wasm32-unknown-unknown/release/pterm.wasm

########################################## local dev

.PHONY: local
local: dist ## build everything and run the site at http://127.0.0.1:3000
	@cargo run

.PHONY: setup
setup: ## install the toolchain bits (wasm target, pinned wasm-bindgen, npm deps)
	@rustup toolchain install # installs the toolchain + wasm target pinned in rust-toolchain.toml
	@if [ "$$(wasm-bindgen --version 2>/dev/null)" != "wasm-bindgen $(WASM_BINDGEN_VERSION)" ]; then \
		cargo install wasm-bindgen-cli --version $(WASM_BINDGEN_VERSION) --locked; \
	fi
	@npm ci --prefix web

########################################## docker (the exact same build heroku runs)

.PHONY: up
up: ## build + run the production docker image at http://127.0.0.1:3000
	@docker compose up --build -d
	@echo "running at http://127.0.0.1:3000 (make down to stop)"

.PHONY: down
down: ## stop the docker environment
	@docker compose down

########################################## heroku staging (needs a `staging` git remote)

.PHONY: staging
staging: ## build the current branch (committed changes only) on heroku staging
	@git push staging HEAD:main --force

.PHONY: staging-down
staging-down: ## scale heroku staging back to zero dynos
	@heroku ps:scale web=0 -r staging

########################################## rust server

.PHONY: build
build: ## compile the rust server binary
	@cargo build

.PHONY: run
run: ## run the rust server (without rebuilding the frontend)
	@cargo run

.PHONY: lint
lint: ## format + lint the rust code
	@cargo fmt
	@cargo clippy -- -D warnings

.PHONY: test
test: lint ## run rust tests
	@cargo test

.PHONY: release
release: ## compile a release build
	@cargo build --release

.PHONY: check
check: ## verify the server and the pterm wasm crate both compile
	@cargo check
	@cd web/pterm && cargo check --lib --target wasm32-unknown-unknown

########################################## frontend

.PHONY: wasm
wasm: ## compile the pterm terminal to wasm + generate js bindings into web/pkg
	@cd web/pterm && cargo build --lib --release --target wasm32-unknown-unknown
	@wasm-bindgen --target bundler --out-dir web/pkg --out-name index $(PTERM_WASM)

web/node_modules: web/package-lock.json
	@npm ci --prefix web
	@touch web/node_modules

.PHONY: dist
dist: wasm web/node_modules ## build and bundle all frontend assets into dist/
	@rm -rf dist
	@npm --prefix web run build

.PHONY: clean
clean: ## remove generated assets
	@rm -rf dist web/pkg

########################################## utils

.PHONY: help
help: ## lists available make commands
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-12s\033[0m %s\n", $$1, $$2}'

.DEFAULT_GOAL := help

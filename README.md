# kathleenfrench.co

_this is the repo for my personal portfolio: a fake terminal in the browser, written in rust + webassembly, served by a rust web server._

----

## how it works

```
web/pterm/          rust crate -> compiled to wasm. this IS the terminal: the intro
                    animation, every command (help, about, trout, ...), the treasure hunt.
web/src/main.js     sets up xterm.js and loads the wasm
static/             everything served as-is (css, images, text content, html templates).
                    mirrors the layout of dist/
src/                the rust web server (actix-web): renders the handlebars templates,
                    serves dist/assets with compression + cache headers, 404 page
config/             server config (Default.yml + Dev.yml/Prod.yml, picked by $ENVIRONMENT)
Dockerfile          the production build. heroku builds this (via heroku.yml), CI builds this,
                    `make up` builds this. if it builds anywhere it builds everywhere.
```

the build, start to finish:

1. `web/pterm` is compiled to `wasm32-unknown-unknown`, then `wasm-bindgen` generates js bindings into `web/pkg`
2. webpack bundles `web/src/main.js` + `web/pkg` into `dist/assets/js/` with content-hashed filenames, injects the `<script>` tag into `dist/templates/index.html`, and copies the rest of `static/` into `dist/`
3. the rust server runs and serves `dist/`

**the text content** (resume, publications, intro logs, the list of dog photos) lives in `static/assets/files/` and is baked _into the wasm_ at compile time via `include_str!`, so editing those files needs a rebuild (`make local`), not just a refresh.

## local dev

### one-time setup

you need [rustup](https://rustup.rs), node 22+, and (optionally) docker.

```sh
make setup
```

this installs the pinned rust toolchain (from `rust-toolchain.toml`) + the wasm target, the `wasm-bindgen` CLI at the exact version `web/pterm/Cargo.lock` needs, and the npm deps.

### running it

```sh
make local        # build everything + run at http://127.0.0.1:3000
```

or, to run the exact production image heroku will run:

```sh
make up           # docker build + run at http://127.0.0.1:3000
make down
```

if you only changed rust server code (`src/`), `make run` skips the frontend rebuild.

> the intro animation only plays once per browser (it's remembered in localStorage). type `replay` in the terminal to see it again.

run `make` with no arguments to list every command.

## config

the server reads `config/Default.yml`, then `config/$ENVIRONMENT.yml` (`Dev` locally, `Prod` in docker). any value can be overridden with an env var prefixed `KF_`, e.g. `KF_LOG_LEVEL=debug`. `$PORT` sets the port (default 3000; heroku sets it automatically).

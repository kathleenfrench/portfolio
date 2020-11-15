# kathleenfrench.co

_this is the repo for my personal portfolio_

----

## local dev

### make commands

running `make` in your terminal will output some useful commands for running the site locally

```
assets                         compile all frontend assets to /static/assets
build                          compile the rust binary
cert                           create a local self-signed cert for dev and install it
check                          verify the rust bin is able to be compiled
clean                          remove generated assets
css                            bundle css
help                           lists some available makefile commands
hot-css                        hot reload css scripts
js                             compile js
local                          configures ssl, compiles/bundles all code, starts the rust server
release                        compile a release build
run                            run the rust app locally
test                           run rust tests
watch                          run the hot-reload server for rust
```

#### run it locally

to build and run the local environment, simply run: `make local`

#### hot reloading

if you want to hot reload changes:
- for `rust` code: `make watch`
- for `css` code: `make hot-css`

### ssl

_note: the `make cert` target is a dependency of `make local`_

using `mkcert` we can generate a self-signed local cert to use in development. `make cert` takes care of all of that for you, at which point the development site points to [https://localhost:3000](https://localhost:3000)
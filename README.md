# kathleenfrench.co

_this is the repo for my personal portfolio_

----

### local dev

#### make commands

running `make` in your terminal will output some useful commands for running the site locally

```
build                          compile the rust server binary
cert                           create a local self-signed cert for dev and install it
check                          verify the rust server bin is able to be compiled
clean                          remove generated assets
dist                           build and bundle all assets (js, css, html)
help                           lists some available makefile commands
lint                           lint the rust code
local                          configures ssl, compiles/bundles all code, starts the rust server
release                        compile a release build
run                            run the rust app server locally
test                           run rust tests
watch                          run the hot-reload server for the rust backend
```

#### ssl

_note: the `make cert` target is a dependency of `make local`_

using `mkcert` we can generate a self-signed local cert to use in development. `make cert` takes care of all of that for you, at which point the development site points to [https://localhost:3000](https://localhost:3000)
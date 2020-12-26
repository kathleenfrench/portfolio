# kathleenfrench.co

_this is the repo for my personal portfolio_

----

### deployment

**heroku**:

using the [heroku buildpack subdir](https://elements.heroku.com/buildpacks/sectorlabs/heroku-buildpack-subdir)

buildpacks are defined in the `.buildpacks` file in the root directory, where the order of the buildpacks implies the order of the build itself. 

the `subdir` buildpack allows for specifying the `web` directory as the specific frontend application, and use the official `heroku/nodejs` buildpack for it. the `emk/rust` custom buildpack is then used for building the `rust` server and running the application.

### local dev

#### make commands

running `make` in your terminal will output some useful commands for running the site locally

```
build                          compile the rust server binary
check                          verify the rust server bin is able to be compiled
clean                          remove generated assets
dist                           build and bundle all assets (js, css, html)
help                           lists some available makefile commands
lint                           lint the rust code
local                          compiles/bundles all code, starts the rust server
release                        compile a release build
run                            run the rust app server locally
test                           run rust tests
watch                          run the hot-reload server for the rust backend
```

# kathleenfrench.co

_this is the repo for my personal portfolio_

----

### deployment

**heroku**:

using the [heroku buildpack subdir](https://elements.heroku.com/buildpacks/sectorlabs/heroku-buildpack-subdir)

buildpacks are defined in the `.buildpacks` file in the root directory, where the order of the buildpacks implies the order of the build itself.

the `subdir` buildpack allows for specifying the `web` directory as the specific frontend application, and use the official `heroku/nodejs` buildpack for it. the `emk/rust` custom buildpack is then used for building the `rust` server and running the application.

as recommended in the docs, the app is also deployed with `SUBDIR_ENABLE_PROFILE_SOURCING=1` via:

```
heroku config:set SUBDIR_ENABLE_PROFILE_SOURCING=1
```

the reasons for this are outlined in greater detail [here](https://devcenter.heroku.com/articles/buildpack-api#profile-d-scripts), but the gist, per the buildpack docs above:

> .profile.d scripts are left behind by buildpacks and are invoked by Heroku when the dyno is starting. This allows buildpacks to update the PATH environment for example. Since buildpacks can be ran in subdirectories, the .profile.d scripts left behind by these buildpacks are not invoked by Heroku.

### local dev

#### compiling for linux

using [rust-musl-builder](https://github.com/emk/rust-musl-builder)

#### docker caching

rust builds in docker `==` slow - so I'm using a hack-y solution for that to run the application locally in `docker compose` via a `dummy.rs` approach which allows you to leverage docker layer caching of dependencies. you can read more on the strategy [in this blog post](https://blog.mgattozzi.dev/caching-rust-docker-builds/).

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

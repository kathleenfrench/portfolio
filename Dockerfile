# production image for kathleenfrench.co. heroku builds this via heroku.yml,
# and `make up` builds the exact same thing locally.
#
# stages:
#   wasm    -> compiles web/pterm (the terminal) to wasm + js bindings
#   web     -> webpack bundles the js/wasm and copies static/ into dist/
#   server  -> compiles the rust server
#   runtime -> tiny debian image with just the server binary, config and dist/

# keep in sync with rust-toolchain.toml
ARG RUST_VERSION=1.99.0
ARG NODE_VERSION=22

################################ wasm
FROM rust:${RUST_VERSION}-slim-bookworm AS wasm
WORKDIR /src

RUN rustup target add wasm32-unknown-unknown

# wasm-bindgen-cli has to exactly match the wasm-bindgen crate in Cargo.lock
COPY web/pterm/Cargo.toml web/pterm/Cargo.lock web/pterm/
RUN version=$(grep -A1 '^name = "wasm-bindgen"$' web/pterm/Cargo.lock | sed -n 's/version = "\(.*\)"/\1/p') \
 && cargo install wasm-bindgen-cli --version "$version" --locked

# include_str! pulls text content from static/assets/files at compile time
COPY static/assets/files static/assets/files
COPY web/pterm web/pterm
RUN cd web/pterm \
 && cargo build --lib --release --target wasm32-unknown-unknown \
 && wasm-bindgen --target bundler --out-dir ../pkg --out-name index \
      target/wasm32-unknown-unknown/release/pterm.wasm

################################ web
FROM node:${NODE_VERSION}-bookworm-slim AS web
WORKDIR /src

COPY web/package.json web/package-lock.json web/
RUN npm ci --prefix web

COPY static static
COPY web/webpack.config.js web/
COPY web/src web/src
COPY --from=wasm /src/web/pkg web/pkg
RUN npm --prefix web run build

################################ server
FROM rust:${RUST_VERSION}-slim-bookworm AS server
WORKDIR /src

# build dependencies against a stub main.rs first, so they're cached in their
# own docker layer and only our code recompiles when src/ changes
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs \
 && cargo build --release \
 && rm -rf src

COPY src src
RUN touch src/main.rs && cargo build --release

################################ runtime
FROM debian:bookworm-slim

RUN useradd --system --uid 10001 portfolio
WORKDIR /app

COPY --from=server /src/target/release/portfolio .
COPY config config
COPY --from=web /src/dist dist

USER portfolio
ENV ENVIRONMENT=Prod PORT=3000
EXPOSE 3000

# heroku sets $PORT at runtime; the server reads it
CMD ["/app/portfolio"]

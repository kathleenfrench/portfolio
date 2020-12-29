# syntax=docker/dockerfile:experimental

################################ w/ wasm-pack
## see: https://github.com/emk/rust-musl-builder
FROM ekidd/rust-musl-builder:latest AS local-wasm-pack

RUN cargo install wasm-pack

################################ wasm
FROM local-wasm-pack as wasm

WORKDIR /app

COPY --chown=rust:rust web web
COPY --chown=rust:rust static static

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/pkg \
    wasm-pack build --out-dir ../pkg --release --no-typescript web/pterm

################################ js assets/webpack
FROM node:slim as assets

RUN npm install -g webpack webpack-cli

COPY static /static

WORKDIR /web

COPY --from=wasm /app/web/pkg pkg
COPY web/package.json web/webpack.docker.js ./
RUN npm install --legacy-peer-deps

COPY web/src src

RUN npm run release

################################ rust-musl-builder
FROM ekidd/rust-musl-builder:latest AS builder

ADD --chown=rust:rust . ./

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release

################################ actual prod image
FROM alpine:latest

ENV ENVIRONMENT Prod

RUN apk --no-cache add ca-certificates

WORKDIR /app

COPY --from=assets /dist dist
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/portfolio .
COPY config config

CMD /app/portfolio
## see: https://github.com/emk/rust-musl-builder

# Our first FROM statement declares the build environment.
FROM ekidd/rust-musl-builder:latest AS builder

## add src code
ADD --chown=rust:rust . ./

RUN cargo build --release

# build our *real* docker image
FROM alpine:latest

RUN apk --no-cache add ca-certificates

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/portfolio \
    /usr/local/bin/

CMD /usr/local/bin/portfolio
#!/bin/bash

echo "running heroku-prebuild script..."

RUSTUP_INSTALL=$(which rustup 2>/dev/null)
CARGO_INSTALL=$(which cargo 2>/dev/null)

echo "RUSTUP: ${RUSTUP_INSTALL}"
echo "CARGO: ${CARGO_INSTALL}"

echo "-----> Downloading rustup"
curl https://sh.rustup.rs -sSf > rustup.sh
chmod u+x rustup.sh

echo "-----> Using rustup to install Rust channel $VERSION"
./rustup.sh -y --default-toolchain "$VERSION"

echo "-----> Installing wasm target"
rustup target add wasm32-unknown-unknown

echo "-----> Installing wasm-gc"
cargo install --git https://github.com/alexcrichton/wasm-gc

rm rustup.sh

# BUILD_DIR=${1:-}
# CACHE_DIR=${2:-}
# ENV_DIR=${3:-}
# BP_DIR=$(cd $(dirname ${0:-}); cd ..; pwd)

# set -eu

# # Record our Rust build environment configuration in an export file, in
# # case another buildpack needs it to build Ruby gems that use Rust or
# # something like that.
# cat <<EOF > $BP_DIR/export
# # Our rustup installation.
# export RUSTUP_HOME="$CACHE_DIR/multirust"
# export CARGO_HOME="$CACHE_DIR/cargo"
# PATH="\$CARGO_HOME/bin:\$PATH"
# EOF

# # Read our build environment back in and evaluate it so that we can use it.
# . $BP_DIR/export

# # Switch to our cache directory.
# mkdir -p "$CACHE_DIR"
# cd "$CACHE_DIR"



# # Make sure we have an appropriate Rust toolchain installed.
# if [ -d "$CARGO_HOME" ]; then
#     echo "-----> Checking for new releases of Rust $VERSION channel"
#     # It's possible that $VERSION has changed, or the `stable` channel has updated.
#     rustup self update
#     rustup update "$VERSION"
#     rustup default "$VERSION"
# else
#     echo "-----> Downloading rustup"
#     curl https://sh.rustup.rs -sSf > rustup.sh
#     chmod u+x rustup.sh

#     echo "-----> Using rustup to install Rust channel $VERSION"
#     ./rustup.sh -y --default-toolchain "$VERSION"

#     echo "-----> Installing wasm target"
#     rustup target add wasm32-unknown-unknown

#     echo "-----> Installing wasm-gc"
#     cargo install --git https://github.com/alexcrichton/wasm-gc

#     rm rustup.sh
# fi
# if [ ! -x "$CARGO_HOME/bin/rustc" ]; then
#   echo "failed: Cannot find Rust binaries at $CARGO_HOME"
#   exit 1
# fi

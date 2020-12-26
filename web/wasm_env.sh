#!/bin/bash

echo "running heroku-prebuild script..."

RUSTUP_INSTALL=$(which rustup 2>/dev/null)
CARGO_INSTALL=$(which cargo 2>/dev/null)

echo "-----> Downloading rustup"
curl https://sh.rustup.rs -sSf > rustup.sh
chmod u+x rustup.sh

echo "-----> Using rustup to install Rust channel stable on host "
./rustup.sh -y --default-toolchain "stable" --default-host "x86_64-unknown-linux-gnu"

echo "-----> Installing wasm target"
rustup target add wasm32-unknown-unknown

echo "-----> Installing wasm-gc"
cargo install --git https://github.com/alexcrichton/wasm-gc

rm rustup.sh
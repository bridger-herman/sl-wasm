#!/bin/sh

set -ex

# Note that typically we'd use `wasm-pack` to build the crate, but the
# `--web` flag is very new to `wasm-bindgen` and as such doesn't have
# support in `wasm-pack` yet. Support will be added soon though!

cargo build --target wasm32-unknown-unknown --release
wasm-bindgen ./target/wasm32-unknown-unknown/release/sl-rs.wasm --out-dir ./assets/pkg --web --no-typescript

python3 -m http.server

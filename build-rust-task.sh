#!/bin/bash

# This should take the name of the Rust project name
taskname=my_project

# Make sure a recent version of Emscripten (e.g. 1.39.8) is activated and available before running this

# Transpile to WASM
# Only release builds work for this target
cargo build --release --target=wasm32-unknown-emscripten

cd target/wasm32-unknown-emscripten/release

# Transform to WAT
wasm2wat "$taskname.wasm" -o "$taskname.wat"

# The WASI functions are exported from `env` in the filesystem from `emscripten-module-wrapper`
sed -i 's/wasi_snapshot_preview1/env/g' "$taskname.wat"

# Transform back to WASM
wat2wasm "$taskname.wat" -o "out_wasm.wasm"

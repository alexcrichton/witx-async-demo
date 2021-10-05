#!/bin/sh

cargo build --manifest-path wasm/Cargo.toml --target wasm32-wasi --release

#!/bin/sh

set -ex
cargo build --manifest-path wasm/Cargo.toml --target wasm32-wasi --release
(cd host && npx tsc host.ts)

#!/bin/sh

set -ex

# Install the `witx-bindgen` CLI tool if it's not already installed here.
if [ ! -d witx-bindgen-install ]; then
  sha=$(grep 'source =.*witx-bindgen' wasm/Cargo.lock | head -n 1 | sed 's/.*#//' | sed 's/"//')
  cargo install \
    --git https://github.com/bytecodealliance/witx-bindgen \
    --rev $sha \
    --root witx-bindgen-install \
    witx-bindgen-cli
fi
witx_bindgen=./witx-bindgen-install/bin/witx-bindgen

# Compile the Rust code to WebAssembly
export RUSTFLAGS=-Clink-args=--export-table
cargo build --manifest-path wasm/Cargo.toml --target wasm32-unknown-unknown --release

rm -rf static
mkdir static
cp ./wasm/target/wasm32-unknown-unknown/release/witx_async_demo.wasm static


if [ ! -d ace ]; then
  mkdir ace
  cd ace
  curl -L https://github.com/ajaxorg/ace-builds/archive/refs/tags/v1.4.13.tar.gz | tar xzf -
  cd ..
fi
cp -r ace/ace-builds-1.4.13/src static/ace

# Generate bindings for JS
cp host/host.ts host/index.html static
$witx_bindgen js \
        --import imports.witx \
	--export exports.witx \
        --out-dir static/witx
(cd host && npx tsc ../static/host.ts --target es6)

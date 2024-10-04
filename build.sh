#!/bin/bash

# Exit on error
set -e

# Name of your application (should match the package name in Cargo.toml)
APP_NAME="my-egui-web-app"

# Output directory
OUTPUT_DIR="dist"

echo "=== Building the project for WebAssembly ==="
cargo build --target wasm32-unknown-unknown --release

echo "=== Creating the output directory ==="
mkdir -p ${OUTPUT_DIR}

echo "=== Generating JavaScript bindings with wasm-bindgen ==="
wasm-bindgen --out-dir ${OUTPUT_DIR} --no-modules --no-typescript target/wasm32-unknown-unknown/release/${APP_NAME}.wasm

echo "=== Copying index.html to the output directory ==="
cp index.html ${OUTPUT_DIR}

echo "=== Build completed. Files are located in the '${OUTPUT_DIR}' directory ==="
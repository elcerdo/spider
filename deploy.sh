#!/usr/bin/env bash

set -e
set -u

PROJECT="spider"

cargo build --release --target wasm32-unknown-unknown

DEMO_DIR="../demo_public/$PROJECT"
WASM_PATH="$(find "target" | grep "$PROJECT.wasm")"

if [ ! -d "$DEMO_DIR" ]; then
  echo "$DEMO_DIR is not a directory."
  exit
fi
if [ ! -f "$WASM_PATH" ]; then
  echo "$WASM_PATH is not file."
  exit
fi

echo "demo $DEMO_DIR"
echo "wasm $WASM_PATH"

wasm-bindgen "$WASM_PATH" --target web --no-typescript --out-dir "$DEMO_DIR"
cp -r "assets" "$DEMO_DIR/assets"
cp "index.html" "$DEMO_DIR/index.html"
#!/usr/bin/env bash
# Build Quantum Tetris for GitHub Pages (docs/wasm).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

AVAIL_KB="$(df -k . | awk 'NR==2 {print $4}')"
MIN_KB=$((3 * 1024 * 1024))
if [[ "$AVAIL_KB" -lt "$MIN_KB" ]]; then
  echo "WARN: less than 3 GiB free — WASM release build may fail"
fi

command -v wasm-bindgen >/dev/null || {
  echo "Install: cargo install wasm-bindgen-cli"
  exit 1
}

rustup target add wasm32-unknown-unknown

echo "→ cargo build (wasm32, release)…"
CARGO_TARGET_DIR=target cargo build --lib -p quantum-tetris --release \
  --no-default-features --features wasm \
  --target wasm32-unknown-unknown

OUT="$ROOT/docs/wasm"
rm -rf "$OUT"
mkdir -p "$OUT"

echo "→ wasm-bindgen…"
wasm-bindgen --out-dir "$OUT" --target web \
  --no-typescript \
  target/wasm32-unknown-unknown/release/quantum_tetris.wasm

if command -v wasm-opt >/dev/null; then
  echo "→ wasm-opt…"
  wasm-opt -Oz "$OUT/quantum_tetris_bg.wasm" -o "$OUT/quantum_tetris_bg.wasm"
else
  echo "TIP: install binaryen (wasm-opt) to shrink the WASM bundle"
fi

echo "OK — serve docs/ with a static server (see docs/WASM.md)"
du -sh "$OUT" 2>/dev/null || true

#!/usr/bin/env bash
# Build Quantum Sub for GitHub Pages (docs/wasm + docs/assets).
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
CARGO_TARGET_DIR=target cargo build --lib -p quantum-town-la --release \
  --no-default-features --features wasm \
  --target wasm32-unknown-unknown

OUT="$ROOT/docs/wasm"
rm -rf "$OUT"
mkdir -p "$OUT"

echo "→ wasm-bindgen…"
wasm-bindgen --out-dir "$OUT" --target web \
  --no-typescript \
  target/wasm32-unknown-unknown/release/quantum_town_la.wasm

echo "→ assets…"
rm -rf "$ROOT/docs/assets"
mkdir -p "$ROOT/docs/assets"
if [[ -d "$ROOT/assets/models" ]]; then
  cp -R "$ROOT/assets/models" "$ROOT/docs/assets/"
fi

echo "OK — open docs/play.html via a local server (see docs/WASM.md)"
du -sh "$OUT" "$ROOT/docs/assets" 2>/dev/null || true

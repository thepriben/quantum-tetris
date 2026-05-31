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

if [[ "${WASM_OPT:-0}" == "1" ]]; then
  command -v wasm-opt >/dev/null || {
    echo "Install binaryen (wasm-opt), or run without WASM_OPT=1"
    exit 1
  }
  echo "→ wasm-opt…"
  wasm-opt -Oz "$OUT/quantum_tetris_bg.wasm" -o "$OUT/quantum_tetris_bg.wasm"
else
  echo "Skipping wasm-opt: Binaryen currently breaks wasm-bindgen externref table exports for this bundle"
fi

echo "→ validate externref table export…"
python3 - "$OUT/quantum_tetris_bg.wasm" <<'PY'
from pathlib import Path
import sys

def read_u32(data, index):
    value = 0
    shift = 0
    while True:
        byte = data[index]
        index += 1
        value |= (byte & 0x7f) << shift
        if not byte & 0x80:
            return value, index
        shift += 7

def read_name(data, index):
    length, index = read_u32(data, index)
    return data[index:index + length].decode("utf-8"), index + length

data = Path(sys.argv[1]).read_bytes()
if data[:4] != b"\0asm":
    raise SystemExit("invalid WASM magic")

index = 8
tables = []
externref_export = None
while index < len(data):
    section_id = data[index]
    index += 1
    size, index = read_u32(data, index)
    end = index + size
    if section_id == 4:
        count, inner = read_u32(data, index)
        for table_index in range(count):
            element_type = data[inner]
            inner += 1
            flags, inner = read_u32(data, inner)
            minimum, inner = read_u32(data, inner)
            maximum = None
            if flags & 1:
                maximum, inner = read_u32(data, inner)
            tables.append((element_type, minimum, maximum))
    elif section_id == 7:
        count, inner = read_u32(data, index)
        for _ in range(count):
            name, inner = read_name(data, inner)
            kind = data[inner]
            inner += 1
            item_index, inner = read_u32(data, inner)
            if kind == 1 and name == "__wbindgen_externrefs":
                externref_export = item_index
    index = end

if externref_export is None:
    raise SystemExit("missing __wbindgen_externrefs export")

try:
    element_type, minimum, maximum = tables[externref_export]
except IndexError:
    raise SystemExit("__wbindgen_externrefs exports an invalid table index")

if element_type != 0x6f:
    raise SystemExit("__wbindgen_externrefs does not export an externref table")
if maximum is not None and maximum - minimum < 4:
    raise SystemExit("__wbindgen_externrefs table cannot grow by wasm-bindgen's required sentinels")

print("OK externref table export")
PY

echo "OK — serve docs/ with a static server (see docs/WASM.md)"
du -sh "$OUT" 2>/dev/null || true

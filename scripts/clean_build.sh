#!/usr/bin/env bash
# Free disk space before a Bevy build by removing target and optional caches.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

AVAIL_KB="$(df -k . | awk 'NR==2 {print $4}')"
MIN_KB=$((2 * 1024 * 1024)) # 2 GiB
if [[ "$AVAIL_KB" -lt "$MIN_KB" ]]; then
  echo "WARN: less than 2 GiB free ($(df -h . | awk 'NR==2 {print $4}')) — build may fail"
fi

echo "Removing target/ ($(du -sh target 2>/dev/null | cut -f1 || echo 'missing'))"
rm -rf target

if [[ "${1:-}" == "--cargo-registry" ]]; then
  echo "Cleaning global cargo cache"
  cargo cache -a 2>/dev/null || cargo clean
fi

df -h . | tail -1
echo "OK - run: CARGO_TARGET_DIR=target cargo run -p quantum-town-la"

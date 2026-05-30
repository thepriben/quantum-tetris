#!/usr/bin/env bash
# Free disk space before a Bevy build (needs ~5 GiB free for debug, ~3 GiB for WASM release).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

avail_h() {
  df -h . | awk 'NR==2 {print $4 " free (" $5 " used)"}'
}

echo "Before: $(avail_h)"

if [[ -d target ]]; then
  echo "Removing target/ ($(du -sh target 2>/dev/null | cut -f1))"
  rm -rf target
fi

if [[ "${1:-}" == "--cargo-cache" ]]; then
  echo "Removing ~/.cargo/registry/cache"
  rm -rf "${HOME}/.cargo/registry/cache"
fi

echo "After:  $(avail_h)"

AVAIL_KB="$(df -k . | awk 'NR==2 {print $4}')"
MIN_KB=$((5 * 1024 * 1024))
if [[ "$AVAIL_KB" -lt "$MIN_KB" ]]; then
  echo ""
  echo "WARN: still under 5 GiB free — cargo run will likely fail (errno=28)."
  echo "Free space elsewhere (Downloads, old Xcode simulators, Docker images, etc.)"
  exit 1
fi

echo "OK — cargo run -p quantum-tetris   or   QUANTUM_MODE=classic cargo run -p quantum-tetris"

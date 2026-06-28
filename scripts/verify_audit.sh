#!/usr/bin/env bash
# Verify an audit journal exported by Quantum Tetris (desktop game over).
set -euo pipefail
cd "$(dirname "$0")/.."
FILE="${1:?Usage: verify_audit.sh audit/qt-....json}"
cargo run -q -p quantum-tetris-quantum --example verify_audit -- "$FILE"

#!/usr/bin/env bash
# Free disk space for Rust/Bevy builds. Safe: never touches source, .git, .env
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

REPORT="$ROOT/.disk_cleanup_report.txt"
: > "$REPORT"

log() { echo "$*" | tee -a "$REPORT"; }

size_of() {
  du -sh "$1" 2>/dev/null | cut -f1 || echo "?"
}

freed_kb=0
remove_dir() {
  local path="$1"
  if [[ ! -d "$path" && ! -e "$path" ]]; then
    return
  fi
  local kb
  kb="$(du -sk "$path" 2>/dev/null | cut -f1 || echo 0)"
  log "  removing $(size_of "$path")  $path"
  rm -rf "$path"
  freed_kb=$((freed_kb + kb))
}

log "=== DISK CLEANUP — $(date) ==="
log "Project: $ROOT"
log ""
log "BEFORE:"
df -h "$ROOT" "$HOME" / 2>&1 | tee -a "$REPORT"
log ""
log "Large dirs (before):"

for path in \
  "$ROOT/target" \
  "$ROOT/docs/wasm" \
  "$HOME/.cargo/registry/cache" \
  "$HOME/.cargo/git/checkouts" \
  "$HOME/Library/Caches/Cargo" \
  "$HOME/Library/Caches/org.rust-lang.rustup"; do
  [[ -e "$path" ]] && log "  $(size_of "$path")  $path"
done

# Cursor sandbox cargo targets (often huge, safe to delete)
while IFS= read -r sandbox_target; do
  [[ -d "$sandbox_target" ]] && log "  $(size_of "$sandbox_target")  $sandbox_target"
done < <(find /var/folders -path "*/cursor-sandbox-cache/*/cargo-target" -type d 2>/dev/null | head -20)

log ""
log "=== REMOVING ==="

remove_dir "$ROOT/target"
remove_dir "$ROOT/docs/wasm"
[[ -f "$ROOT/Cargo.toml" ]] && (cd "$ROOT" && cargo clean 2>/dev/null || true)
remove_dir "$HOME/.cargo/registry/cache"
remove_dir "$HOME/Library/Caches/Cargo"

while IFS= read -r sandbox_target; do
  remove_dir "$sandbox_target"
done < <(find /var/folders -path "*/cursor-sandbox-cache/*/cargo-target" -type d 2>/dev/null | head -20)

if [[ "${1:-}" == "--aggressive" ]]; then
  log ""
  log "Aggressive mode:"
  remove_dir "$HOME/.cargo/registry"
  remove_dir "$HOME/.cargo/git"
  # Xcode derived data (often 10-30 GB)
  remove_dir "$HOME/Library/Developer/Xcode/DerivedData"
  # pip cache
  remove_dir "$HOME/Library/Caches/pip"
  command -v docker >/dev/null && docker system prune -af 2>/dev/null | tee -a "$REPORT" || true
fi

FREED_GB="$(echo "scale=2; $freed_kb / 1024 / 1024" | bc 2>/dev/null || echo "?")"
log ""
log "Estimated freed: ~${FREED_GB} GiB"
log ""
log "AFTER:"
df -h "$ROOT" "$HOME" / 2>&1 | tee -a "$REPORT"

AVAIL_KB="$(df -k "$ROOT" | awk 'NR==2 {print $4}')"
MIN_KB=$((5 * 1024 * 1024))
log ""
if [[ "$AVAIL_KB" -lt "$MIN_KB" ]]; then
  log "WARN: still under 5 GiB free — try: ./scripts/free_disk.sh --aggressive"
  log "Also check: ~/Downloads, Docker, Xcode simulators, Trash (Empty Trash)."
  exit 1
fi

log "OK — you can compile again:"
log "  cargo test -p quantum-tetris-quantum"
log "  cargo run -p quantum-tetris"

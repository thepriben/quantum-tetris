#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="$ROOT/assets/models"
ZIP="/tmp/nature-kit.zip"
mkdir -p "$DEST"

echo "→ Nature Kit rocks (Kenney CC0)…"
curl -fsSL -o "$ZIP" "https://opengameart.org/sites/default/files/Nature%20Kit%20%282.1%29.zip"
unzip -qo "$ZIP" "Models/GLTF format/rock_largeA.glb" "Models/GLTF format/rock_smallA.glb" -d /tmp/nkout
cp "/tmp/nkout/Models/GLTF format/"rock_*.glb "$DEST/"
ls -la "$DEST"

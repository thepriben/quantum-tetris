# Build WASM (GitHub Pages)

Le jeu tourne dans le navigateur via **wasm-bindgen** + QIP ou classique in-process (pas de Python).

## Jouer en local (navigateur)

```bash
./scripts/fetch_assets.sh    # optionnel (rochers GLB)
./scripts/build_wasm.sh
python3 -m http.server 8080 --directory docs
```

Ouvrir [http://localhost:8080/play.html](http://localhost:8080/play.html)  
Mode quantique : [play.html?mode=quantum](play.html?mode=quantum)

> Un serveur HTTP est requis (pas `file://`) — COOP/wasm + chargement des assets.

## Structure déployée (`docs/`)

| Chemin | Rôle |
| --- | --- |
| `index.html` | Landing |
| `play.html` | Canvas + loader WASM |
| `wasm/` | Généré par `build_wasm.sh` (`.wasm`, `.js`) |
| `assets/models/` | GLB copiés depuis `assets/models/` |

## Build manuel

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli

CARGO_TARGET_DIR=target cargo build --lib -p quantum-town-la --release \
  --no-default-features --features wasm \
  --target wasm32-unknown-unknown

wasm-bindgen --out-dir docs/wasm --target web \
  target/wasm32-unknown-unknown/release/quantum_town_la.wasm
```

`.cargo/config.toml` active `getrandom_backend="wasm_js"` pour wasm32.

## Entrées Rust

| Export | Backend |
| --- | --- |
| `run_wasm()` | Classique (uniforme) |
| `run_wasm_quantum()` | QIP |

## GitHub Pages

Le workflow [`.github/workflows/pages.yml`](../.github/workflows/pages.yml) exécute `build_wasm.sh` puis publie `docs/`.

## Contrôles navigateur

Flèches clavier + Espace (identique au binaire desktop).

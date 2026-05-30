# Web build (WASM)

## Goal

Play **Quantum Town: LA** in the browser for article demos, easy sharing, and WASM leaderboard export.

## Stack

| Piece | Choice |
| --- | --- |
| Engine | Bevy 0.18+ |
| Target | `wasm32-unknown-unknown` |
| Physics | bevy_rapier3d (wasm-enabled config) |
| UI | bevy_egui |
| Quantum | **QIP** default in browser |

## Backends in WASM

| Backend | Browser |
| --- | --- |
| QIP (Rust) | Yes |
| Qiskit | No (no Python in tab) |
| BlueQubit | Via server proxy if CORS blocks direct API |

## Planned build commands (Sprint 7)

```bash
rustup target add wasm32-unknown-unknown
cargo build -p quantum-town-la --release --target wasm32-unknown-unknown --features wasm
wasm-bindgen --out-dir dist/wasm --target web target/wasm32-unknown-unknown/release/quantum-town-la.wasm
```

## GitHub Pages

Workflow deploys `docs/` (landing now; WASM bundle later).

URL: https://thepriben.github.io/quantum-town-la/

## Constraints

- Keep wasm + assets under ~15 MB gzip where possible
- Circuits capped at 3 qubits in browser builds
- Desktop-first until Sprint 7

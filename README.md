# Quantum Tetris

[![CI](https://github.com/thepriben/quantum-tetris/actions/workflows/ci.yml/badge.svg)](https://github.com/thepriben/quantum-tetris/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**Quantum Tetris** — neon arcade stacker where **every piece, spin, and drop speed** comes from a small quantum circuit. Play in **classic** mode (uniform random) or **quantum** mode (Born-rule distributions, Qiskit Aer on desktop).

---

## Play locally

```bash
cp .env.example .env
cargo run -p quantum-tetris
```

| Mode | Command |
| --- | --- |
| **Classic** (default) | `QUANTUM_MODE=classic cargo run -p quantum-tetris` |
| **Quantum / Qiskit** | `QUANTUM_MODE=qiskit cargo run -p quantum-tetris` |

Qiskit needs Python 3 and `pip install -r scripts/requirements.txt`. If Aer is unavailable, the game falls back to classic.

### What works where

| Where | Classic | Quantum |
| --- | --- | --- |
| **Your Mac** (`cargo run`) | yes | yes — Qiskit Aer via Python |
| **GitHub Actions CI** | yes (tests) | yes — job `quantum-qiskit` + Born/Qiskit parity |
| **Browser / GitHub Pages** | yes (WASM) | yes — Born-rule statevector (Qiskit-matched) |

The browser build runs a **Rust statevector simulator** with the same gate set as Qiskit Aer. Probabilities are cross-checked against Qiskit in CI. Desktop quantum mode uses the real **Qiskit Aer** subprocess.

**GitHub Pages** requires a **public** repository on the free plan.

**Play online:** [thepriben.github.io/quantum-tetris/play.html](https://thepriben.github.io/quantum-tetris/play.html)

---

## Controls (arcade)

| Key | Action |
| --- | --- |
| **← →** | Move piece |
| **↑** | **Rotate** |
| **↓** | **Speed up** (soft drop) |
| **Space** | **Observe!** — hard drop + quantum bonus |

In-game **CLASSIC** / **QUANTUM** buttons switch modes (browser and desktop).

---

## Quantum hooks

| When | Circuit | Effect |
| --- | --- | --- |
| Each spawn | `quantum-teleportation-gate-v1` ×2 | Bell bits → **family**; message qubit → shape; second shot → **next** piece |
| Spawn pose | `imp-brain-v1` | Rotation + spawn column |
| Gravity | `enemy-profile-hunter-v1` | Drop interval for this piece |
| Space | `observation-pulse-v1` | Score / line bonus |
| Line clear | `q-shard-stabilizer-v1` | Score multiplier |

HUD shows teleporter readout, family, and confidence %.

Details → [docs/QUANTUM.md](docs/QUANTUM.md) · Browser → [docs/WASM.md](docs/WASM.md)

---

## Project layout

```
crates/game/     Bevy Tetris + HUD
crates/quantum/  Circuit IR + classic / Born / Qiskit backends
docs/            WASM bundle + GitHub Pages
scripts/         build_wasm.sh, quantum_shim.py
```

---

## License

MIT — [LICENSE](LICENSE).

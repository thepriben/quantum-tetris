# Quantum Tetris: LA

[![CI](https://github.com/thepriben/quantum-town-la/actions/workflows/ci.yml/badge.svg)](https://github.com/thepriben/quantum-town-la/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**Quantum Tetris** — neon arcade stacker where **every piece, spin, and drop speed** comes from a small quantum circuit. Play in **classic** mode (uniform random) or **Qiskit Aer** (Born-rule distributions).

---

## Play locally

```bash
cp .env.example .env
cargo run -p quantum-town-la
```

| Mode | Command |
| --- | --- |
| **Classic** (default) | `QUANTUM_MODE=classic cargo run -p quantum-town-la` |
| **Qiskit** | `QUANTUM_MODE=qiskit cargo run -p quantum-town-la` |

Qiskit needs Python 3 and `pip install -r scripts/requirements.txt`. If Aer is unavailable, the game falls back to classic.

### What works where

| Where | Classic | Qiskit (Python Aer) |
| --- | --- | --- |
| **Your Mac** (`cargo run`) | yes | yes — `QUANTUM_MODE=qiskit` |
| **GitHub Actions CI** | yes (tests) | yes — job `quantum-qiskit` on every push |
| **Browser / GitHub Pages** | yes (WASM) | **no** — no Python in the browser |

The site deploys a **WASM** build: circuits run in Rust with uniform random bits. **Qiskit cannot run inside a web page**; it needs a Python subprocess (desktop only).

On GitHub, Qiskit is validated by CI (`cargo test` + `scripts/quantum_shim.py`). To **play** with Born-rule distributions, use the desktop game with Qiskit locally.

**GitHub Pages** also requires a **public** repository on the free plan (private repos block Pages).

---

## Controls (arcade)

| Key | Action |
| --- | --- |
| **← →** | Move piece |
| **↑** | **Rotate** |
| **↓** | **Speed up** (soft drop) |
| **Space** | **Observe!** — hard drop + quantum bonus |

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
crates/quantum/  Circuit IR + classic / Qiskit backends
docs/            WASM bundle + GitHub Pages
scripts/         build_wasm.sh, quantum_shim.py
```

---

## License

MIT — [LICENSE](LICENSE).

# Quantum Tetris: LA

[![CI](https://github.com/thepriben/quantum-town-la/actions/workflows/ci.yml/badge.svg)](https://github.com/thepriben/quantum-town-la/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**Quantum Tetris** — a pretty neon stacker where **every piece and special move comes from a small quantum circuit**. Same rules in **classic** (uniform random) and **QIP** (Born rule) modes.

---

## Play locally

```bash
cp .env.example .env
cargo run -p quantum-town-la
```

| Mode | Command |
| --- | --- |
| **Classic** | `QUANTUM_MODE=classic cargo run -p quantum-town-la` |
| **Quantum** | `QUANTUM_MODE=quantum cargo run -p quantum-town-la` |

---

## Controls

| Key | Action |
| --- | --- |
| **← →** | Move piece |
| **↑** | Rotate |
| **↓** | Soft drop |
| **Space** | **Observe** — hard drop + `observation-pulse-v1` bonus |

---

## Quantum hooks

| When | Circuit | Effect |
| --- | --- | --- |
| New piece | `quantum-teleportation-gate-v1` | 3 bits → piece type (I–L) |
| Spawn pose | `imp-brain-v1` | 2 bits → rotation + column |
| Space | `observation-pulse-v1` | 2 bits → score / line bonus |
| Line clear | `q-shard-stabilizer-v1` | 2 bits → score multiplier |

HUD shows `[bits]` and **confidence %** after each measurement.

Details → [docs/QUANTUM.md](docs/QUANTUM.md) · Browser → [docs/WASM.md](docs/WASM.md)

---

## License

MIT — [LICENSE](LICENSE).

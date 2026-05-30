# Quantum Sub: LA

[![CI](https://github.com/thepriben/quantum-town-la/actions/workflows/ci.yml/badge.svg)](https://github.com/thepriben/quantum-town-la/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A small local **Bevy** game: pilot a submarine, collect energy, avoid mines. **Arrow keys** + **Space**. Every action runs a **2–3 qubit circuit**; measured bits drive currents, warps, mines, and pickups.

**Classic mode** (default) uses uniform random outcomes on the same circuits. **QIP mode** uses an in-process Rust simulator (Born rule). Details → [docs/QUANTUM.md](docs/QUANTUM.md).

---

## Play locally

**Requirements:** Rust 1.89+, macOS / Linux / Windows.

```bash
git clone https://github.com/thepriben/quantum-town-la.git
cd quantum-town-la
cp .env.example .env          # QUANTUM_MODE=classic
./scripts/fetch_assets.sh     # optional GLB rocks
cargo run -p quantum-town-la
```

| Mode | Command |
| --- | --- |
| **Classic** (recommended) | `QUANTUM_MODE=classic cargo run -p quantum-town-la` |
| **Quantum (QIP)** | `QUANTUM_MODE=quantum cargo run -p quantum-town-la` |

Disk full at link time? `./scripts/clean_build.sh`

---

## Controls

| Key | Action |
| --- | --- |
| **Arrow keys** | Drive (camera-relative) |
| **Space** | Measure / act — always a circuit |

**Space** priority: south gate (3/3 energy) → nearby cell → warp → observe.

**Goal:** 3 energy cells, south gate, **2:00** timer. HUD shows mode, timer, energy dots, current arrow, `[bits]` and **confidence %**.

---

## Browser (WASM)

```bash
./scripts/build_wasm.sh
python3 -m http.server 8080 --directory docs
# → http://localhost:8080/play.html
```

See [docs/WASM.md](docs/WASM.md). GitHub Pages builds WASM on push to `main`.

---

## Development

```bash
cargo test --workspace
```

| Doc | Content |
| --- | --- |
| [docs/QUANTUM.md](docs/QUANTUM.md) | Circuits, backends, classic vs QIP |
| [docs/WASM.md](docs/WASM.md) | Browser build |
| [docs/DESIGN.md](docs/DESIGN.md) | Short design notes |

---

## License

MIT — [LICENSE](LICENSE).

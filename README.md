# Quantum Town: LA

[![CI](https://github.com/thepriben/quantum-town-la/actions/workflows/ci.yml/badge.svg)](https://github.com/thepriben/quantum-town-la/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**A cute first-person playground where quantum circuits actually drive the game.**

Explore a Los Angeles block rebuilt from OpenStreetMap. Play as a round, Animal-Crossing-style walker. Collect **Q-Shards**, outsmart **Quantum Imps** whose mood is literally in superposition, and dive into **Schrödinger Gates** that send you to probabilistic exits.

> Research question: *What could a quantum computer actually do in a video game?*  
> Answer here: **enemy AI** and **probabilistic teleporters** — with only 2–3 qubits.

---

## Why this project?

| Idea | Detail |
| --- | --- |
| **Small scope** | Weekends-sized MVP in Rust + [Bevy](https://bevyengine.org/) |
| **Real quantum hooks** | Pluggable `QuantumBackend` — not cosmetic shaders |
| **Cute, not Doom** | Pastel world, stun-not-kill, collectathon timer |
| **Records** | Daily seed + local leaderboard |
| **Web demo** | WASM build planned (QIP backend, no Python in browser) |
| **Paper trail** | English preprint + French [*Programmez!*](docs/ARTICLE.md) article |

Former working title: *Quantum Doom LA* → renamed to reflect the softer aesthetic.

---

## Quantum backends

Select with `QUANTUM_BACKEND`:

| Backend | Mode | Best for |
| --- | --- | --- |
| **`qip`** (default) | In-process Rust ([qip](https://crates.io/crates/qip)) | Dev, offline, **WASM** |
| **`qiskit`** | Local Python subprocess | IBM-style simulator demo |
| **`bluequbit`** | Remote REST API | Cloud runs, article benchmarks |

```bash
export QUANTUM_BACKEND=qip          # default
export QUANTUM_BACKEND=qiskit       # requires Python + Qiskit
export QUANTUM_BACKEND=bluequbit
export BLUEQUBIT_API_KEY=your_key
```

---

## Quick start

**Requirements:** Rust 1.89+, Linux/macOS/Windows.

```bash
git clone https://github.com/thepriben/quantum-town-la.git
cd quantum-town-la
cargo run -p quantum-town-la
```

Sprint 1 delivers a minimal Bevy scene (movement & combat land in upcoming sprints).

---

## Repository layout

```
quantum-town-la/
├── crates/
│   ├── game/      # Bevy binary (desktop + future wasm)
│   ├── quantum/   # Circuits, Measurement, QuantumBackend
│   └── osm/       # Overpass → level blueprint
├── assets/
├── docs/          # Design, article plan, WASM, records, web landing
└── scripts/       # Qiskit shim (Sprint 8)
```

---

## Documentation

| Doc | Description |
| --- | --- |
| [docs/DESIGN.md](docs/DESIGN.md) | Full game & engine design, sprints, ECS |
| [docs/ARTICLE.md](docs/ARTICLE.md) | arXiv preprint + *Programmez!* outline |
| [docs/WASM.md](docs/WASM.md) | Browser build plan |
| [docs/RECORDS.md](docs/RECORDS.md) | Scoring & leaderboards |
| [Live site](https://thepriben.github.io/quantum-town-la/) | Project landing (GitHub Pages) |

---

## Win condition (target)

Collect **6 Q-Shards** within **3:00**. Exit through the municipal gate.  
Score = time remaining × purity bonus (fewer forced quantum observations on Imps = better).

---

## Roadmap

- [x] **Sprint 0** — repo, docs, quantum crate skeleton
- [ ] **Sprint 1** — FPS movement, Rapier collisions, hitscan, HUD
- [ ] **Sprint 2** — OSM building extrusion (200 m LA radius)
- [ ] **Sprint 3** — QIP 2-qubit simulator wired in
- [ ] **Sprint 4** — Quantum Imp AI + debug probability panel
- [ ] **Sprint 5** — Schrödinger Gates (3 qubits, 8 exits)
- [ ] **Sprint 6** — Shards, timer, local records
- [ ] **Sprint 7** — WASM + GitHub Pages playable demo
- [ ] **Sprint 8** — Qiskit + BlueQubit backends
- [ ] **Sprint 9** — Visual polish, preprint submission

---

## License

MIT — see [LICENSE](LICENSE).

## Author

[Benoît Prieur](https://github.com/thepriben) — preprint & *Programmez!* article in progress.

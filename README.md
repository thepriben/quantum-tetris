# Quantum Tetris

<p align="center">
  <strong>🇬🇧 English</strong> · <strong>🇫🇷 <a href="README.fr.md">Français</a></strong>
</p>

<p align="center">
  <a href="https://thepriben.github.io/quantum-tetris/"><strong>▶ Play online</strong></a>
</p>

<img width="800" alt="image" src="https://github.com/user-attachments/assets/c09b7921-8ae3-41c6-b516-d941ff36349a" />

Tetris where stochastic game events (piece, spawn state, cadence, bonuses…) are driven by measured quantum circuits in the default mode. The player still controls movement with ← → ↑ ↓ and Space; the game draws everything else from circuit measurements.

> **The player:** ← → ↑ ↓ and Space · **The game:** everything else.

**Runtime.** Bevy (Rust), shipped as a desktop binary or WASM in the browser. Quantum mode defaults to **RustQIP** statevector simulation, so the same circuit presets run locally and online.

---

## Gameplay & circuits

Each stochastic moment invokes a circuit from the shared preset list (see [`docs/QUANTUM.md`](docs/QUANTUM.md)). The diagrams below are generated from the same gate definitions mirrored by [`scripts/render_circuit_diagrams.py`](scripts/render_circuit_diagrams.py).

### Active Piece Draw

| | |
|---|---|
| **In-game** | Sets the falling tetromino. |
| **Circuit** | `quantum-teleportation-gate-v1` — one teleportation-inspired Bell measurement; the 3-bit readout maps directly to the active tetromino. |

<p align="left"><img src="docs/circuits/quantum-teleportation-gate-v1.png" alt="quantum-teleportation-gate-v1" width="720"></p>

### Rotation & column

| | |
|---|---|
| **In-game** | Sets spawn orientation and entry column. |
| **Circuit** | `imp-brain-v1` — 2 measured qubits → rotation (0–3) and spawn column. |

<p align="left"><img src="docs/circuits/imp-brain-v1.png" alt="imp-brain-v1" width="720"></p>

### Drop cadence

| | |
|---|---|
| **In-game** | Interval between grid steps; decreases as level rises. |
| **Circuit** | `enemy-profile-hunter-v1` — 2 measured qubits → drop interval for the active piece. |

<p align="left"><img src="docs/circuits/enemy-profile-hunter-v1.png" alt="enemy-profile-hunter-v1" width="720"></p>

### Space — hard drop

| | |
|---|---|
| **In-game** | Instant lock; score bonus, sometimes one extra line. |
| **Circuit** | `observation-pulse-v1` — measure on hard drop; bits select the score bonus. |

<p align="left"><img src="docs/circuits/observation-pulse-v1.png" alt="observation-pulse-v1" width="720"></p>

### Line clear

| | |
|---|---|
| **In-game** | Score multiplier ×1 to ×4 from the draw. |
| **Circuit** | `q-shard-stabilizer-v1` — after a line clear; bits set the multiplier (×1–×4). |

<p align="left"><img src="docs/circuits/q-shard-stabilizer-v1.png" alt="q-shard-stabilizer-v1" width="720"></p>

---

## Quick start

**Desktop**

```bash
cp .env.example .env          # optional; QUANTUM_MODE=quantum is the default
cargo run -p quantum-tetris
```

| Mode | Command |
| --- | --- |
| Quantum — RustQIP (default) | `cargo run -p quantum-tetris` |
| Classic | `QUANTUM_MODE=classic cargo run -p quantum-tetris` |

**Browser** (needs ~3 GiB free disk for the release WASM build)

```bash
cargo install wasm-bindgen-cli   # once
./scripts/build_wasm.sh
python3 -m http.server 8080 --directory docs
# → http://localhost:8080/
```

**Controls:** ← → move · ↑ rotate · ↓ soft drop · **Space** hard drop + quantum observe (`observation-pulse-v1`).

---

## Architecture

The repo separates the **game engine** (Bevy), the **quantum layer** (circuit IR + backends), and **tooling** (diagram rendering, WASM build, GitHub Pages).

```mermaid
flowchart TB
  subgraph clients["Clients"]
    DESK["Desktop binary<br/>cargo run"]
    WASM["Browser<br/>docs/index.html"]
  end

  subgraph game["crates/game — Bevy 0.18"]
    APP["app.rs — TetrisPlugin"]
    TET["tetris.rs — loop & spawn"]
    FX["measurement_fx.rs — bits → gameplay"]
    UI["ui.rs — grid + HUD + i18n"]
    CFG["config.rs — QuantumSession"]
  end

  subgraph quantum["crates/quantum — simulation"]
    IR["circuit.rs — QuantumCircuit + Gate"]
    BE["backends/ — Classic · RustQIP"]
  end

  DESK --> APP
  WASM --> APP
  APP --> TET
  TET --> FX
  TET --> CFG
  CFG --> BE
  BE --> IR
```

**Core idea:** the game only calls `QuantumBackend::run(circuit) → Measurement`, then decodes bitstrings in `measurement_fx.rs`. Switching between Classic and RustQIP never changes the Tetris logic.

### Spawn pipeline

Three measurements run before each piece appears:

```mermaid
sequenceDiagram
  participant T as tetris.rs
  participant Q as QuantumSession
  participant M as measurement_fx.rs

  T->>Q: piece_circuit() — teleportation-inspired 3-bit draw
  Q-->>T: bits → active piece
  T->>Q: rotation_circuit() — imp-brain-v1
  Q-->>M: rotation + spawn column
  T->>Q: speed_circuit() — hunter-profile-v1
  Q-->>M: drop interval
  M-->>T: ActivePiece + HUD
```

### Backends

| Backend | Where | Mechanism |
| --- | --- | --- |
| **Classic** | everywhere | uniform `rand` — baseline without a simulator |
| **RustQIP** | desktop + browser | in-process statevector simulator |

- `QUANTUM_MODE=classic|quantum` (alias `QUANTUM_BACKEND`)
- In-game **CLASSIC** / **RUSTQIP** buttons

---

## Repository layout

```
quantum-tetris/
├── crates/
│   ├── game/              # Bevy — binary + WASM cdylib
│   └── quantum/           # Circuit IR + backends + tests
├── docs/
│   ├── index.html         # Game + mechanics guide (English default)
│   ├── circuits/*.png     # circuit diagrams
│   ├── QUANTUM.md         # Circuit & bit-mapping reference
│   └── WASM.md            # Browser build notes
├── scripts/
│   ├── build_wasm.sh
│   ├── clean_build.sh
│   └── render_circuit_diagrams.py
└── .github/workflows/
    ├── ci.yml
    └── pages.yml          # WASM + diagrams → GitHub Pages
```

---

## CI / deployment

| Workflow | Trigger | Actions |
| --- | --- | --- |
| **CI** | push / PR `main` | fmt, clippy, tests, WASM check |
| **Pages** | push `main` | Release WASM build, circuit PNGs, deploy `docs/` |

Live: [thepriben.github.io/quantum-tetris/](https://thepriben.github.io/quantum-tetris/)

---

## License

MIT — [LICENSE](LICENSE)

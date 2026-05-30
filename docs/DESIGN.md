# Design — Quantum Tetris: LA

2D neon Tetris built with **Bevy 0.18**. No 3D assets — pure UI grid + arcade HUD.

---

## Game loop

1. **Startup** — spawn UI, run first teleporter pair, drop first piece.
2. **Update** (chained systems):
   - `tick_gravity` — timed drop from hunter-profile interval
   - `handle_input` — arrows + Space
   - `refresh_ui` — grid colors + HUD text
3. **Lock** — piece lands → optional line clear (stabilizer circuit) → next teleporter pair.

---

## Modules (`crates/game/src/`)

| Module | Role |
| --- | --- |
| `app.rs` | `TetrisPlugin`, window, Bevy setup |
| `board.rs` | 10×20 grid, hidden spawn buffer, collision |
| `pieces.rs` | Tetromino shapes, colors, families |
| `tetris.rs` | Gravity, input, quantum spawn pipeline |
| `measurement_fx.rs` | Bitstring → gameplay parameters |
| `ui.rs` | Neon grid + arcade control panel |
| `config.rs` | `QUANTUM_MODE`, backend session |
| `game_state.rs` | Score, lines, HUD strings |

---

## Quantum crate (`crates/quantum/`)

Portable circuit IR (`Gate`, `QuantumCircuit`) and backends:

- **Classic** — always compiled
- **Qiskit** — feature `backend-qiskit`, subprocess to `scripts/quantum_shim.py`

Game enables Qiskit via its own `qiskit` feature (on by default for desktop).

---

## Backends removed

- **QIP** (Rust in-process simulator) — removed; use classic or Qiskit instead.
- **BlueQubit** — never shipped in this repo.

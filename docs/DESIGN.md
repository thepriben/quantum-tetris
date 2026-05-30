# Design — Quantum Tetris

2D neon Tetris built with **Bevy 0.18**. No 3D assets — pure UI grid + arcade HUD. The default runtime path is quantum: RustQIP samples the circuit statevector locally and in WASM; Classic is only a baseline mode.

---

## Game loop

1. **Startup** — spawn UI, run first teleportation-inspired pair, drop first piece.
2. **Update** (chained systems):
   - `tick_gravity` — timed drop from hunter-profile interval
   - `handle_input` — arrows + Space
   - `refresh_ui` — grid colors + HUD text
3. **Lock** — piece lands → optional line clear (stabilizer circuit) → next teleportation-inspired pair.

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
- **RustQIP** — in-process statevector (`RustQipBackend`), desktop + WASM

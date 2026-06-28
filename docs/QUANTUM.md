# Quantum layer — Quantum Tetris

The game always calls `QuantumBackend::run(circuit)`. The backend only changes *how* bitstrings are sampled; gameplay decodes the bits the same way in every mode.

Default gameplay is quantum: desktop and browser both use the RustQIP statevector backend unless the player explicitly switches to **CLASSIC**.

---

## Backends

| Backend | Select with | What it does |
| --- | --- | --- |
| **Classic** | `QUANTUM_MODE=classic` or in-game **CLASSIC** | Uniform random bits — baseline without a simulator |
| **Quantum (RustQIP)** | default, `QUANTUM_MODE=quantum`, or in-game **RUSTQIP** | In-process statevector via [RustQIP](https://github.com/Renmusxd/RustQIP) — desktop and browser |

```bash
# .env or shell
QUANTUM_MODE=quantum    # RustQIP (default)
QUANTUM_MODE=classic   # uniform baseline

# alias
QUANTUM_BACKEND=classic|quantum
```

**Classic** — every bitstring has equal weight; useful to compare against quantum sampling.

**RustQIP** — same gate list everywhere; exact probabilities come from the in-process statevector backend.

---

## Circuits → gameplay

| Label | Qubits | When | Game effect |
| --- | --- | --- | --- |
| `quantum-teleportation-gate-v1` | 3 | Each spawn | One 3-bit readout chooses the current tetromino |
| `imp-brain-v1` | 2 | Each spawn | Rotation 0–3 + spawn column |
| `enemy-profile-hunter-v1` | 2 | Each spawn | Gravity interval (seconds) |
| `observation-pulse-v1` | 2 | Space | Hard-drop bonus |
| `q-shard-stabilizer-v1` | 2 | Line clear | Score multiplier |

### Teleporter Piece Draw

`quantum-teleportation-gate-v1` exposes the measured 3-bit readout as gameplay state. It is teleportation-inspired rather than a hidden correction protocol: the game intentionally reads the correction/message bits instead of applying classical feed-forward and discarding them.

| Bits | Piece |
| --- | --- |
| `000` | I |
| `001` | O |
| `010` | T |
| `011` | S |
| `100` | Z |
| `101` | J |
| `110` | L |
| `111` | T |

> **Fairness note (C4).** Both `010` and `111` map to T, so under uniform 3-bit draws P(T)=¼ while every other piece is at ⅛. The audit journal records `mapping_policy: teleport-v1` so this bias is explicit and checkable. See [AUDIT.md](AUDIT.md).

### Bit mappings (2-qubit circuits)

**imp-brain** — rotation + column:

| Bits | Rotation | Spawn X |
| --- | --- | --- |
| `00` | 0 | 3 |
| `01` | 1 | 5 |
| `10` | 2 | 2 |
| `11` | 3 | 4 |

**observation-pulse** (Space):

| Bits | Effect |
| --- | --- |
| `00` | +50 score |
| `01` | +120 score |
| `10` | +80 score + 1 line echo |
| `11` | +200 score |

**shard-stabilizer** (line clear): multiplier ×1 / ×2 / ×3 / ×4 for `00`…`11`.

---

## Crate API

```rust
use quantum_tetris_quantum::{build_backend, BackendKind, QuantumCircuit, QuantumBackend};

let mut backend = build_backend(BackendKind::Classic).unwrap();
let m = backend.run(&QuantumCircuit::teleporter()).unwrap();
println!("{}", m.bits); // e.g. "101"
```

---

## WASM note

The browser build uses **RustQIP quantum mode** by default. See [WASM.md](WASM.md).

Audit journals are printed to the browser console on game over (no file export). For file export and offline verification, use the desktop build — see [AUDIT.md](AUDIT.md).

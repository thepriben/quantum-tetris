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
| `quantum-teleportation-gate-v1` | 3 | Each spawn (×2) | Paired shots choose the current piece. Bell-measurement bits (q0,q1) → family; the two message/receiver bits (q2) disambiguate variants |
| `imp-brain-v1` | 2 | Each spawn | Rotation 0–3 + spawn column |
| `enemy-profile-hunter-v1` | 2 | Each spawn | Gravity interval (seconds) |
| `observation-pulse-v1` | 2 | Space | Hard-drop bonus |
| `q-shard-stabilizer-v1` | 2 | Line clear | Score multiplier |

### Teleporter families (Bell measurement)

`quantum-teleportation-gate-v1` exposes the sender-side Bell measurement bits as gameplay state. It is teleportation-inspired rather than a hidden correction protocol: the game intentionally reads the correction/message bits instead of applying classical feed-forward and discarding them.

| Bell | Family | Pieces |
| --- | --- | --- |
| `00` | Line | I |
| `01` | Block | O |
| `10` | Fork | T |
| `11` | Corner | J, L, S, Z (message bits from current + next shot) |

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

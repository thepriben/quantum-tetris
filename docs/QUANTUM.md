# Quantum layer — Quantum Tetris

The game always calls `QuantumBackend::run(circuit)`. The backend only changes *how* bitstrings are sampled; gameplay decodes the bits the same way in every mode.

---

## Backends

| Backend | Select with | What it does |
| --- | --- | --- |
| **Classic** | `QUANTUM_MODE=classic` or in-game **CLASSIC** | Uniform random bits — baseline without a simulator |
| **Quantum (RustQIP)** | default, `QUANTUM_MODE=quantum`, or in-game **RUSTQIP** | In-process statevector via [RustQIP](https://github.com/Renmusxd/RustQIP) — desktop and browser |
| **Qiskit Aer** | `QUANTUM_MODE=qiskit` or in-game **QISKIT** (desktop only) | Aer via Python `scripts/quantum_shim.py` — development and CI parity |

```bash
# .env or shell
QUANTUM_MODE=quantum    # RustQIP (default)
QUANTUM_MODE=classic   # uniform baseline
QUANTUM_MODE=qiskit    # Aer (desktop only, needs Python)

# alias
QUANTUM_BACKEND=classic|quantum|qiskit
```

**Classic** — every bitstring has equal weight; useful to compare against quantum sampling.

**RustQIP** — same gate list everywhere; probabilities checked against Qiskit in CI (`rustqip_qiskit_parity`).

**Qiskit Aer** — optional on desktop; not available in the browser (no Python runtime).

---

## Circuits → gameplay

| Label | Qubits | When | Game effect |
| --- | --- | --- | --- |
| `quantum-teleportation-gate-v1` | 3 | Each spawn (×2) | **Shot 1**: current piece. **Shot 2**: next piece. Bell bits (q0,q1) → family; message qubit (q2) → variant |
| `imp-brain-v1` | 2 | Each spawn | Rotation 0–3 + spawn column |
| `enemy-profile-hunter-v1` | 2 | Each spawn | Gravity interval (seconds) |
| `observation-pulse-v1` | 2 | Space | Hard-drop bonus |
| `q-shard-stabilizer-v1` | 2 | Line clear | Score multiplier |

### Teleporter families (Bell measurement)

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

## Qiskit setup (optional)

```bash
python -m pip install -r scripts/requirements.txt
QUANTUM_MODE=qiskit cargo run -p quantum-tetris
```

CI runs Qiskit integration tests separately (`cargo test -p quantum-tetris-quantum --features backend-qiskit`).

Environment:

| Variable | Purpose |
| --- | --- |
| `QUANTUM_PYTHON` | Python executable (default `python3`) |
| `QUANTUM_PYTHON_SHIM` | Path to `quantum_shim.py` |
| `QUANTUM_SHOTS` | Aer shots (default 1024) |

---

## WASM note

The browser build uses **classic only** — no Python in the tab. See [WASM.md](WASM.md).

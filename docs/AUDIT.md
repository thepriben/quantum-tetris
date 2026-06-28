# Audit layer (C5 + C6)

Quantum Tetris records every stochastic gameplay draw in an **append-only audit journal** with **hash-chained receipts** (layers C5 and C6 of the randomness stack).

## What gets recorded

Each call to the quantum backend during gameplay appends one entry:

| Field | Meaning |
| --- | --- |
| `seq` | Monotonic draw index in the session |
| `moment` | Gameplay context (`spawn_piece`, `spawn_rotation`, `observe`, …) |
| `circuit` | Circuit label (e.g. `quantum-teleportation-gate-v1`) |
| `bits` | Measured bitstring |
| `effect` | Decoded gameplay outcome (e.g. `piece=T`, `rot=2`) |
| `backend_used` | Backend that produced the draw |
| `entry_hash` | SHA-256 chain link (C5 receipt) |

The journal also stores:

- **`seed_commitment`** — `SHA-256(session_seed)`, published before the first draw (commit phase)
- **`seed_revealed`** — the seed itself, written when the session ends (reveal phase)
- **`mapping_policy`** — currently `teleport-v1` (3-bit → 7 pieces, documented T bias)
- **`distribution_policy`** — currently `raw-3bit` (uniform over eight bitstrings)

## When the journal is finalized

- **Game over** — the journal is revealed, verified, and exported.
- **Desktop** — written to `audit/{session_id}.json` (directory git-ignored).
- **Browser (WASM)** — pretty-printed to the browser console.

Press **Space** after game over to start a new session (a fresh journal begins automatically).

## Offline verification

```bash
cargo run -p quantum-tetris-quantum --example verify_audit -- audit/qt-1234567890.json
```

The tool checks:

1. Hash chain integrity (tampering any entry breaks verification)
2. Seed reveal matches the pre-published commitment

## API (Rust)

```rust
use quantum_tetris_quantum::{AuditJournal, QuantumCircuit, BackendKind};

let mut journal = AuditJournal::with_seed("demo", BackendKind::Classic.label(), "424242");
// … record_draw for each circuit …
journal.reveal_seed();
journal.verify()?; // Ok(()) if intact
```

In the game crate, `QuantumSession::run_draw` + `audit_draw` wrap the backend and journal.

## Mapping bias (layer C4)

Under uniform 3-bit draws, piece **T** appears twice as often as any other piece because `111` maps to T. See `crates/quantum/tests/piece_bias.rs` for an empirical check on both Classic and RustQIP backends.

## Related reading

- [`docs/QUANTUM.md`](QUANTUM.md) — circuit definitions and bit mappings

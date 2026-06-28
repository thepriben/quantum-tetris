# Auditable randomness (layers C5 + C6)

Quantum Tetris records every stochastic gameplay draw in an **append-only audit journal** with **hash-chained receipts**. This is the reference implementation pinned by the [`preprint-randomness`](https://github.com/thepriben/quantum-tetris/releases/tag/preprint-randomness) release.

Good entropy alone does not guarantee competitive fairness: you also need explicit **distribution policy**, **mapping policy**, **cryptographic receipts**, and an **audit trail**. This repo implements the last two layers in production gameplay code.

---

## Six-layer stack (context)

| Layer | Role in Quantum Tetris | Where |
| --- | --- | --- |
| **C1 — Entropy source** | RustQIP statevector simulator or Classic uniform RNG | [`docs/QUANTUM.md`](QUANTUM.md) |
| **C2 — Extraction** | Measured bitstring from circuit run | `QuantumBackend::run` |
| **C3 — Distribution policy** | `raw-3bit` — eight bitstrings used as sampled | journal field `distribution_policy` |
| **C4 — Mapping policy** | `teleport-v1` — 3-bit → seven tetromino labels (T bias) | [`mapping.rs`](../crates/quantum/src/mapping.rs), [`QUANTUM.md`](QUANTUM.md) |
| **C5 — Cryptographic receipt** | SHA-256 hash chain per draw | [`audit.rs`](../crates/quantum/src/audit.rs) |
| **C6 — Audit journal** | Commit-reveal session seed + append-only JSON log | [`audit.rs`](../crates/quantum/src/audit.rs), [`audit_io.rs`](../crates/game/src/audit_io.rs) |

Layers C1–C4 define *what randomness means in the game*. Layers C5–C6 make every draw **verifiable after the fact** without trusting the client binary alone.

---

## Session lifecycle

```
New game
  └─ AuditJournal::new(session_id, backend)
       └─ seed_commitment = SHA-256(session_seed)   ← commit (before first draw)

Each stochastic moment (spawn, observe, line clear, …)
  └─ backend.run(circuit)
  └─ journal.record_draw(…)                        ← C5 receipt + C6 append

Game over
  └─ journal.reveal_seed()                           ← reveal
  └─ journal.verify()
  └─ export audit/{session_id}.json                ← desktop only

Space (after game over)
  └─ new session → fresh journal
```

**Desktop** writes `audit/{session_id}.json` (directory is git-ignored).  
**Browser (WASM)** prints the same JSON to the developer console on game over.

---

## What gets recorded

Each quantum backend call during gameplay appends one entry:

| Field | Meaning |
| --- | --- |
| `seq` | Monotonic draw index in the session |
| `moment` | Gameplay context (`spawn_piece`, `spawn_rotation`, `spawn_gravity`, `observe`, `line_clear`, …) |
| `circuit` | Circuit label (e.g. `quantum-teleportation-gate-v1`) |
| `bits` | Measured bitstring |
| `effect` | Decoded gameplay outcome (e.g. `piece=T`, `rot=2`) |
| `backend_used` | Backend that produced the draw (`classic`, `rustqip`, …) |
| `entry_hash` | SHA-256 chain link (C5 receipt) |

Journal-level metadata:

| Field | Meaning |
| --- | --- |
| `version` | Journal format version (currently `1`) |
| `session_id` | Unique session identifier (e.g. `qt-1730123456789`) |
| `backend` | Default backend label for the session |
| `mapping_policy` | `teleport-v1` |
| `distribution_policy` | `raw-3bit` |
| `seed_commitment` | `SHA-256(session_seed)` — fixed before the first draw |
| `seed_revealed` | Session seed, written when the session ends |
| `chain_head` | Latest hash after the last entry |

The internal session seed is **never** serialized until reveal.

---

## Hash chain (C5)

1. **Genesis:**  
   `SHA-256("genesis|{session_id}|{seed_commitment}|{backend}|{mapping_policy}|{distribution_policy}")`

2. **Each entry:**  
   `SHA-256("entry|{chain_head}|{seq}|{moment}|{circuit}|{bits}|{effect_or_-}|{backend_used}")`

3. **`chain_head`** advances to the latest `entry_hash`. Tampering with any field breaks verification at that sequence number.

4. **Seed reveal:** after game over, `SHA-256(seed_revealed)` must equal `seed_commitment`.

---

## Example journal (truncated)

```json
{
  "version": 1,
  "session_id": "qt-1730123456789",
  "backend": "rustqip",
  "mapping_policy": "teleport-v1",
  "distribution_policy": "raw-3bit",
  "seed_commitment": "a1b2c3…",
  "seed_revealed": "4242424242",
  "entries": [
    {
      "seq": 1,
      "moment": "spawn_piece",
      "circuit": "quantum-teleportation-gate-v1",
      "bits": "010",
      "effect": "piece=T",
      "backend_used": "rustqip",
      "entry_hash": "d4e5f6…"
    }
  ],
  "chain_head": "d4e5f6…"
}
```

---

## Offline verification

After a desktop session, verify the exported file:

```bash
./scripts/verify_audit.sh audit/qt-1730123456789.json
```

Equivalent:

```bash
cargo run -p quantum-tetris-quantum --example verify_audit -- audit/qt-1730123456789.json
```

The verifier checks:

1. Hash chain integrity (any tampered entry fails)
2. Revealed seed matches the pre-published commitment

Exit code `0` means the journal is intact.

---

## Rust API

```rust
use quantum_tetris_quantum::{AuditJournal, QuantumCircuit, BackendKind};

let mut journal = AuditJournal::with_seed("demo", BackendKind::Classic.label(), "424242");
journal.record_draw(
    &QuantumCircuit::teleporter(),
    &measurement,
    "spawn_piece",
    Some("piece=T"),
    "classic",
);
journal.reveal_seed();
journal.verify()?; // Ok(()) if intact
```

In the game crate, `QuantumSession` wraps the backend and journal:

- `run_draw` / `audit_draw` — run circuit + append receipt
- `finalize_audit` — reveal, verify, return journal (called on game over)
- `seed_commitment()` — publish commitment before play (for future spectator tooling)

Integration lives in [`crates/game/src/config.rs`](../crates/game/src/config.rs) and [`crates/game/src/tetris.rs`](../crates/game/src/tetris.rs).

---

## Mapping bias (layer C4)

Under uniform 3-bit draws, piece **T** appears twice as often as any other piece because both `010` and `111` map to T:

| Bits | Piece |
| --- | --- |
| `010` | T |
| `111` | T |

Analytical probability: P(T) = ¼, P(each other piece) = ⅛.  
See [`crates/quantum/tests/piece_bias.rs`](../crates/quantum/tests/piece_bias.rs) for an empirical check on Classic and RustQIP backends.

---

## Tests

| Test | File | What it checks |
| --- | --- | --- |
| Hash chain + seed reveal | `crates/quantum/src/audit.rs` (unit) | Commitment, tamper detection, JSON omit internal seed |
| Session export round-trip | `crates/quantum/tests/audit_integration.rs` | Record draws → JSON → reload → verify |
| T-piece bias | `crates/quantum/tests/piece_bias.rs` | Empirical frequencies vs analytical C4 |
| Spawn audit count | `crates/game/src/tetris.rs` (unit) | Three journal entries per piece spawn |

Run the full suite:

```bash
cargo test
```

---

## Source files

| Path | Role |
| --- | --- |
| `crates/quantum/src/audit.rs` | Journal, hash chain, commit-reveal |
| `crates/quantum/src/mapping.rs` | C4 teleport mapping + probabilities |
| `crates/game/src/config.rs` | `QuantumSession` integration |
| `crates/game/src/tetris.rs` | Gameplay moments → `audit_draw` |
| `crates/game/src/audit_io.rs` | Desktop export to `audit/` |
| `crates/quantum/examples/verify_audit.rs` | CLI verifier |
| `scripts/verify_audit.sh` | Shell wrapper |

---

## Related reading

- [`docs/QUANTUM.md`](QUANTUM.md) — circuits, backends, bit mappings
- [Releases](https://github.com/thepriben/quantum-tetris/releases) — `programmez-hs23` (magazine article code) · `preprint-randomness` (C5/C6 audit layer)

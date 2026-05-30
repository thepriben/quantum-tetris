# Quantum Town: LA — Design Document

**Display name:** Quantum Town: LA  
**Repository / crate:** `quantum-town-la`  
**Former name:** Quantum Doom LA (retired — aesthetic is cute, not violent)

---

## 1. Vision

A **weekend-sized** Rust game with a **genuine quantum demonstration**: gameplay systems call a real circuit simulator through a stable Rust API.

### Aesthetic

- **Cute first-person explorer** — round avatar, pastel palette, Animal Crossing-inspired tone.
- **Light FPS structure** — Wolfenstein-like clarity without gore; stun effects instead of blood.
- **Real-world map** — ~200 m radius around a Los Angeles GPS anchor, built from OSM buildings and highways.

### Research hook

> *What could a quantum computer actually do in a video game?*

Two visible, teachable answers:

1. **Quantum Imp AI** — behavior sampled from a **2-qubit** circuit.
2. **Schrödinger Gates** — teleporters with **probabilistic destinations** (3 qubits → 8 exits).

Few qubits runs on any laptop and scales to Qiskit, IBM Quantum, or **BlueQubit** later.

---

## 2. Win condition

**Standard mode — Stabilization**

- Collect **6 Q-Shards** before the **3:00** timer expires.
- Leave through the **Municipal Gate**.
- **Score** = `time_left_ms × purity_multiplier`
- `purity_multiplier = 1.0 + 0.1 × (6 - forced_observations)`
- Forced observation = player clicks "Observe" on an Imp, collapsing its behavior early (pedagogical, costs score).

**Daily seed mode**

- Seed derived from UTC date + map anchor for comparable runs and leaderboards.

---

## 3. Core mechanics

### 3.1 Player (Walker)

- WASD + mouse look.
- **Tuning fork** — short-range hitscan; stuns Imps (cartoon stars, no gore).
- **Observe** — forces a measurement on a targeted Imp (debug + score trade-off).
- **Schrödinger tubes** — enter a pipe; backend picks exit portal.

### 3.2 Quantum Imp

*Lore:* "Its mood does not exist until you stare too hard."

| Measurement | Behavior |
| --- | --- |
| `00` | Direct charge |
| `01` | Flank via alleys |
| `10` | Flee toward another shard |
| `11` | Ambush behind OSM wall |

**Circuit:** `H(q0); H(q1); CX(q0,q1); measure`  
**Tick:** `quantum_step()` every **1 s** per active Imp.

### 3.3 Schrödinger Gate

*Lore:* "The destination does not exist until you step inside."

- **3 qubits** → **8 labeled exits** (A-H).
- Base circuit: triple `H`, then measure.
- Advanced: `Ry` rotations for biased probabilities (e.g. 70% / 20% / 10%).

Debug UI shows outcome histogram (egui panel).

---

## 4. Quantum backends

```rust
pub trait QuantumBackend: Send {
    fn run(&mut self, circuit: &QuantumCircuit) -> Result<Measurement, QuantumError>;
}
```

| ID | Implementation | Runtime |
| --- | --- | --- |
| `qip` | [qip](https://crates.io/crates/qip) crate | In-process Rust |
| `qiskit` | Python Qiskit via JSON stdin/stdout | Local subprocess |
| `bluequbit` | HTTPS REST | Remote cloud |

```bash
QUANTUM_BACKEND=qip|qiskit|bluequbit
BLUEQUBIT_API_KEY=...   # when using bluequbit
QISKIT_PYTHON=python3   # optional override
```

Gameplay depends only on `QuantumCircuit` + `Measurement`.

---

## 5. Records

See [RECORDS.md](RECORDS.md).

---

## 6. Web (WASM)

See [WASM.md](WASM.md). Demo: https://thepriben.github.io/quantum-town-la/

---

## 7. OSM level generation (Sprint 2)

Overpass API: `building=*`, `highway=*` within **200 m** of LA anchor `(34.0522, -118.2437)`.

---

## 8. ECS architecture (Bevy)

```
Player — Health, Inventory (shards), Stats
QuantumImp — QuantumBrain, BehaviorState
SchrödingerGate — QuantumTeleport, PortalVisual
Plugins: bevy, bevy_rapier3d, bevy_egui
```

---

## 9. Sprint plan

| Sprint | Deliverable |
| --- | --- |
| 1 | FPS movement, Rapier, hitscan, HUD |
| 2 | OSM building extrusion |
| 3 | QIP backend, 2-qubit circuits |
| 4 | Imp AI + probability panel |
| 5 | 3-qubit teleporters |
| 6 | Shards, timer, local records |
| 7 | WASM + GitHub Pages |
| 8 | Qiskit + BlueQubit live |
| 9 | Art pass, arXiv + Programmez! |

---

## 10. Lore

| Entity | Name | Tagline |
| --- | --- | --- |
| Hero | Walker | Lost quantum tourist in LA |
| Foe | Quantum Imp | Superposition of bad moods |
| Portal | Schrödinger Gate | The exit picks you |
| Item | Q-Shard | Stable measurement crystal |

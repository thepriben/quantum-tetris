# Article plan — arXiv (EN) + Programmez! (FR)

## Working title (English)

**Quantum Town: A Pedagogical FPS Where Gameplay Mechanics Are Driven by Simulated Quantum Circuits**

## French title (*Programmez!*)

**Quantum Town : la physique quantique au service d'un mini-FPS mignon**

---

## Abstract (English preprint)

We present *Quantum Town: LA*, a minimal first-person game in Rust (Bevy) where enemy behavior and probabilistic teleporters are driven by small quantum circuits (2-3 qubits). The game uses a pluggable `QuantumBackend` invoked on each decision step. We ship three backends: embedded Rust (QIP), local Qiskit via subprocess, and BlueQubit cloud API. A WebAssembly build enables browser demos. We show that a handful of qubits suffices for visible, teachable mechanics and release the project as open source.

**Keywords:** quantum computing, game development, Rust, Bevy, pedagogy, Qiskit, BlueQubit

---

## French summary (*Programmez!*)

*Quantum Town: LA* est un mini-FPS au rendu mignon ou deux mecaniques — l'IA des Quantum Imps et les teleporteurs Schrödinger Gate — sont calculees par des circuits de 2 a 3 qubits. Le moteur Rust (Bevy) appelle un backend interchangeable : QIP embarque, Qiskit local ou BlueQubit distant. Version WASM pour jouer dans le navigateur.

---

## Outline

1. Introduction — quantum games; gap between large demos and teachable prototypes
2. Related work — Quandoom, quantum chess, OSM in games
3. Game design — cute aesthetic, Q-Shards win condition, records
4. Quantum in gameplay — 2-qubit Imp circuit; 3-qubit teleporter
5. Implementation — Bevy ECS, OSM pipeline, backends, WASM
6. Evaluation — latency per backend, debug panel screenshots
7. Conclusion — few qubits, visible fun, open source
8. Appendix — circuits, sample measurement JSON

---

## Figures

1. Cute avatar + OSM block
2. Debug panel: |00> 25% ...
3. Architecture: Game → QuantumBackend → {QIP, Qiskit, BlueQubit}
4. Circuit diagrams
5. WASM demo screenshot

---

## Links for paper

- Code: https://github.com/thepriben/quantum-town-la
- Demo: https://thepriben.github.io/quantum-town-la/

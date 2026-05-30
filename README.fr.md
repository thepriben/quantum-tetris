# Quantum Tetris

<p align="center">
  <strong>🇬🇧 <a href="README.md">English</a></strong> · <strong>🇫🇷 Français</strong>
</p>

<p align="center">
  <a href="https://thepriben.github.io/quantum-tetris/"><strong>▶ Jouer en ligne</strong></a>
</p>

Tetris où les tirages aléatoires (pièce, vitesse, bonus…) ne passent pas par un PRNG classique : le moteur exécute des circuits quantiques prédéfinis et lit les bits mesurés. Seuls les déplacements ← → ↑ ↓ et la pose forcée (Espace) échappent à cette logique.

> **Le joueur :** ← → ↑ ↓ et Espace · **Le jeu :** tout le reste.

**Exécution.** Bevy (Rust), en binaire desktop ou WASM dans le navigateur. Mode quantum par défaut : simulateur statevector **RustQIP**, identique en local et en ligne. **Qiskit Aer** (Python) reste disponible sur desktop pour comparaison et CI.

---

## Gameplay & circuits

À chaque moment aléatoire, le moteur invoque un circuit de la liste partagée (voir [`docs/QUANTUM.md`](docs/QUANTUM.md)). Les diagrammes reprennent exactement ces circuits ([`scripts/render_circuit_diagrams.py`](scripts/render_circuit_diagrams.py)).

### Pièce en cours & « suiv. »

| | |
|---|---|
| **En jeu** | Détermine la forme active et l’aperçu **suivant**. |
| **Circuit** | `quantum-teleportation-gate-v1` (×2) — paire intriquée ; les bits mesurés fixent la famille (I, O, T…) et la variante. |

<p align="left"><img src="docs/circuits/quantum-teleportation-gate-v1.png" alt="quantum-teleportation-gate-v1" width="480"></p>

### Rotation & colonne

| | |
|---|---|
| **En jeu** | Fixe l’orientation à l’apparition et la colonne d’entrée. |
| **Circuit** | `imp-brain-v1` — 2 qubits mesurés → rotation (0–3) et colonne d'apparition. |

<p align="left"><img src="docs/circuits/imp-brain-v1.png" alt="imp-brain-v1" width="480"></p>

### Cadence de chute

| | |
|---|---|
| **En jeu** | Intervalle entre deux descentes ; diminue avec le niveau. |
| **Circuit** | `enemy-profile-hunter-v1` — 2 qubits mesurés → intervalle de chute pour la pièce en cours. |

<p align="left"><img src="docs/circuits/enemy-profile-hunter-v1.png" alt="enemy-profile-hunter-v1" width="480"></p>

### Espace — chute forcée

| | |
|---|---|
| **En jeu** | Pose immédiate ; bonus de score, parfois une ligne supplémentaire. |
| **Circuit** | `observation-pulse-v1` — mesure à la pose forcée ; les bits choisissent le bonus. |

<p align="left"><img src="docs/circuits/observation-pulse-v1.png" alt="observation-pulse-v1" width="480"></p>

### Ligne complétée

| | |
|---|---|
| **En jeu** | Multiplicateur de points ×1 à ×4 selon le tirage. |
| **Circuit** | `q-shard-stabilizer-v1` — après effacement d'une ligne ; les bits fixent le multiplicateur (×1–×4). |

<p align="left"><img src="docs/circuits/q-shard-stabilizer-v1.png" alt="q-shard-stabilizer-v1" width="480"></p>

---

## Démarrage rapide

**Desktop**

```bash
cp .env.example .env          # QUANTUM_MODE=quantum (RustQIP) par défaut
cargo run -p quantum-tetris
```

| Mode | Commande |
| --- | --- |
| Quantum — RustQIP (défaut) | `cargo run -p quantum-tetris` |
| Classique | `QUANTUM_MODE=classic cargo run -p quantum-tetris` |
| Quantum — Qiskit (desktop uniquement) | `pip install -r scripts/requirements.txt` puis `QUANTUM_MODE=qiskit cargo run -p quantum-tetris` |

**Navigateur** (nécessite ~3 Go d’espace libre pour le build WASM release)

```bash
cargo install wasm-bindgen-cli   # une fois
./scripts/build_wasm.sh
python3 -m http.server 8080 --directory docs
# → http://localhost:8080/
```

Si le linker échoue avec `errno=28`, libérez de l’espace disque ou lancez `./scripts/clean_build.sh`. Le bundle WASM fait ~70 Mo — le premier chargement peut prendre une minute.

**Contrôles :** ← → déplacer · ↑ rotation · ↓ chute douce · **Espace** chute forcée + observe (`observation-pulse-v1`).

---

## Architecture

Le dépôt sépare le **moteur de jeu** (Bevy), la **couche quantique** (IR + backends) et **l’outillage** (Python Qiskit, build WASM, Pages).

```mermaid
flowchart TB
  subgraph clients["Clients"]
    DESK["Binaire desktop<br/>cargo run"]
    WASM["Navigateur<br/>docs/index.html"]
  end

  subgraph game["crates/game — Bevy 0.18"]
    APP["app.rs — TetrisPlugin"]
    TET["tetris.rs — boucle & spawn"]
    FX["measurement_fx.rs — bits → gameplay"]
    UI["ui.rs — grille + HUD + i18n"]
    CFG["config.rs — QuantumSession"]
  end

  subgraph quantum["crates/quantum — simulation"]
    IR["circuit.rs — QuantumCircuit + Gate"]
    BE["backends/ — Classic · RustQIP · Qiskit"]
    SHIM["python_shim.rs ↔ quantum_shim.py"]
  end

  DESK --> APP
  WASM --> APP
  APP --> TET
  TET --> FX
  TET --> CFG
  CFG --> BE
  BE --> IR
  BE -.->|desktop| SHIM
  SHIM -.->|subprocess JSON| PY["scripts/quantum_shim.py<br/>Qiskit Aer"]
```

**Principe :** le jeu n’appelle que `QuantumBackend::run(circuit) → Measurement`, puis décode les bitstrings via `measurement_fx.rs`. Changer de backend ne modifie pas la logique Tetris.

### Pipeline au spawn

Quatre mesures s’enchaînent avant chaque pièce :

```mermaid
sequenceDiagram
  participant T as tetris.rs
  participant Q as QuantumSession
  participant M as measurement_fx.rs

  T->>Q: piece_circuit() — téléportation #1
  Q-->>T: bits → pièce en cours
  T->>Q: piece_circuit() — téléportation #2
  Q-->>T: bits → pièce suivante (preview)
  T->>Q: rotation_circuit() — imp-brain-v1
  Q-->>M: rotation + colonne spawn
  T->>Q: speed_circuit() — hunter-profile-v1
  Q-->>M: intervalle de chute
  M-->>T: ActivePiece + HUD
```

### Backends

| Backend | Où | Mécanisme |
| --- | --- | --- |
| **Classic** | partout | `rand` uniforme — baseline sans simulateur |
| **RustQIP** | desktop + navigateur | simulateur statevector in-process |
| **Qiskit Aer** | desktop uniquement (+ CI) | subprocess Python → `quantum_shim.py` |

- `QUANTUM_MODE=classic|quantum|qiskit` (alias `QUANTUM_BACKEND`)
- Bascule **CLASSIQUE** / **RUSTQIP** in-game (navigateur) ; **QISKIT** aussi sur desktop
- Parité RustQIP ↔ Qiskit en CI (`rustqip_qiskit_parity`)

---

## Arborescence

```
quantum-tetris/
├── crates/
│   ├── game/              # Bevy — binaire + cdylib WASM
│   └── quantum/           # IR circuits + backends + tests
├── docs/
│   ├── index.html         # Jeu + guide (anglais par défaut)
│   ├── circuits/*.png     # Diagrammes Qiskit
│   ├── QUANTUM.md         # Référence circuits & bits
│   └── WASM.md            # Notes build navigateur
├── scripts/
│   ├── build_wasm.sh
│   ├── clean_build.sh
│   ├── quantum_shim.py
│   └── render_circuit_diagrams.py
└── .github/workflows/
    ├── ci.yml
    └── pages.yml          # WASM + diagrammes → GitHub Pages
```

---

## CI / déploiement

| Workflow | Déclencheur | Actions |
| --- | --- | --- |
| **CI** | push / PR `main` | fmt, clippy, tests, check WASM, parité Qiskit |
| **Pages** | push `main` | Build WASM release, PNG circuits, deploy `docs/` |

En ligne : [thepriben.github.io/quantum-tetris/](https://thepriben.github.io/quantum-tetris/)

---

## Licence

MIT — [LICENSE](LICENSE)

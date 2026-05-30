# Design notes — Quantum Sub: LA

Jeu **local**, **2 touches** (flèches + Espace), session ~2 min.

## Boucle

1. Piloter contre le **courant** (mesure `imp-brain` toutes les 2 s).
2. **Espace** sur les cellules → stabilizer ; ailleurs → warp ou observation.
3. Éviter 4 **mines** (profils hunter/patrol mesurés).
4. Portail **sud** (−Z) avec 3/3 énergie.

## Fichiers Bevy (`crates/game/src/`)

| Fichier | Rôle |
| --- | --- |
| `app.rs` | `DrivePlugin`, startup |
| `arena.rs` | Sol, rochers GLB, balises, cellules, portail, mines |
| `vehicle.rs` | Sous-marin procédural + caméra |
| `quantum_drive.rs` | Timers, `try_space_action`, mines |
| `measurement_fx.rs` | Bits → gameplay |
| `ui.rs` | HUD graphique |
| `input.rs` | Flèches + Espace |

Quantique : [QUANTUM.md](QUANTUM.md).

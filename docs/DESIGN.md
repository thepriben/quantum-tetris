# Design — Quantum Tetris: LA

10×20 stacker, **2D neon UI**, arrows + Space.

## Loop

1. **Teleporter** circuit → next piece (I–L).
2. **Imp-brain** → rotation + spawn column.
3. Player moves / rotates / soft-drops.
4. **Space** → hard drop + **observation-pulse** bonus.
5. Line clear → **shard-stabilizer** score multiplier.

## Modules

| File | Role |
| --- | --- |
| `board.rs` | Grid, collision, line clear |
| `pieces.rs` | Tetromino shapes |
| `tetris.rs` | Systems + quantum spawn |
| `measurement_fx.rs` | Bits → gameplay |
| `ui.rs` | Grid + HUD |

Quantum details → [QUANTUM.md](QUANTUM.md).

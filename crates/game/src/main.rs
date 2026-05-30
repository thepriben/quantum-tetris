//! Native desktop binary for Quantum Tetris.

use quantum_tetris::{run_game, GameConfig};

fn main() {
    #[cfg(feature = "desktop")]
    if let Ok(path) = dotenvy::dotenv() {
        eprintln!("Loaded .env from {}", path.display());
    }

    run_game(GameConfig::desktop());
}

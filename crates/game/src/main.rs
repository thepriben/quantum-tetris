//! Native desktop binary for Quantum Sub: LA.

use quantum_town_la::{run_game, GameConfig};

fn main() {
    #[cfg(feature = "desktop")]
    if let Ok(path) = dotenvy::dotenv() {
        eprintln!("Loaded .env from {}", path.display());
    }

    run_game(GameConfig::desktop());
}

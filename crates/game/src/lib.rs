//! **Quantum Tetris: LA** — circuit-driven piece picker, classic or QIP.

pub mod app;
pub mod board;
pub mod config;
pub mod game_state;
pub mod input;
pub mod measurement_fx;
pub mod pieces;
pub mod tetris;
pub mod ui;

pub use app::{build_app, run_game, DrivePlugin, TetrisPlugin};
pub use config::{GameConfig, GamePlatform, QuantumSession};

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use wasm_bindgen::prelude::*;

#[cfg_attr(all(feature = "wasm", target_arch = "wasm32"), wasm_bindgen)]
#[cfg(feature = "wasm")]
pub fn run_wasm() {
    run_game(GameConfig::wasm(quantum_town_quantum::BackendKind::Classic));
}

#[cfg_attr(all(feature = "wasm", target_arch = "wasm32"), wasm_bindgen)]
#[cfg(feature = "wasm")]
pub fn run_wasm_quantum() {
    run_game(GameConfig::wasm(quantum_town_quantum::BackendKind::Qip));
}

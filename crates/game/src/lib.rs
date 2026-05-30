//! **Quantum Sub: LA** — underwater driving, classic or QIP (desktop + WASM).

pub mod app;
pub mod arena;
pub mod config;
pub mod game_state;
pub mod input;
pub mod measurement_fx;
pub mod quantum_drive;
pub mod ui;
pub mod vehicle;
pub mod visuals;

pub use app::{build_app, run_game, DrivePlugin};
pub use config::{GameConfig, GamePlatform, QuantumSession};

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use wasm_bindgen::prelude::*;

/// Browser entry — classic (uniform outcomes), same circuits.
#[cfg_attr(all(feature = "wasm", target_arch = "wasm32"), wasm_bindgen)]
#[cfg(feature = "wasm")]
pub fn run_wasm() {
    run_game(GameConfig::wasm(quantum_town_quantum::BackendKind::Classic));
}

/// Browser entry — in-process QIP simulator.
#[cfg_attr(all(feature = "wasm", target_arch = "wasm32"), wasm_bindgen)]
#[cfg(feature = "wasm")]
pub fn run_wasm_quantum() {
    run_game(GameConfig::wasm(quantum_town_quantum::BackendKind::Qip));
}

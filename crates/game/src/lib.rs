//! **Quantum Tetris** — circuit-driven Tetris with classic and RustQIP backends.

pub mod app;
pub mod audit_io;
pub mod board;
pub mod config;
pub mod game_state;
pub mod i18n;
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
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub fn set_web_locale(lang: &str) {
    crate::i18n::push_web_locale(lang);
}

#[cfg_attr(all(feature = "wasm", target_arch = "wasm32"), wasm_bindgen)]
#[cfg(feature = "wasm")]
pub fn run_wasm() {
    run_game(GameConfig::wasm());
}

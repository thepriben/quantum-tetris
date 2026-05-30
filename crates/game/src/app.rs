//! Quantum Tetris — classic or Qiskit piece picker.

use crate::board::Board;
use crate::config::{GameConfig, QuantumSession};
use crate::game_state::GameRun;
use crate::tetris;
use crate::ui;
use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

pub struct TetrisPlugin {
    config: GameConfig,
}

impl TetrisPlugin {
    pub fn new(config: GameConfig) -> Self {
        Self { config }
    }
}

impl Plugin for TetrisPlugin {
    fn build(&self, app: &mut App) {
        let session = QuantumSession::with_fallback(self.config.backend_kind);

        app.insert_resource(self.config.clone())
            .insert_resource(ClearColor(crate::ui::BG))
            .insert_resource(session)
            .insert_resource(GameRun::new(self.config.backend_kind))
            .insert_resource(Board::default())
            .add_systems(Startup, ui::setup_ui)
            .add_systems(Startup, tetris::init_first_piece.after(ui::setup_ui))
            .add_systems(
                Update,
                (tetris::tick_gravity, tetris::handle_input, ui::refresh_ui).chain(),
            );
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn workspace_asset_path() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets"))
        .to_string_lossy()
        .into_owned()
}

pub fn build_app(config: GameConfig) -> App {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Quantum Tetris: LA".into(),
                    resolution: (900, 720).into(),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                #[cfg(not(target_arch = "wasm32"))]
                file_path: workspace_asset_path(),
                #[cfg(target_arch = "wasm32")]
                file_path: "assets".into(),
                ..default()
            }),
    );
    app.add_plugins(TetrisPlugin::new(config));
    app
}

pub fn run_game(config: GameConfig) {
    #[cfg(feature = "desktop")]
    eprintln!(
        "\n\
         Quantum Tetris: LA — {}\n\
         ─────────────────────────────────────\n\
         • Backends: classic (uniform) or Qiskit Aer (Born rule)\n\
         • Teleporter: Bell bits → family, message qubit → shape\n\
         • ↑ rotate · ↓ faster · Space = observe (hard drop)\n\
         ─────────────────────────────────────\n",
        config.backend_label()
    );
    build_app(config).run();
}

/// Alias kept for WASM docs.
pub type DrivePlugin = TetrisPlugin;

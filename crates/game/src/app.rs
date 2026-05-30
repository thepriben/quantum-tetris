//! Quantum Tetris — Bevy app wired to classic, RustQIP, or Qiskit backends.

use crate::board::Board;
use crate::config::{GameConfig, QuantumSession};
use crate::game_state::GameRun;
use crate::i18n;
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
            .insert_resource(i18n::initial_locale())
            .insert_resource(session)
            .insert_resource(GameRun::new(self.config.backend_kind))
            .insert_resource(Board::default())
            .add_systems(Startup, ui::setup_ui)
            .add_systems(Startup, tetris::init_first_piece.after(ui::setup_ui))
            .add_systems(
                Update,
                (
                    #[cfg(not(target_arch = "wasm32"))]
                    ui::handle_lang_button,
                    ui::handle_mode_buttons,
                    tetris::tick_gravity,
                    tetris::handle_input,
                    #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
                    i18n::sync_web_locale,
                    ui::refresh_ui,
                )
                    .chain(),
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
                    title: "Quantum Tetris".into(),
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
         Quantum Tetris — {}\n\
         ─────────────────────────────────────\n\
         • RustQIP by default; CLASSIC or QISKIT (desktop) in-game\n\
         • Measured bits → piece, speed, bonuses\n\
         • ↑ rotate · ↓ faster · Space = observe (hard drop)\n\
         ─────────────────────────────────────\n",
        config.backend_label()
    );
    build_app(config).run();
}

/// Alias kept for WASM docs.
pub type DrivePlugin = TetrisPlugin;

//! Quantum Sub — underwater driving game (classic vs QIP).

use crate::arena;
use crate::config::{GameConfig, QuantumSession};
use crate::game_state::{GameRun, RunState, ENERGY_GOAL};
use crate::quantum_drive::{self, QuantumTick};
use crate::ui;
use crate::vehicle::{self, CameraRig};
use crate::visuals;
use bevy::{asset::AssetPlugin, prelude::*};
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

fn playing(run: Res<GameRun>) -> bool {
    run.state == RunState::Playing
}

pub struct DrivePlugin {
    config: GameConfig,
}

impl DrivePlugin {
    pub fn new(config: GameConfig) -> Self {
        Self { config }
    }
}

impl Plugin for DrivePlugin {
    fn build(&self, app: &mut App) {
        let session = QuantumSession::with_fallback(self.config.backend_kind);

        app.insert_resource(self.config.clone())
            .insert_resource(ClearColor(Color::srgb(0.08, 0.28, 0.42)))
            .insert_resource(GlobalAmbientLight {
                color: Color::srgb(0.55, 0.82, 0.95),
                brightness: 320.0,
                ..default()
            })
            .insert_resource(session)
            .insert_resource(GameRun::new(self.config.backend_kind))
            .insert_resource(QuantumTick::default())
            .insert_resource(CameraRig::default())
            .add_systems(Startup, setup_world)
            // Timer + HUD always run (win/loss screen).
            .add_systems(Update, (quantum_drive::tick_run_timer, ui::refresh_hud))
            // Submarine + mines: single chain avoids B0001 on Transform.
            .add_systems(
                Update,
                (
                    quantum_drive::tick_circuit_world,
                    quantum_drive::update_hints,
                    vehicle::drive_submarine,
                    quantum_drive::try_space_action,
                    quantum_drive::move_mines,
                    quantum_drive::mine_hull_damage,
                    vehicle::camera_follow_sub,
                )
                    .chain()
                    .run_if(playing),
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

#[cfg(target_arch = "wasm32")]
fn workspace_asset_path() -> String {
    "assets".into()
}

pub fn build_app(config: GameConfig) -> App {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                file_path: workspace_asset_path(),
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: config.window_title.into(),
                    resolution: (1280, 720).into(),
                    ..default()
                }),
                ..default()
            }),
    );
    app.add_plugins(DrivePlugin::new(config));
    app
}

pub fn run_game(config: GameConfig) {
    #[cfg(feature = "desktop")]
    eprintln!(
        "\n\
         Quantum Sub: LA — {}\n\
         ─────────────────────────────────────\n\
         • Collect {} energy cells → south gate\n\
         • Arrows — drive · Space — measure / act\n\
         • Classic: uniform circuit outcomes\n\
         • Quantum: QIP simulator (Born rule)\n\
         ─────────────────────────────────────\n",
        config.backend_label(),
        ENERGY_GOAL
    );
    build_app(config).run();
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    run: Res<GameRun>,
    asset_server: Res<AssetServer>,
) {
    visuals::spawn_water_dome(&mut commands, &mut meshes, &mut materials);
    visuals::spawn_underwater_lighting(&mut commands);
    arena::spawn_arena(&mut commands, &mut meshes, &mut materials, &asset_server);
    arena::spawn_submarine_entity(&mut commands, &mut meshes, &mut materials);
    vehicle::setup_camera(&mut commands);
    ui::setup_hud(&mut commands, &run);
}

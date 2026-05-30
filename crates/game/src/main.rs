//! Quantum Town: LA — desktop entry point.
//!
//! Sprint 1 adds Rapier physics, FPS controls, and egui HUD.
//! See <https://github.com/thepriben/quantum-town-la/blob/main/docs/DESIGN.md>.

use bevy::prelude::*;
use quantum_doom_quantum::{BackendKind, QuantumCircuit, build_backend};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Quantum Town: LA".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_scene, log_quantum_backend))
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Pastel ground plane — placeholder until OSM geometry lands (Sprint 2).
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(40.0, 40.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.78, 0.92, 0.86))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        PointLight {
            intensity: 900_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(6.0, 12.0, 6.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    info!("Quantum Town: LA — Sprint 0 bootstrap scene ready.");
}

fn log_quantum_backend() {
    let kind = BackendKind::from_env();
    match build_backend(kind) {
        Ok(mut backend) => {
            let measurement = backend
                .run(&QuantumCircuit::imp_brain())
                .expect("qip stub should succeed");
            info!(
                "Quantum backend {:?} sample Imp bits: {}",
                kind, measurement.bits
            );
        }
        Err(error) => warn!("Quantum backend init failed: {error}"),
    }
}

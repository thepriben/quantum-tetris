//! Underwater arena — floor, rocks (GLB), beacons, energy, exit.

use crate::game_state::ENERGY_GOAL;
use crate::vehicle::Submarine;
use quantum_town_quantum::EnemyBehavior;
use crate::visuals::{glow, matte, water_pool};
use bevy::prelude::*;

#[derive(Component)]
pub struct EnergyCell;

#[derive(Component)]
pub struct ExitGate;

#[derive(Component)]
pub struct WarpBeacon(pub u8);

#[derive(Component)]
pub struct Mine {
    pub hunter: bool,
    pub phase: f32,
    pub behavior: EnemyBehavior,
    pub last_bits: String,
}

#[derive(Component)]
pub struct ArenaRock;

/// Eight warp destinations (quantum teleporter bits → index).
pub const BEACON_POSITIONS: [Vec3; 8] = [
    Vec3::new(14.0, -1.5, 10.0),
    Vec3::new(-14.0, -1.5, 10.0),
    Vec3::new(14.0, -1.5, -10.0),
    Vec3::new(-14.0, -1.5, -10.0),
    Vec3::new(0.0, -1.5, 14.0),
    Vec3::new(0.0, -1.5, -14.0),
    Vec3::new(10.0, -1.5, 0.0),
    Vec3::new(-10.0, -1.5, 0.0),
];

pub fn spawn_arena(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &AssetServer,
) {
    let floor = materials.add(water_pool());
    let sand = materials.add(matte(Color::srgb(0.42, 0.62, 0.72), 0.95));
    let beacon_mat = materials.add(glow(Color::srgb(0.55, 0.85, 1.0), 1.2));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(80.0, 80.0))),
        MeshMaterial3d(floor),
        Transform::from_xyz(0.0, -2.2, 0.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(sand),
        Transform::from_xyz(0.0, -2.15, 0.0),
    ));

    spawn_glb_rocks(commands, asset_server);

    for (i, pos) in BEACON_POSITIONS.iter().enumerate() {
        commands.spawn((
            WarpBeacon(i as u8),
            Mesh3d(meshes.add(Cylinder::new(0.35, 2.0))),
            MeshMaterial3d(beacon_mat.clone()),
            Transform::from_xyz(pos.x, -1.0, pos.z),
        ));
    }

    let energy_mat = materials.add(glow(Color::srgb(0.3, 1.0, 0.95), 2.0));
    for pos in [
        Vec3::new(-8.0, -1.2, 6.0),
        Vec3::new(7.0, -1.2, -5.0),
        Vec3::new(0.0, -1.2, 9.0),
    ] {
        commands.spawn((
            EnergyCell,
            Mesh3d(meshes.add(Sphere::new(0.35))),
            MeshMaterial3d(energy_mat.clone()),
            Transform::from_xyz(pos.x, pos.y, pos.z),
        ));
    }

    let gate_mat = materials.add(glow(Color::srgb(1.0, 0.85, 0.35), 2.5));
    commands.spawn((
        ExitGate,
        Mesh3d(meshes.add(Torus {
            minor_radius: 0.15,
            major_radius: 2.2,
        })),
        MeshMaterial3d(gate_mat),
        Transform::from_xyz(0.0, -1.0, -16.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));

    let mine_red = materials.add(glow(Color::srgb(1.0, 0.4, 0.38), 1.0));
    let mine_violet = materials.add(glow(Color::srgb(0.7, 0.5, 0.95), 0.9));
    for (pos, hunter) in [
        (Vec3::new(5.0, -1.0, 0.0), true),
        (Vec3::new(-6.0, -1.0, 3.0), true),
        (Vec3::new(3.0, -1.0, -7.0), false),
        (Vec3::new(-4.0, -1.0, -4.0), false),
    ] {
        commands.spawn((
            Mine {
                hunter,
                phase: pos.x * 0.3,
                behavior: EnemyBehavior::Attack,
                last_bits: "00".into(),
            },
            Mesh3d(meshes.add(Sphere::new(0.55))),
            MeshMaterial3d(if hunter { mine_red.clone() } else { mine_violet.clone() }),
            Transform::from_xyz(pos.x, pos.y, pos.z),
        ));
    }

    let _ = ENERGY_GOAL;
}

fn spawn_glb_rocks(commands: &mut Commands, asset_server: &AssetServer) {
    use std::path::PathBuf;
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/models");
    let placements: [(&str, Vec3, f32); 6] = [
        ("rock_largeA.glb", Vec3::new(-10.0, -2.0, 5.0), 2.2),
        ("rock_largeA.glb", Vec3::new(11.0, -2.0, -6.0), 1.8),
        ("rock_smallA.glb", Vec3::new(-3.0, -2.0, -3.0), 1.4),
        ("rock_smallA.glb", Vec3::new(6.0, -2.0, 8.0), 1.2),
        ("rock_largeA.glb", Vec3::new(0.0, -2.0, 12.0), 2.0),
        ("rock_smallA.glb", Vec3::new(-7.0, -2.0, -10.0), 1.0),
    ];
    for (file, pos, scale) in placements {
        if !root.join(file).exists() {
            continue;
        }
        commands.spawn((
            ArenaRock,
            SceneRoot(asset_server.load(format!("models/{file}#Scene0"))),
            Transform::from_translation(pos).with_scale(Vec3::splat(scale)),
        ));
    }
}

pub fn spawn_submarine_entity(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let hull = materials.add(matte(Color::srgb(0.92, 0.55, 0.35), 0.75));
    let window = materials.add(glow(Color::srgb(0.55, 0.95, 1.0), 1.5));
    let prop = materials.add(matte(Color::srgb(0.35, 0.38, 0.42), 0.9));

    commands
        .spawn((
            Submarine,
            Transform::from_xyz(0.0, -1.2, 8.0),
            Visibility::default(),
        ))
        .with_children(|sub| {
            sub.spawn((
                Mesh3d(meshes.add(Capsule3d::new(0.55, 1.4))),
                MeshMaterial3d(hull),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
            sub.spawn((
                Mesh3d(meshes.add(Sphere::new(0.28))),
                MeshMaterial3d(window),
                Transform::from_xyz(0.0, 0.15, 0.75),
            ));
            sub.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.08, 0.35))),
                MeshMaterial3d(prop),
                Transform::from_xyz(0.0, 0.0, -0.95)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ));
        });
}

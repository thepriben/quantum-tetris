//! Underwater rendering — fog, bloom, materials.

use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    pbr::{DistanceFog, FogFalloff},
    post_process::bloom::Bloom,
    prelude::*,
    render::{render_resource::Face, view::Hdr},
};

pub fn underwater_camera_bundle() -> impl Bundle {
    (
        Camera3d::default(),
        Hdr,
        Tonemapping::TonyMcMapface,
        Bloom {
            intensity: 0.18,
            ..Bloom::NATURAL
        },
        DistanceFog {
            color: Color::srgba(0.12, 0.38, 0.58, 1.0),
            falloff: FogFalloff::Linear {
                start: 18.0,
                end: 55.0,
            },
            directional_light_color: Color::srgb(0.5, 0.85, 1.0),
            directional_light_exponent: 10.0,
        },
        Msaa::Sample4,
    )
}

pub fn apply_world_atmosphere(
    mut clear: ResMut<ClearColor>,
    mut ambient: ResMut<GlobalAmbientLight>,
) {
    clear.0 = Color::srgb(0.08, 0.28, 0.42);
    ambient.color = Color::srgb(0.55, 0.82, 0.95);
    ambient.brightness = 320.0;
}

pub fn spawn_underwater_lighting(commands: &mut Commands) {
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.75, 0.92, 1.0),
            illuminance: 22_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.9, 0.35, 0.0)),
    ));
    commands.spawn((
        PointLight {
            color: Color::srgb(0.4, 0.9, 1.0),
            intensity: 600_000.0,
            range: 45.0,
            ..default()
        },
        Transform::from_xyz(0.0, 4.0, 0.0),
    ));
}

pub fn spawn_water_dome(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let dome = materials.add(StandardMaterial {
        base_color: Color::srgba(0.2, 0.55, 0.75, 0.15),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        cull_mode: Some(Face::Front),
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0))),
        MeshMaterial3d(dome),
        Transform::from_xyz(0.0, 6.0, 0.0).with_scale(Vec3::splat(70.0)),
    ));
}

pub fn matte(color: Color, roughness: f32) -> StandardMaterial {
    StandardMaterial {
        base_color: color,
        perceptual_roughness: roughness,
        reflectance: 0.2,
        ..default()
    }
}

pub fn glow(color: Color, strength: f32) -> StandardMaterial {
    StandardMaterial {
        base_color: color,
        emissive: (color.to_linear() * strength).into(),
        perceptual_roughness: 0.25,
        ..default()
    }
}

pub fn water_pool() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb(0.15, 0.45, 0.62),
        emissive: LinearRgba::new(0.1, 0.35, 0.5, 0.0),
        perceptual_roughness: 0.05,
        reflectance: 0.4,
        ..default()
    }
}

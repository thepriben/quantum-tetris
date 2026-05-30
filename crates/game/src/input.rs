//! Arrow keys + Space (single action).

use bevy::prelude::*;

/// Camera-relative movement from arrow keys.
pub fn movement_relative_to_camera(keys: &ButtonInput<KeyCode>, cam: &Transform) -> Vec3 {
    let flat_forward = flat_forward_xz(cam);
    let flat_right = Vec3::new(-flat_forward.z, 0.0, flat_forward.x);

    let mut direction = Vec3::ZERO;
    if keys.pressed(KeyCode::ArrowUp) {
        direction += flat_forward;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        direction -= flat_forward;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        direction -= flat_right;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        direction += flat_right;
    }

    if direction.length_squared() > 0.0 {
        direction.normalize()
    } else {
        Vec3::ZERO
    }
}

/// Unique gameplay action — always runs a circuit.
pub fn space_just_pressed(keys: &ButtonInput<KeyCode>) -> bool {
    keys.just_pressed(KeyCode::Space)
}

fn flat_forward_xz(transform: &Transform) -> Vec3 {
    let f = transform.forward().as_vec3();
    let flat = Vec3::new(f.x, 0.0, f.z);
    if flat.length_squared() < 0.001 {
        Vec3::new(0.0, 0.0, -1.0)
    } else {
        flat.normalize()
    }
}

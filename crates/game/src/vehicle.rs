//! Submarine piloting + chase camera.

use crate::game_state::GameRun;
use crate::input;
use crate::visuals;
use bevy::prelude::*;

#[derive(Component)]
pub struct Submarine;

#[derive(Component, Default)]
pub struct SubVelocity(pub Vec3);

#[derive(Component)]
pub struct FollowCamera;

#[derive(Resource)]
pub struct CameraRig {
    pub distance: f32,
    pub height: f32,
    pub look_height: f32,
    pub lerp: f32,
}

impl Default for CameraRig {
    fn default() -> Self {
        Self {
            distance: 12.0,
            height: 4.5,
            look_height: 0.8,
            lerp: 0.12,
        }
    }
}

pub fn drive_submarine(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    run: Res<GameRun>,
    camera: Query<&Transform, (With<FollowCamera>, Without<Submarine>)>,
    mut query: Query<(&mut Transform, &mut SubVelocity), With<Submarine>>,
) {
    let Ok((mut transform, mut velocity)) = query.single_mut() else {
        return;
    };
    let Ok(cam) = camera.single() else {
        return;
    };

    let thrust = 20.0;
    let mut wish = input::movement_relative_to_camera(&keys, cam) * thrust;

    if run.quantum_current.length_squared() > 0.01 {
        wish += run.quantum_current * (12.0 * run.current_strength);
    }

    velocity.0 = velocity.0.lerp(wish, 4.0 * time.delta_secs());
    velocity.0 *= 0.92;
    transform.translation += velocity.0 * time.delta_secs();

    if velocity.0.length_squared() > 0.5 {
        let flat = Vec3::new(velocity.0.x, 0.0, velocity.0.z).normalize();
        let look_target = transform.translation + flat;
        transform.look_at(look_target, Vec3::Y);
    }

    transform.translation.x = transform.translation.x.clamp(-18.0, 18.0);
    transform.translation.z = transform.translation.z.clamp(-18.0, 18.0);
    transform.translation.y = -1.2 + (time.elapsed_secs() * 1.8).sin() * 0.08;
}

pub fn camera_follow_sub(
    rig: Res<CameraRig>,
    sub: Query<&Transform, (With<Submarine>, Without<FollowCamera>)>,
    mut camera: Query<&mut Transform, (With<FollowCamera>, Without<Submarine>)>,
) {
    let Ok(sub_tf) = sub.single() else {
        return;
    };
    let Ok(mut cam) = camera.single_mut() else {
        return;
    };

    let f = sub_tf.forward().as_vec3();
    let mut flat = Vec3::new(f.x, 0.0, f.z);
    if flat.length_squared() < 0.001 {
        flat = Vec3::new(0.0, 0.0, -1.0);
    } else {
        flat = flat.normalize();
    }

    let focus = sub_tf.translation + Vec3::new(0.0, rig.look_height, 0.0);
    let desired = focus - flat * rig.distance + Vec3::new(0.0, rig.height, 0.0);
    cam.translation = cam.translation.lerp(desired, rig.lerp);
    cam.look_at(focus, Vec3::Y);
}

pub fn setup_camera(commands: &mut Commands) {
    commands.spawn((
        visuals::underwater_camera_bundle(),
        FollowCamera,
        Transform::from_xyz(0.0, 2.0, 14.0).looking_at(Vec3::new(0.0, -1.0, 0.0), Vec3::Y),
    ));
}

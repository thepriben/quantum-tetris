//! Every gameplay beat runs a circuit; collapsed bits drive the world.

use crate::arena::{BEACON_POSITIONS, EnergyCell, ExitGate, Mine};
use crate::config::QuantumSession;
use crate::game_state::{GameRun, RunState, ENERGY_GOAL, MAX_COHERENCE, RUN_DURATION_SECS};
use crate::input;
use crate::measurement_fx::{
    hunter_speed, observation_from_measurement, ocean_from_measurement, patrol_step,
    record_measurement, shard_coherence_bonus, warp_beacon_index, warp_nudge_from_bits,
};
use crate::vehicle::Submarine;
use bevy::prelude::*;
use quantum_town_quantum::{EnemyBehavior, QuantumCircuit};

const CELL_RADIUS: f32 = 1.8;
const GATE_RADIUS: f32 = 3.0;

#[derive(Resource)]
pub struct QuantumTick(Timer);

impl Default for QuantumTick {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Repeating))
    }
}

pub fn tick_run_timer(time: Res<Time>, mut run: ResMut<GameRun>) {
    if run.state != RunState::Playing {
        return;
    }
    run.time_remaining -= time.delta_secs();
    run.warp_cooldown = (run.warp_cooldown - time.delta_secs()).max(0.0);
    run.observe_cooldown = (run.observe_cooldown - time.delta_secs()).max(0.0);
    if run.time_remaining <= 0.0 {
        run.state = RunState::LostTime;
        run.last_event = "Time's up".into();
    }
}

pub fn tick_circuit_world(
    time: Res<Time>,
    mut clock: ResMut<QuantumTick>,
    session: Res<QuantumSession>,
    mut run: ResMut<GameRun>,
    mut mines: Query<&mut Mine>,
) {
    clock.0.tick(time.delta());
    if !clock.0.just_finished() || run.state != RunState::Playing {
        return;
    }

    let mut backend = session.backend.lock().expect("quantum backend");

    let ocean = match backend.run(&QuantumCircuit::imp_brain()) {
        Ok(m) => m,
        Err(_) => return,
    };
    let (drift, label, strength) = ocean_from_measurement(&ocean);
    record_measurement(&mut run, &ocean);
    run.quantum_current = drift;
    run.current_strength = strength;
    run.current_label = label;
    run.last_event = format!("imp-brain → {}", run.current_label);

    for mut mine in &mut mines {
        let circuit = if mine.hunter {
            QuantumCircuit::hunter_profile()
        } else {
            QuantumCircuit::patrol_profile()
        };
        if let Ok(m) = backend.run(&circuit) {
            mine.last_bits = m.bits.clone();
            mine.behavior =
                EnemyBehavior::from_bits(&m.bits).unwrap_or(EnemyBehavior::Attack);
        }
    }
}

/// Space: stabilizer near cell → warp if ready → observe fallback.
pub fn try_space_action(
    keys: Res<ButtonInput<KeyCode>>,
    session: Res<QuantumSession>,
    mut run: ResMut<GameRun>,
    mut sub: Query<&mut Transform, (With<Submarine>, Without<EnergyCell>, Without<ExitGate>)>,
    cells: Query<(Entity, &Transform), (With<EnergyCell>, Without<Submarine>)>,
    gate: Query<&Transform, (With<ExitGate>, Without<Submarine>)>,
    mut commands: Commands,
) {
    if !input::space_just_pressed(&keys) || run.state != RunState::Playing {
        return;
    }
    let Ok(mut tf) = sub.single_mut() else {
        return;
    };
    let sub_pos = tf.translation;

    if run.energy >= ENERGY_GOAL {
        if let Ok(gate_tf) = gate.single() {
            if sub_pos.distance(gate_tf.translation) < GATE_RADIUS {
                run.state = RunState::Won;
                run.last_event = format!("You win! {:.0}s left", run.time_remaining);
                return;
            }
        }
    }

    for (entity, cell_tf) in &cells {
        if sub_pos.distance(cell_tf.translation) <= CELL_RADIUS {
            apply_stabilizer(&session, &mut run, &mut commands, entity);
            return;
        }
    }

    if run.warp_cooldown <= 0.0 {
        apply_warp(&session, &mut run, &mut tf);
        return;
    }

    if run.observe_cooldown <= 0.0 {
        apply_observe(&session, &mut run);
        return;
    }

    run.hint = format!(
        "Cooldown warp {:.0}s / observe {:.0}s",
        run.warp_cooldown, run.observe_cooldown
    );
}

fn apply_stabilizer(
    session: &QuantumSession,
    run: &mut GameRun,
    commands: &mut Commands,
    entity: Entity,
) {
    let measurement = session
        .backend
        .lock()
        .expect("quantum backend")
        .run(&QuantumCircuit::shard_stabilizer())
        .expect("shard");
    let bonus = shard_coherence_bonus(&measurement.bits);
    commands.entity(entity).despawn();
    run.energy = run.energy.saturating_add(1);
    run.coherence = (run.coherence + bonus).min(MAX_COHERENCE);
    record_measurement(run, &measurement);
    run.last_event = format!(
        "stabilizer +{bonus:.0} coh — {}/{}",
        run.energy, ENERGY_GOAL
    );
    run.hint = format!("Energy {}/{} — south gate", run.energy, ENERGY_GOAL);
}

fn apply_warp(session: &QuantumSession, run: &mut GameRun, tf: &mut Transform) {
    let measurement = session
        .backend
        .lock()
        .expect("quantum backend")
        .run(&QuantumCircuit::quantum_teleportation())
        .expect("teleport");

    let index = warp_beacon_index(&measurement, BEACON_POSITIONS.len());
    let dest = BEACON_POSITIONS[index];
    tf.translation = dest + warp_nudge_from_bits(&measurement.bits) + Vec3::new(0.0, -0.2, 0.0);
    tf.translation.x = tf.translation.x.clamp(-18.0, 18.0);
    tf.translation.z = tf.translation.z.clamp(-18.0, 18.0);

    record_measurement(run, &measurement);
    let coherence_gain = if run.is_quantum { 10.0 } else { 6.0 };
    run.coherence = (run.coherence + coherence_gain).min(MAX_COHERENCE);
    run.last_event = format!("teleport → beacon {index}");
    run.warp_cooldown = 4.0;
    run.hint = "Space — observe (warp recharging)".into();
}

fn apply_observe(session: &QuantumSession, run: &mut GameRun) {
    let measurement = session
        .backend
        .lock()
        .expect("quantum backend")
        .run(&QuantumCircuit::observation_pulse())
        .expect("observe");

    let fx = observation_from_measurement(&measurement);
    record_measurement(run, &measurement);

    if fx.clear_current {
        run.quantum_current = Vec3::ZERO;
        run.current_strength = 0.0;
        run.current_label = "calm".into();
    } else {
        run.quantum_current *= 0.35;
        run.current_strength *= 0.35;
        run.current_label = format!("residual ({})", fx.label);
    }

    run.coherence = (run.coherence + fx.coherence_bonus).min(MAX_COHERENCE);
    run.time_remaining = (run.time_remaining + fx.time_bonus_secs).min(RUN_DURATION_SECS);
    run.last_event = format!(
        "observe {label} +{t:.0}s",
        label = fx.label,
        t = fx.time_bonus_secs
    );
    run.observe_cooldown = 5.0;
}

pub fn move_mines(
    time: Res<Time>,
    sub: Query<&Transform, (With<Submarine>, Without<Mine>)>,
    mut mines: Query<(&Mine, &mut Transform), Without<Submarine>>,
) {
    let Ok(sub_tf) = sub.single() else {
        return;
    };
    let t = time.elapsed_secs();
    let dt = time.delta_secs();

    for (mine, mut tf) in &mut mines {
        if mine.hunter {
            let to = sub_tf.translation - tf.translation;
            let flat = Vec3::new(to.x, 0.0, to.z);
            if flat.length_squared() > 0.2 {
                let speed = hunter_speed(mine.behavior);
                tf.translation += flat.normalize() * speed * dt;
            }
        } else {
            let pos = tf.translation;
            tf.translation += patrol_step(
                mine.behavior,
                pos,
                sub_tf.translation,
                mine.phase,
                t,
                dt,
            );
        }
        tf.translation.y = -1.0;
        tf.translation.x = tf.translation.x.clamp(-18.0, 18.0);
        tf.translation.z = tf.translation.z.clamp(-18.0, 18.0);
    }
}

pub fn mine_hull_damage(
    sub: Query<&Transform, (With<Submarine>, Without<Mine>)>,
    mines: Query<(&Transform, &Mine), Without<Submarine>>,
    mut run: ResMut<GameRun>,
) {
    if run.state != RunState::Playing {
        return;
    }
    let Ok(sub_tf) = sub.single() else {
        return;
    };
    for (mine_tf, mine) in &mines {
        if sub_tf.translation.distance(mine_tf.translation) < 1.2 {
            let stress = match mine.behavior {
                EnemyBehavior::Ambush => 0.65,
                EnemyBehavior::Attack => 0.45,
                EnemyBehavior::Flank => 0.35,
                EnemyBehavior::Flee => 0.2,
            };
            run.hull_stress += stress;
            run.coherence = (run.coherence - (6.0 + stress * 4.0)).max(0.0);
            run.time_remaining -= 0.08 * stress;
            run.last_bits = mine.last_bits.clone();
            run.last_event = format!("mine [{bits}] stress", bits = mine.last_bits);
        }
    }
    if run.hull_stress > 12.0 {
        run.coherence = 0.0;
    }
}

pub fn update_hints(
    sub: Query<&Transform, (With<Submarine>, Without<EnergyCell>, Without<ExitGate>)>,
    cells: Query<&Transform, (With<EnergyCell>, Without<Submarine>)>,
    gate: Query<&Transform, (With<ExitGate>, Without<Submarine>)>,
    mut run: ResMut<GameRun>,
) {
    if run.state != RunState::Playing {
        return;
    }
    let Ok(sub_tf) = sub.single() else {
        return;
    };
    let pos = sub_tf.translation;

    if run.energy >= ENERGY_GOAL {
        if let Ok(gate_tf) = gate.single() {
            if pos.distance(gate_tf.translation) < GATE_RADIUS + 2.0 {
                run.hint = "Space — exit via south gate".into();
                return;
            }
        }
    }

    for cell_tf in &cells {
        if pos.distance(cell_tf.translation) < CELL_RADIUS + 1.0 {
            run.hint = "Space — stabilize cell (circuit)".into();
            return;
        }
    }

    if run.warp_cooldown <= 0.0 {
        run.hint = "Space — teleport (3 bits → beacon)".into();
    } else if run.observe_cooldown <= 0.0 {
        run.hint = "Space — observe (clears current)".into();
    } else {
        run.hint = format!(
            "Space cooldown {:.0}s",
            run.warp_cooldown.max(run.observe_cooldown)
        );
    }
}

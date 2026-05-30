//! Map collapsed bitstrings from circuits onto gameplay parameters.

use bevy::prelude::*;
use quantum_town_quantum::{EnemyBehavior, Measurement};

/// Probability (0–1) of the collapsed outcome in the backend histogram.
pub fn outcome_confidence(m: &Measurement) -> f32 {
    m.probabilities
        .iter()
        .find(|(bits, _)| bits == &m.bits)
        .map(|(_, p)| *p)
        .unwrap_or_else(|| 1.0 / (1u32 << m.bits.len().max(1) as u32) as f32)
}

pub fn record_measurement(run: &mut crate::game_state::GameRun, m: &Measurement) {
    run.last_bits = m.bits.clone();
    run.last_confidence = outcome_confidence(m) * 100.0;
}

/// Effects of measuring `observation-pulse-v1`.
#[derive(Debug, Clone, Copy)]
pub struct ObservationFx {
    pub clear_current: bool,
    pub time_bonus_secs: f32,
    pub coherence_bonus: f32,
    pub label: &'static str,
}

/// Ocean current from `imp-brain-v1` (2 bits → behavior → drift).
pub fn ocean_from_measurement(m: &Measurement) -> (Vec3, String, f32) {
    let behavior = EnemyBehavior::from_bits(&m.bits).unwrap_or(EnemyBehavior::Attack);
    let strength = drift_strength(&m.bits);
    (behavior_to_drift(behavior) * strength, behavior_label(behavior), strength)
}

pub fn drift_strength(bits: &str) -> f32 {
    let ones = bits.chars().filter(|&c| c == '1').count() as f32;
    0.85 + ones * 0.25
}

pub fn behavior_to_drift(b: EnemyBehavior) -> Vec3 {
    match b {
        EnemyBehavior::Attack => Vec3::new(0.0, 0.0, -1.0),
        EnemyBehavior::Flank => Vec3::new(1.0, 0.0, 0.0),
        EnemyBehavior::Flee => Vec3::new(0.0, 0.0, 1.0),
        EnemyBehavior::Ambush => Vec3::new(-1.0, 0.0, 0.0),
    }
}

pub fn behavior_label(b: EnemyBehavior) -> String {
    match b {
        EnemyBehavior::Attack => "push forward".into(),
        EnemyBehavior::Flank => "drift right".into(),
        EnemyBehavior::Flee => "drift back".into(),
        EnemyBehavior::Ambush => "drift left".into(),
    }
}

/// `quantum-teleportation-gate-v1` — three bits → beacon index.
pub fn warp_beacon_index(m: &Measurement, beacon_count: usize) -> usize {
    m.bits_as_usize() % beacon_count.max(1)
}

/// Correction bit from the message qubit — small positional nudge after warp.
pub fn warp_nudge_from_bits(bits: &str) -> Vec3 {
    match bits.chars().next() {
        Some('1') => Vec3::new(1.2, 0.0, 0.8),
        _ => Vec3::new(-1.2, 0.0, -0.8),
    }
}

pub fn observation_from_measurement(m: &Measurement) -> ObservationFx {
    observation_from_bits(&m.bits)
}

pub fn observation_from_bits(bits: &str) -> ObservationFx {
    match bits {
        "00" => ObservationFx {
            clear_current: true,
            time_bonus_secs: 5.0,
            coherence_bonus: 12.0,
            label: "effondrement doux",
        },
        "01" => ObservationFx {
            clear_current: true,
            time_bonus_secs: 3.0,
            coherence_bonus: 22.0,
            label: "purge laterale",
        },
        "10" => ObservationFx {
            clear_current: false,
            time_bonus_secs: 7.0,
            coherence_bonus: 8.0,
            label: "residu de courant",
        },
        "11" => ObservationFx {
            clear_current: true,
            time_bonus_secs: 10.0,
            coherence_bonus: 28.0,
            label: "mesure forte",
        },
        _ => ObservationFx {
            clear_current: true,
            time_bonus_secs: 4.0,
            coherence_bonus: 10.0,
            label: "mesure",
        },
    }
}

/// `q-shard-stabilizer-v1` on energy pickup.
pub fn shard_coherence_bonus(bits: &str) -> f32 {
    match bits {
        "00" => 12.0,
        "01" => 16.0,
        "10" => 20.0,
        "11" => 25.0,
        _ => 15.0,
    }
}

pub fn hunter_speed(behavior: EnemyBehavior) -> f32 {
    match behavior {
        EnemyBehavior::Attack => 4.5,
        EnemyBehavior::Flank => 3.2,
        EnemyBehavior::Flee => 1.8,
        EnemyBehavior::Ambush => 5.2,
    }
}

/// Patrol mine displacement from measured behavior (per second).
pub fn patrol_step(
    behavior: EnemyBehavior,
    mine_pos: Vec3,
    sub_pos: Vec3,
    phase: f32,
    t: f32,
    dt: f32,
) -> Vec3 {
    let to_sub = Vec3::new(sub_pos.x - mine_pos.x, 0.0, sub_pos.z - mine_pos.z);
    let flat = if to_sub.length_squared() > 0.01 {
        to_sub.normalize()
    } else {
        Vec3::Z
    };
    let tangent = Vec3::new(-flat.z, 0.0, flat.x);

    match behavior {
        EnemyBehavior::Attack => {
            let r = 4.0;
            let hub = Vec3::new(phase * 2.0, 0.0, phase * 1.5);
            let angle = t * 0.9 + phase;
            let ring = hub + Vec3::new(angle.cos() * r, 0.0, angle.sin() * r);
            let to_ring = Vec3::new(ring.x - mine_pos.x, 0.0, ring.z - mine_pos.z);
            let ring_push = if to_ring.length_squared() > 0.01 {
                to_ring.normalize() * 3.0 * dt
            } else {
                Vec3::ZERO
            };
            ring_push + flat * 2.2 * dt
        }
        EnemyBehavior::Flank => tangent * 4.0 * dt + flat * 1.2 * dt,
        EnemyBehavior::Flee => -flat * 3.5 * dt,
        EnemyBehavior::Ambush => {
            if to_sub.length() < 9.0 {
                flat * 6.0 * dt
            } else {
                tangent * 2.0 * dt
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome_confidence_reads_histogram() {
        let m = Measurement {
            bits: "01".into(),
            probabilities: vec![
                ("00".into(), 0.25),
                ("01".into(), 0.75),
                ("10".into(), 0.0),
                ("11".into(), 0.0),
            ],
        };
        assert!((outcome_confidence(&m) - 0.75).abs() < 1e-5);
    }

    #[test]
    fn observation_maps_all_two_bit_outcomes() {
        for bits in ["00", "01", "10", "11"] {
            let fx = observation_from_bits(bits);
            assert!(fx.time_bonus_secs > 0.0);
            assert!(fx.coherence_bonus > 0.0);
        }
    }

    #[test]
    fn warp_index_in_range() {
        let m = Measurement {
            bits: "101".into(),
            probabilities: vec![],
        };
        assert!(warp_beacon_index(&m, 8) < 8);
    }

    #[test]
    fn shard_bonus_monotonic_with_ones() {
        assert!(shard_coherence_bonus("11") >= shard_coherence_bonus("00"));
    }
}

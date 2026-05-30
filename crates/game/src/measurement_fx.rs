//! Map circuit measurements to Tetris gameplay.

use quantum_town_quantum::{Measurement, QuantumCircuit};

pub fn record_measurement(run: &mut crate::game_state::GameRun, m: &Measurement) {
    run.last_bits = m.bits.clone();
    run.last_confidence = outcome_confidence(m) * 100.0;
}

pub fn outcome_confidence(m: &Measurement) -> f32 {
    m.probabilities
        .iter()
        .find(|(bits, _)| bits == &m.bits)
        .map(|(_, p)| *p)
        .unwrap_or_else(|| 1.0 / (1u32 << m.bits.len().max(1) as u32) as f32)
}

/// `quantum-teleportation-gate-v1` → piece index 0–6.
pub fn piece_index(m: &Measurement) -> usize {
    m.bits_as_usize() % 7
}

/// `imp-brain-v1` → rotation 0–3.
pub fn rotation_from_bits(bits: &str) -> u8 {
    match bits {
        "00" => 0,
        "01" => 1,
        "10" => 2,
        "11" => 3,
        _ => 0,
    }
}

/// `imp-brain-v1` → spawn column bias (0–6).
pub fn spawn_x_from_bits(bits: &str) -> i32 {
    match bits {
        "00" => 3,
        "01" => 5,
        "10" => 2,
        "11" => 4,
        _ => 3,
    }
}

/// `observation-pulse-v1` on Space — hard drop bonus.
#[derive(Debug, Clone, Copy)]
pub struct ObserveFx {
    pub line_bonus: u32,
    pub score_bonus: u32,
    pub label: &'static str,
}

pub fn observe_from_bits(bits: &str) -> ObserveFx {
    match bits {
        "00" => ObserveFx {
            line_bonus: 0,
            score_bonus: 50,
            label: "soft collapse",
        },
        "01" => ObserveFx {
            line_bonus: 0,
            score_bonus: 120,
            label: "phase shift",
        },
        "10" => ObserveFx {
            line_bonus: 1,
            score_bonus: 80,
            label: "row echo",
        },
        "11" => ObserveFx {
            line_bonus: 0,
            score_bonus: 200,
            label: "strong measure",
        },
        _ => ObserveFx {
            line_bonus: 0,
            score_bonus: 40,
            label: "measure",
        },
    }
}

/// `q-shard-stabilizer-v1` when clearing lines.
pub fn line_clear_bonus(bits: &str, lines: u32) -> u32 {
    let mult = match bits {
        "00" => 1,
        "01" => 2,
        "10" => 3,
        "11" => 4,
        _ => 1,
    };
    lines * 100 * mult
}

pub fn piece_circuit() -> QuantumCircuit {
    QuantumCircuit::quantum_teleportation()
}

pub fn rotation_circuit() -> QuantumCircuit {
    QuantumCircuit::imp_brain()
}

pub fn observe_circuit() -> QuantumCircuit {
    QuantumCircuit::observation_pulse()
}

pub fn line_circuit() -> QuantumCircuit {
    QuantumCircuit::shard_stabilizer()
}

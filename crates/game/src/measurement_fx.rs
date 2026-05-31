//! Map circuit measurements to Tetris gameplay.

use crate::pieces::PieceKind;
use quantum_tetris_quantum::{Measurement, QuantumCircuit};

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

/// One shot of `quantum-teleportation-gate-v1` (3 bits, MSB = q0).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeleportReadout {
    /// Bell basis on qubits 0–1: `00`, `01`, `10`, `11`.
    pub bell: String,
    /// Message / receiver qubit (q2), kept visible in the HUD readout.
    pub message: bool,
}

pub fn read_teleport(bits: &str) -> TeleportReadout {
    let chars: Vec<char> = bits.chars().collect();
    let bell = if chars.len() >= 2 {
        format!("{}{}", chars[0], chars[1])
    } else {
        "11".into()
    };
    let message = chars.get(2).is_some_and(|c| *c == '1');
    TeleportReadout { bell, message }
}

/// Decode the falling piece from one teleporter shot.
///
/// The full 3-bit readout indexes seven tetrominoes. The spare eighth state
/// maps to T, keeping the draw simple without inventing a fake preview.
pub fn piece_from_teleport(measurement: &Measurement) -> (PieceKind, TeleportReadout) {
    let readout = read_teleport(&measurement.bits);
    (piece_from_teleport_bits(&measurement.bits), readout)
}

fn piece_from_teleport_bits(bits: &str) -> PieceKind {
    match bits {
        "000" => PieceKind::I,
        "001" => PieceKind::O,
        "010" => PieceKind::T,
        "011" => PieceKind::S,
        "100" => PieceKind::Z,
        "101" => PieceKind::J,
        "110" => PieceKind::L,
        "111" => PieceKind::T,
        _ => PieceKind::T,
    }
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

/// `enemy-profile-hunter-v1` → gravity interval for the falling piece (seconds).
pub fn drop_interval_from_bits(bits: &str, level: u32) -> f32 {
    let base = (0.85 - level as f32 * 0.04).max(0.15);
    match bits {
        "00" => base,
        "01" => (base * 0.75).max(0.12),
        "10" => (base * 1.25).min(1.2),
        "11" => (base * 0.55).max(0.1),
        _ => base,
    }
}

pub fn speed_circuit() -> QuantumCircuit {
    QuantumCircuit::hunter_profile()
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

#[cfg(test)]
mod tests {
    use super::*;

    fn m(bits: &str) -> Measurement {
        Measurement {
            bits: bits.into(),
            probabilities: vec![],
        }
    }

    #[test]
    fn teleporter_readout_splits_bell_and_message_bits() {
        let readout = read_teleport("101");
        assert_eq!(readout.bell, "10");
        assert!(readout.message);
    }

    #[test]
    fn teleport_bits_select_piece() {
        assert_eq!(piece_from_teleport(&m("000")).0, PieceKind::I);
        assert_eq!(piece_from_teleport(&m("001")).0, PieceKind::O);
        assert_eq!(piece_from_teleport(&m("010")).0, PieceKind::T);
        assert_eq!(piece_from_teleport(&m("011")).0, PieceKind::S);
        assert_eq!(piece_from_teleport(&m("100")).0, PieceKind::Z);
        assert_eq!(piece_from_teleport(&m("101")).0, PieceKind::J);
        assert_eq!(piece_from_teleport(&m("110")).0, PieceKind::L);
        assert_eq!(piece_from_teleport(&m("111")).0, PieceKind::T);
    }

    #[test]
    fn piece_draw_keeps_teleport_readout_for_hud() {
        let (_, readout) = piece_from_teleport(&m("101"));
        assert_eq!(readout.bell, "10");
        assert!(readout.message);
    }
}

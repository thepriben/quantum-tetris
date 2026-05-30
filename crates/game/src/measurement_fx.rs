//! Map circuit measurements to Tetris gameplay.

use crate::pieces::{PieceFamily, PieceKind};
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

/// One shot of `quantum-teleportation-gate-v1` (3 bits, MSB = q0).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeleportReadout {
    /// Bell basis on qubits 0–1: `00`, `01`, `10`, `11`.
    pub bell: String,
    /// Message / receiver qubit (q2) — selects a variant inside the family.
    pub message: bool,
    pub family: PieceFamily,
}

pub fn read_teleport(bits: &str) -> TeleportReadout {
    let chars: Vec<char> = bits.chars().collect();
    let bell = if chars.len() >= 2 {
        format!("{}{}", chars[0], chars[1])
    } else {
        "11".into()
    };
    let message = chars.get(2).is_some_and(|c| *c == '1');
    TeleportReadout {
        family: PieceFamily::from_bell(&bell),
        bell,
        message,
    }
}

/// Decode **current** and **next** piece from two consecutive teleporter shots.
///
/// - Bell bits (2) → tetromino **family** (4 outcomes).
/// - Message bit + partner message bit → concrete shape (needed for the 4-piece Corner family).
pub fn pieces_from_teleport_pair(
    current: &Measurement,
    upcoming: &Measurement,
) -> (PieceKind, PieceKind, TeleportReadout, TeleportReadout) {
    let now = read_teleport(&current.bits);
    let next = read_teleport(&upcoming.bits);
    let kind = piece_in_family(now.family, now.message, next.message);
    let next_kind = piece_in_family(next.family, next.message, now.message);
    (kind, next_kind, now, next)
}

fn piece_in_family(family: PieceFamily, primary_msg: bool, partner_msg: bool) -> PieceKind {
    match family {
        PieceFamily::Line => PieceKind::I,
        PieceFamily::Block => PieceKind::O,
        PieceFamily::Fork => PieceKind::T,
        PieceFamily::Corner => {
            let idx = (primary_msg as u8) << 1 | (partner_msg as u8);
            [PieceKind::J, PieceKind::L, PieceKind::S, PieceKind::Z][idx as usize]
        }
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
    fn bell_bits_select_family() {
        assert_eq!(read_teleport("000").family, PieceFamily::Line);
        assert_eq!(read_teleport("010").family, PieceFamily::Block);
        assert_eq!(read_teleport("100").family, PieceFamily::Fork);
        assert_eq!(read_teleport("110").family, PieceFamily::Corner);
    }

    #[test]
    fn fixed_families_ignore_message_bit() {
        let (kind, _, _, _) = pieces_from_teleport_pair(&m("001"), &m("000"));
        assert_eq!(kind, PieceKind::I);
        let (kind, _, _, _) = pieces_from_teleport_pair(&m("011"), &m("000"));
        assert_eq!(kind, PieceKind::O);
        let (kind, _, _, _) = pieces_from_teleport_pair(&m("101"), &m("000"));
        assert_eq!(kind, PieceKind::T);
    }

    #[test]
    fn corner_family_uses_both_message_bits() {
        let (j, _, _, _) = pieces_from_teleport_pair(&m("110"), &m("000"));
        assert_eq!(j, PieceKind::J);
        let (l, _, _, _) = pieces_from_teleport_pair(&m("110"), &m("001"));
        assert_eq!(l, PieceKind::L);
        let (s, _, _, _) = pieces_from_teleport_pair(&m("111"), &m("000"));
        assert_eq!(s, PieceKind::S);
        let (z, _, _, _) = pieces_from_teleport_pair(&m("111"), &m("001"));
        assert_eq!(z, PieceKind::Z);
    }

    #[test]
    fn upcoming_piece_comes_from_second_teleport() {
        let (_, next, _, readout) = pieces_from_teleport_pair(&m("000"), &m("101"));
        assert_eq!(next, PieceKind::T);
        assert_eq!(readout.bell, "10");
        assert_eq!(readout.family, PieceFamily::Fork);
    }
}

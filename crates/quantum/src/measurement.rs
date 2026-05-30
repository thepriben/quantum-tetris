//! Measurement outcomes and gameplay enums derived from bitstrings.

use serde::{Deserialize, Serialize};

/// A single shot result plus optional probability table for debug UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measurement {
    /// Collapsed bitstring (MSB-first), e.g. `"01"`.
    pub bits: String,
    /// Outcome histogram shown in the debug panel.
    pub probabilities: Vec<(String, f32)>,
}

impl Measurement {
    /// Uniform distribution over all bitstrings for `qubits` width.
    pub fn uniform_superposition(qubits: u8) -> Self {
        let uniform = 1.0 / (1u32 << qubits) as f32;
        let mut probabilities = Vec::with_capacity(1usize << qubits);
        for value in 0..(1u32 << qubits) {
            let bits = format!("{value:0width$b}", width = qubits as usize);
            probabilities.push((bits.clone(), uniform));
        }
        Self {
            bits: probabilities[0].0.clone(),
            probabilities,
        }
    }
}

/// Quantum Imp behavior lookup after measuring two qubits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnemyBehavior {
    /// `00` — direct charge
    Attack,
    /// `01` — flank through alleys
    Flank,
    /// `10` — flee toward another shard
    Flee,
    /// `11` — ambush behind cover
    Ambush,
}

impl EnemyBehavior {
    /// Map a two-bit string to imp behavior.
    pub fn from_bits(bits: &str) -> Option<Self> {
        match bits {
            "00" => Some(Self::Attack),
            "01" => Some(Self::Flank),
            "10" => Some(Self::Flee),
            "11" => Some(Self::Ambush),
            _ => None,
        }
    }
}

//! Measurement outcomes and gameplay enums derived from bitstrings.

use rand::Rng;
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
    /// Parse the collapsed bitstring as an integer (MSB-first).
    pub fn bits_as_usize(&self) -> usize {
        usize::from_str_radix(self.bits.as_str(), 2).unwrap_or(0)
    }

    /// Build from a probability vector (index = bit value, MSB = qubit 0).
    pub fn from_probabilities(qubits: usize, probabilities: &[f64]) -> Self {
        let entries = normalized_entries(qubits, probabilities);
        if entries.is_empty() {
            return Self::uniform_superposition(qubits as u8);
        }
        let bits = sample_from_entries(&entries);
        Self {
            bits,
            probabilities: entries,
        }
    }

    /// Uniform distribution over all bitstrings for `qubits` width.
    pub fn uniform_superposition(qubits: u8) -> Self {
        let uniform = 1.0 / (1u32 << qubits) as f32;
        let mut probabilities = Vec::with_capacity(1usize << qubits);
        for value in 0..(1u32 << qubits) {
            let bits = format!("{value:0width$b}", width = qubits as usize);
            probabilities.push((bits.clone(), uniform));
        }
        let bits = sample_from_entries(&probabilities);
        Self {
            bits,
            probabilities,
        }
    }
}

fn sample_from_entries(entries: &[(String, f32)]) -> String {
    sample_from_entries_with(entries, &mut rand::rng())
}

fn sample_from_entries_with(entries: &[(String, f32)], rng: &mut impl Rng) -> String {
    let roll: f32 = rng.random();
    let mut cumulative = 0.0;
    for (bits, probability) in entries {
        cumulative += probability;
        if roll <= cumulative {
            return bits.clone();
        }
    }
    entries
        .last()
        .map(|(bits, _)| bits.clone())
        .unwrap_or_default()
}

/// Normalized outcome table (probabilities sum to ~1).
pub(crate) fn normalized_entries(qubits: usize, probabilities: &[f64]) -> Vec<(String, f32)> {
    let width = qubits;
    let mut entries = Vec::new();
    let mut total = 0.0_f64;
    for (value, &probability) in probabilities.iter().enumerate() {
        if probability <= 0.0 {
            continue;
        }
        let bits = format!("{value:0width$b}", width = width);
        entries.push((bits, probability as f32));
        total += probability;
    }
    if entries.is_empty() {
        return Vec::new();
    }
    if (total - 1.0).abs() > 1e-6 {
        for entry in &mut entries {
            entry.1 = (entry.1 as f64 / total) as f32;
        }
    }
    entries
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn enemy_behavior_maps_all_outcomes() {
        assert_eq!(EnemyBehavior::from_bits("00"), Some(EnemyBehavior::Attack));
        assert_eq!(EnemyBehavior::from_bits("01"), Some(EnemyBehavior::Flank));
        assert_eq!(EnemyBehavior::from_bits("10"), Some(EnemyBehavior::Flee));
        assert_eq!(EnemyBehavior::from_bits("11"), Some(EnemyBehavior::Ambush));
        assert_eq!(EnemyBehavior::from_bits("101"), None);
        assert_eq!(EnemyBehavior::from_bits(""), None);
    }

    #[test]
    fn uniform_superposition_has_four_outcomes_for_two_qubits() {
        let measurement = Measurement::uniform_superposition(2);
        assert_eq!(measurement.probabilities.len(), 4);
        assert!(measurement
            .probabilities
            .iter()
            .any(|(bits, _)| bits == "11"));
        assert!(measurement
            .probabilities
            .iter()
            .any(|(bits, _)| bits == &measurement.bits));
        let total: f32 = measurement.probabilities.iter().map(|(_, p)| p).sum();
        assert!((total - 1.0).abs() < 1e-5);
    }

    #[test]
    fn uniform_superposition_eight_outcomes_for_three_qubits() {
        let measurement = Measurement::uniform_superposition(3);
        assert_eq!(measurement.probabilities.len(), 8);
        assert_eq!(measurement.bits.len(), 3);
    }

    #[test]
    fn from_probabilities_normalizes_weights() {
        let entries = normalized_entries(2, &[0.2, 0.0, 0.3, 0.5]);
        let total: f32 = entries.iter().map(|(_, p)| p).sum();
        assert!((total - 1.0).abs() < 1e-5);
        assert_eq!(entries.len(), 3);
    }

    #[test]
    fn from_probabilities_empty_falls_back_to_uniform() {
        let measurement = Measurement::from_probabilities(2, &[0.0, 0.0, 0.0, 0.0]);
        assert_eq!(measurement.probabilities.len(), 4);
    }

    #[test]
    fn sample_is_deterministic_with_seeded_rng() {
        let entries = vec![
            ("00".into(), 0.25),
            ("01".into(), 0.25),
            ("10".into(), 0.25),
            ("11".into(), 0.25),
        ];
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        assert_eq!(
            sample_from_entries_with(&entries, &mut rng),
            sample_from_entries_with(&entries, &mut rand::rngs::StdRng::seed_from_u64(42))
        );
    }

    #[test]
    fn measurement_json_roundtrip() {
        let measurement = Measurement::uniform_superposition(2);
        let json = serde_json::to_string(&measurement).unwrap();
        let back: Measurement = serde_json::from_str(&json).unwrap();
        assert_eq!(back.bits, measurement.bits);
        assert_eq!(back.probabilities.len(), measurement.probabilities.len());
    }
}

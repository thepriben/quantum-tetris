//! Classic / arcade mode — uniform random bitstring (local testing, no deps).

use crate::{Measurement, QuantumBackend, QuantumCircuit, QuantumError};
use rand::Rng;

/// Picks a random outcome with `rand` (honest placeholder, not a quantum simulator).
#[derive(Debug, Default)]
pub struct ClassicBackend;

impl QuantumBackend for ClassicBackend {
    fn run(&mut self, circuit: &QuantumCircuit) -> Result<Measurement, QuantumError> {
        if circuit.qubits == 0 || circuit.qubits > 8 {
            return Err(QuantumError::UnsupportedQubits(circuit.qubits));
        }
        let space = 1u32 << circuit.qubits;
        let value = rand::rng().random_range(0..space);
        let width = circuit.qubits as usize;
        let bits = format!("{value:0width$b}");
        let probability = 1.0 / space as f32;
        let probabilities = (0..space)
            .map(|index| (format!("{index:0width$b}"), probability))
            .collect();
        Ok(Measurement {
            bits,
            probabilities,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::QuantumCircuit;

    #[test]
    fn classic_imp_brain_returns_two_bits() {
        let mut backend = ClassicBackend;
        let measurement = backend
            .run(&QuantumCircuit::imp_brain())
            .expect("classic run");
        assert_eq!(measurement.bits.len(), 2);
        assert_eq!(measurement.probabilities.len(), 4);
    }

    #[test]
    fn classic_teleporter_returns_three_bits() {
        let mut backend = ClassicBackend;
        let measurement = backend
            .run(&QuantumCircuit::teleporter())
            .expect("classic run");
        assert_eq!(measurement.bits.len(), 3);
        assert_eq!(measurement.probabilities.len(), 8);
    }
}

//! [RustQIP](https://github.com/Renmusxd/RustQIP) statevector backend — WASM-safe, Qiskit-matched in CI.

use crate::{Gate, Measurement, QuantumBackend, QuantumCircuit, QuantumError};
use qip::builder::LocalBuilder;
use qip::builder_traits::{CircuitBuilder, CliffordTBuilder, RotationsBuilder};
use rand::Rng;

fn map_circuit_error(err: qip::errors::CircuitError) -> QuantumError {
    QuantumError::BackendFailure {
        backend: "rustqip",
        message: err.to_string(),
    }
}

fn apply_gates(
    b: &mut LocalBuilder<f64>,
    qubits: &mut Vec<<LocalBuilder<f64> as CircuitBuilder>::Register>,
    circuit: &QuantumCircuit,
) -> Result<(), QuantumError> {
    for gate in &circuit.gates {
        match gate {
            Gate::H(q) => {
                let idx = usize::from(*q);
                let reg = qubits.remove(idx);
                qubits.insert(idx, b.h(reg));
            }
            Gate::X(q) => {
                let idx = usize::from(*q);
                let reg = qubits.remove(idx);
                qubits.insert(idx, b.x(reg));
            }
            Gate::Z(q) => {
                let idx = usize::from(*q);
                let reg = qubits.remove(idx);
                qubits.insert(idx, b.z(reg));
            }
            Gate::Ry { qubit, theta_deg } => {
                let idx = usize::from(*qubit);
                let theta = f64::from(*theta_deg).to_radians();
                let reg = qubits.remove(idx);
                qubits.insert(idx, b.ry(reg, theta));
            }
            Gate::Cx { control, target } => {
                let c = usize::from(*control);
                let t = usize::from(*target);
                let (lo, hi) = if c < t { (c, t) } else { (t, c) };
                let reg_hi = qubits.remove(hi);
                let reg_lo = qubits.remove(lo);
                let (out_lo, out_hi) = if c < t {
                    b.cnot(reg_lo, reg_hi).map_err(map_circuit_error)?
                } else {
                    let (a, b_out) = b.cnot(reg_hi, reg_lo).map_err(map_circuit_error)?;
                    (b_out, a)
                };
                qubits.insert(lo, out_lo);
                qubits.insert(hi, out_hi);
            }
        }
    }
    Ok(())
}

/// Exact Born probabilities via RustQIP (used by tests and parity checks).
pub fn rustqip_probabilities(circuit: &QuantumCircuit) -> Result<Vec<(String, f32)>, QuantumError> {
    if circuit.qubits == 0 || circuit.qubits > 8 {
        return Err(QuantumError::UnsupportedQubits(circuit.qubits));
    }
    let n = circuit.qubits;
    let mut builder = LocalBuilder::<f64>::default();
    let mut qubits = Vec::with_capacity(n as usize);
    for _ in 0..n {
        qubits.push(builder.qubit());
    }
    apply_gates(&mut builder, &mut qubits, circuit)?;
    let (state, _) = builder.calculate_state();
    let width = n as usize;
    Ok((0..state.len())
        .map(|index| {
            let amplitude = state[index];
            let probability = (amplitude.re * amplitude.re + amplitude.im * amplitude.im) as f32;
            (format!("{index:0width$b}"), probability)
        })
        .collect())
}

fn sample(probabilities: &[(String, f32)]) -> String {
    let roll: f32 = rand::rng().random();
    let mut cumulative = 0.0f32;
    for (bits, probability) in probabilities {
        cumulative += probability;
        if roll <= cumulative {
            return bits.clone();
        }
    }
    probabilities
        .last()
        .map(|(bits, _)| bits.clone())
        .unwrap_or_default()
}

/// Samples from RustQIP Born-rule probabilities (Qiskit-matched in CI).
#[derive(Debug, Default)]
pub struct RustQipBackend;

impl QuantumBackend for RustQipBackend {
    fn run(&mut self, circuit: &QuantumCircuit) -> Result<Measurement, QuantumError> {
        let probabilities = rustqip_probabilities(circuit)?;
        let bits = sample(&probabilities);
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
    fn imp_brain_is_uniform() {
        let probabilities = rustqip_probabilities(&QuantumCircuit::imp_brain()).expect("rustqip");
        assert_eq!(probabilities.len(), 4);
        for (_, probability) in &probabilities {
            assert!((probability - 0.25).abs() < 1e-5);
        }
    }

    #[test]
    fn probabilities_sum_to_one() {
        for circuit in [
            QuantumCircuit::imp_brain(),
            QuantumCircuit::hunter_profile(),
            QuantumCircuit::teleporter(),
        ] {
            let probabilities = rustqip_probabilities(&circuit).expect("rustqip");
            let total: f32 = probabilities.iter().map(|(_, p)| p).sum();
            assert!(
                (total - 1.0).abs() < 1e-5,
                "{} total={total}",
                circuit.label
            );
        }
    }
}

//! Born-rule statevector simulator — same gate set as Qiskit Aer, WASM-safe.

use crate::{Gate, Measurement, QuantumBackend, QuantumCircuit, QuantumError};
use rand::Rng;

type Amplitude = (f64, f64);

fn c_add(a: Amplitude, b: Amplitude) -> Amplitude {
    (a.0 + b.0, a.1 + b.1)
}

fn c_mul((ar, ai): Amplitude, (br, bi): Amplitude) -> Amplitude {
    (ar * br - ai * bi, ar * bi + ai * br)
}

fn amplitude_probability(amplitude: Amplitude) -> f32 {
    (amplitude.0 * amplitude.0 + amplitude.1 * amplitude.1) as f32
}

fn apply_one_qubit(
    state: &mut [Amplitude],
    n: u8,
    qubit: u8,
    m00: Amplitude,
    m01: Amplitude,
    m10: Amplitude,
    m11: Amplitude,
) {
    let dim = 1usize << n;
    let mask = 1usize << qubit;
    for index in 0..dim {
        if index & mask != 0 {
            continue;
        }
        let paired = index | mask;
        let lower = state[index];
        let upper = state[paired];
        state[index] = c_add(c_mul(m00, lower), c_mul(m01, upper));
        state[paired] = c_add(c_mul(m10, lower), c_mul(m11, upper));
    }
}

fn apply_h(state: &mut [Amplitude], qubit: u8, n: u8) {
    let scale = 1.0 / 2.0_f64.sqrt();
    apply_one_qubit(
        state,
        n,
        qubit,
        (scale, 0.0),
        (scale, 0.0),
        (scale, 0.0),
        (-scale, 0.0),
    );
}

fn apply_x(state: &mut [Amplitude], qubit: u8, n: u8) {
    apply_one_qubit(
        state,
        n,
        qubit,
        (0.0, 0.0),
        (1.0, 0.0),
        (1.0, 0.0),
        (0.0, 0.0),
    );
}

fn apply_z(state: &mut [Amplitude], qubit: u8, n: u8) {
    apply_one_qubit(
        state,
        n,
        qubit,
        (1.0, 0.0),
        (0.0, 0.0),
        (0.0, 0.0),
        (-1.0, 0.0),
    );
}

fn apply_ry(state: &mut [Amplitude], qubit: u8, theta_deg: f32, n: u8) {
    let half = (theta_deg as f64).to_radians() / 2.0;
    let cosine = half.cos();
    let sine = half.sin();
    apply_one_qubit(
        state,
        n,
        qubit,
        (cosine, 0.0),
        (-sine, 0.0),
        (sine, 0.0),
        (cosine, 0.0),
    );
}

fn apply_cx(state: &mut [Amplitude], control: u8, target: u8, n: u8) {
    let dim = 1usize << n;
    let control_mask = 1usize << control;
    let target_mask = 1usize << target;
    for index in 0..dim {
        if index & control_mask == 0 || index & target_mask != 0 {
            continue;
        }
        let flipped = index | target_mask;
        state.swap(index, flipped);
    }
}

fn apply_gate(state: &mut [Amplitude], gate: &Gate, n: u8) {
    match *gate {
        Gate::H(qubit) => apply_h(state, qubit, n),
        Gate::X(qubit) => apply_x(state, qubit, n),
        Gate::Z(qubit) => apply_z(state, qubit, n),
        Gate::Cx { control, target } => apply_cx(state, control, target, n),
        Gate::Ry { qubit, theta_deg } => apply_ry(state, qubit, theta_deg, n),
    }
}

/// Exact Born probabilities for a circuit (used by tests and parity checks).
pub fn born_probabilities(circuit: &QuantumCircuit) -> Result<Vec<(String, f32)>, QuantumError> {
    if circuit.qubits == 0 || circuit.qubits > 8 {
        return Err(QuantumError::UnsupportedQubits(circuit.qubits));
    }
    let n = circuit.qubits;
    let dim = 1usize << n;
    let mut state = vec![(0.0, 0.0); dim];
    state[0] = (1.0, 0.0);
    for gate in &circuit.gates {
        apply_gate(&mut state, gate, n);
    }
    let width = n as usize;
    Ok((0..dim)
        .map(|index| {
            let bits = format!("{index:0width$b}");
            (bits, amplitude_probability(state[index]))
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

/// Samples from exact Born-rule probabilities (matches Qiskit Aer in the limit).
#[derive(Debug, Default)]
pub struct BornBackend;

impl QuantumBackend for BornBackend {
    fn run(&mut self, circuit: &QuantumCircuit) -> Result<Measurement, QuantumError> {
        let probabilities = born_probabilities(circuit)?;
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
        let probabilities = born_probabilities(&QuantumCircuit::imp_brain()).expect("born");
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
            let probabilities = born_probabilities(&circuit).expect("born");
            let total: f32 = probabilities.iter().map(|(_, p)| p).sum();
            assert!(
                (total - 1.0).abs() < 1e-5,
                "{} total={total}",
                circuit.label
            );
        }
    }
}

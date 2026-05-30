//! Golden probabilities for every gameplay preset — no Python required.
//!
//! Values were cross-checked against Qiskit `Statevector` (same as CI
//! `rustqip_qiskit_parity`). Catches bit-order and gate-mapping regressions
//! locally without installing Qiskit.

#![allow(clippy::excessive_precision)]

use quantum_tetris_quantum::{rustqip_probabilities, QuantumCircuit};

const TOL: f32 = 1e-4;

fn assert_golden(circuit: QuantumCircuit, expected: &[(&str, f32)]) {
    let probs = rustqip_probabilities(&circuit).expect("rustqip");
    assert_eq!(
        probs.len(),
        expected.len(),
        "{}: outcome count",
        circuit.label
    );
    for ((bits, got), (exp_bits, exp_p)) in probs.iter().zip(expected.iter()) {
        assert_eq!(bits, exp_bits, "{}: bit label order", circuit.label);
        assert!(
            (got - exp_p).abs() < TOL,
            "{} {bits}: rustqip={got} expected={exp_p}",
            circuit.label
        );
    }
}

#[test]
fn imp_brain_golden() {
    assert_golden(
        QuantumCircuit::imp_brain(),
        &[("00", 0.25), ("01", 0.25), ("10", 0.25), ("11", 0.25)],
    );
}

#[test]
fn hunter_profile_golden() {
    assert_golden(
        QuantumCircuit::hunter_profile(),
        &[
            ("00", 0.39694631),
            ("01", 0.10305369),
            ("10", 0.39694631),
            ("11", 0.10305369),
        ],
    );
}

#[test]
fn patrol_profile_golden() {
    assert_golden(
        QuantumCircuit::patrol_profile(),
        &[
            ("00", 0.15634835),
            ("01", 0.15634835),
            ("10", 0.34365165),
            ("11", 0.34365165),
        ],
    );
}

#[test]
fn observation_pulse_golden() {
    assert_golden(
        QuantumCircuit::observation_pulse(),
        &[
            ("00", 0.34365165),
            ("01", 0.15634835),
            ("10", 0.15634835),
            ("11", 0.34365165),
        ],
    );
}

#[test]
fn shard_stabilizer_golden() {
    assert_golden(
        QuantumCircuit::shard_stabilizer(),
        &[
            ("00", 0.44700269),
            ("01", 0.05299731),
            ("10", 0.05299731),
            ("11", 0.44700269),
        ],
    );
}

#[test]
fn teleporter_golden() {
    assert_golden(
        QuantumCircuit::teleporter(),
        &[
            ("000", 0.17979639),
            ("001", 0.17979639),
            ("010", 0.07020361),
            ("011", 0.07020361),
            ("100", 0.07020361),
            ("101", 0.07020361),
            ("110", 0.17979639),
            ("111", 0.17979639),
        ],
    );
}

#[test]
fn golden_bitstrings_follow_qiskit_index_order() {
    for circuit in [
        QuantumCircuit::imp_brain(),
        QuantumCircuit::hunter_profile(),
        QuantumCircuit::teleporter(),
    ] {
        let width = circuit.qubits as usize;
        let probs = rustqip_probabilities(&circuit).expect("rustqip");
        for (index, (bits, _)) in probs.iter().enumerate() {
            assert_eq!(
                bits,
                &format!("{index:0width$b}"),
                "{}: index {index}",
                circuit.label
            );
        }
    }
}

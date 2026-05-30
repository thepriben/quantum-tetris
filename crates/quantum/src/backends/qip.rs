//! QIP / Rust in-process backend (default).
//!
//! Sprint 3 replaces the uniform stub with real [qip](https://crates.io/crates/qip) simulation.

use crate::{Measurement, QuantumBackend, QuantumCircuit, QuantumError};

/// Embedded Rust quantum backend — default for desktop and WASM builds.
#[derive(Debug, Default)]
pub struct QipBackend;

impl QuantumBackend for QipBackend {
    fn run(&mut self, circuit: &QuantumCircuit) -> Result<Measurement, QuantumError> {
        if circuit.qubits == 0 || circuit.qubits > 8 {
            return Err(QuantumError::UnsupportedQubits(circuit.qubits));
        }
        // Sprint 3: compile `circuit.gates` to qip and sample once.
        Ok(Measurement::uniform_superposition(circuit.qubits))
    }
}

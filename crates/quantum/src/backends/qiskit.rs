//! Qiskit + Aer via Python subprocess (local dev and GitHub Actions).

use crate::python_shim;
use crate::{Measurement, QuantumBackend, QuantumCircuit, QuantumError};

/// Runs circuits through `scripts/quantum_shim.py` with Aer.
#[derive(Debug, Default)]
pub struct QiskitBackend;

impl QuantumBackend for QiskitBackend {
    fn run(&mut self, circuit: &QuantumCircuit) -> Result<Measurement, QuantumError> {
        python_shim::run_qiskit(circuit)
    }
}

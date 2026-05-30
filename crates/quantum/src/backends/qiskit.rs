//! Local Qiskit backend via Python subprocess (Sprint 8).
//!
//! Planned interface: JSON circuit in stdin, measurement JSON on stdout.

use crate::{Measurement, QuantumBackend, QuantumCircuit, QuantumError};

/// Runs circuits through a local `python3` + Qiskit helper script.
#[derive(Debug)]
pub struct QiskitBackend {
    python: String,
}

impl QiskitBackend {
    /// Build backend using `QISKIT_PYTHON` or `python3`.
    pub fn new() -> Self {
        let python = std::env::var("QISKIT_PYTHON").unwrap_or_else(|_| "python3".into());
        Self { python }
    }
}

impl QuantumBackend for QiskitBackend {
    fn run(&mut self, circuit: &QuantumCircuit) -> Result<Measurement, QuantumError> {
        let _ = (&self.python, circuit);
        Err(QuantumError::BackendFailure {
            backend: "qiskit",
            message: "Sprint 8: run scripts/qiskit_shim.py via subprocess".into(),
        })
    }
}

impl Default for QiskitBackend {
    fn default() -> Self {
        Self::new()
    }
}

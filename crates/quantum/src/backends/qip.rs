//! QIP / Rust in-process backend (default).

use crate::qip_runner;
use crate::{Measurement, QuantumBackend, QuantumCircuit, QuantumError};

/// Embedded Rust quantum backend — default for desktop and WASM builds.
#[derive(Debug, Default)]
pub struct QipBackend;

impl QuantumBackend for QipBackend {
    fn run(&mut self, circuit: &QuantumCircuit) -> Result<Measurement, QuantumError> {
        qip_runner::run_qip(circuit)
    }
}

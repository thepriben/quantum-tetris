//! BlueQubit remote REST backend (Sprint 8).
//!
//! Requires `BLUEQUBIT_API_KEY` in the environment.

use crate::{Measurement, QuantumBackend, QuantumCircuit, QuantumError};

/// Cloud quantum backend using the BlueQubit HTTP API.
#[derive(Debug, Default)]
pub struct BlueQubitBackend;

impl QuantumBackend for BlueQubitBackend {
    fn run(&mut self, circuit: &QuantumCircuit) -> Result<Measurement, QuantumError> {
        let _key = std::env::var("BLUEQUBIT_API_KEY").map_err(|_| {
            QuantumError::Config("BLUEQUBIT_API_KEY is not set".into())
        })?;
        let _ = circuit;
        Err(QuantumError::BackendFailure {
            backend: "bluequbit",
            message: "Sprint 8: POST circuit JSON to BlueQubit API".into(),
        })
    }
}

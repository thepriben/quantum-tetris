//! # quantum-town-quantum
//!
//! Quantum simulation layer for **Quantum Sub: LA**.
//!
//! This crate defines a small circuit IR, measurement results, and a
//! [`QuantumBackend`] trait with three adapters:
//!
//! | Backend | Feature | Description |
//! | --- | --- | --- |
//! | Classic | always | Uniform `rand` baseline |
//! | QIP | `backend-qip` | In-process [qip](https://crates.io/crates/qip) quantum runtime |
//!
//! ## Example
//!
//! ```rust
//! use quantum_town_quantum::{BackendKind, QuantumBackend, QuantumCircuit, build_backend};
//!
//! let mut backend = build_backend(BackendKind::Qip).unwrap();
//! let circuit = QuantumCircuit::imp_brain();
//! let measurement = backend.run(&circuit).unwrap();
//! println!("Imp behavior bits: {}", measurement.bits);
//! ```

pub mod backends;
pub mod circuit;
pub mod error;
pub mod measurement;

#[cfg(feature = "backend-qiskit")]
mod python_shim;
#[cfg(feature = "backend-qip")]
mod qip_runner;

#[cfg(feature = "backend-qip")]
pub use backends::QipBackend;
#[cfg(feature = "backend-qiskit")]
pub use backends::QiskitBackend;
pub use backends::{build_backend, BackendKind, ClassicBackend};
pub use circuit::{Gate, QuantumCircuit};
pub use error::QuantumError;
pub use measurement::{EnemyBehavior, Measurement};

/// Pluggable quantum execution surface used by the game crate.
pub trait QuantumBackend: Send {
    /// Run `circuit` and return shot results (single shot for gameplay ticks).
    fn run(&mut self, circuit: &QuantumCircuit) -> Result<Measurement, QuantumError>;
}

impl QuantumBackend for Box<dyn QuantumBackend> {
    fn run(&mut self, circuit: &QuantumCircuit) -> Result<Measurement, QuantumError> {
        (**self).run(circuit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preset_circuits_run_on_qip() {
        let mut backend = build_backend(BackendKind::Qip).expect("qip");
        assert!(backend.run(&QuantumCircuit::imp_brain()).is_ok());
        assert!(backend.run(&QuantumCircuit::teleporter()).is_ok());
    }
}

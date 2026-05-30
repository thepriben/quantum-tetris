//! # quantum-tetris-quantum
//!
//! Quantum simulation layer for **Quantum Tetris**.
//!
//! Small circuit IR, measurement results, and [`QuantumBackend`] adapters:
//!
//! | Backend | Platform | Description |
//! | --- | --- | --- |
//! | Classic | all | Uniform random baseline |
//! | Quantum | desktop + `backend-qiskit` | Qiskit Aer via `scripts/quantum_shim.py` |
//! | Quantum | WASM / no Python | [RustQIP](https://github.com/Renmusxd/RustQIP) statevector (Qiskit-matched) |
//!
//! ## Example
//!
//! ```rust
//! use quantum_tetris_quantum::{BackendKind, QuantumBackend, QuantumCircuit, build_backend};
//!
//! let mut backend = build_backend(BackendKind::Classic).unwrap();
//! let circuit = QuantumCircuit::imp_brain();
//! let measurement = backend.run(&circuit).unwrap();
//! println!("Imp brain bits: {}", measurement.bits);
//! ```

pub mod backends;
pub mod circuit;
pub mod error;
pub mod measurement;

#[cfg(feature = "backend-qiskit")]
mod python_shim;

#[cfg(feature = "backend-qiskit")]
pub use backends::QiskitBackend;
pub use backends::{
    build_backend, rustqip_probabilities, BackendKind, ClassicBackend, RustQipBackend,
};
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
    fn preset_circuits_run_on_classic() {
        let mut backend = build_backend(BackendKind::Classic).expect("classic");
        assert!(backend.run(&QuantumCircuit::imp_brain()).is_ok());
        assert!(backend.run(&QuantumCircuit::teleporter()).is_ok());
    }

    #[test]
    fn preset_circuits_run_on_rustqip() {
        let mut backend = RustQipBackend;
        assert!(backend.run(&QuantumCircuit::imp_brain()).is_ok());
        assert!(backend.run(&QuantumCircuit::teleporter()).is_ok());
    }
}

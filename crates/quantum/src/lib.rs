//! # quantum-doom-quantum
//!
//! Quantum simulation layer for **[Quantum Town: LA](https://github.com/thepriben/quantum-town-la)**.
//!
//! This crate defines a small circuit IR, measurement results, and a
//! [`QuantumBackend`] trait with three adapters:
//!
//! | Backend | Feature | Description |
//! | --- | --- | --- |
//! | QIP stub | `backend-qip` (default) | Uniform superposition placeholder until Sprint 3 wires [qip](https://crates.io/crates/qip) |
//! | Qiskit | `backend-qiskit` | Local Python subprocess (Sprint 8) |
//! | BlueQubit | `backend-bluequbit` | Remote HTTPS API (Sprint 8) |
//!
//! ## Example
//!
//! ```rust
//! use quantum_doom_quantum::{BackendKind, QuantumBackend, QuantumCircuit, build_backend};
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

pub use backends::{BackendKind, BlueQubitBackend, QipBackend, QiskitBackend, build_backend};
pub use circuit::{Gate, QuantumCircuit};
pub use error::QuantumError;
pub use measurement::{EnemyBehavior, Measurement};

/// Pluggable quantum execution surface used by the game crate.
pub trait QuantumBackend: Send {
    /// Run `circuit` and return shot results (single shot for gameplay ticks).
    fn run(&mut self, circuit: &QuantumCircuit) -> Result<Measurement, QuantumError>;
}

//! Backend adapters selected at runtime via [`BackendKind`].

mod bluequbit;
mod qip;
mod qiskit;

pub use bluequbit::BlueQubitBackend;
pub use qip::QipBackend;
pub use qiskit::QiskitBackend;

use crate::{QuantumBackend, QuantumError};

/// Supported quantum execution backends for Quantum Town: LA.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    /// In-process Rust simulator ([qip](https://crates.io/crates/qip) — Sprint 3).
    Qip,
    /// Local Python Qiskit subprocess (Sprint 8).
    Qiskit,
    /// BlueQubit cloud REST API (Sprint 8).
    BlueQubit,
}

impl BackendKind {
    /// Parse `QUANTUM_BACKEND` environment variable.
    pub fn from_env() -> Self {
        match std::env::var("QUANTUM_BACKEND")
            .unwrap_or_else(|_| "qip".into())
            .to_ascii_lowercase()
            .as_str()
        {
            "qiskit" => Self::Qiskit,
            "bluequbit" | "blue-qubit" => Self::BlueQubit,
            _ => Self::Qip,
        }
    }
}

/// Construct a boxed backend from kind.
pub fn build_backend(kind: BackendKind) -> Result<Box<dyn QuantumBackend>, QuantumError> {
    match kind {
        BackendKind::Qip => Ok(Box::new(QipBackend::default())),
        BackendKind::Qiskit => Ok(Box::new(QiskitBackend::default())),
        BackendKind::BlueQubit => Ok(Box::new(BlueQubitBackend::default())),
    }
}

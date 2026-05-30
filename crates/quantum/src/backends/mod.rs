//! Backend adapters: **classic** random baseline and **quantum** QIP runtime.

mod classic;
#[cfg(feature = "backend-qip")]
mod qip;
#[cfg(feature = "backend-qiskit")]
mod qiskit;

pub use classic::ClassicBackend;
#[cfg(feature = "backend-qip")]
pub use qip::QipBackend;
#[cfg(feature = "backend-qiskit")]
pub use qiskit::QiskitBackend;

use crate::{QuantumBackend, QuantumError};

/// Supported backends for Quantum Sub: LA.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    /// Uniform random bits: a classical baseline, not a quantum simulator.
    Classic,
    /// Rust [qip](https://crates.io/crates/qip) simulator: desktop and WASM.
    Qip,
    /// Python Qiskit/Aer reference backend for local experiments and CI.
    #[cfg(feature = "backend-qiskit")]
    Qiskit,
}

impl BackendKind {
    /// Parse `classic` / `quantum` / `qip` (aliases).
    pub fn parse(name: &str) -> Self {
        match name.trim().to_ascii_lowercase().as_str() {
            "classic" | "random" | "rand" | "classique" => Self::Classic,
            "quantum" | "qip" => Self::Qip,
            "qiskit" => {
                #[cfg(feature = "backend-qiskit")]
                {
                    Self::Qiskit
                }
                #[cfg(not(feature = "backend-qiskit"))]
                {
                    Self::Qip
                }
            }
            _ => Self::Qip,
        }
    }

    /// `QUANTUM_MODE` defaults to `quantum`; use `classic` for quick baseline tests.
    pub fn from_env() -> Self {
        Self::parse(
            &std::env::var("QUANTUM_MODE")
                .or_else(|_| std::env::var("QUANTUM_BACKEND"))
                .unwrap_or_else(|_| "quantum".into()),
        )
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Classic => "classic (rand)",
            Self::Qip => "quantum (QIP Rust)",
            #[cfg(feature = "backend-qiskit")]
            Self::Qiskit => "quantum (Qiskit Aer)",
        }
    }
}

/// Construct a boxed backend from kind.
pub fn build_backend(kind: BackendKind) -> Result<Box<dyn QuantumBackend>, QuantumError> {
    match kind {
        BackendKind::Classic => Ok(Box::new(ClassicBackend)),
        BackendKind::Qip => build_qip(),
        #[cfg(feature = "backend-qiskit")]
        BackendKind::Qiskit => Ok(Box::new(QiskitBackend)),
    }
}

fn build_qip() -> Result<Box<dyn QuantumBackend>, QuantumError> {
    #[cfg(feature = "backend-qip")]
    {
        Ok(Box::new(QipBackend))
    }
    #[cfg(not(feature = "backend-qip"))]
    {
        Err(QuantumError::UnknownBackend(
            "qip (compile with feature backend-qip)".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_backend_names() {
        assert_eq!(BackendKind::parse("classic"), BackendKind::Classic);
        assert_eq!(BackendKind::parse("quantum"), BackendKind::Qip);
        assert_eq!(BackendKind::parse("qip"), BackendKind::Qip);
        #[cfg(feature = "backend-qiskit")]
        assert_eq!(BackendKind::parse("qiskit"), BackendKind::Qiskit);
        #[cfg(not(feature = "backend-qiskit"))]
        assert_eq!(BackendKind::parse("qiskit"), BackendKind::Qip);
        assert_eq!(BackendKind::parse("unknown"), BackendKind::Qip);
    }

    #[test]
    fn build_enabled_backends() {
        assert!(build_backend(BackendKind::Classic).is_ok());
        assert!(build_backend(BackendKind::Qip).is_ok());
    }
}

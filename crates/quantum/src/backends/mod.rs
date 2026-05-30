//! Backend adapters: **classic** uniform random and **Qiskit Aer** (Python).

mod classic;
#[cfg(feature = "backend-qiskit")]
mod qiskit;

pub use classic::ClassicBackend;
#[cfg(feature = "backend-qiskit")]
pub use qiskit::QiskitBackend;

use crate::{QuantumBackend, QuantumError};

/// Supported backends for Quantum Tetris: LA.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    /// Uniform random bitstrings — arcade baseline, WASM-friendly.
    Classic,
    /// Qiskit Aer via `scripts/quantum_shim.py` (desktop / CI).
    #[cfg(feature = "backend-qiskit")]
    Qiskit,
}

impl BackendKind {
    /// Parse `classic` or `qiskit` (`quantum` is kept as a Qiskit alias).
    pub fn parse(name: &str) -> Self {
        match name.trim().to_ascii_lowercase().as_str() {
            "classic" | "random" | "rand" => Self::Classic,
            "qiskit" | "quantum" | "aer" => {
                #[cfg(feature = "backend-qiskit")]
                {
                    Self::Qiskit
                }
                #[cfg(not(feature = "backend-qiskit"))]
                {
                    Self::Classic
                }
            }
            _ => Self::Classic,
        }
    }

    /// `QUANTUM_MODE` defaults to `classic`.
    pub fn from_env() -> Self {
        Self::parse(
            &std::env::var("QUANTUM_MODE")
                .or_else(|_| std::env::var("QUANTUM_BACKEND"))
                .unwrap_or_else(|_| "classic".into()),
        )
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Classic => "classic (uniform)",
            #[cfg(feature = "backend-qiskit")]
            Self::Qiskit => "quantum (Qiskit Aer)",
        }
    }
}

/// Construct a boxed backend from kind.
pub fn build_backend(kind: BackendKind) -> Result<Box<dyn QuantumBackend>, QuantumError> {
    match kind {
        BackendKind::Classic => Ok(Box::new(ClassicBackend)),
        #[cfg(feature = "backend-qiskit")]
        BackendKind::Qiskit => Ok(Box::new(QiskitBackend)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_backend_names() {
        assert_eq!(BackendKind::parse("classic"), BackendKind::Classic);
        #[cfg(feature = "backend-qiskit")]
        {
            assert_eq!(BackendKind::parse("qiskit"), BackendKind::Qiskit);
            assert_eq!(BackendKind::parse("quantum"), BackendKind::Qiskit);
        }
        #[cfg(not(feature = "backend-qiskit"))]
        {
            assert_eq!(BackendKind::parse("qiskit"), BackendKind::Classic);
        }
        assert_eq!(BackendKind::parse("unknown"), BackendKind::Classic);
    }

    #[test]
    fn build_classic_backend() {
        assert!(build_backend(BackendKind::Classic).is_ok());
    }
}

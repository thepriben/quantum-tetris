//! Backend adapters: **classic** uniform random, **RustQIP** statevector, and **Qiskit Aer** (Python).

mod classic;
#[cfg(feature = "backend-qiskit")]
mod qiskit;
mod rustqip;

pub use classic::ClassicBackend;
#[cfg(feature = "backend-qiskit")]
pub use qiskit::QiskitBackend;
pub use rustqip::{rustqip_probabilities, RustQipBackend};

use crate::{QuantumBackend, QuantumError};

/// Supported backends for Quantum Tetris.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    /// Uniform random bitstrings — arcade baseline.
    Classic,
    /// Born-rule distributions: Qiskit Aer on desktop, RustQIP statevector in WASM.
    Quantum,
}

impl BackendKind {
    /// Parse `classic`, `quantum`, `qiskit`, or `born`.
    pub fn parse(name: &str) -> Self {
        match name.trim().to_ascii_lowercase().as_str() {
            "classic" | "random" | "rand" => Self::Classic,
            "quantum" | "qiskit" | "aer" | "born" => Self::Quantum,
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
            Self::Quantum => {
                #[cfg(all(feature = "backend-qiskit", not(target_arch = "wasm32")))]
                {
                    "quantum (Qiskit Aer)"
                }
                #[cfg(not(all(feature = "backend-qiskit", not(target_arch = "wasm32"))))]
                {
                    "quantum (RustQIP · Qiskit-matched)"
                }
            }
        }
    }
}

/// Construct a boxed backend from kind.
pub fn build_backend(kind: BackendKind) -> Result<Box<dyn QuantumBackend>, QuantumError> {
    match kind {
        BackendKind::Classic => Ok(Box::new(ClassicBackend)),
        BackendKind::Quantum => {
            #[cfg(all(feature = "backend-qiskit", not(target_arch = "wasm32")))]
            {
                if crate::python_shim::qiskit_available() {
                    Ok(Box::new(QiskitBackend))
                } else {
                    eprintln!(
                        "[quantum] Qiskit Aer unavailable (pip install -r scripts/requirements.txt), \
                         using RustQIP simulator"
                    );
                    Ok(Box::new(RustQipBackend))
                }
            }
            #[cfg(not(all(feature = "backend-qiskit", not(target_arch = "wasm32"))))]
            {
                Ok(Box::new(RustQipBackend))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_backend_names() {
        assert_eq!(BackendKind::parse("classic"), BackendKind::Classic);
        assert_eq!(BackendKind::parse("qiskit"), BackendKind::Quantum);
        assert_eq!(BackendKind::parse("quantum"), BackendKind::Quantum);
        assert_eq!(BackendKind::parse("born"), BackendKind::Quantum);
        assert_eq!(BackendKind::parse("unknown"), BackendKind::Classic);
    }

    #[test]
    fn build_classic_backend() {
        assert!(build_backend(BackendKind::Classic).is_ok());
    }

    #[test]
    fn build_quantum_backend() {
        assert!(build_backend(BackendKind::Quantum).is_ok());
    }
}

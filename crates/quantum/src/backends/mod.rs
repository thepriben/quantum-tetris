//! Backend adapters: **classic** uniform random and **RustQIP** statevector.

mod classic;
mod rustqip;

pub use classic::ClassicBackend;
pub use rustqip::{rustqip_probabilities, RustQipBackend};

use crate::{QuantumBackend, QuantumError};

/// Supported backends for Quantum Tetris.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    /// Uniform random bitstrings — arcade baseline.
    Classic,
    /// Statevector via [RustQIP](https://github.com/Renmusxd/RustQIP) — desktop and WASM.
    Quantum,
}

impl BackendKind {
    /// Parse `classic` or `quantum` / `rustqip`.
    pub fn parse(name: &str) -> Self {
        match name.trim().to_ascii_lowercase().as_str() {
            "classic" | "random" | "rand" => Self::Classic,
            "quantum" | "rustqip" => Self::Quantum,
            _ => Self::Quantum,
        }
    }

    pub fn is_quantum(self) -> bool {
        !matches!(self, Self::Classic)
    }

    /// `QUANTUM_MODE` defaults to `quantum` (RustQIP).
    pub fn from_env() -> Self {
        Self::parse(
            &std::env::var("QUANTUM_MODE")
                .or_else(|_| std::env::var("QUANTUM_BACKEND"))
                .unwrap_or_else(|_| "quantum".into()),
        )
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Classic => "classic (uniform)",
            Self::Quantum => "quantum (RustQIP)",
        }
    }
}

/// Construct a boxed backend from kind.
pub fn build_backend(kind: BackendKind) -> Result<Box<dyn QuantumBackend>, QuantumError> {
    match kind {
        BackendKind::Classic => Ok(Box::new(ClassicBackend)),
        BackendKind::Quantum => Ok(Box::new(RustQipBackend)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_backend_names() {
        assert_eq!(BackendKind::parse("classic"), BackendKind::Classic);
        assert_eq!(BackendKind::parse("quantum"), BackendKind::Quantum);
        assert_eq!(BackendKind::parse("rustqip"), BackendKind::Quantum);
        assert_eq!(BackendKind::parse("unknown"), BackendKind::Quantum);
    }

    #[test]
    fn default_unknown_parse_is_quantum() {
        assert_eq!(BackendKind::parse("unknown"), BackendKind::Quantum);
    }

    #[test]
    fn is_quantum_excludes_classic_only() {
        assert!(!BackendKind::Classic.is_quantum());
        assert!(BackendKind::Quantum.is_quantum());
    }

    #[test]
    fn quantum_backend_is_rustqip() {
        let mut backend = build_backend(BackendKind::Quantum).expect("quantum");
        let m = backend
            .run(&crate::QuantumCircuit::imp_brain())
            .expect("run");
        assert_eq!(m.bits.len(), 2);
        assert_eq!(m.probabilities.len(), 4);
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

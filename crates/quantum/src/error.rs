//! Error types for quantum backend calls.

use thiserror::Error;

/// Errors returned when building or running a quantum backend.
#[derive(Debug, Error)]
pub enum QuantumError {
    /// The requested backend is unknown or disabled at compile time.
    #[error("unknown backend: {0}")]
    UnknownBackend(String),

    /// Circuit width is outside what the gameplay layer supports.
    #[error("unsupported qubit count: {0}")]
    UnsupportedQubits(u8),

    /// Remote or subprocess backend failed.
    #[error("{backend} backend failed: {message}")]
    BackendFailure {
        backend: &'static str,
        message: String,
    },

    /// Environment variable or API key missing.
    #[error("configuration error: {0}")]
    Config(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_messages_are_actionable() {
        let error = QuantumError::UnsupportedQubits(9);
        assert!(error.to_string().contains('9'));

        let error = QuantumError::BackendFailure {
            backend: "qip",
            message: "cnot failed".into(),
        };
        assert!(error.to_string().contains("qip"));
        assert!(error.to_string().contains("cnot"));
    }
}

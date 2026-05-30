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
    BackendFailure { backend: &'static str, message: String },

    /// Environment variable or API key missing.
    #[error("configuration error: {0}")]
    Config(String),
}

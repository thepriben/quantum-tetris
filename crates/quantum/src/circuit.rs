//! Portable quantum circuit intermediate representation.

use serde::{Deserialize, Serialize};

/// Supported gameplay gates (enough for Imp brain + teleporters).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Gate {
    /// Hadamard on `qubit`.
    H(u8),
    /// CNOT control → target.
    Cx { control: u8, target: u8 },
    /// Rotation Y for biased teleporter distributions (Sprint 5).
    Ry { qubit: u8, theta_deg: f32 },
}

/// Circuit executed by any [`crate::QuantumBackend`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumCircuit {
    /// Number of qubits allocated.
    pub qubits: u8,
    /// Gate sequence in program order.
    pub gates: Vec<Gate>,
    /// Human-readable label for logs and article screenshots.
    pub label: String,
}

impl QuantumCircuit {
    /// Standard 2-qubit Imp brain: H-H, CX, measure.
    pub fn imp_brain() -> Self {
        Self {
            qubits: 2,
            gates: vec![Gate::H(0), Gate::H(1), Gate::Cx { control: 0, target: 1 }],
            label: "imp-brain-v1".into(),
        }
    }

    /// Standard 3-qubit teleporter: H on each line, measure.
    pub fn teleporter() -> Self {
        Self {
            qubits: 3,
            gates: vec![Gate::H(0), Gate::H(1), Gate::H(2)],
            label: "schrodinger-gate-v1".into(),
        }
    }
}

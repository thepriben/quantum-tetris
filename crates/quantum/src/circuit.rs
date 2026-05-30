//! Portable quantum circuit intermediate representation.

use serde::{Deserialize, Serialize};

/// Supported gameplay gates for the small circuits used by the game.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Gate {
    /// Hadamard on `qubit`.
    H(u8),
    /// Pauli-X on `qubit`.
    X(u8),
    /// Pauli-Z on `qubit`.
    Z(u8),
    /// CNOT control → target.
    Cx { control: u8, target: u8 },
    /// Rotation Y for biased gameplay distributions.
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
    /// Balanced 2-qubit Imp brain: H-H, CX, then measure.
    pub fn imp_brain() -> Self {
        Self {
            qubits: 2,
            gates: vec![
                Gate::H(0),
                Gate::H(1),
                Gate::Cx {
                    control: 0,
                    target: 1,
                },
            ],
            label: "imp-brain-v1".into(),
        }
    }

    /// 2-qubit circuit biased toward direct pressure and pursuit.
    pub fn hunter_profile() -> Self {
        Self {
            qubits: 2,
            gates: vec![
                Gate::Ry {
                    qubit: 0,
                    theta_deg: 54.0,
                },
                Gate::H(1),
                Gate::Cx {
                    control: 0,
                    target: 1,
                },
            ],
            label: "enemy-profile-hunter-v1".into(),
        }
    }

    /// 2-qubit circuit biased toward patrol, flanking, and ambush states.
    pub fn patrol_profile() -> Self {
        Self {
            qubits: 2,
            gates: vec![
                Gate::H(0),
                Gate::Ry {
                    qubit: 1,
                    theta_deg: 112.0,
                },
                Gate::Cx {
                    control: 1,
                    target: 0,
                },
                Gate::Z(0),
            ],
            label: "enemy-profile-patrol-v1".into(),
        }
    }

    /// 2-qubit observation pulse: a deliberate measurement used by the player.
    pub fn observation_pulse() -> Self {
        Self {
            qubits: 2,
            gates: vec![
                Gate::H(0),
                Gate::Ry {
                    qubit: 1,
                    theta_deg: 68.0,
                },
                Gate::Cx {
                    control: 0,
                    target: 1,
                },
            ],
            label: "observation-pulse-v1".into(),
        }
    }

    /// 2-qubit stabilizer used when collecting a Q-Shard.
    pub fn shard_stabilizer() -> Self {
        Self {
            qubits: 2,
            gates: vec![
                Gate::H(0),
                Gate::Cx {
                    control: 0,
                    target: 1,
                },
                Gate::Ry {
                    qubit: 1,
                    theta_deg: 38.0,
                },
            ],
            label: "q-shard-stabilizer-v1".into(),
        }
    }

    /// Teleportation-inspired 3-qubit circuit: prepare a message qubit, create a
    /// Bell pair, perform the Bell-basis sender measurement, then sample all bits.
    ///
    /// The game maps the three measured bits to exits. The full classroom
    /// teleportation protocol would apply classical conditional corrections to
    /// the receiver qubit; the measured correction bits are intentionally exposed
    /// as gameplay state here.
    pub fn quantum_teleportation() -> Self {
        Self {
            qubits: 3,
            gates: vec![
                Gate::Ry {
                    qubit: 0,
                    theta_deg: 64.0,
                },
                Gate::H(1),
                Gate::Cx {
                    control: 1,
                    target: 2,
                },
                Gate::Cx {
                    control: 0,
                    target: 1,
                },
                Gate::H(0),
            ],
            label: "quantum-teleportation-gate-v1".into(),
        }
    }

    /// Backward-compatible name used by older gameplay code and tests.
    pub fn teleporter() -> Self {
        Self::quantum_teleportation()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imp_brain_has_expected_gates() {
        let circuit = QuantumCircuit::imp_brain();
        assert_eq!(circuit.qubits, 2);
        assert_eq!(circuit.label, "imp-brain-v1");
        assert_eq!(
            circuit.gates,
            vec![
                Gate::H(0),
                Gate::H(1),
                Gate::Cx {
                    control: 0,
                    target: 1
                }
            ]
        );
    }

    #[test]
    fn teleporter_has_three_h_gates() {
        let circuit = QuantumCircuit::teleporter();
        assert_eq!(circuit.qubits, 3);
        assert_eq!(circuit.label, "quantum-teleportation-gate-v1");
        assert!(circuit.gates.iter().any(|gate| matches!(gate, Gate::H(1))));
        assert!(circuit.gates.iter().any(|gate| matches!(
            gate,
            Gate::Cx {
                control: 1,
                target: 2
            }
        )));
        assert!(circuit.gates.iter().any(|gate| matches!(
            gate,
            Gate::Cx {
                control: 0,
                target: 1
            }
        )));
    }

    #[test]
    fn gameplay_decision_circuits_are_small() {
        for circuit in [
            QuantumCircuit::hunter_profile(),
            QuantumCircuit::patrol_profile(),
            QuantumCircuit::observation_pulse(),
            QuantumCircuit::shard_stabilizer(),
        ] {
            assert_eq!(circuit.qubits, 2);
            assert!(!circuit.gates.is_empty());
        }
    }

    #[test]
    fn circuit_serde_roundtrip() {
        let circuit = QuantumCircuit::imp_brain();
        let json = serde_json::to_string(&circuit).unwrap();
        let back: QuantumCircuit = serde_json::from_str(&json).unwrap();
        assert_eq!(back.qubits, circuit.qubits);
        assert_eq!(back.gates, circuit.gates);
        assert_eq!(back.label, circuit.label);
    }

    #[test]
    fn gate_serde_uses_external_tags() {
        let json = serde_json::to_string(&Gate::Cx {
            control: 0,
            target: 1,
        })
        .unwrap();
        assert!(json.contains("Cx"));
    }
}

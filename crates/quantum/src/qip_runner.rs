//! Compile [`QuantumCircuit`] IR into [qip](https://crates.io/crates/qip) and sample one shot.

use crate::circuit::Gate;
use crate::{Measurement, QuantumCircuit, QuantumError};
use qip::prelude::*;

/// Run `circuit` on the in-process QIP simulator.
pub fn run_qip(circuit: &QuantumCircuit) -> Result<Measurement, QuantumError> {
    if circuit.qubits == 0 || circuit.qubits > 8 {
        return Err(QuantumError::UnsupportedQubits(circuit.qubits));
    }

    let mut builder = LocalBuilder::<f64>::default();
    type Wire = <LocalBuilder<f64> as CircuitBuilder>::Register;
    let mut wires: Vec<Option<Wire>> = (0..circuit.qubits).map(|_| Some(builder.qubit())).collect();

    for gate in &circuit.gates {
        match *gate {
            Gate::H(qubit) => {
                let index = qubit as usize;
                let wire = wires[index]
                    .take()
                    .ok_or_else(|| QuantumError::BackendFailure {
                        backend: "qip",
                        message: format!("qubit {qubit} not allocated"),
                    })?;
                wires[index] = Some(builder.h(wire));
            }
            Gate::X(qubit) => {
                let index = qubit as usize;
                let wire = wires[index]
                    .take()
                    .ok_or_else(|| QuantumError::BackendFailure {
                        backend: "qip",
                        message: format!("qubit {qubit} not allocated"),
                    })?;
                wires[index] = Some(builder.x(wire));
            }
            Gate::Z(qubit) => {
                let index = qubit as usize;
                let wire = wires[index]
                    .take()
                    .ok_or_else(|| QuantumError::BackendFailure {
                        backend: "qip",
                        message: format!("qubit {qubit} not allocated"),
                    })?;
                wires[index] = Some(builder.z(wire));
            }
            Gate::Cx { control, target } => {
                let control_wire =
                    wires[control as usize]
                        .take()
                        .ok_or_else(|| QuantumError::BackendFailure {
                            backend: "qip",
                            message: format!("control qubit {control} not allocated"),
                        })?;
                let target_wire =
                    wires[target as usize]
                        .take()
                        .ok_or_else(|| QuantumError::BackendFailure {
                            backend: "qip",
                            message: format!("target qubit {target} not allocated"),
                        })?;
                let (control_reg, target_reg) =
                    builder.cnot(control_wire, target_wire).map_err(|error| {
                        QuantumError::BackendFailure {
                            backend: "qip",
                            message: error.to_string(),
                        }
                    })?;
                wires[control as usize] = Some(control_reg);
                wires[target as usize] = Some(target_reg);
            }
            Gate::Ry { qubit, theta_deg } => {
                let theta = f64::from(theta_deg) * std::f64::consts::PI / 180.0;
                let wire =
                    wires[qubit as usize]
                        .take()
                        .ok_or_else(|| QuantumError::BackendFailure {
                            backend: "qip",
                            message: format!("qubit {qubit} not allocated"),
                        })?;
                wires[qubit as usize] = Some(builder.ry(wire, theta));
            }
        }
    }

    let mut wire_iter = wires
        .into_iter()
        .map(|wire| {
            wire.ok_or_else(|| QuantumError::BackendFailure {
                backend: "qip",
                message: "wire missing after gate sequence".into(),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut merged = wire_iter.remove(0);
    for wire in wire_iter {
        merged = builder.merge_two_registers(merged, wire);
    }

    let (_, handle) = builder.measure_stochastic(merged);
    let (_, measurements) = builder.calculate_state();
    let probabilities = measurements.get_stochastic_measurement(handle);
    let width = circuit.qubits as usize;
    Ok(Measurement::from_probabilities(width, probabilities))
}

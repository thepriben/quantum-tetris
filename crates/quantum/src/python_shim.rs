//! Qiskit + Aer subprocess, enabled only with `backend-qiskit`.

use crate::{Measurement, QuantumCircuit, QuantumError};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Resolve the shim script: `QUANTUM_PYTHON_SHIM` or repo `scripts/quantum_shim.py`.
pub fn shim_path() -> PathBuf {
    if let Ok(path) = std::env::var("QUANTUM_PYTHON_SHIM") {
        return PathBuf::from(path);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../scripts/quantum_shim.py")
}

fn python_executable() -> String {
    std::env::var("QISKIT_PYTHON")
        .or_else(|_| std::env::var("QUANTUM_PYTHON"))
        .unwrap_or_else(|_| "python3".into())
}

/// Run `circuit` through `scripts/quantum_shim.py` (Qiskit Aer).
pub fn run_qiskit(circuit: &QuantumCircuit) -> Result<Measurement, QuantumError> {
    const BACKEND: &str = "qiskit";
    let script = shim_path();
    if !script.is_file() {
        return Err(QuantumError::Config(format!(
            "Python shim not found at {} (set QUANTUM_PYTHON_SHIM)",
            script.display()
        )));
    }

    let payload = serde_json::json!({
        "backend": BACKEND,
        "circuit": circuit,
    });

    let mut child = Command::new(python_executable())
        .arg(&script)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(spawn_error)?;

    if let Some(mut stdin) = child.stdin.take() {
        let bytes = serde_json::to_vec(&payload).map_err(|error| QuantumError::BackendFailure {
            backend: BACKEND,
            message: error.to_string(),
        })?;
        stdin
            .write_all(&bytes)
            .map_err(|error| QuantumError::BackendFailure {
                backend: BACKEND,
                message: format!("stdin write failed: {error}"),
            })?;
    }

    let output = child.wait_with_output().map_err(spawn_error)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(QuantumError::BackendFailure {
            backend: BACKEND,
            message: if stderr.is_empty() {
                format!("process exited with {}", output.status)
            } else {
                stderr.trim().to_string()
            },
        });
    }

    serde_json::from_slice(&output.stdout).map_err(|error| QuantumError::BackendFailure {
        backend: BACKEND,
        message: format!("invalid JSON from shim: {error}"),
    })
}

fn spawn_error(error: std::io::Error) -> QuantumError {
    QuantumError::BackendFailure {
        backend: "qiskit",
        message: format!("failed to spawn Python ({}): {error}", python_executable()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::QuantumCircuit;

    #[test]
    fn shim_path_points_at_repo_script() {
        let path = shim_path();
        assert!(path.ends_with("scripts/quantum_shim.py"));
        assert!(path.is_file());
    }

    #[test]
    fn payload_serializes_circuit_for_shim() {
        let circuit = QuantumCircuit::imp_brain();
        let payload = serde_json::json!({
            "backend": "qiskit",
            "circuit": circuit,
        });
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("imp-brain-v1"));
        assert!(json.contains("\"H\""));
    }
}

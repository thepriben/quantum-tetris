//! Qiskit integration — runs on GitHub Actions with Aer (see ci.yml).

use quantum_tetris_quantum::{build_backend, BackendKind, QuantumBackend, QuantumCircuit};
use std::process::Command;

fn python_ready() -> bool {
    if std::env::var("QUANTUM_SKIP_PYTHON_TESTS").is_ok() {
        return false;
    }
    let python = std::env::var("QUANTUM_PYTHON").unwrap_or_else(|_| "python3".into());
    Command::new(&python)
        .args(["-c", "import qiskit, qiskit_aer"])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

#[test]
fn qiskit_runs_imp_brain() {
    if !python_ready() {
        eprintln!("skip: pip install -r scripts/requirements.txt");
        return;
    }
    let mut backend = build_backend(BackendKind::Quantum).expect("qiskit backend");
    let measurement = backend
        .run(&QuantumCircuit::imp_brain())
        .expect("qiskit run");
    assert_eq!(measurement.bits.len(), 2);
    assert_eq!(measurement.probabilities.len(), 4);
    let total: f32 = measurement.probabilities.iter().map(|(_, p)| p).sum();
    assert!((total - 1.0).abs() < 0.05);
}

#[test]
fn qiskit_runs_teleporter() {
    if !python_ready() {
        return;
    }
    let mut backend = build_backend(BackendKind::Quantum).expect("qiskit backend");
    let measurement = backend
        .run(&QuantumCircuit::teleporter())
        .expect("teleporter");
    assert_eq!(measurement.bits.len(), 3);
    assert_eq!(measurement.probabilities.len(), 8);
}

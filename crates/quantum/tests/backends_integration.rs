use quantum_tetris_quantum::{build_backend, BackendKind, QuantumBackend, QuantumCircuit};

#[test]
fn box_dyn_backend_delegates_run() {
    let mut backend: Box<dyn QuantumBackend> =
        build_backend(BackendKind::Classic).expect("classic");
    let measurement = backend
        .run(&QuantumCircuit::imp_brain())
        .expect("run via trait object");
    assert_eq!(measurement.bits.len(), 2);
}

#[test]
fn build_backend_matches_env_parse() {
    assert!(build_backend(BackendKind::parse("classic")).is_ok());
    assert!(build_backend(BackendKind::parse("quantum")).is_ok());
    #[cfg(all(feature = "backend-qiskit", not(target_arch = "wasm32")))]
    {
        let qiskit = build_backend(BackendKind::parse("qiskit"));
        let python = std::env::var("QUANTUM_PYTHON").unwrap_or_else(|_| "python3".into());
        let has_aer = std::process::Command::new(python)
            .args(["-c", "import qiskit, qiskit_aer"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if has_aer {
            assert!(qiskit.is_ok(), "qiskit backend with Aer installed");
        } else {
            assert!(qiskit.is_err(), "qiskit backend without Python should fail");
        }
    }
}

#[test]
fn teleporter_three_qubits() {
    let mut backend = build_backend(BackendKind::Classic).expect("classic");
    let measurement = backend
        .run(&QuantumCircuit::teleporter())
        .expect("teleporter");
    assert_eq!(measurement.bits.len(), 3);
}

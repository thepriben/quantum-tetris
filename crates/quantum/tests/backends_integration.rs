use quantum_town_quantum::{build_backend, BackendKind, QuantumBackend, QuantumCircuit};

#[test]
fn box_dyn_backend_delegates_run() {
    let mut backend: Box<dyn QuantumBackend> = build_backend(BackendKind::Qip).expect("qip");
    let measurement = backend
        .run(&QuantumCircuit::imp_brain())
        .expect("run via trait object");
    assert_eq!(measurement.bits.len(), 2);
}

#[test]
fn build_backend_matches_env_parse() {
    assert!(build_backend(BackendKind::parse("quantum")).is_ok());
    assert!(build_backend(BackendKind::parse("classic")).is_ok());
}

#[test]
fn teleporter_three_qubits_on_qip() {
    let mut backend = build_backend(BackendKind::Qip).expect("qip");
    let measurement = backend
        .run(&QuantumCircuit::teleporter())
        .expect("teleporter");
    assert_eq!(measurement.bits.len(), 3);
}

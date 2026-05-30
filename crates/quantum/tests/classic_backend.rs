use quantum_town_quantum::{build_backend, BackendKind, QuantumBackend, QuantumCircuit};

#[test]
fn backend_parse_aliases() {
    assert_eq!(BackendKind::parse("classic"), BackendKind::Classic);
    assert_eq!(BackendKind::parse("quantum"), BackendKind::Qip);
    assert_eq!(BackendKind::parse(""), BackendKind::Qip);
}

#[test]
fn classic_runs_many_times() {
    let mut backend = build_backend(BackendKind::Classic).expect("classic");
    for _ in 0..32 {
        let measurement = backend.run(&QuantumCircuit::imp_brain()).expect("run");
        assert_eq!(measurement.bits.len(), 2);
    }
}

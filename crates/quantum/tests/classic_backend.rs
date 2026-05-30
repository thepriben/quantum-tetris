use quantum_tetris_quantum::{BackendKind, ClassicBackend, QuantumBackend, QuantumCircuit};

#[test]
fn backend_parse_aliases() {
    assert_eq!(BackendKind::parse("classic"), BackendKind::Classic);
    assert_eq!(BackendKind::parse("random"), BackendKind::Classic);
    assert_eq!(BackendKind::parse("qiskit"), BackendKind::Quantum);
    assert_eq!(BackendKind::parse("born"), BackendKind::Quantum);
}

#[test]
fn classic_runs_many_times() {
    let mut backend = ClassicBackend;
    for _ in 0..32 {
        let m = backend
            .run(&QuantumCircuit::imp_brain())
            .expect("classic run");
        assert_eq!(m.bits.len(), 2);
    }
}

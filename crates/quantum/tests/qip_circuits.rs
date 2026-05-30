use quantum_town_quantum::{
    build_backend, BackendKind, EnemyBehavior, QuantumBackend, QuantumCircuit, QuantumError,
};

fn qip() -> Box<dyn QuantumBackend> {
    build_backend(BackendKind::Qip).expect("qip backend")
}

#[test]
fn imp_brain_yields_valid_two_bit_strings() {
    let mut backend = qip();
    for _ in 0..8 {
        let measurement = backend
            .run(&QuantumCircuit::imp_brain())
            .expect("imp brain run");
        assert_eq!(measurement.bits.len(), 2);
        assert!(measurement.bits.chars().all(|c| c == '0' || c == '1'));
        assert!(EnemyBehavior::from_bits(&measurement.bits).is_some());
    }
}

#[test]
fn imp_brain_histogram_is_uniform_bell_state() {
    let mut backend = qip();
    let measurement = backend
        .run(&QuantumCircuit::imp_brain())
        .expect("imp brain run");
    assert_eq!(measurement.probabilities.len(), 4);
    for (_, probability) in &measurement.probabilities {
        assert!(
            (*probability - 0.25).abs() < 0.02,
            "expected ~0.25, got {probability}"
        );
    }
}

#[test]
fn teleporter_yields_eight_outcomes() {
    let mut backend = qip();
    let measurement = backend
        .run(&QuantumCircuit::quantum_teleportation())
        .expect("quantum teleportation run");
    assert_eq!(measurement.bits.len(), 3);
    assert_eq!(measurement.probabilities.len(), 8);
    let total: f32 = measurement.probabilities.iter().map(|(_, p)| p).sum();
    assert!((total - 1.0).abs() < 0.01);

    let has_bias = measurement
        .probabilities
        .iter()
        .any(|(_, probability)| (*probability - 0.125).abs() > 0.02);
    assert!(
        has_bias,
        "teleportation circuit should expose a biased message qubit"
    );
}

#[test]
fn two_qubit_gameplay_decision_circuits_run_on_qip() {
    let mut backend = qip();
    for circuit in [
        QuantumCircuit::hunter_profile(),
        QuantumCircuit::patrol_profile(),
        QuantumCircuit::observation_pulse(),
        QuantumCircuit::shard_stabilizer(),
    ] {
        let measurement = backend.run(&circuit).expect("decision circuit run");
        assert_eq!(measurement.bits.len(), 2);
        assert_eq!(measurement.probabilities.len(), 4);
    }
}

#[test]
fn rejects_zero_qubits() {
    let mut backend = qip();
    let circuit = QuantumCircuit {
        qubits: 0,
        gates: vec![],
        label: "empty".into(),
    };
    let error = backend.run(&circuit).expect_err("0 qubits");
    assert!(matches!(error, QuantumError::UnsupportedQubits(0)));
}

#[test]
fn rejects_more_than_eight_qubits() {
    let mut backend = qip();
    let circuit = QuantumCircuit {
        qubits: 9,
        gates: vec![quantum_town_quantum::Gate::H(0)],
        label: "too-wide".into(),
    };
    let error = backend.run(&circuit).expect_err("9 qubits");
    assert!(matches!(error, QuantumError::UnsupportedQubits(9)));
}
